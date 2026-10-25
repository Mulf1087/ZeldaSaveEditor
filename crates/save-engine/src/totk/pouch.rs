//! Pouch item contents: weapons, bows, shields, armor, arrows, materials, food, key items, and
//! Zonai devices (save-format prefix `Pouch.SpecialParts.*`). Every property
//! is stored as a separate struct-of-arrays blob: `[u32 capacity][element_0][element_1]...`,
//! resolved through the same pointer-hash mechanism as `SAVE_POS`/`SEQUENCE_CURRENT_BANC`
//! (`totk::hashtable::scan_offsets`, unchanged). The hash itself is computed via
//! `totk::murmur3::hash32` from the field's name string, since (unlike the core slice's
//! hashes) these aren't literal constants in the source.

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::strings;
use crate::totk::murmur3::hash32;

/// Reads the `u32` item-capacity prefix at a resolved struct-of-arrays address.
pub(crate) fn array_capacity(buf: &SaveBuffer, array_addr: usize) -> Result<u32, SaveError> {
    buf.read_u32(array_addr)
}

/// Offset of element `index` in a fixed-`stride`-byte struct-of-arrays blob at `array_addr`.
pub(crate) fn element_offset(array_addr: usize, index: usize, stride: usize) -> usize {
    array_addr + 4 + index * stride
}

pub(crate) const STRIDE_STRING64: usize = 0x40;
pub(crate) const STRIDE_I32: usize = 4;

pub(crate) fn read_string64_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> String {
    strings::read_string64(buf, element_offset(array_addr, index, STRIDE_STRING64))
}
pub(crate) fn write_string64_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: &str,
) -> Result<(), SaveError> {
    strings::write_string64(buf, element_offset(array_addr, index, STRIDE_STRING64), value)
}
pub(crate) fn read_i32_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<i32, SaveError> {
    buf.read_i32(element_offset(array_addr, index, STRIDE_I32))
}
pub(crate) fn write_i32_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: i32,
) -> Result<(), SaveError> {
    buf.write_i32(element_offset(array_addr, index, STRIDE_I32), value)
}
/// `Enum`-typed fields (modifier, dye color, ...) are raw u32 hashes: same stride as i32,
/// unsigned read/write, no interpretation.
pub(crate) fn read_u32_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<u32, SaveError> {
    buf.read_u32(element_offset(array_addr, index, STRIDE_I32))
}
pub(crate) fn write_u32_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: u32,
) -> Result<(), SaveError> {
    buf.write_u32(element_offset(array_addr, index, STRIDE_I32), value)
}

/// Blanks (empties) the id field for `index`, marking a pouch slot unused, mirroring
/// `Pouch.prototype.save`'s "remove empty items" loop. Used by every category's `set_*`.
fn clear_id_elem(buf: &mut SaveBuffer, array_addr: usize, index: usize) -> Result<(), SaveError> {
    write_string64_elem(buf, array_addr, index, "")
}

/// Bit-packed `BoolArray`: bit `index` lives at `array_addr + 4 + index/8`, bit position
/// `index % 8`, mirroring `Variable._read`'s `BoolArray` branch (`tempFile.readU8(offset +
/// floor(bitIndex/8))`, `>> (bitIndex%8) & 1`). Shared by horses (`IsFamiliarityChecked`) and
/// AutoBuild (`IsFavorite`).
pub(crate) fn read_bool_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<bool, SaveError> {
    let byte = buf.read_u8(array_addr + 4 + index / 8)?;
    Ok(((byte >> (index % 8)) & 1) != 0)
}
pub(crate) fn write_bool_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: bool,
) -> Result<(), SaveError> {
    let byte_offset = array_addr + 4 + index / 8;
    let mut byte = buf.read_u8(byte_offset)?;
    let mask = 1u8 << (index % 8);
    if value {
        byte |= mask;
    } else {
        byte &= !mask;
    }
    buf.write_u8(byte_offset, byte)
}

/// Builds the `(hash, key, is_pointer=true)` list `scan_offsets` expects from a
/// `(field_name, key)` table, hashing each field name at call time via `murmur3::hash32`.
pub(crate) fn resolve_category(
    buf: &SaveBuffer,
    hash_table_end: usize,
    field_names: &[(&'static str, &'static str)],
) -> Result<std::collections::HashMap<&'static str, usize>, SaveError> {
    let hashes: Vec<(u32, &'static str, bool)> = field_names
        .iter()
        .map(|&(name, key)| (hash32(name), key, true))
        .collect();
    crate::totk::hashtable::scan_offsets(buf, hash_table_end, &hashes)
}

// ---------------------------------------------------------------------------------------------
// Weapons
// ---------------------------------------------------------------------------------------------

pub struct WeaponEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
    pub record_extra_durability: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.WEAPONS`,
/// hashed at call time via `murmur3::hash32` rather than stored as literal constants, since
/// these are computed hashes in the source, unlike the core slice's.
fn weapon_field_names() -> [(&'static str, &'static str); 8] {
    [
        ("Pouch.Weapon.Content.Name", "NAME"),
        ("Pouch.Weapon.Content.Life", "LIFE"),
        ("Pouch.Weapon.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Weapon.Content.Effect.Value", "MODIFIER_VALUE"),
        ("Pouch.Weapon.Content.Combined.Name", "FUSE_ID"),
        ("Pouch.Weapon.Content.Combined.Life", "FUSE_DURABILITY"),
        ("Pouch.Weapon.Content.ExtraLife", "EXTRA_DURABILITY"),
        ("Pouch.Weapon.Content.RecordExtraLife", "RECORD_EXTRA_DURABILITY"),
    ]
}

pub fn read_weapons(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<WeaponEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &weapon_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let record_extra_durability_addr = offsets["RECORD_EXTRA_DURABILITY"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(WeaponEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
            fuse_id: read_string64_elem(buf, fuse_id_addr, i),
            fuse_durability: read_i32_elem(buf, fuse_durability_addr, i)?,
            extra_durability: read_i32_elem(buf, extra_durability_addr, i)?,
            record_extra_durability: read_i32_elem(buf, record_extra_durability_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_weapons(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[WeaponEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &weapon_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let record_extra_durability_addr = offsets["RECORD_EXTRA_DURABILITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
        write_string64_elem(buf, fuse_id_addr, i, &entry.fuse_id)?;
        write_i32_elem(buf, fuse_durability_addr, i, entry.fuse_durability)?;
        write_i32_elem(buf, extra_durability_addr, i, entry.extra_durability)?;
        write_i32_elem(buf, record_extra_durability_addr, i, entry.record_extra_durability)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Bows
// ---------------------------------------------------------------------------------------------

pub struct BowEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.BOWS`.
/// Bows have no fuse fields (`Equipment.prototype.isFusable` excludes bows in the source).
fn bow_field_names() -> [(&'static str, &'static str); 4] {
    [
        ("Pouch.Bow.Content.Name", "NAME"),
        ("Pouch.Bow.Content.Life", "LIFE"),
        ("Pouch.Bow.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Bow.Content.Effect.Value", "MODIFIER_VALUE"),
    ]
}

pub fn read_bows(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<BowEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &bow_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(BowEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_bows(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[BowEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &bow_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Shields
// ---------------------------------------------------------------------------------------------

pub struct ShieldEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.SHIELDS`.
/// Shields are fusable (unlike bows) but have no `RecordExtraLife` field (unlike weapons): 7
/// entries vs weapons' 8.
fn shield_field_names() -> [(&'static str, &'static str); 7] {
    [
        ("Pouch.Shield.Content.Name", "NAME"),
        ("Pouch.Shield.Content.Life", "LIFE"),
        ("Pouch.Shield.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Shield.Content.Effect.Value", "MODIFIER_VALUE"),
        ("Pouch.Shield.Content.Combined.Name", "FUSE_ID"),
        ("Pouch.Shield.Content.Combined.Life", "FUSE_DURABILITY"),
        ("Pouch.Shield.Content.ExtraLife", "EXTRA_DURABILITY"),
    ]
}

pub fn read_shields(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<ShieldEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &shield_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(ShieldEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
            fuse_id: read_string64_elem(buf, fuse_id_addr, i),
            fuse_durability: read_i32_elem(buf, fuse_durability_addr, i)?,
            extra_durability: read_i32_elem(buf, extra_durability_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_shields(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[ShieldEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &shield_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
        write_string64_elem(buf, fuse_id_addr, i, &entry.fuse_id)?;
        write_i32_elem(buf, fuse_durability_addr, i, entry.fuse_durability)?;
        write_i32_elem(buf, extra_durability_addr, i, entry.extra_durability)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Armor
// ---------------------------------------------------------------------------------------------

pub struct ArmorEntry {
    pub id: String,
    pub dye_color: u32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.ARMOR`.
/// Armor has no valid_num entry in the core slice's HASHES table; instead, we scan until empty id.
fn armor_field_names() -> [(&'static str, &'static str); 2] {
    [
        ("Pouch.Armor.Content.Name", "NAME"),
        ("Pouch.Armor.Content.ColorVariation", "DYE_COLOR"),
    ]
}

pub fn read_armor(
    buf: &SaveBuffer,
    hash_table_end: usize,
) -> Result<Vec<ArmorEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &armor_field_names())?;
    let name_addr = offsets["NAME"];
    let dye_color_addr = offsets["DYE_COLOR"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(ArmorEntry {
            id,
            dye_color: read_u32_elem(buf, dye_color_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_armor(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[ArmorEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &armor_field_names())?;
    let name_addr = offsets["NAME"];
    let dye_color_addr = offsets["DYE_COLOR"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_u32_elem(buf, dye_color_addr, i, entry.dye_color)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Arrows
// ---------------------------------------------------------------------------------------------

pub struct ArrowEntry {
    pub id: String,
    pub quantity: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.ARROW`.
/// No valid_num entry in the core slice's HASHES table; scan until empty id (like Armor).
fn arrow_field_names() -> [(&'static str, &'static str); 2] {
    [
        ("Pouch.Arrow.Content.Name", "NAME"),
        ("Pouch.Arrow.Content.StockNum", "QUANTITY"),
    ]
}

pub fn read_arrows(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<ArrowEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &arrow_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(ArrowEntry {
            id,
            quantity: read_i32_elem(buf, quantity_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_arrows(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[ArrowEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &arrow_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, quantity_addr, i, entry.quantity)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Materials
// ---------------------------------------------------------------------------------------------

pub struct MaterialEntry {
    pub id: String,
    pub quantity: i32,
    pub get_order: i32,
    pub use_order: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.MATERIAL`.
/// No valid_num entry; scan until empty id.
fn material_field_names() -> [(&'static str, &'static str); 4] {
    [
        ("Pouch.Material.Content.Name", "NAME"),
        ("Pouch.Material.Content.StockNum", "QUANTITY"),
        ("Pouch.Material.Content.GetOrder", "GET_ORDER"),
        ("Pouch.Material.Content.UseOrder", "USE_ORDER"),
    ]
}

pub fn read_materials(
    buf: &SaveBuffer,
    hash_table_end: usize,
) -> Result<Vec<MaterialEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &material_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let get_order_addr = offsets["GET_ORDER"];
    let use_order_addr = offsets["USE_ORDER"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(MaterialEntry {
            id,
            quantity: read_i32_elem(buf, quantity_addr, i)?,
            get_order: read_i32_elem(buf, get_order_addr, i)?,
            use_order: read_i32_elem(buf, use_order_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_materials(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[MaterialEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &material_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let get_order_addr = offsets["GET_ORDER"];
    let use_order_addr = offsets["USE_ORDER"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, quantity_addr, i, entry.quantity)?;
        write_i32_elem(buf, get_order_addr, i, entry.get_order)?;
        write_i32_elem(buf, use_order_addr, i, entry.use_order)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Key items
// ---------------------------------------------------------------------------------------------

pub struct KeyItemEntry {
    pub id: String,
    /// Legitimately `-1` for most (non-stackable) key items. That's "not applicable", not
    /// an error; don't clamp or reject it.
    pub quantity: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.KEYITEM`.
/// No valid_num entry; scan until empty id.
fn key_item_field_names() -> [(&'static str, &'static str); 2] {
    [
        ("Pouch.KeyItem.Content.Name", "NAME"),
        ("Pouch.KeyItem.Content.StockNum", "QUANTITY"),
    ]
}

pub fn read_key_items(
    buf: &SaveBuffer,
    hash_table_end: usize,
) -> Result<Vec<KeyItemEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &key_item_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(KeyItemEntry {
            id,
            quantity: read_i32_elem(buf, quantity_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_key_items(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[KeyItemEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &key_item_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, quantity_addr, i, entry.quantity)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Zonai devices
// ---------------------------------------------------------------------------------------------

pub struct DeviceEntry {
    pub id: String,
    pub quantity: i32,
    pub use_order: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.SPECIALPARTS`.
/// The UI category label "devices" (Zonai devices) maps to save-format prefix
/// `Pouch.SpecialParts.*`, not `Pouch.Devices.*` (confirmed against the real fixture: decodes
/// real device ids like `SpObj_WindGenerator_Capsule_A_01`). No valid_num;
/// scan until empty id.
fn device_field_names() -> [(&'static str, &'static str); 3] {
    [
        ("Pouch.SpecialParts.Content.Name", "NAME"),
        ("Pouch.SpecialParts.Content.StockNum", "QUANTITY"),
        ("Pouch.SpecialParts.Content.UseOrder", "USE_ORDER"),
    ]
}

pub fn read_devices(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<DeviceEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &device_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let use_order_addr = offsets["USE_ORDER"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(DeviceEntry {
            id,
            quantity: read_i32_elem(buf, quantity_addr, i)?,
            use_order: read_i32_elem(buf, use_order_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_devices(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[DeviceEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &device_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let use_order_addr = offsets["USE_ORDER"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, quantity_addr, i, entry.quantity)?;
        write_i32_elem(buf, use_order_addr, i, entry.use_order)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Food
// ---------------------------------------------------------------------------------------------

pub struct FoodEntry {
    pub id: String,
    pub quantity: i32,
    pub hearts_heal: i32,
    /// `Enum`-typed field, raw u32 hash, no interpretation.
    pub effect: u32,
    pub effect_multiplier: i32,
    pub effect_time: i32,
    pub price: i32,
    /// Up to 5 ingredient ids used to cook this item; unused slots are empty strings.
    pub recipe: [String; 5],
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.FOOD`.
/// No valid_num entry; scan until empty id. `MaterialName` (recipe) resolves through the same
/// table as everything else, but addresses into it flat (`item_index * 5 + slot`) since it's one
/// `item_count * 5`-element `String64Array` rather than a per-item array, see `read_recipe`.
fn food_field_names() -> [(&'static str, &'static str); 8] {
    [
        ("Pouch.Food.Content.Name", "NAME"),
        ("Pouch.Food.Content.StockNum", "QUANTITY"),
        ("Pouch.Food.Content.LifeRecover", "HEARTS_HEAL"),
        ("Pouch.Food.Content.Effect.Type", "EFFECT"),
        ("Pouch.Food.Content.Effect.Level", "EFFECT_MULTIPLIER"),
        ("Pouch.Food.Content.Effect.Time", "EFFECT_TIME"),
        ("Pouch.Food.Content.Price", "PRICE"),
        ("Pouch.Food.Content.MaterialName", "RECIPE"),
    ]
}

/// Reads item `item_index`'s 5 recipe slots out of the flat `item_count * 5`-element
/// `MaterialName` array (slots `[item_index*5, item_index*5+4]`).
fn read_recipe(buf: &SaveBuffer, recipe_array_addr: usize, item_index: usize) -> [String; 5] {
    let mut recipe: [String; 5] = Default::default();
    for slot in 0..5 {
        recipe[slot] = read_string64_elem(buf, recipe_array_addr, item_index * 5 + slot);
    }
    recipe
}
fn write_recipe(
    buf: &mut SaveBuffer,
    recipe_array_addr: usize,
    item_index: usize,
    recipe: &[String; 5],
) -> Result<(), SaveError> {
    for (slot, value) in recipe.iter().enumerate() {
        write_string64_elem(buf, recipe_array_addr, item_index * 5 + slot, value)?;
    }
    Ok(())
}

pub fn read_food(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<FoodEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &food_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let hearts_heal_addr = offsets["HEARTS_HEAL"];
    let effect_addr = offsets["EFFECT"];
    let effect_multiplier_addr = offsets["EFFECT_MULTIPLIER"];
    let effect_time_addr = offsets["EFFECT_TIME"];
    let price_addr = offsets["PRICE"];
    let recipe_addr = offsets["RECIPE"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(FoodEntry {
            id,
            quantity: read_i32_elem(buf, quantity_addr, i)?,
            hearts_heal: read_i32_elem(buf, hearts_heal_addr, i)?,
            effect: read_u32_elem(buf, effect_addr, i)?,
            effect_multiplier: read_i32_elem(buf, effect_multiplier_addr, i)?,
            effect_time: read_i32_elem(buf, effect_time_addr, i)?,
            price: read_i32_elem(buf, price_addr, i)?,
            recipe: read_recipe(buf, recipe_addr, i),
        });
    }
    Ok(entries)
}

pub fn write_food(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[FoodEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &food_field_names())?;
    let name_addr = offsets["NAME"];
    let quantity_addr = offsets["QUANTITY"];
    let hearts_heal_addr = offsets["HEARTS_HEAL"];
    let effect_addr = offsets["EFFECT"];
    let effect_multiplier_addr = offsets["EFFECT_MULTIPLIER"];
    let effect_time_addr = offsets["EFFECT_TIME"];
    let price_addr = offsets["PRICE"];
    let recipe_addr = offsets["RECIPE"];
    let capacity = array_capacity(buf, name_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, quantity_addr, i, entry.quantity)?;
        write_i32_elem(buf, hearts_heal_addr, i, entry.hearts_heal)?;
        write_u32_elem(buf, effect_addr, i, entry.effect)?;
        write_i32_elem(buf, effect_multiplier_addr, i, entry.effect_multiplier)?;
        write_i32_elem(buf, effect_time_addr, i, entry.effect_time)?;
        write_i32_elem(buf, price_addr, i, entry.price)?;
        write_recipe(buf, recipe_addr, i, &entry.recipe)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}
