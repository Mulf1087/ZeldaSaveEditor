//! Local save discovery and archive import/export for the MacNativeGames installation.
//! Archive edits are always exported as a new ZIP; the supplied ZIP is never rewritten.

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::State;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::dto::OpenResult;
use crate::error::ShellError;
use crate::state::{lock, AppState, ArchiveSource};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocatedSave {
    pub source: &'static str,
    pub game: &'static str,
    pub label: String,
    pub path: String,
    pub entry: Option<String>,
}

fn home() -> Result<PathBuf, ShellError> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| ShellError::io(io::Error::new(io::ErrorKind::NotFound, "HOME is unavailable")))
}

fn zip_io(error: zip::result::ZipError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

fn find_live_saves(root: &Path, game: &'static str, file_name: &str, saves: &mut Vec<LocatedSave>) {
    let Ok(profiles) = fs::read_dir(root) else { return };
    for profile in profiles.flatten() {
        let Ok(slots) = fs::read_dir(profile.path()) else { continue };
        for slot in slots.flatten() {
            let slot_name = slot.file_name().to_string_lossy().into_owned();
            if !(slot_name.starts_with("slot_") || slot_name.parse::<u8>().is_ok()) {
                continue;
            }
            let path = slot.path().join(file_name);
            if path.is_file() {
                saves.push(LocatedSave {
                    source: "live",
                    game,
                    label: format!("{} · {} · 当前游戏存档", if game == "botw" { "旷野之息" } else { "王国之泪" }, slot_name),
                    path: path.to_string_lossy().into_owned(),
                    entry: None,
                });
            }
        }
    }
}

fn find_archive_saves(games: &Path, saves: &mut Vec<LocatedSave>) {
    let Ok(files) = fs::read_dir(games) else { return };
    for item in files.flatten() {
        let path = item.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else { continue };
        if !name.ends_with(".zip") || !name.contains("存档") { continue; }
        let Ok(file) = File::open(&path) else { continue };
        let Ok(mut archive) = ZipArchive::new(file) else { continue };
        for index in 0..archive.len() {
            let Ok(entry) = archive.by_index(index) else { continue };
            let (game, slot) = if entry.name().ends_with("/game_data.sav") {
                ("botw", entry.name().rsplit('/').nth(1).unwrap_or("?"))
            } else if entry.name().ends_with("/progress.sav") {
                ("totk", entry.name().rsplit('/').nth(1).unwrap_or("?"))
            } else { continue };
            if entry.size() > 8 * 1024 * 1024 { continue; }
            saves.push(LocatedSave {
                source: "archive",
                game,
                label: format!("{} · {} · {}", if game == "botw" { "旷野之息" } else { "王国之泪" }, slot, name),
                path: path.to_string_lossy().into_owned(),
                entry: Some(entry.name().to_string()),
            });
        }
    }
}

#[tauri::command]
pub fn discover_local_saves() -> Result<Vec<LocatedSave>, ShellError> {
    let home = home()?;
    let native = home.join("Library/Application Support/MacNativeGames");
    let mut saves = Vec::new();
    find_live_saves(&native.join("BotW/profile/save"), "botw", "game_data.sav", &mut saves);
    find_live_saves(&native.join("TOTK/profile/save"), "totk", "progress.sav", &mut saves);
    find_archive_saves(&home.join("Documents/Games"), &mut saves);
    saves.sort_by(|a, b| (a.game, a.source, &a.label).cmp(&(b.game, b.source, &b.label)));
    Ok(saves)
}

#[tauri::command]
pub fn open_archive_save(
    path: String,
    entry: String,
    state: State<'_, AppState>,
) -> Result<OpenResult, ShellError> {
    let source_path = PathBuf::from(path);
    let mut archive = ZipArchive::new(File::open(&source_path).map_err(ShellError::io)?)
        .map_err(|error| ShellError::io(zip_io(error)))?;
    let mut member = archive.by_name(&entry).map_err(|error| ShellError::io(zip_io(error)))?;
    if member.size() > 8 * 1024 * 1024 {
        return Err(ShellError::io(io::Error::new(io::ErrorKind::InvalidData, "save file is too large")));
    }
    let mut bytes = Vec::with_capacity(member.size() as usize);
    member.read_to_end(&mut bytes).map_err(ShellError::io)?;
    let (save, result) = crate::commands::file::detect_and_build(bytes)?;
    let save = save.ok_or_else(|| ShellError::io(io::Error::new(io::ErrorKind::InvalidData, "archive entry is not an editable save")))?;
    *lock(&state.save) = Some(save);
    *lock(&state.path) = None;
    *lock(&state.archive_source) = Some(ArchiveSource { path: source_path, entry });
    Ok(result)
}

fn copy_archive_with_replacement(source: &ArchiveSource, output: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut archive = ZipArchive::new(File::open(&source.path)?).map_err(zip_io)?;
    let mut writer = ZipWriter::new(File::create(output)?);
    let mut replaced = false;
    for index in 0..archive.len() {
        let mut member = archive.by_index(index).map_err(zip_io)?;
        let options = FileOptions::default()
            .compression_method(member.compression())
            .last_modified_time(member.last_modified());
        let name = member.name().to_string();
        if member.is_dir() {
            writer.add_directory(name, options).map_err(zip_io)?;
        } else {
            writer.start_file(name.clone(), options).map_err(zip_io)?;
            if name == source.entry {
                writer.write_all(bytes)?;
                replaced = true;
            } else {
                io::copy(&mut member, &mut writer)?;
            }
        }
    }
    if !replaced {
        return Err(io::Error::new(io::ErrorKind::NotFound, "selected save disappeared from archive"));
    }
    writer.finish().map_err(zip_io)?.sync_all()?;
    Ok(())
}

#[tauri::command]
pub fn export_archive_copy(state: State<'_, AppState>) -> Result<String, ShellError> {
    let source = lock(&state.archive_source).clone().ok_or_else(ShellError::no_save_loaded)?;
    let output_dir = home()?.join("Documents/Games/存档编辑器输出");
    fs::create_dir_all(&output_dir).map_err(ShellError::io)?;
    let stem = source.path.file_stem().and_then(|s| s.to_str()).unwrap_or("存档");
    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let mut counter = 0;
    let output = loop {
        let suffix = if counter == 0 { String::new() } else { format!("-{counter}") };
        let candidate = output_dir.join(format!("{stem}-已编辑-{seconds}{suffix}.zip"));
        if !candidate.exists() { break candidate; }
        counter += 1;
    };
    let temp = output.with_extension("zip.tmp");
    let write_result = crate::commands::file::write_current_save_with(state.inner(), |bytes| {
        copy_archive_with_replacement(&source, &temp, bytes)?;
        fs::rename(&temp, &output)
    });
    if write_result.is_err() { let _ = fs::remove_file(&temp); }
    write_result?;
    Ok(output.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use save_engine::Save;

    #[test]
    fn archive_copy_changes_only_selected_member() {
        let base = std::env::temp_dir().join(format!("zelda_archive_test_{}", std::process::id()));
        fs::create_dir_all(&base).unwrap();
        let original = base.join("source.zip");
        let edited = base.join("edited.zip");
        {
            let mut writer = ZipWriter::new(File::create(&original).unwrap());
            writer.start_file("save/slot_00/progress.sav", FileOptions::default()).unwrap();
            writer.write_all(b"old").unwrap();
            writer.start_file("save/option.sav", FileOptions::default()).unwrap();
            writer.write_all(b"untouched").unwrap();
            writer.finish().unwrap();
        }
        let source = ArchiveSource { path: original.clone(), entry: "save/slot_00/progress.sav".to_string() };
        copy_archive_with_replacement(&source, &edited, b"new").unwrap();
        let mut before = ZipArchive::new(File::open(original).unwrap()).unwrap();
        let mut after = ZipArchive::new(File::open(edited).unwrap()).unwrap();
        let mut edited_bytes = Vec::new();
        after.by_name("save/slot_00/progress.sav").unwrap().read_to_end(&mut edited_bytes).unwrap();
        assert_eq!(edited_bytes, b"new");
        let mut other = Vec::new();
        after.by_name("save/option.sav").unwrap().read_to_end(&mut other).unwrap();
        assert_eq!(other, b"untouched");
        let mut original_bytes = Vec::new();
        before.by_name("save/slot_00/progress.sav").unwrap().read_to_end(&mut original_bytes).unwrap();
        assert_eq!(original_bytes, b"old");
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn local_archives_export_an_edit_without_touching_sources() {
        let Ok(list_path) = std::env::var("ZELDA_TEST_ARCHIVE_LIST") else { return };
        let list = fs::read_to_string(list_path).unwrap();
        let temp = std::env::temp_dir().join(format!("zelda_archive_local_{}", std::process::id()));
        fs::create_dir_all(&temp).unwrap();
        for (index, line) in list.lines().filter(|line| !line.is_empty()).enumerate() {
            let (path, entry) = line.split_once('\t').expect("path and entry separated by tab");
            let source = ArchiveSource { path: PathBuf::from(path), entry: entry.to_string() };
            let original_zip = fs::read(path).unwrap();
            let mut before = ZipArchive::new(File::open(path).unwrap()).unwrap();
            let mut original_save = Vec::new();
            before.by_name(entry).unwrap().read_to_end(&mut original_save).unwrap();
            let mut save = Save::detect(original_save.clone()).unwrap();
            let expected_rupees = match &mut save {
                Save::Botw(save) => {
                    let value = save.rupees().unwrap() + 1;
                    save.set_rupees(value).unwrap();
                    value
                }
                Save::Totk(save) => {
                    let value = save.current_rupees().unwrap() + 1;
                    save.set_current_rupees(value).unwrap();
                    value
                }
            };
            let output = temp.join(format!("edited-{index}.zip"));
            copy_archive_with_replacement(&source, &output, &save.to_bytes()).unwrap();
            let mut after = ZipArchive::new(File::open(output).unwrap()).unwrap();
            assert_eq!(before.len(), after.len());
            for file_index in 0..before.len() {
                let mut old = Vec::new();
                let mut new = Vec::new();
                let mut member = before.by_index(file_index).unwrap();
                let name = member.name().to_string();
                member.read_to_end(&mut old).unwrap();
                after.by_name(&name).unwrap().read_to_end(&mut new).unwrap();
                if name == entry {
                    assert_ne!(new, old);
                    let rupees = match Save::detect(new).unwrap() {
                        Save::Botw(save) => save.rupees().unwrap(),
                        Save::Totk(save) => save.current_rupees().unwrap(),
                    };
                    assert_eq!(rupees, expected_rupees);
                } else {
                    assert_eq!(new, old, "unselected archive entry changed: {name}");
                }
            }
            assert_eq!(fs::read(path).unwrap(), original_zip, "source ZIP changed: {path}");
        }
        fs::remove_dir_all(temp).unwrap();
    }
}
