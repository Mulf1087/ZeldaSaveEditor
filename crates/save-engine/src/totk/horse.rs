//! Owned horses (save-format prefix `OwnedHorseList.*`, a separate top-level hash namespace,
//! NOT `Pouch.*`, despite the conceptually similar struct-of-arrays shape, confirmed against
//! the real fixture). Resolved through the same
//! `totk::hashtable::scan_offsets` mechanism as everything else in this crate, with field hashes
//! computed at call time via `totk::murmur3::hash32` (see `pouch.rs`'s module doc for why).
//!
//! Reuses `pouch.rs`'s generic struct-of-arrays helpers (`array_capacity`, `element_offset`,
//! `resolve_category`, the `_elem` readers/writers) since horses share the same
//! `[u32 capacity][element_0][element_1]...` layout. Only the stride/encoding differs for three
//! fields (bit-packed bool, `u64`, `WString16`), handled by the bespoke helpers below.

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::pouch::{
    array_capacity, element_offset, read_bool_elem, read_i32_elem, read_string64_elem,
    read_u32_elem, resolve_category, write_bool_elem, write_i32_elem, write_string64_elem,
    write_u32_elem, STRIDE_I32,
};
use crate::totk::strings;

/// `OwnedHorseList.UidHash` is a `UInt64Array`: 8-byte stride, not 4.
fn read_u64_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<u64, SaveError> {
    buf.read_u64(array_addr + 4 + index * 8)
}
fn write_u64_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: u64,
) -> Result<(), SaveError> {
    buf.write_u64(array_addr + 4 + index * 8, value)
}

/// `OwnedHorseList.Name` is a `WString16Array`: 0x20-byte stride, not 0x40 (String64's stride).
fn read_wstring16_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<String, SaveError> {
    strings::read_wstring16(buf, array_addr + 4 + index * 0x20)
}
fn write_wstring16_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: &str,
) -> Result<(), SaveError> {
    strings::write_wstring16(buf, array_addr + 4 + index * 0x20, value)
}

/// `Familiarity` is a `FloatArray`: same 4-byte stride as `i32`/`u32`, `f32` encoding.
fn read_f32_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<f32, SaveError> {
    buf.read_f32(element_offset(array_addr, index, STRIDE_I32))
}
fn write_f32_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: f32,
) -> Result<(), SaveError> {
    buf.write_f32(element_offset(array_addr, index, STRIDE_I32), value)
}

pub struct HorseEntry {
    pub id: String,
    pub name: String,
    pub mane: u32,
    pub saddle: u32,
    pub rein: u32,
    pub bond: f32,
    pub bond_checked: bool,
    pub stats_strength: i32,
    pub stats_speed: i32,
    pub stats_stamina: i32,
    pub stats_pull: i32,
    pub horse_type: i32,
    pub color_type: i32,
    pub foot_type: i32,
    pub amiibo_uid_hash: u64,
    pub room_id: i32,
    pub icon_pattern: u32,
    pub icon_eye_color: u32,
    pub icon_primary_color: (u32, u32, u32),
    pub icon_secondary_color: (u32, u32, u32),
    pub icon_nose_color: (u32, u32, u32),
    pub icon_hair_primary_color: (u32, u32, u32),
    pub icon_hair_secondary_color: (u32, u32, u32),
}

/// Field names exactly as they appear in the source's `OwnedHorseList.*` hash prefix, hashed at
/// call time via `murmur3::hash32` (every hash constant here was cross-checked against the
/// real fixture).
fn horse_field_names() -> [(&'static str, &'static str); 33] {
    [
        ("OwnedHorseList.ActorName", "ID"),
        ("OwnedHorseList.Name", "NAME"),
        ("OwnedHorseList.Mane", "MANE"),
        ("OwnedHorseList.Saddle", "SADDLE"),
        ("OwnedHorseList.Rein", "REIN"),
        ("OwnedHorseList.Familiarity", "BOND"),
        ("OwnedHorseList.IsFamiliarityChecked", "BOND_CHECKED"),
        ("OwnedHorseList.Toughness", "STATS_STRENGTH"),
        ("OwnedHorseList.Speed", "STATS_SPEED"),
        ("OwnedHorseList.ChargeNum", "STATS_STAMINA"),
        ("OwnedHorseList.HorsePower", "STATS_PULL"),
        ("OwnedHorseList.HorseType", "HORSE_TYPE"),
        ("OwnedHorseList.ColorType", "COLOR_TYPE"),
        ("OwnedHorseList.FootType", "FOOT_TYPE"),
        ("OwnedHorseList.UidHash", "AMIIBO_UID_HASH"),
        ("OwnedHorseList.RoomID", "ROOM_ID"),
        ("OwnedHorseList.Body.Pattern", "ICON_PATTERN"),
        ("OwnedHorseList.Body.EyeColor", "ICON_EYE_COLOR"),
        ("OwnedHorseList.Body.PrimaryColor.Red", "ICON_PRIMARY_R"),
        ("OwnedHorseList.Body.PrimaryColor.Green", "ICON_PRIMARY_G"),
        ("OwnedHorseList.Body.PrimaryColor.Blue", "ICON_PRIMARY_B"),
        ("OwnedHorseList.Body.SecondaryColor.Red", "ICON_SECONDARY_R"),
        ("OwnedHorseList.Body.SecondaryColor.Green", "ICON_SECONDARY_G"),
        ("OwnedHorseList.Body.SecondaryColor.Blue", "ICON_SECONDARY_B"),
        ("OwnedHorseList.Body.NoseColor.Red", "ICON_NOSE_R"),
        ("OwnedHorseList.Body.NoseColor.Green", "ICON_NOSE_G"),
        ("OwnedHorseList.Body.NoseColor.Blue", "ICON_NOSE_B"),
        ("OwnedHorseList.Hair.PrimaryColor.Red", "ICON_HAIR_PRIMARY_R"),
        ("OwnedHorseList.Hair.PrimaryColor.Green", "ICON_HAIR_PRIMARY_G"),
        ("OwnedHorseList.Hair.PrimaryColor.Blue", "ICON_HAIR_PRIMARY_B"),
        ("OwnedHorseList.Hair.SecondaryColor.Red", "ICON_HAIR_SECONDARY_R"),
        ("OwnedHorseList.Hair.SecondaryColor.Green", "ICON_HAIR_SECONDARY_G"),
        ("OwnedHorseList.Hair.SecondaryColor.Blue", "ICON_HAIR_SECONDARY_B"),
    ]
}

/// Horses have no `*_valid_num` scalar in the core slice's `HASHES` table, so scan until `id`
/// (`ActorName`) is empty, same as Armor/Arrows/Materials/Food/Devices.
pub fn read_horses(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<HorseEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &horse_field_names())?;
    let id_addr = offsets["ID"];
    let name_addr = offsets["NAME"];
    let mane_addr = offsets["MANE"];
    let saddle_addr = offsets["SADDLE"];
    let rein_addr = offsets["REIN"];
    let bond_addr = offsets["BOND"];
    let bond_checked_addr = offsets["BOND_CHECKED"];
    let stats_strength_addr = offsets["STATS_STRENGTH"];
    let stats_speed_addr = offsets["STATS_SPEED"];
    let stats_stamina_addr = offsets["STATS_STAMINA"];
    let stats_pull_addr = offsets["STATS_PULL"];
    let horse_type_addr = offsets["HORSE_TYPE"];
    let color_type_addr = offsets["COLOR_TYPE"];
    let foot_type_addr = offsets["FOOT_TYPE"];
    let amiibo_uid_hash_addr = offsets["AMIIBO_UID_HASH"];
    let room_id_addr = offsets["ROOM_ID"];
    let icon_pattern_addr = offsets["ICON_PATTERN"];
    let icon_eye_color_addr = offsets["ICON_EYE_COLOR"];
    let icon_primary_r_addr = offsets["ICON_PRIMARY_R"];
    let icon_primary_g_addr = offsets["ICON_PRIMARY_G"];
    let icon_primary_b_addr = offsets["ICON_PRIMARY_B"];
    let icon_secondary_r_addr = offsets["ICON_SECONDARY_R"];
    let icon_secondary_g_addr = offsets["ICON_SECONDARY_G"];
    let icon_secondary_b_addr = offsets["ICON_SECONDARY_B"];
    let icon_nose_r_addr = offsets["ICON_NOSE_R"];
    let icon_nose_g_addr = offsets["ICON_NOSE_G"];
    let icon_nose_b_addr = offsets["ICON_NOSE_B"];
    let icon_hair_primary_r_addr = offsets["ICON_HAIR_PRIMARY_R"];
    let icon_hair_primary_g_addr = offsets["ICON_HAIR_PRIMARY_G"];
    let icon_hair_primary_b_addr = offsets["ICON_HAIR_PRIMARY_B"];
    let icon_hair_secondary_r_addr = offsets["ICON_HAIR_SECONDARY_R"];
    let icon_hair_secondary_g_addr = offsets["ICON_HAIR_SECONDARY_G"];
    let icon_hair_secondary_b_addr = offsets["ICON_HAIR_SECONDARY_B"];
    let capacity = array_capacity(buf, id_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, id_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(HorseEntry {
            id,
            name: read_wstring16_elem(buf, name_addr, i)?,
            mane: read_u32_elem(buf, mane_addr, i)?,
            saddle: read_u32_elem(buf, saddle_addr, i)?,
            rein: read_u32_elem(buf, rein_addr, i)?,
            bond: read_f32_elem(buf, bond_addr, i)?,
            bond_checked: read_bool_elem(buf, bond_checked_addr, i)?,
            stats_strength: read_i32_elem(buf, stats_strength_addr, i)?,
            stats_speed: read_i32_elem(buf, stats_speed_addr, i)?,
            stats_stamina: read_i32_elem(buf, stats_stamina_addr, i)?,
            stats_pull: read_i32_elem(buf, stats_pull_addr, i)?,
            horse_type: read_i32_elem(buf, horse_type_addr, i)?,
            color_type: read_i32_elem(buf, color_type_addr, i)?,
            foot_type: read_i32_elem(buf, foot_type_addr, i)?,
            amiibo_uid_hash: read_u64_elem(buf, amiibo_uid_hash_addr, i)?,
            room_id: read_i32_elem(buf, room_id_addr, i)?,
            icon_pattern: read_u32_elem(buf, icon_pattern_addr, i)?,
            icon_eye_color: read_u32_elem(buf, icon_eye_color_addr, i)?,
            icon_primary_color: (
                read_u32_elem(buf, icon_primary_r_addr, i)?,
                read_u32_elem(buf, icon_primary_g_addr, i)?,
                read_u32_elem(buf, icon_primary_b_addr, i)?,
            ),
            icon_secondary_color: (
                read_u32_elem(buf, icon_secondary_r_addr, i)?,
                read_u32_elem(buf, icon_secondary_g_addr, i)?,
                read_u32_elem(buf, icon_secondary_b_addr, i)?,
            ),
            icon_nose_color: (
                read_u32_elem(buf, icon_nose_r_addr, i)?,
                read_u32_elem(buf, icon_nose_g_addr, i)?,
                read_u32_elem(buf, icon_nose_b_addr, i)?,
            ),
            icon_hair_primary_color: (
                read_u32_elem(buf, icon_hair_primary_r_addr, i)?,
                read_u32_elem(buf, icon_hair_primary_g_addr, i)?,
                read_u32_elem(buf, icon_hair_primary_b_addr, i)?,
            ),
            icon_hair_secondary_color: (
                read_u32_elem(buf, icon_hair_secondary_r_addr, i)?,
                read_u32_elem(buf, icon_hair_secondary_g_addr, i)?,
                read_u32_elem(buf, icon_hair_secondary_b_addr, i)?,
            ),
        });
    }
    Ok(entries)
}

pub fn write_horses(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[HorseEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &horse_field_names())?;
    let id_addr = offsets["ID"];
    let name_addr = offsets["NAME"];
    let mane_addr = offsets["MANE"];
    let saddle_addr = offsets["SADDLE"];
    let rein_addr = offsets["REIN"];
    let bond_addr = offsets["BOND"];
    let bond_checked_addr = offsets["BOND_CHECKED"];
    let stats_strength_addr = offsets["STATS_STRENGTH"];
    let stats_speed_addr = offsets["STATS_SPEED"];
    let stats_stamina_addr = offsets["STATS_STAMINA"];
    let stats_pull_addr = offsets["STATS_PULL"];
    let horse_type_addr = offsets["HORSE_TYPE"];
    let color_type_addr = offsets["COLOR_TYPE"];
    let foot_type_addr = offsets["FOOT_TYPE"];
    let amiibo_uid_hash_addr = offsets["AMIIBO_UID_HASH"];
    let room_id_addr = offsets["ROOM_ID"];
    let icon_pattern_addr = offsets["ICON_PATTERN"];
    let icon_eye_color_addr = offsets["ICON_EYE_COLOR"];
    let icon_primary_r_addr = offsets["ICON_PRIMARY_R"];
    let icon_primary_g_addr = offsets["ICON_PRIMARY_G"];
    let icon_primary_b_addr = offsets["ICON_PRIMARY_B"];
    let icon_secondary_r_addr = offsets["ICON_SECONDARY_R"];
    let icon_secondary_g_addr = offsets["ICON_SECONDARY_G"];
    let icon_secondary_b_addr = offsets["ICON_SECONDARY_B"];
    let icon_nose_r_addr = offsets["ICON_NOSE_R"];
    let icon_nose_g_addr = offsets["ICON_NOSE_G"];
    let icon_nose_b_addr = offsets["ICON_NOSE_B"];
    let icon_hair_primary_r_addr = offsets["ICON_HAIR_PRIMARY_R"];
    let icon_hair_primary_g_addr = offsets["ICON_HAIR_PRIMARY_G"];
    let icon_hair_primary_b_addr = offsets["ICON_HAIR_PRIMARY_B"];
    let icon_hair_secondary_r_addr = offsets["ICON_HAIR_SECONDARY_R"];
    let icon_hair_secondary_g_addr = offsets["ICON_HAIR_SECONDARY_G"];
    let icon_hair_secondary_b_addr = offsets["ICON_HAIR_SECONDARY_B"];
    let capacity = array_capacity(buf, id_addr)? as usize;
    if entries.len() > capacity {
        return Err(SaveError::IndexOutOfRange { index: entries.len(), max: capacity });
    }

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, id_addr, i, &entry.id)?;
        write_wstring16_elem(buf, name_addr, i, &entry.name)?;
        write_u32_elem(buf, mane_addr, i, entry.mane)?;
        write_u32_elem(buf, saddle_addr, i, entry.saddle)?;
        write_u32_elem(buf, rein_addr, i, entry.rein)?;
        write_f32_elem(buf, bond_addr, i, entry.bond)?;
        write_bool_elem(buf, bond_checked_addr, i, entry.bond_checked)?;
        write_i32_elem(buf, stats_strength_addr, i, entry.stats_strength)?;
        write_i32_elem(buf, stats_speed_addr, i, entry.stats_speed)?;
        write_i32_elem(buf, stats_stamina_addr, i, entry.stats_stamina)?;
        write_i32_elem(buf, stats_pull_addr, i, entry.stats_pull)?;
        write_i32_elem(buf, horse_type_addr, i, entry.horse_type)?;
        write_i32_elem(buf, color_type_addr, i, entry.color_type)?;
        write_i32_elem(buf, foot_type_addr, i, entry.foot_type)?;
        write_u64_elem(buf, amiibo_uid_hash_addr, i, entry.amiibo_uid_hash)?;
        write_i32_elem(buf, room_id_addr, i, entry.room_id)?;
        write_u32_elem(buf, icon_pattern_addr, i, entry.icon_pattern)?;
        write_u32_elem(buf, icon_eye_color_addr, i, entry.icon_eye_color)?;
        write_u32_elem(buf, icon_primary_r_addr, i, entry.icon_primary_color.0)?;
        write_u32_elem(buf, icon_primary_g_addr, i, entry.icon_primary_color.1)?;
        write_u32_elem(buf, icon_primary_b_addr, i, entry.icon_primary_color.2)?;
        write_u32_elem(buf, icon_secondary_r_addr, i, entry.icon_secondary_color.0)?;
        write_u32_elem(buf, icon_secondary_g_addr, i, entry.icon_secondary_color.1)?;
        write_u32_elem(buf, icon_secondary_b_addr, i, entry.icon_secondary_color.2)?;
        write_u32_elem(buf, icon_nose_r_addr, i, entry.icon_nose_color.0)?;
        write_u32_elem(buf, icon_nose_g_addr, i, entry.icon_nose_color.1)?;
        write_u32_elem(buf, icon_nose_b_addr, i, entry.icon_nose_color.2)?;
        write_u32_elem(buf, icon_hair_primary_r_addr, i, entry.icon_hair_primary_color.0)?;
        write_u32_elem(buf, icon_hair_primary_g_addr, i, entry.icon_hair_primary_color.1)?;
        write_u32_elem(buf, icon_hair_primary_b_addr, i, entry.icon_hair_primary_color.2)?;
        write_u32_elem(buf, icon_hair_secondary_r_addr, i, entry.icon_hair_secondary_color.0)?;
        write_u32_elem(buf, icon_hair_secondary_g_addr, i, entry.icon_hair_secondary_color.1)?;
        write_u32_elem(buf, icon_hair_secondary_b_addr, i, entry.icon_hair_secondary_color.2)?;
    }
    for i in entries.len()..capacity {
        write_string64_elem(buf, id_addr, i, "")?;
    }
    Ok(())
}
