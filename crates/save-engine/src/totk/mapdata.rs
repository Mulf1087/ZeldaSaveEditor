//! Map-related struct-of-arrays fields: player-placed pins ("stamps", the in-game "add pin"/
//! "clear pins" feature), fast-travel markers, and teleporters. The source exposes all three
//! through its "Map pins editor" mini-dialog, built on a generic Variable/Struct engine; that
//! generic engine itself is a separate deferred slice, but these three concrete fields are ported
//! directly here the same way pouch/horse/AutoBuild are, without needing the generic machinery.
//!
//! Same fixed-capacity `[u32 capacity][element_0][element_1]...` struct-of-arrays layout
//! throughout. `StampData`'s three hashes are literal constants already known from the source's
//! `SavegameEditor.Hashes` table; `MapPinData`/`WarpMarkerData`'s are not in that table, so
//! they're murmur3-hashed from name strings at call time, like pouch/horse fields.
//!
//! Enum-typed fields (`icon`, `layer`, marker `color`) are raw hashes, same convention as every
//! other `Enum`-typed field in this crate (`pouch::WeaponEntry::modifier`,
//! `pouch::ArmorEntry::dye_color`, ...): not decoded into a Rust enum here, just exposed as-is.

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashtable::scan_offsets;
use crate::totk::murmur3::hash32;

/// `MapPin.MAX` in the source: the fixed number of stamp slots.
pub const CAPACITY: usize = 300;

/// `MapPin.ICON_NONE`: a slot with this icon hash is unused.
pub const ICON_NONE: u32 = 0x7e3d1e46;

pub struct MapPinEntry {
    /// `MapData.IconData.StampData.Type`, ported from `MapPin.ICON_*`.
    pub icon: u32,
    pub x: f32,
    pub y: f32,
    /// `MapData.IconData.StampData.Layer`, ported from `MapPin.MAP_*`.
    pub layer: u32,
}

impl MapPinEntry {
    pub fn is_free(&self) -> bool {
        self.icon == ICON_NONE
    }
}

fn resolve(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashMap<&'static str, usize>, SaveError> {
    scan_offsets(
        buf,
        hash_table_end,
        &[
            (0x14d7f4c4, "TYPE", true),  // MapData.IconData.StampData.Type
            (0xf24fc2e7, "POS", true),   // MapData.IconData.StampData.Pos
            (0xd2025694, "LAYER", true), // MapData.IconData.StampData.Layer
        ],
    )
}

fn read_vector2f_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<(f32, f32), SaveError> {
    let base = array_addr + 4 + index * 0x08;
    Ok((buf.read_f32(base)?, buf.read_f32(base + 4)?))
}
fn write_vector2f_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: (f32, f32),
) -> Result<(), SaveError> {
    let base = array_addr + 4 + index * 0x08;
    buf.write_f32(base, value.0)?;
    buf.write_f32(base + 4, value.1)
}

pub fn read_map_pins(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<MapPinEntry>, SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let layer_addr = offsets["LAYER"];
    let capacity = buf.read_u32(type_addr)? as usize;

    let mut entries = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let icon = buf.read_u32(type_addr + 4 + i * 4)?;
        let (x, y) = read_vector2f_elem(buf, pos_addr, i)?;
        let layer = buf.read_u32(layer_addr + 4 + i * 4)?;
        entries.push(MapPinEntry { icon, x, y, layer });
    }
    Ok(entries)
}

/// Writes back exactly `capacity` (300) entries, same "always full width, no trimming"
/// convention as AutoBuild's Draft slots: a free pin is a slot whose icon is `ICON_NONE`, not an
/// omitted array entry.
pub fn write_map_pins(buf: &mut SaveBuffer, hash_table_end: usize, entries: &[MapPinEntry]) -> Result<(), SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let layer_addr = offsets["LAYER"];
    let capacity = buf.read_u32(type_addr)? as usize;

    if entries.len() != capacity {
        return Err(SaveError::SizeMismatch { expected: capacity, actual: entries.len() });
    }

    for (i, entry) in entries.iter().enumerate() {
        buf.write_u32(type_addr + 4 + i * 4, entry.icon)?;
        write_vector2f_elem(buf, pos_addr, i, (entry.x, entry.y))?;
        buf.write_u32(layer_addr + 4 + i * 4, entry.layer)?;
    }
    Ok(())
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

// ---------------------------------------------------------------------------------------------
// Fast-travel markers
// ---------------------------------------------------------------------------------------------

pub struct MapMarkerEntry {
    /// `MapData.IconData.MapPinData.Type`, raw hash (Invalid/Red/Blue/Yellow/Green/Purple/
    /// LightBlue). "Invalid" is the same hash as `ICON_NONE` above: same string, same meaning.
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl MapMarkerEntry {
    pub fn is_free(&self) -> bool {
        self.color == ICON_NONE
    }
}

fn resolve_markers(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashMap<&'static str, usize>, SaveError> {
    scan_offsets(
        buf,
        hash_table_end,
        &[
            (hash32("MapData.IconData.MapPinData.Type"), "TYPE", true),
            (hash32("MapData.IconData.MapPinData.Pos"), "POS", true),
        ],
    )
}

pub fn read_map_markers(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<MapMarkerEntry>, SaveError> {
    let offsets = resolve_markers(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let capacity = buf.read_u32(type_addr)? as usize;

    let mut entries = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let color = buf.read_u32(type_addr + 4 + i * 4)?;
        let (x, y, z) = read_vector3f_elem(buf, pos_addr, i)?;
        entries.push(MapMarkerEntry { color, x, y, z });
    }
    Ok(entries)
}

pub fn write_map_markers(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[MapMarkerEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_markers(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let capacity = buf.read_u32(type_addr)? as usize;

    if entries.len() != capacity {
        return Err(SaveError::SizeMismatch { expected: capacity, actual: entries.len() });
    }

    for (i, entry) in entries.iter().enumerate() {
        buf.write_u32(type_addr + 4 + i * 4, entry.color)?;
        write_vector3f_elem(buf, pos_addr, i, (entry.x, entry.y, entry.z))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Teleporters
// ---------------------------------------------------------------------------------------------

pub struct TeleporterEntry {
    /// `MapData.IconData.WarpMarkerData.Index`.
    pub index: i32,
    pub pos: (f32, f32, f32),
    pub rot: (f32, f32, f32),
}

fn resolve_teleporters(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashMap<&'static str, usize>, SaveError> {
    scan_offsets(
        buf,
        hash_table_end,
        &[
            (hash32("MapData.IconData.WarpMarkerData.Index"), "INDEX", true),
            (hash32("MapData.IconData.WarpMarkerData.Pos"), "POS", true),
            (hash32("MapData.IconData.WarpMarkerData.Rot"), "ROT", true),
        ],
    )
}

pub fn read_teleporters(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<TeleporterEntry>, SaveError> {
    let offsets = resolve_teleporters(buf, hash_table_end)?;
    let index_addr = offsets["INDEX"];
    let pos_addr = offsets["POS"];
    let rot_addr = offsets["ROT"];
    let capacity = buf.read_u32(index_addr)? as usize;

    let mut entries = Vec::with_capacity(capacity);
    for i in 0..capacity {
        entries.push(TeleporterEntry {
            index: buf.read_i32(index_addr + 4 + i * 4)?,
            pos: read_vector3f_elem(buf, pos_addr, i)?,
            rot: read_vector3f_elem(buf, rot_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_teleporters(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[TeleporterEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_teleporters(buf, hash_table_end)?;
    let index_addr = offsets["INDEX"];
    let pos_addr = offsets["POS"];
    let rot_addr = offsets["ROT"];
    let capacity = buf.read_u32(index_addr)? as usize;

    if entries.len() != capacity {
        return Err(SaveError::SizeMismatch { expected: capacity, actual: entries.len() });
    }

    for (i, entry) in entries.iter().enumerate() {
        buf.write_i32(index_addr + 4 + i * 4, entry.index)?;
        write_vector3f_elem(buf, pos_addr, i, entry.pos)?;
        write_vector3f_elem(buf, rot_addr, i, entry.rot)?;
    }
    Ok(())
}
