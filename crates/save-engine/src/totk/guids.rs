//! The "discovered GUIDs" array: a separate appended region of the save file, not part of the
//! normal hash-table field storage, tracking specific in-world entity/event instances by 64-bit
//! GUID (individual bubbul enemies, sage-will pedestals, and a few other categories out of scope
//! here). Located one pointer dereference past the hash table's own sentinel slot
//! (`hash_table_end`): the 4 bytes stored there are the address of the array itself, mirroring
//! `SavegameEditor._getOffsets`'s `this.guidsArrayOffset=tempFile.readU32(i+4)` at the sentinel
//! row (the same row `totk::hashtable::find_hash_table_end` already scans for, just reading its
//! value instead of stopping at its position).
//!
//! Unlike every other field in this crate, this array has no reserved slack after it: growing it
//! means shifting every byte that follows and fixing up every pointer that shifted, which is
//! what `append_guids` and `hashtable::insert_and_relocate` exist for.

use std::collections::HashSet;

use crate::binary::SaveBuffer;
use crate::error::SaveError;

/// Reads every GUID out of the appended, zero-terminated `[u64_le]...` array, mirroring
/// `SavegameEditor.guidsArray`'s load loop (`lower`/`upper` u32 halves, low word first, stopping
/// at the first all-zero entry).
pub fn read_discovered_guids(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashSet<u64>, SaveError> {
    let (guids, _) = read_discovered_guids_with_end(buf, hash_table_end)?;
    Ok(guids)
}

/// Same scan as [`read_discovered_guids`], but also returns the offset of the array's zero
/// terminator (the true end of the raw slot range). Kept separate so `append_guids` can compute
/// its insertion point from the actual scanned byte range instead of from `guids.len()`, which
/// silently under-counts if the array ever contains a duplicate entry (a `HashSet` dedups).
fn read_discovered_guids_with_end(buf: &SaveBuffer, hash_table_end: usize) -> Result<(HashSet<u64>, usize), SaveError> {
    let array_addr = buf.read_u32(hash_table_end)? as usize;
    let mut guids = HashSet::new();
    let mut offset = array_addr;
    while offset + 8 <= buf.len() {
        let value = buf.read_u64(offset)?;
        if value == 0 {
            break;
        }
        guids.insert(value);
        offset += 8;
    }
    Ok((guids, offset))
}

/// Adds every GUID in `new_guids` that isn't already in the discovered set, in one batched
/// insertion: computes the whole missing list first (no buffer mutation), then makes exactly
/// one `hashtable::insert_and_relocate` call sized for the whole batch, then writes all new
/// entries at once. Mirrors the source's own `_setGuids`, which pushes to an in-memory array
/// per item but only calls the real file write once after the whole batch, critical for mass-
/// unlocking dozens or hundreds of items without dozens or hundreds of separate buffer shifts.
pub fn append_guids(buf: &mut SaveBuffer, hash_table_end: usize, new_guids: &[u64]) -> Result<(), SaveError> {
    let (discovered, terminator_offset) = read_discovered_guids_with_end(buf, hash_table_end)?;
    let missing: Vec<u64> = new_guids.iter().copied().filter(|g| !discovered.contains(g)).collect();
    if missing.is_empty() {
        return Ok(());
    }

    let additional_bytes = missing.len() * 8;

    crate::totk::hashtable::insert_and_relocate(buf, hash_table_end, terminator_offset, additional_bytes)?;

    for (i, guid) in missing.iter().enumerate() {
        buf.write_u64(terminator_offset + i * 8, *guid)?;
    }
    Ok(())
}
