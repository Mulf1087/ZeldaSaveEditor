pub mod abilities;
pub mod autobuilder;
pub mod caption;
pub mod completism;
pub mod completism_data;
pub mod guids;
pub mod hashbrowser;
pub mod hashdict;
pub mod hashtable;
pub mod horse;
pub mod mapdata;
pub mod murmur3;
pub mod pouch;
pub mod strings;
pub mod versions;

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;

/// Hash -> (field name, is_pointer) table, ported from `zelda-totk.js`'s `Hashes` array,
/// restricted to the 11 hashes this crate exposes (MapData icons, AutoBuilder, and everything
/// else in the source's 18-entry table are out of scope here). Unlike BOTW's `HASHES`, this
/// table has no ordering requirement, since
/// `hashtable::scan_offsets` looks each hash up in a map, not via a moving cursor.
const HASHES: [(u32, &str, bool); 11] = [
    (0xfbe01da1, "MAX_LIFE", false),
    (0xa77921d7, "CURRENT_RUPEES", false),
    (0xf9212c74, "MAX_STAMINA", false),
    (0x15ec5858, "HORSE_INN_MEMBER_POINT", false),
    (0xe573f564, "PLAYTIME", false),
    (0xafd01d68, "MAX_ENERGY", false),
    (0xc884818d, "SAVE_POS", true),
    (0x1d6189da, "SEQUENCE_CURRENT_BANC", true),
    (0xd7a3f6ba, "POUCH_WEAPON_VALID_NUM", true),
    (0xc61785c2, "POUCH_BOW_VALID_NUM", true),
    (0x05271e7d, "POUCH_SHIELD_VALID_NUM", true),
];

pub struct TotkSave {
    buf: SaveBuffer,
    offsets: HashMap<&'static str, usize>,
    hash_table_end: usize,
    pub version_label: &'static str,
    pub modded: bool,
}

impl TotkSave {
    pub fn load(bytes: Vec<u8>) -> Result<Self, SaveError> {
        versions::check_magic_and_size(&bytes)?;
        let (version_label, modded) = versions::label(&bytes);
        let mut buf = SaveBuffer::new(bytes);
        buf.little_endian = true;
        let hash_table_end = hashtable::find_hash_table_end(&buf)?;
        let offsets = hashtable::scan_offsets(&buf, hash_table_end, &HASHES)?;
        Ok(TotkSave {
            buf,
            offsets,
            hash_table_end,
            version_label,
            modded,
        })
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buf.into_bytes()
    }

    fn offset(&self, name: &'static str) -> Result<usize, SaveError> {
        self.offsets.get(name).copied().ok_or(SaveError::MissingField(name))
    }

    pub fn max_life(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_LIFE")?)
    }
    pub fn set_max_life(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_LIFE")?;
        self.buf.write_u32(o, val)
    }

    pub fn current_rupees(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("CURRENT_RUPEES")?)
    }
    pub fn set_current_rupees(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("CURRENT_RUPEES")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_stamina(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_STAMINA")?)
    }
    pub fn set_max_stamina(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_STAMINA")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_energy(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_ENERGY")?)
    }
    pub fn set_max_energy(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_ENERGY")?;
        self.buf.write_u32(o, val)
    }

    /// The source itself marks this hash's meaning as unconfirmed ("unknown key" in its
    /// own comment). Exposed under its source name rather than asserting false
    /// confidence about what it represents.
    pub fn playtime(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("PLAYTIME")?)
    }
    pub fn set_playtime(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("PLAYTIME")?;
        self.buf.write_u32(o, val)
    }

    pub fn horse_inn_member_point(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("HORSE_INN_MEMBER_POINT")?)
    }
    pub fn set_horse_inn_member_point(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("HORSE_INN_MEMBER_POINT")?;
        self.buf.write_u32(o, val)
    }

    pub fn save_pos(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("SAVE_POS")?;
        Ok((self.buf.read_f32(o)?, self.buf.read_f32(o + 4)?, self.buf.read_f32(o + 8)?))
    }
    pub fn set_save_pos(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("SAVE_POS")?;
        self.buf.write_f32(o, x)?;
        self.buf.write_f32(o + 4, y)?;
        self.buf.write_f32(o + 8, z)?;
        Ok(())
    }

    pub fn sequence_current_banc(&self) -> Result<String, SaveError> {
        let o = self.offset("SEQUENCE_CURRENT_BANC")?;
        Ok(strings::read_string64(&self.buf, o))
    }
    pub fn set_sequence_current_banc(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("SEQUENCE_CURRENT_BANC")?;
        strings::write_string64(&mut self.buf, o, value)
    }

    pub fn pouch_weapon_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_WEAPON_VALID_NUM")?)
    }
    pub fn set_pouch_weapon_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_WEAPON_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_bow_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_BOW_VALID_NUM")?)
    }
    pub fn set_pouch_bow_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_BOW_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_shield_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_SHIELD_VALID_NUM")?)
    }
    pub fn set_pouch_shield_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_SHIELD_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_weapons(&self) -> Result<Vec<pouch::WeaponEntry>, SaveError> {
        pouch::read_weapons(&self.buf, self.hash_table_end, self.pouch_weapon_valid_num()?)
    }
    pub fn set_pouch_weapons(&mut self, entries: &[pouch::WeaponEntry]) -> Result<(), SaveError> {
        pouch::write_weapons(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_weapon_valid_num(entries.len() as u32)
    }

    pub fn pouch_bows(&self) -> Result<Vec<pouch::BowEntry>, SaveError> {
        pouch::read_bows(&self.buf, self.hash_table_end, self.pouch_bow_valid_num()?)
    }
    pub fn set_pouch_bows(&mut self, entries: &[pouch::BowEntry]) -> Result<(), SaveError> {
        pouch::write_bows(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_bow_valid_num(entries.len() as u32)
    }

    pub fn pouch_shields(&self) -> Result<Vec<pouch::ShieldEntry>, SaveError> {
        pouch::read_shields(&self.buf, self.hash_table_end, self.pouch_shield_valid_num()?)
    }
    pub fn set_pouch_shields(&mut self, entries: &[pouch::ShieldEntry]) -> Result<(), SaveError> {
        pouch::write_shields(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_shield_valid_num(entries.len() as u32)
    }

    pub fn armor(&self) -> Result<Vec<pouch::ArmorEntry>, SaveError> {
        pouch::read_armor(&self.buf, self.hash_table_end)
    }
    pub fn set_armor(&mut self, entries: &[pouch::ArmorEntry]) -> Result<(), SaveError> {
        pouch::write_armor(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn arrows(&self) -> Result<Vec<pouch::ArrowEntry>, SaveError> {
        pouch::read_arrows(&self.buf, self.hash_table_end)
    }
    pub fn set_arrows(&mut self, entries: &[pouch::ArrowEntry]) -> Result<(), SaveError> {
        pouch::write_arrows(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn materials(&self) -> Result<Vec<pouch::MaterialEntry>, SaveError> {
        pouch::read_materials(&self.buf, self.hash_table_end)
    }
    pub fn set_materials(&mut self, entries: &[pouch::MaterialEntry]) -> Result<(), SaveError> {
        pouch::write_materials(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn key_items(&self) -> Result<Vec<pouch::KeyItemEntry>, SaveError> {
        pouch::read_key_items(&self.buf, self.hash_table_end)
    }
    pub fn set_key_items(&mut self, entries: &[pouch::KeyItemEntry]) -> Result<(), SaveError> {
        pouch::write_key_items(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn devices(&self) -> Result<Vec<pouch::DeviceEntry>, SaveError> {
        pouch::read_devices(&self.buf, self.hash_table_end)
    }
    pub fn set_devices(&mut self, entries: &[pouch::DeviceEntry]) -> Result<(), SaveError> {
        pouch::write_devices(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn food(&self) -> Result<Vec<pouch::FoodEntry>, SaveError> {
        pouch::read_food(&self.buf, self.hash_table_end)
    }
    pub fn set_food(&mut self, entries: &[pouch::FoodEntry]) -> Result<(), SaveError> {
        pouch::write_food(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn horses(&self) -> Result<Vec<horse::HorseEntry>, SaveError> {
        horse::read_horses(&self.buf, self.hash_table_end)
    }
    pub fn set_horses(&mut self, entries: &[horse::HorseEntry]) -> Result<(), SaveError> {
        horse::write_horses(&mut self.buf, self.hash_table_end, entries)
    }

    // --- completionism (read-only: the source tool has no mass-unlock for these) ---

    pub fn shrines_found(&self) -> Result<usize, SaveError> {
        completism::shrines_found(&self.buf, self.hash_table_end)
    }
    pub fn shrines_cleared(&self) -> Result<usize, SaveError> {
        completism::shrines_cleared(&self.buf, self.hash_table_end)
    }
    pub fn koroks_hidden(&self) -> Result<usize, SaveError> {
        completism::koroks_hidden(&self.buf, self.hash_table_end)
    }
    pub fn koroks_carried(&self) -> Result<usize, SaveError> {
        completism::koroks_carried(&self.buf, self.hash_table_end)
    }
    pub fn locations_visited(&self) -> Result<usize, SaveError> {
        completism::locations_visited(&self.buf, self.hash_table_end)
    }
    pub fn defeated_hinox(&self) -> Result<usize, SaveError> {
        completism::defeated_hinox(&self.buf, self.hash_table_end)
    }
    pub fn defeated_talus(&self) -> Result<usize, SaveError> {
        completism::defeated_talus(&self.buf, self.hash_table_end)
    }
    pub fn defeated_molduga(&self) -> Result<usize, SaveError> {
        completism::defeated_molduga(&self.buf, self.hash_table_end)
    }
    pub fn defeated_bubbuls(&self) -> Result<usize, SaveError> {
        completism::defeated_bubbuls(&self.buf, self.hash_table_end)
    }
    pub fn sage_wills_found(&self) -> Result<usize, SaveError> {
        completism::sage_wills_found(&self.buf, self.hash_table_end)
    }
    pub fn old_maps_found(&self) -> Result<usize, SaveError> {
        completism::old_maps_found(&self.buf, self.hash_table_end)
    }
    pub fn addison_completed(&self) -> Result<usize, SaveError> {
        completism::addison_completed(&self.buf, self.hash_table_end)
    }
    pub fn unlock_all_bubbuls(&mut self) -> Result<usize, SaveError> {
        completism::unlock_all_bubbuls(&mut self.buf, self.hash_table_end)
    }
    pub fn unlock_all_sage_wills(&mut self) -> Result<usize, SaveError> {
        completism::unlock_all_sage_wills(&mut self.buf, self.hash_table_end)
    }
    pub fn unlock_all_addison(&mut self) -> Result<usize, SaveError> {
        completism::unlock_all_addison(&mut self.buf, self.hash_table_end)
    }
    pub fn completism(&self) -> Result<Vec<crate::completism::CompletismCategory>, SaveError> {
        completism::snapshot(&self.buf, self.hash_table_end)
    }
    pub fn set_completism(&mut self, id: &str, metric: usize) -> Result<usize, SaveError> {
        completism::set_all(&mut self.buf, self.hash_table_end, id, metric)
    }

    // --- AutoBuild ---

    pub fn autobuilds(&self) -> Result<Vec<autobuilder::AutoBuildEntry>, SaveError> {
        autobuilder::read_autobuilds(&self.buf, self.hash_table_end)
    }
    pub fn set_autobuilds(&mut self, entries: &[autobuilder::AutoBuildEntry]) -> Result<(), SaveError> {
        autobuilder::write_autobuilds(&mut self.buf, self.hash_table_end, entries)
    }

    // --- Map pins ---

    pub fn map_pins(&self) -> Result<Vec<mapdata::MapPinEntry>, SaveError> {
        mapdata::read_map_pins(&self.buf, self.hash_table_end)
    }
    pub fn set_map_pins(&mut self, entries: &[mapdata::MapPinEntry]) -> Result<(), SaveError> {
        mapdata::write_map_pins(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn map_markers(&self) -> Result<Vec<mapdata::MapMarkerEntry>, SaveError> {
        mapdata::read_map_markers(&self.buf, self.hash_table_end)
    }
    pub fn set_map_markers(&mut self, entries: &[mapdata::MapMarkerEntry]) -> Result<(), SaveError> {
        mapdata::write_map_markers(&mut self.buf, self.hash_table_end, entries)
    }

    pub fn teleporters(&self) -> Result<Vec<mapdata::TeleporterEntry>, SaveError> {
        mapdata::read_teleporters(&self.buf, self.hash_table_end)
    }
    pub fn set_teleporters(&mut self, entries: &[mapdata::TeleporterEntry]) -> Result<(), SaveError> {
        mapdata::write_teleporters(&mut self.buf, self.hash_table_end, entries)
    }

    // --- Advanced: generic hash browser ---

    pub fn browse_hashes(&self) -> Result<Vec<hashbrowser::HashRow>, SaveError> {
        hashbrowser::browse(&self.buf, self.hash_table_end)
    }
    pub fn set_hash_field(&mut self, hash: u32, value: f64) -> Result<(), SaveError> {
        hashbrowser::write_scalar(&mut self.buf, self.hash_table_end, hash, value)
    }

    // --- Key-item ability toggles ---

    pub fn abilities(&self) -> Result<Vec<abilities::AbilityState>, SaveError> {
        abilities::read_abilities(&self.buf, self.hash_table_end)
    }
    pub fn set_ability(&mut self, id: &str, on: bool) -> Result<(), SaveError> {
        abilities::set_ability(&mut self.buf, self.hash_table_end, id, on)
    }
}
