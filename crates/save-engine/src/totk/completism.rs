//! Completionism counts and mass-set actions for every tracked category (towers, shrines,
//! lightroots, koroks, locations/caves/wells/chasms, bosses, schematics, compendium, and the
//! GUID-based bubbuls/sage wills/Addison).
//!
//! All category data (flag hashes and discovery GUIDs) lives in [`completism_data`], generated
//! directly from the source's `CompletismHashes`. Flag categories are counted by matching a
//! stored value against an expected one and mass-set by writing that value to every present
//! flag (`Completism._count`/`_set`); GUID categories are counted against the save's discovered
//! list and mass-set by appending the missing GUIDs (`_countGuids`/`_setGuids`).
//!
//! `_set` only touches flags whose hash is already present in the save, exactly like the source's
//! `_getOffsetsByHashes`: a location the game has never written can't be forced. Item rewards
//! and the secondary map-marker writes the source performs alongside some sets are intentionally
//! not reproduced here; this layer only moves the completion flags themselves.

use crate::binary::SaveBuffer;
use crate::completism::{metric, CompletismCategory};
use crate::error::SaveError;
use crate::totk::completism_data as data;
use crate::totk::guids::{append_guids, read_discovered_guids};
use crate::totk::hashtable::resolve_present_hashes;
use crate::totk::murmur3::hash32;

/// Counts how many *distinct* hashes in `hashes` are present in the save and hold `expected`.
/// Shared shape behind `Completism._count`. Counting distinct offsets (rather than raw array
/// positions) means a hash repeated in the source's own list is never double-counted.
fn count_matching(
    buf: &SaveBuffer,
    hash_table_end: usize,
    hashes: &[u32],
    expected: u32,
) -> Result<usize, SaveError> {
    let offsets = resolve_present_hashes(buf, hash_table_end, hashes)?;
    let mut count = 0;
    for &offset in offsets.values() {
        if buf.read_u32(offset)? == expected {
            count += 1;
        }
    }
    Ok(count)
}

/// Writes `value_true` to every present flag in `hashes` whose current value isn't already
/// `value_true` and (when `only_when` is `Some`) currently equals it. Returns how many changed.
/// Mirrors `Completism._set`; flags absent from the save are skipped like `_getOffsetsByHashes`.
fn set_all_matching(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    hashes: &[u32],
    value_true: u32,
    only_when: Option<u32>,
) -> Result<usize, SaveError> {
    let offsets = resolve_present_hashes(buf, hash_table_end, hashes)?;
    let mut count = 0;
    for &offset in offsets.values() {
        let val = buf.read_u32(offset)?;
        if val != value_true && only_when.map_or(true, |w| val == w) {
            buf.write_u32(offset, value_true)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Counts how many of `guids` appear in the save's discovered-GUIDs array (`_countGuids`).
fn count_guids(buf: &SaveBuffer, hash_table_end: usize, guids: &[u64]) -> Result<usize, SaveError> {
    let discovered = read_discovered_guids(buf, hash_table_end)?;
    Ok(guids.iter().filter(|g| discovered.contains(g)).count())
}

/// Appends whatever GUIDs in `guids` aren't already discovered (`_setGuids`). Returns the count
/// newly added.
fn unlock_all_guids(buf: &mut SaveBuffer, hash_table_end: usize, guids: &[u64]) -> Result<usize, SaveError> {
    let discovered = read_discovered_guids(buf, hash_table_end)?;
    let missing: Vec<u64> = guids.iter().copied().filter(|g| !discovered.contains(g)).collect();
    let count = missing.len();
    append_guids(buf, hash_table_end, &missing)?;
    Ok(count)
}

// --- individual counts kept for direct/legacy callers and unit tests ---

pub fn shrines_found(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::SHRINES_FOUND, 1)
}

/// `SHRINES_STATUS` holds a hash of the state name (`Hidden`/`Appear`/`Open`/`Enter`/`Clear`);
/// "cleared" means the stored value equals `hash("Clear")`.
pub fn shrines_cleared(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::SHRINES_STATUS, hash32("Clear"))
}

pub fn koroks_hidden(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::KOROKS_HIDDEN, 1)
}

/// `KOROKS_CARRY` holds a hash of `NotClear`/`Clear`. "Delivered to Hestu" means `Clear`.
pub fn koroks_carried(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::KOROKS_CARRY, hash32("Clear"))
}

pub fn locations_visited(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::LOCATIONS_VISITED, 1)
}

pub fn defeated_hinox(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::BOSSES_HINOXES_DEFEATED, 1)
}

pub fn defeated_talus(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::BOSSES_TALUSES_DEFEATED, 1)
}

pub fn defeated_molduga(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::BOSSES_MOLDUGAS_DEFEATED, 1)
}

pub fn defeated_bubbuls(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_guids(buf, hash_table_end, &data::BUBBULS_GUIDS)
}

pub fn sage_wills_found(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_guids(buf, hash_table_end, &data::SAGE_WILLS_FOUND)
}

pub fn old_maps_found(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_matching(buf, hash_table_end, &data::TREASURE_MAPS_FOUND, 1)
}

pub fn addison_completed(buf: &SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    count_guids(buf, hash_table_end, &data::ADDISON_COMPLETED)
}

pub fn unlock_all_bubbuls(buf: &mut SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    unlock_all_guids(buf, hash_table_end, &data::BUBBULS_GUIDS)
}

pub fn unlock_all_sage_wills(buf: &mut SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    unlock_all_guids(buf, hash_table_end, &data::SAGE_WILLS_FOUND)
}

pub fn unlock_all_addison(buf: &mut SaveBuffer, hash_table_end: usize) -> Result<usize, SaveError> {
    unlock_all_guids(buf, hash_table_end, &data::ADDISON_COMPLETED)
}

// --- structured snapshot for the completionism panel ---

/// Builds every category card with its current found/total figures, in the source's display order.
pub fn snapshot(buf: &SaveBuffer, hte: usize) -> Result<Vec<CompletismCategory>, SaveError> {
    let clear = hash32("Clear");
    let open = hash32("Open");
    let unopened = hash32("Unopened");
    let m = |hashes: &[u32], expected: u32| count_matching(buf, hte, hashes, expected);
    let g = |guids: &[u64]| count_guids(buf, hte, guids);

    let cats = vec![
        CompletismCategory {
            id: "towers",
            label: "Skyview Towers",
            metrics: vec![
                metric("found", m(&data::TOWERS_FOUND, 1)?, data::TOWERS_FOUND.len()),
                metric("activated", m(&data::TOWERS_ACTIVATED, 1)?, data::TOWERS_ACTIVATED.len()),
            ],
        },
        CompletismCategory {
            id: "shrines",
            label: "Shrines",
            metrics: vec![
                metric("found", m(&data::SHRINES_FOUND, 1)?, data::SHRINES_FOUND.len()),
                metric("cleared", m(&data::SHRINES_STATUS, clear)?, data::SHRINES_STATUS.len()),
            ],
        },
        CompletismCategory {
            id: "lightroots",
            label: "Lightroots",
            metrics: vec![
                metric("found", m(&data::LIGHTROOTS_FOUND, 1)?, data::LIGHTROOTS_FOUND.len()),
                metric("activated", m(&data::LIGHTROOTS_STATUS, open)?, data::LIGHTROOTS_STATUS.len()),
            ],
        },
        CompletismCategory {
            id: "koroks",
            label: "Koroks",
            metrics: vec![metric("found", m(&data::KOROKS_HIDDEN, 1)?, data::KOROKS_HIDDEN.len())],
        },
        CompletismCategory {
            id: "strayed_koroks",
            label: "Strayed koroks",
            metrics: vec![metric("carried", m(&data::KOROKS_CARRY, clear)?, data::KOROKS_CARRY.len())],
        },
        CompletismCategory {
            id: "bubbuls",
            label: "Bubbuls",
            metrics: vec![metric("defeated", g(&data::BUBBULS_GUIDS)?, data::BUBBULS_GUIDS.len())],
        },
        CompletismCategory {
            id: "locations",
            label: "Locations",
            metrics: vec![metric("visited", m(&data::LOCATIONS_VISITED, 1)?, data::LOCATIONS_VISITED.len())],
        },
        CompletismCategory {
            id: "caves",
            label: "Caves",
            metrics: vec![metric("visited", m(&data::LOCATION_CAVES_VISITED, 1)?, data::LOCATION_CAVES_VISITED.len())],
        },
        CompletismCategory {
            id: "wells",
            label: "Wells",
            metrics: vec![metric("visited", m(&data::LOCATION_WELLS_VISITED, 1)?, data::LOCATION_WELLS_VISITED.len())],
        },
        CompletismCategory {
            id: "chasms",
            label: "Chasms",
            metrics: vec![metric("visited", m(&data::LOCATION_CHASMS_VISITED, 1)?, data::LOCATION_CHASMS_VISITED.len())],
        },
        CompletismCategory {
            id: "hinox",
            label: "Hinox",
            metrics: vec![metric("defeated", m(&data::BOSSES_HINOXES_DEFEATED, 1)?, data::BOSSES_HINOXES_DEFEATED.len())],
        },
        CompletismCategory {
            id: "talus",
            label: "Talus",
            metrics: vec![metric("defeated", m(&data::BOSSES_TALUSES_DEFEATED, 1)?, data::BOSSES_TALUSES_DEFEATED.len())],
        },
        CompletismCategory {
            id: "molduga",
            label: "Molduga",
            metrics: vec![metric("defeated", m(&data::BOSSES_MOLDUGAS_DEFEATED, 1)?, data::BOSSES_MOLDUGAS_DEFEATED.len())],
        },
        CompletismCategory {
            id: "flux",
            label: "Flux Construct",
            metrics: vec![metric("defeated", m(&data::BOSSES_FLUX_CONSTRUCT_DEFEATED, 1)?, data::BOSSES_FLUX_CONSTRUCT_DEFEATED.len())],
        },
        CompletismCategory {
            id: "frox",
            label: "Frox",
            metrics: vec![metric("defeated", m(&data::BOSSES_FROXS_DEFEATED, 1)?, data::BOSSES_FROXS_DEFEATED.len())],
        },
        CompletismCategory {
            id: "gleeok",
            label: "Gleeok",
            metrics: vec![metric("defeated", m(&data::BOSSES_GLEEOKS_DEFEATED, 1)?, data::BOSSES_GLEEOKS_DEFEATED.len())],
        },
        CompletismCategory {
            id: "sage_wills",
            label: "Sage's Wills",
            metrics: vec![metric("found", g(&data::SAGE_WILLS_FOUND)?, data::SAGE_WILLS_FOUND.len())],
        },
        CompletismCategory {
            id: "old_maps",
            label: "Old maps",
            metrics: vec![metric("found", m(&data::TREASURE_MAPS_FOUND, 1)?, data::TREASURE_MAPS_FOUND.len())],
        },
        CompletismCategory {
            id: "addison",
            label: "Addison signposts",
            metrics: vec![metric("completed", g(&data::ADDISON_COMPLETED)?, data::ADDISON_COMPLETED.len())],
        },
        CompletismCategory {
            id: "schema_stones",
            label: "Schema stones",
            metrics: vec![metric("found", m(&data::SCHEMATICS_STONE_FOUND, 1)?, data::SCHEMATICS_STONE_FOUND.len())],
        },
        CompletismCategory {
            id: "yiga_schematics",
            label: "Yiga schematics",
            metrics: vec![metric("found", m(&data::SCHEMATICS_YIGA_FOUND, 1)?, data::SCHEMATICS_YIGA_FOUND.len())],
        },
        CompletismCategory {
            id: "compendium",
            label: "Hyrule Compendium",
            metrics: vec![metric(
                "unlocked",
                data::COMPENDIUM_STATUS.len() - m(&data::COMPENDIUM_STATUS, unopened)?,
                data::COMPENDIUM_STATUS.len(),
            )],
        },
    ];
    Ok(cats)
}

/// Mass-sets one metric of a category to complete, returning how many entries changed. `id`
/// matches [`CompletismCategory::id`]; `metric` is the index into that card's `metrics`
/// (0 = primary, 1 = the secondary line on the towers/shrines/lightroots cards).
pub fn set_all(
    buf: &mut SaveBuffer,
    hte: usize,
    id: &str,
    metric: usize,
) -> Result<usize, SaveError> {
    let clear = hash32("Clear");
    let open = hash32("Open");
    match (id, metric) {
        ("towers", 0) => set_all_matching(buf, hte, &data::TOWERS_FOUND, 1, None),
        ("towers", 1) => {
            let changed = set_all_matching(buf, hte, &data::TOWERS_ACTIVATED, 1, None)?;
            set_all_matching(buf, hte, &data::TOWERS_MAP_REVEALED, 1, None)?;
            Ok(changed)
        }
        ("shrines", 0) => {
            let changed = set_all_matching(buf, hte, &data::SHRINES_FOUND, 1, None)?;
            set_all_matching(buf, hte, &data::SHRINES_STATUS, open, Some(hash32("Hidden")))?;
            set_all_matching(buf, hte, &data::SHRINES_STATUS, open, Some(hash32("Appear")))?;
            Ok(changed)
        }
        ("shrines", 1) => set_all_matching(buf, hte, &data::SHRINES_STATUS, clear, None),
        ("lightroots", 0) => set_all_matching(buf, hte, &data::LIGHTROOTS_FOUND, 1, None),
        ("lightroots", 1) => set_all_matching(buf, hte, &data::LIGHTROOTS_STATUS, open, None),
        ("koroks", 0) => set_all_matching(buf, hte, &data::KOROKS_HIDDEN, 1, None),
        ("strayed_koroks", 0) => set_all_matching(buf, hte, &data::KOROKS_CARRY, clear, None),
        ("bubbuls", 0) => unlock_all_guids(buf, hte, &data::BUBBULS_GUIDS),
        ("locations", 0) => set_all_matching(buf, hte, &data::LOCATIONS_VISITED, 1, None),
        ("caves", 0) => {
            let changed = set_all_matching(buf, hte, &data::LOCATION_CAVES_VISITED, 1, None)?;
            set_all_matching(buf, hte, &data::LOCATION_CAVES_VISITED2, 1, None)?;
            Ok(changed)
        }
        ("wells", 0) => {
            let changed = set_all_matching(buf, hte, &data::LOCATION_WELLS_VISITED, 1, None)?;
            set_all_matching(buf, hte, &data::LOCATION_WELLS_VISITED2, 1, None)?;
            Ok(changed)
        }
        ("chasms", 0) => {
            let changed = set_all_matching(buf, hte, &data::LOCATION_CHASMS_VISITED, 1, None)?;
            set_all_matching(buf, hte, &data::LOCATION_CHASMS_VISITED2, 1, None)?;
            Ok(changed)
        }
        ("hinox", 0) => set_all_matching(buf, hte, &data::BOSSES_HINOXES_DEFEATED, 1, None),
        ("talus", 0) => set_all_matching(buf, hte, &data::BOSSES_TALUSES_DEFEATED, 1, None),
        ("molduga", 0) => set_all_matching(buf, hte, &data::BOSSES_MOLDUGAS_DEFEATED, 1, None),
        ("flux", 0) => set_all_matching(buf, hte, &data::BOSSES_FLUX_CONSTRUCT_DEFEATED, 1, None),
        ("frox", 0) => set_all_matching(buf, hte, &data::BOSSES_FROXS_DEFEATED, 1, None),
        ("gleeok", 0) => set_all_matching(buf, hte, &data::BOSSES_GLEEOKS_DEFEATED, 1, None),
        ("sage_wills", 0) => unlock_all_guids(buf, hte, &data::SAGE_WILLS_FOUND),
        ("old_maps", 0) => set_all_matching(buf, hte, &data::TREASURE_MAPS_FOUND, 1, None),
        ("addison", 0) => unlock_all_guids(buf, hte, &data::ADDISON_COMPLETED),
        ("schema_stones", 0) => set_all_matching(buf, hte, &data::SCHEMATICS_STONE_FOUND, 1, None),
        ("yiga_schematics", 0) => set_all_matching(buf, hte, &data::SCHEMATICS_YIGA_FOUND, 1, None),
        ("compendium", 0) => set_all_matching(buf, hte, &data::COMPENDIUM_STATUS, hash32("Buy"), None),
        _ => Err(SaveError::MissingField("unknown completism category")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_lists_have_no_accidental_duplicates() {
        let mut seen = std::collections::HashSet::new();
        for &h in data::KOROKS_HIDDEN.iter() {
            assert!(seen.insert(h), "duplicate korok hidden hash {h:#x}");
        }
    }

    #[test]
    fn guid_lists_have_no_accidental_duplicates() {
        for (name, guids) in [
            ("bubbul", &data::BUBBULS_GUIDS[..]),
            ("sage will", &data::SAGE_WILLS_FOUND[..]),
            ("addison", &data::ADDISON_COMPLETED[..]),
        ] {
            let mut seen = std::collections::HashSet::new();
            for &g in guids {
                assert!(seen.insert(g), "duplicate {name} guid {g:#x}");
            }
        }
        let mut seen = std::collections::HashSet::new();
        for &h in data::TREASURE_MAPS_FOUND.iter() {
            assert!(seen.insert(h), "duplicate old-map hash {h:#x}");
        }
    }

    fn buffer_with_slots(slots: &[(u32, u32)]) -> SaveBuffer {
        let mut data = vec![0u8; 0x28];
        for (hash, value) in slots {
            data.extend_from_slice(&hash.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        let mut buf = SaveBuffer::new(data);
        buf.little_endian = true;
        buf
    }

    #[test]
    fn count_matching_does_not_double_count_a_repeated_hash() {
        let buf = buffer_with_slots(&[(0xaaaa, 1), (0xbbbb, 0)]);
        let hash_table_end = buf.len();
        let count = count_matching(&buf, hash_table_end, &[0xaaaa, 0xaaaa, 0xbbbb], 1).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn set_all_matching_flips_only_present_flags_and_reports_changes() {
        let mut buf = buffer_with_slots(&[(0xaaaa, 0), (0xbbbb, 1)]);
        let hte = buf.len();
        // 0xcccc is absent from the save, so it can't be forced and isn't counted.
        let changed = set_all_matching(&mut buf, hte, &[0xaaaa, 0xbbbb, 0xcccc], 1, None).unwrap();
        assert_eq!(changed, 1, "only the one present-and-unset flag changes");
        assert_eq!(count_matching(&buf, hte, &[0xaaaa, 0xbbbb], 1).unwrap(), 2);
    }
}
