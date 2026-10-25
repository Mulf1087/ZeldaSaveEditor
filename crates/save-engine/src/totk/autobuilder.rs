//! AutoBuild schematics ("Draft" slots): saved Ultrahand structures the player can recall from
//! the in-game AutoBuild menu. Distinct addressing shape from everything else in this crate:
//! `CombinedActorInfo` is a variable-length-per-element binary array
//! (`[u32 capacity][len_0][bytes_0][len_1][bytes_1]...`), not a fixed stride like every other
//! struct-of-arrays field (pouch/horse). `Index`/`CombinedActorInfo`/`CameraPos`/`CameraAt`
//! hashes are literal constants already known from the source's own `Hashes` table (unlike
//! pouch/horse fields, which are murmur3-hashed from name strings at call time); `IsFavorite`
//! is *not* in that literal table, so it's hashed at call time like pouch/horse fields.

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashtable::scan_offsets;
use crate::totk::murmur3::hash32;
use crate::totk::pouch::{read_bool_elem, write_bool_elem};

pub struct AutoBuildEntry {
    /// The in-game schematic slot id this entry occupies, or `-1` for an unused Draft slot.
    pub index: i32,
    /// Raw `CombinedActorInfo` blob, opaque binary game data, not interpreted by this crate.
    pub combined_actor_info: Vec<u8>,
    pub camera_pos: (f32, f32, f32),
    pub camera_at: (f32, f32, f32),
    pub is_favorite: bool,
}

fn resolve(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashMap<&'static str, usize>, SaveError> {
    scan_offsets(
        buf,
        hash_table_end,
        &[
            (0xd27f8651, "INDEX", true), // AutoBuilder.Draft.Content.Index
            (0xa56722b6, "CAI", true),   // AutoBuilder.Draft.Content.CombinedActorInfo
            (0xc5bf2815, "CAMERA_POS", true), // AutoBuilder.Draft.Content.CameraPos
            (0xef74dca7, "CAMERA_AT", true),  // AutoBuilder.Draft.Content.CameraAt
            (hash32("AutoBuilder.Draft.Content.IsFavorite"), "IS_FAVORITE", true),
        ],
    )
}

fn read_vector3f_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<(f32, f32, f32), SaveError> {
    let base = array_addr + 4 + index * 0x0c;
    Ok((buf.read_f32(base)?, buf.read_f32(base + 4)?, buf.read_f32(base + 8)?))
}
fn write_vector3f_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: (f32, f32, f32),
) -> Result<(), SaveError> {
    let base = array_addr + 4 + index * 0x0c;
    buf.write_f32(base, value.0)?;
    buf.write_f32(base + 4, value.1)?;
    buf.write_f32(base + 8, value.2)
}

/// Reads element `index` out of a `[u32 capacity][len_0][bytes_0]...` variable-length binary
/// array, mirroring `readBinaryArray`. Every entry's length must be walked from the start since
/// there's no fixed stride.
fn read_binary_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<Vec<u8>, SaveError> {
    let capacity = buf.read_u32(array_addr)? as usize;
    if index >= capacity {
        return Err(SaveError::IndexOutOfRange { index, max: capacity });
    }
    let mut offset = array_addr + 4;
    for i in 0..capacity {
        let len = buf.read_u32(offset)? as usize;
        offset += 4;
        if i == index {
            return buf.read_bytes(offset, len);
        }
        offset += len;
    }
    unreachable!("index already bounds-checked against capacity above")
}

/// Overwrites element `index`'s bytes in place, mirroring `writeBinary`, which never resizes an
/// entry, only overwrites its existing content. Unlike the source (which writes `data` at the
/// stored offset regardless of length, silently corrupting the next entry's header if `data` is
/// longer than the existing slot, or leaving stale trailing bytes if shorter), this rejects a
/// length mismatch instead of risking either.
fn write_binary_elem(buf: &mut SaveBuffer, array_addr: usize, index: usize, data: &[u8]) -> Result<(), SaveError> {
    let capacity = buf.read_u32(array_addr)? as usize;
    if index >= capacity {
        return Err(SaveError::IndexOutOfRange { index, max: capacity });
    }
    let mut offset = array_addr + 4;
    for i in 0..capacity {
        let len = buf.read_u32(offset)? as usize;
        offset += 4;
        if i == index {
            if data.len() != len {
                return Err(SaveError::SizeMismatch { expected: len, actual: data.len() });
            }
            return buf.write_bytes(offset, data);
        }
        offset += len;
    }
    unreachable!("index already bounds-checked against capacity above")
}

pub fn read_autobuilds(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<AutoBuildEntry>, SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let index_addr = offsets["INDEX"];
    let cai_addr = offsets["CAI"];
    let camera_pos_addr = offsets["CAMERA_POS"];
    let camera_at_addr = offsets["CAMERA_AT"];
    let is_favorite_addr = offsets["IS_FAVORITE"];
    let capacity = buf.read_u32(index_addr)? as usize;

    let mut entries = Vec::with_capacity(capacity);
    for i in 0..capacity {
        entries.push(AutoBuildEntry {
            index: buf.read_i32(index_addr + 4 + i * 4)?,
            combined_actor_info: read_binary_elem(buf, cai_addr, i)?,
            camera_pos: read_vector3f_elem(buf, camera_pos_addr, i)?,
            camera_at: read_vector3f_elem(buf, camera_at_addr, i)?,
            is_favorite: read_bool_elem(buf, is_favorite_addr, i)?,
        });
    }
    Ok(entries)
}

/// Writes back exactly `capacity` entries. Unlike pouch/horse categories, AutoBuild's Draft
/// array has no "unused slot" convention this crate manages (an entry's own `index` field is
/// the source's signal for "unused", typically `-1`); every slot is always written, none are
/// blanked. `entries.len()` must equal the slot's fixed capacity exactly.
pub fn write_autobuilds(buf: &mut SaveBuffer, hash_table_end: usize, entries: &[AutoBuildEntry]) -> Result<(), SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let index_addr = offsets["INDEX"];
    let cai_addr = offsets["CAI"];
    let camera_pos_addr = offsets["CAMERA_POS"];
    let camera_at_addr = offsets["CAMERA_AT"];
    let is_favorite_addr = offsets["IS_FAVORITE"];
    let capacity = buf.read_u32(index_addr)? as usize;

    if entries.len() != capacity {
        return Err(SaveError::SizeMismatch { expected: capacity, actual: entries.len() });
    }

    for (i, entry) in entries.iter().enumerate() {
        buf.write_i32(index_addr + 4 + i * 4, entry.index)?;
        write_binary_elem(buf, cai_addr, i, &entry.combined_actor_info)?;
        write_vector3f_elem(buf, camera_pos_addr, i, entry.camera_pos)?;
        write_vector3f_elem(buf, camera_at_addr, i, entry.camera_at)?;
        write_bool_elem(buf, is_favorite_addr, i, entry.is_favorite)?;
    }
    Ok(())
}
