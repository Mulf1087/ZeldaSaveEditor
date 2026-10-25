use std::sync::{Mutex, MutexGuard, PoisonError};

use save_engine::Save;
use tauri_plugin_fs::FilePath;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ArchiveSource {
    pub path: PathBuf,
    pub entry: String,
}

#[derive(Default)]
pub struct AppState {
    pub save: Mutex<Option<Save>>,
    pub path: Mutex<Option<FilePath>>,
    pub archive_source: Mutex<Option<ArchiveSource>>,
}

/// Locks a Mutex, recovering the inner value instead of panicking if a previous holder panicked
/// while holding the lock. A poisoned AppState lock would otherwise break every command for the
/// rest of the process; the guarded value itself is still perfectly usable after a poison.
pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}
