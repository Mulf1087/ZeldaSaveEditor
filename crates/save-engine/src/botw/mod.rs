pub mod completism;
pub mod items;
pub mod strings;
pub mod versions;

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::hashtable::scan_offsets;

pub const MAX_ITEMS: usize = 420;

/// Hash -> field name table, ported from `zelda-botw.js`'s `Hashes` array. MUST stay
/// sorted in ascending hash order, see `hashtable::scan_offsets`'s doc for why.
const HASHES: [(u32, &str); 33] = [
    (0x0bee9e46, "MAP"),
    (0x0cbf052a, "FLAGS_BOW"),
    (0x1e3fd294, "FLAGSV_BOW"),
    (0x23149bf8, "RUPEES"),
    (0x2906f327, "MAX_HEARTS"),
    (0x333aa6e5, "HORSE_SADDLES"),
    (0x3adff047, "MAX_STAMINA"),
    (0x441b7231, "DEFEATED_MOLDUGA_COUNTER"),
    (0x54679940, "DEFEATED_HINOX_COUNTER"),
    (0x57ee221d, "FLAGS_WEAPON"),
    (0x5f283289, "ITEMS"),
    (0x6150c6be, "HORSE_REINS"),
    (0x698266be, "DEFEATED_TALUS_COUNTER"),
    (0x69f17e8a, "FLAGSV_SHIELD"),
    (0x6a09fc59, "ITEMS_QUANTITY"),
    (0x73c29681, "PLAYTIME"),
    (0x7b74e117, "HORSE_NAMES"),
    (0x8a94e07a, "KOROK_SEED_COUNTER"),
    (0x9383490e, "MapApp_MapIconNo"),
    (0x97f925c3, "RELIC_GERUDO"),
    (0x982ba201, "HORSE_POSITION"),
    (0x9c6cfd3f, "HORSE_MANES"),
    (0xa40ba103, "PLAYER_POSITION"),
    (0xa6d926bc, "FLAGSV_WEAPON"),
    (0xc247b696, "HORSE_TYPES"),
    (0xc5238d2b, "FLAGS_SHIELD"),
    (0xc9328299, "MOTORCYCLE"),
    (0xce7afed3, "MONS"),
    (0xd913b769, "MAPTYPE"),
    (0xe1a0ca54, "HORSE_BONDS"),
    (0xea9def3f, "MapApp_MapIconPos"),
    (0xf1cf4807, "RELIC_GORON"),
    (0xfda0cde4, "RELIC_RITO"),
];

pub struct BotwSave {
    buf: SaveBuffer,
    offsets: HashMap<&'static str, usize>,
    pub version_index: usize,
    pub modded: bool,
}

impl BotwSave {
    pub fn load(bytes: Vec<u8>) -> Result<Self, SaveError> {
        let detected = versions::detect(&bytes).ok_or(SaveError::UnknownFormat)?;
        let mut buf = SaveBuffer::new(bytes);
        buf.little_endian = detected.little_endian;
        let offsets = scan_offsets(&buf, &HASHES);
        Ok(BotwSave {
            buf,
            offsets,
            version_index: detected.index,
            modded: detected.modded,
        })
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buf.into_bytes()
    }

    fn offset(&self, name: &'static str) -> Result<usize, SaveError> {
        self.offsets.get(name).copied().ok_or(SaveError::MissingField(name))
    }

    // --- scalar stats ---

    pub fn rupees(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("RUPEES")?)
    }
    pub fn set_rupees(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RUPEES")?;
        self.buf.write_u32(o, val)
    }

    pub fn mons(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MONS")?)
    }
    pub fn set_mons(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MONS")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_hearts(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_HEARTS")?)
    }
    pub fn set_max_hearts(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_HEARTS")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_stamina(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_STAMINA")?)
    }
    pub fn set_max_stamina(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_STAMINA")?;
        self.buf.write_u32(o, val)
    }

    pub fn relic_gerudo(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("RELIC_GERUDO")?)
    }
    pub fn set_relic_gerudo(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_GERUDO")?;
        self.buf.write_u32(o, val)
    }

    pub fn relic_goron(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("RELIC_GORON")?)
    }
    pub fn set_relic_goron(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_GORON")?;
        self.buf.write_u32(o, val)
    }

    pub fn relic_rito(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("RELIC_RITO")?)
    }
    pub fn set_relic_rito(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_RITO")?;
        self.buf.write_u32(o, val)
    }

    pub fn korok_seed_counter(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("KOROK_SEED_COUNTER")?)
    }
    pub fn set_korok_seed_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("KOROK_SEED_COUNTER")?;
        self.buf.write_u32(o, val)
    }

    pub fn defeated_hinox_counter(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("DEFEATED_HINOX_COUNTER")?)
    }
    pub fn set_defeated_hinox_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_HINOX_COUNTER")?;
        self.buf.write_u32(o, val)
    }

    pub fn defeated_talus_counter(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("DEFEATED_TALUS_COUNTER")?)
    }
    pub fn set_defeated_talus_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_TALUS_COUNTER")?;
        self.buf.write_u32(o, val)
    }

    pub fn defeated_molduga_counter(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("DEFEATED_MOLDUGA_COUNTER")?)
    }
    pub fn set_defeated_molduga_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_MOLDUGA_COUNTER")?;
        self.buf.write_u32(o, val)
    }

    /// Raw seconds. Formatting into `H:MM:SS` (`_timeToString` upstream) is a UI concern.
    pub fn playtime_seconds(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("PLAYTIME")?)
    }
    pub fn set_playtime_seconds(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("PLAYTIME")?;
        self.buf.write_u32(o, val)
    }

    /// Not present in every save version, hence `Option` rather than `Result`. The single
    /// read/write below is at a hash-verified offset, which `scan_offsets` already
    /// guarantees is `offset + 4 <= len`, which is bounds-safe by construction, so a bounds
    /// failure here would indicate a bug in `scan_offsets`, not untrusted input.
    pub fn motorcycle(&self) -> Option<bool> {
        self.offsets
            .get("MOTORCYCLE")
            .map(|&o| self.buf.read_u32(o).expect("hash-verified offset is in bounds") != 0)
    }
    pub fn set_motorcycle(&mut self, val: bool) {
        if let Some(&o) = self.offsets.get("MOTORCYCLE") {
            self.buf
                .write_u32(o, if val { 1 } else { 0 })
                .expect("hash-verified offset is in bounds");
        }
    }

    // --- positions ---

    pub fn player_position(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("PLAYER_POSITION")?;
        Ok((self.buf.read_f32(o)?, self.buf.read_f32(o + 8)?, self.buf.read_f32(o + 16)?))
    }
    pub fn set_player_position(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("PLAYER_POSITION")?;
        self.buf.write_f32(o, x)?;
        self.buf.write_f32(o + 8, y)?;
        self.buf.write_f32(o + 16, z)?;
        Ok(())
    }

    pub fn horse_position(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("HORSE_POSITION")?;
        Ok((self.buf.read_f32(o)?, self.buf.read_f32(o + 8)?, self.buf.read_f32(o + 16)?))
    }
    pub fn set_horse_position(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("HORSE_POSITION")?;
        self.buf.write_f32(o, x)?;
        self.buf.write_f32(o + 8, y)?;
        self.buf.write_f32(o + 16, z)?;
        Ok(())
    }

    pub fn map(&self) -> Result<String, SaveError> {
        let o = self.offset("MAP")?;
        strings::read_padded_string(&self.buf, o, 8)
    }
    pub fn set_map(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("MAP")?;
        strings::write_padded_string(&mut self.buf, o, value, 8)
    }

    pub fn map_type(&self) -> Result<String, SaveError> {
        let o = self.offset("MAPTYPE")?;
        strings::read_padded_string(&self.buf, o, 8)
    }
    pub fn set_map_type(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("MAPTYPE")?;
        strings::write_padded_string(&mut self.buf, o, value, 8)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BotwItem {
    pub name: String,
    pub quantity: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemKind {
    Weapon,
    Bow,
    Shield,
    Other,
}

/// Classifies an item name id exactly enough to reproduce the upstream modifier-slot
/// counting logic. Display categorization (armor/materials/food/other) is a UI concern
/// and is intentionally not reproduced here.
fn classify(name: &str) -> ItemKind {
    if name.starts_with("Weapon_Sword_") || name.starts_with("Weapon_Lsword_") || name.starts_with("Weapon_Spear_") {
        ItemKind::Weapon
    } else if name.starts_with("Weapon_Bow_")
        || matches!(name, "NormalArrow" | "FireArrow" | "IceArrow" | "ElectricArrow" | "BombArrow_A" | "AncientArrow")
    {
        ItemKind::Bow
    } else if name.starts_with("Weapon_Shield_") {
        ItemKind::Shield
    } else {
        ItemKind::Other
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemModifier {
    pub modifier: u32,
    pub value: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum ModifierCategory {
    Weapon,
    Bow,
    Shield,
}

impl BotwSave {
    /// Reads item slots until the first empty name (or `MAX_ITEMS`), mirroring `load()`'s
    /// item loop.
    pub fn items(&self) -> Result<Vec<BotwItem>, SaveError> {
        let items_offset = self.offset("ITEMS")?;
        let qty_offset = self.offset("ITEMS_QUANTITY")?;
        let mut result = Vec::new();
        for i in 0..MAX_ITEMS {
            let name = strings::read_string64(&self.buf, items_offset, i)?;
            if name.is_empty() {
                break;
            }
            let quantity = self.buf.read_u32(qty_offset + i * 8)?;
            result.push(BotwItem { name, quantity });
        }
        Ok(result)
    }

    pub fn set_item(&mut self, index: usize, name: &str, quantity: u32) -> Result<(), SaveError> {
        if index >= MAX_ITEMS {
            return Err(SaveError::IndexOutOfRange { index, max: MAX_ITEMS });
        }
        let items_offset = self.offset("ITEMS")?;
        let qty_offset = self.offset("ITEMS_QUANTITY")?;
        strings::write_string64(&mut self.buf, items_offset, index, name)?;
        self.buf.write_u32(qty_offset + index * 8, quantity)?;
        Ok(())
    }

    /// Pairs each item from `items()` with its display category (weapons/bows/shields/armor/
    /// materials/food/key items), 1:1 in the same order (index `i` here is still the same
    /// slot `set_item(i, ...)` writes to). For armor items, `BotwItem::quantity` is really a dye
    /// color index rather than a count (same shared storage field, different meaning based on
    /// category, see `items::categorize` doc); this crate doesn't split that into a separate
    /// type, mirroring how the source reinterprets the one stored value per category at render
    /// time rather than storing it differently.
    pub fn items_with_category(&self) -> Result<Vec<(BotwItem, items::ItemCategory)>, SaveError> {
        Ok(self
            .items()?
            .into_iter()
            .map(|item| {
                let category = items::categorize(&item.name);
                (item, category)
            })
            .collect())
    }

    /// Returns (weapon, bow, shield) modifier lists. Slot counts are derived by walking
    /// the item list and tracking which contiguous category block is currently active,
    /// ported 1:1 from the upstream `search` state machine in `load()`.
    /// Maps each item (by global slot) to the modifier slot it owns, if any. Weapons, bows and
    /// shields each occupy a contiguous leading block; within a block the position is the index
    /// into that category's parallel modifier array. Items with no modifier (or past the blocks)
    /// map to `None`. This is the single source of truth for both reading modifiers and keeping
    /// them aligned when an item is added or removed.
    fn item_modifier_slots(&self, items: &[BotwItem]) -> Vec<Option<(ModifierCategory, usize)>> {
        let mut counts = [0usize; 3]; // weapon, bow, shield
        let mut search = 0u8; // 0: weapons, 1: bows, 2: shields, 3: done
        let mut out = Vec::with_capacity(items.len());

        for item in items {
            let kind = classify(&item.name);
            if search == 0 && kind == ItemKind::Bow {
                search = 1;
            } else if search == 0 && kind == ItemKind::Shield {
                search = 2;
            } else if search == 1 && kind == ItemKind::Shield {
                search = 2;
            } else if kind == ItemKind::Other {
                search = 3;
            }

            let slot = if kind == ItemKind::Weapon && search == 0 {
                let p = counts[0];
                counts[0] += 1;
                Some((ModifierCategory::Weapon, p))
            } else if kind == ItemKind::Bow && search == 1 && item.name.starts_with("Weapon_") {
                let p = counts[1];
                counts[1] += 1;
                Some((ModifierCategory::Bow, p))
            } else if kind == ItemKind::Shield && search == 2 {
                let p = counts[2];
                counts[2] += 1;
                Some((ModifierCategory::Shield, p))
            } else {
                None
            };
            out.push(slot);
        }
        out
    }

    pub fn modifiers(&self) -> Result<(Vec<ItemModifier>, Vec<ItemModifier>, Vec<ItemModifier>), SaveError> {
        let items = self.items()?;
        let mut counts = [0usize; 3]; // weapon, bow, shield
        for slot in self.item_modifier_slots(&items) {
            match slot {
                Some((ModifierCategory::Weapon, _)) => counts[0] += 1,
                Some((ModifierCategory::Bow, _)) => counts[1] += 1,
                Some((ModifierCategory::Shield, _)) => counts[2] += 1,
                None => {}
            }
        }

        Ok((
            self.read_modifier_slots("FLAGS_WEAPON", "FLAGSV_WEAPON", counts[0])?,
            self.read_modifier_slots("FLAGS_BOW", "FLAGSV_BOW", counts[1])?,
            self.read_modifier_slots("FLAGS_SHIELD", "FLAGSV_SHIELD", counts[2])?,
        ))
    }

    fn read_modifier_slots(
        &self,
        flag_hash: &'static str,
        value_hash: &'static str,
        count: usize,
    ) -> Result<Vec<ItemModifier>, SaveError> {
        let flag_offset = self.offset(flag_hash)?;
        let value_offset = self.offset(value_hash)?;
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            out.push(ItemModifier {
                modifier: self.buf.read_u32(flag_offset + i * 8)?,
                value: self.buf.read_u32(value_offset + i * 8)?,
            });
        }
        Ok(out)
    }

    pub fn set_modifier(
        &mut self,
        category: ModifierCategory,
        index: usize,
        modifier: u32,
        value: u32,
    ) -> Result<(), SaveError> {
        if index >= MAX_ITEMS {
            return Err(SaveError::IndexOutOfRange { index, max: MAX_ITEMS });
        }
        let (flag_hash, value_hash) = match category {
            ModifierCategory::Weapon => ("FLAGS_WEAPON", "FLAGSV_WEAPON"),
            ModifierCategory::Bow => ("FLAGS_BOW", "FLAGSV_BOW"),
            ModifierCategory::Shield => ("FLAGS_SHIELD", "FLAGSV_SHIELD"),
        };
        let flag_offset = self.offset(flag_hash)?;
        let value_offset = self.offset(value_hash)?;
        self.buf.write_u32(flag_offset + index * 8, modifier)?;
        self.buf.write_u32(value_offset + index * 8, value)?;
        Ok(())
    }

    /// Writes `items` back into the flat inventory in order and clears the slot just past the end
    /// so the array stays null-terminated (the read loop stops at the first empty name).
    fn write_items(&mut self, items: &[BotwItem]) -> Result<(), SaveError> {
        for (i, item) in items.iter().enumerate() {
            self.set_item(i, &item.name, item.quantity)?;
        }
        if items.len() < MAX_ITEMS {
            self.set_item(items.len(), "", 0)?;
        }
        Ok(())
    }

    fn write_modifiers(
        &mut self,
        weapons: &[ItemModifier],
        bows: &[ItemModifier],
        shields: &[ItemModifier],
    ) -> Result<(), SaveError> {
        for (i, m) in weapons.iter().enumerate() {
            self.set_modifier(ModifierCategory::Weapon, i, m.modifier, m.value)?;
        }
        for (i, m) in bows.iter().enumerate() {
            self.set_modifier(ModifierCategory::Bow, i, m.modifier, m.value)?;
        }
        for (i, m) in shields.iter().enumerate() {
            self.set_modifier(ModifierCategory::Shield, i, m.modifier, m.value)?;
        }
        Ok(())
    }

    /// Removes the item at `index`, shifting later items down one so the inventory stays
    /// contiguous. If the removed item carried a weapon/bow/shield modifier, its slot is pulled
    /// out of the matching modifier array too so the remaining bonuses don't shift onto the wrong
    /// items.
    pub fn remove_item(&mut self, index: usize) -> Result<(), SaveError> {
        let mut items = self.items()?;
        if index >= items.len() {
            return Err(SaveError::IndexOutOfRange { index, max: items.len() });
        }
        let removed = self.item_modifier_slots(&items)[index];
        let (mut weapons, mut bows, mut shields) = self.modifiers()?;
        if let Some((cat, pos)) = removed {
            match cat {
                ModifierCategory::Weapon => drop(weapons.remove(pos)),
                ModifierCategory::Bow => drop(bows.remove(pos)),
                ModifierCategory::Shield => drop(shields.remove(pos)),
            }
        }
        items.remove(index);
        self.write_items(&items)?;
        self.write_modifiers(&weapons, &bows, &shields)?;
        Ok(())
    }

    /// Inserts a copy of the item at `index` immediately after it, shifting later items (and, for
    /// weapon/bow/shield, the matching modifier slot) up one. Fails if the inventory is already at
    /// `MAX_ITEMS`.
    pub fn duplicate_item(&mut self, index: usize) -> Result<(), SaveError> {
        let mut items = self.items()?;
        if index >= items.len() {
            return Err(SaveError::IndexOutOfRange { index, max: items.len() });
        }
        if items.len() >= MAX_ITEMS {
            return Err(SaveError::IndexOutOfRange { index: items.len(), max: MAX_ITEMS });
        }
        let source = self.item_modifier_slots(&items)[index];
        let (mut weapons, mut bows, mut shields) = self.modifiers()?;
        if let Some((cat, pos)) = source {
            match cat {
                ModifierCategory::Weapon => weapons.insert(pos, weapons[pos]),
                ModifierCategory::Bow => bows.insert(pos, bows[pos]),
                ModifierCategory::Shield => shields.insert(pos, shields[pos]),
            }
        }
        let copy = items[index].clone();
        items.insert(index + 1, copy);
        self.write_items(&items)?;
        self.write_modifiers(&weapons, &bows, &shields)?;
        Ok(())
    }
}

pub const NUM_HORSE_SLOTS: usize = 6;

#[derive(Clone, Debug, PartialEq)]
pub struct BotwHorse {
    pub name: Option<String>,
    pub saddle: Option<String>,
    pub reins: Option<String>,
    pub horse_type: String,
}

impl BotwSave {
    pub fn horses(&self) -> Result<Vec<BotwHorse>, SaveError> {
        let names_offset = self.offset("HORSE_NAMES")?;
        let saddles_offset = self.offset("HORSE_SADDLES")?;
        let reins_offset = self.offset("HORSE_REINS")?;
        let types_offset = self.offset("HORSE_TYPES")?;

        // Slots 0-4 are the five named, stabled horses. Slot 5 is the source's "Untamed
        // horse" slot (the wild horse Link is currently riding but hasn't registered at a
        // stable), which only ever has a type, never a name/saddle/reins.
        let mut out = Vec::with_capacity(NUM_HORSE_SLOTS);
        for i in 0..NUM_HORSE_SLOTS {
            let horse_type = strings::read_string64(&self.buf, types_offset, i)?;
            if i < 5 {
                out.push(BotwHorse {
                    name: Some(strings::read_string64(&self.buf, names_offset, i)?),
                    saddle: Some(strings::read_string64(&self.buf, saddles_offset, i)?),
                    reins: Some(strings::read_string64(&self.buf, reins_offset, i)?),
                    horse_type,
                });
            } else {
                out.push(BotwHorse {
                    name: None,
                    saddle: None,
                    reins: None,
                    horse_type,
                });
            }
        }
        Ok(out)
    }

    pub fn set_horse_name(&mut self, index: usize, value: &str) -> Result<(), SaveError> {
        if index >= 5 {
            return Err(SaveError::IndexOutOfRange { index, max: 5 });
        }
        let o = self.offset("HORSE_NAMES")?;
        strings::write_string64(&mut self.buf, o, index, value)
    }

    pub fn set_horse_saddle(&mut self, index: usize, value: &str) -> Result<(), SaveError> {
        if index >= 5 {
            return Err(SaveError::IndexOutOfRange { index, max: 5 });
        }
        let o = self.offset("HORSE_SADDLES")?;
        strings::write_string64(&mut self.buf, o, index, value)
    }

    pub fn set_horse_reins(&mut self, index: usize, value: &str) -> Result<(), SaveError> {
        if index >= 5 {
            return Err(SaveError::IndexOutOfRange { index, max: 5 });
        }
        let o = self.offset("HORSE_REINS")?;
        strings::write_string64(&mut self.buf, o, index, value)
    }

    pub fn set_horse_type(&mut self, index: usize, value: &str) -> Result<(), SaveError> {
        if index >= NUM_HORSE_SLOTS {
            return Err(SaveError::IndexOutOfRange { index, max: NUM_HORSE_SLOTS });
        }
        let o = self.offset("HORSE_TYPES")?;
        strings::write_string64(&mut self.buf, o, index, value)
    }

    /// Marks every unfound korok as found. Returns how many were newly found. Also mirrors
    /// `unlockKoroks`'s two side effects: sets the `HiddenKorok_Complete` flag (always, not
    /// gated on the count), and bumps its quantity by the same count if an `Obj_KorokNuts` item
    /// already exists in the inventory, so the pouch total stays consistent
    /// with the newly-found seeds.
    pub fn unlock_all_koroks(&mut self) -> Result<usize, SaveError> {
        let count = completism::unlock_all_koroks(&mut self.buf)? as u32;
        let current = self.korok_seed_counter()?;
        self.set_korok_seed_counter(current + count)?;
        let items = self.items()?;
        if let Some(index) = items.iter().position(|item| item.name == "Obj_KorokNuts") {
            self.set_item(index, "Obj_KorokNuts", items[index].quantity + count)?;
        }
        Ok(count as usize)
    }

    /// Marks every undefeated Hinox as defeated and bumps `defeated_hinox_counter` by the
    /// same amount. Returns how many were newly defeated.
    pub fn unlock_all_defeated_hinox(&mut self) -> Result<usize, SaveError> {
        let count = completism::unlock_all_defeated_hinox(&mut self.buf)? as u32;
        let current = self.defeated_hinox_counter()?;
        self.set_defeated_hinox_counter(current + count)?;
        Ok(count as usize)
    }

    /// Marks every undefeated Talus as defeated and bumps `defeated_talus_counter` by the
    /// same amount. Returns how many were newly defeated.
    pub fn unlock_all_defeated_talus(&mut self) -> Result<usize, SaveError> {
        let count = completism::unlock_all_defeated_talus(&mut self.buf)? as u32;
        let current = self.defeated_talus_counter()?;
        self.set_defeated_talus_counter(current + count)?;
        Ok(count as usize)
    }

    /// Marks every undefeated Molduga as defeated and bumps `defeated_molduga_counter` by the
    /// same amount. Returns how many were newly defeated.
    pub fn unlock_all_defeated_molduga(&mut self) -> Result<usize, SaveError> {
        let count = completism::unlock_all_defeated_molduga(&mut self.buf)? as u32;
        let current = self.defeated_molduga_counter()?;
        self.set_defeated_molduga_counter(current + count)?;
        Ok(count as usize)
    }

    /// Marks every unvisited location as visited. Returns how many were newly visited. Unlike
    /// koroks/hinox/talus/molduga, there's no companion scalar counter to bump, since the source's
    /// `visitAllLocations` doesn't have one either.
    pub fn unlock_all_locations(&mut self) -> Result<usize, SaveError> {
        completism::unlock_all_locations(&mut self.buf)
    }

    pub fn completism(&self) -> Result<Vec<crate::completism::CompletismCategory>, SaveError> {
        completism::snapshot(&self.buf)
    }

    /// Mass-completes one completionism category by id (the ids from [`Self::completism`]),
    /// routing to the matching `unlock_all_*` (which also bump the scalar counter and, for koroks,
    /// add Korok Nut items). `metric` is unused: every BOTW card has a single metric.
    pub fn set_completism(&mut self, id: &str, _metric: usize) -> Result<usize, SaveError> {
        match id {
            "koroks" => self.unlock_all_koroks(),
            "locations" => self.unlock_all_locations(),
            "hinox" => self.unlock_all_defeated_hinox(),
            "talus" => self.unlock_all_defeated_talus(),
            "molduga" => self.unlock_all_defeated_molduga(),
            _ => Err(SaveError::MissingField("unknown completism category")),
        }
    }
}
