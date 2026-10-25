use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategoryDto {
    Weapon,
    Bow,
    Shield,
    Armor,
    Material,
    Food,
    KeyItem,
}

impl From<save_engine::botw::items::ItemCategory> for ItemCategoryDto {
    fn from(c: save_engine::botw::items::ItemCategory) -> Self {
        use save_engine::botw::items::ItemCategory;
        match c {
            ItemCategory::Weapon => ItemCategoryDto::Weapon,
            ItemCategory::Bow => ItemCategoryDto::Bow,
            ItemCategory::Shield => ItemCategoryDto::Shield,
            ItemCategory::Armor => ItemCategoryDto::Armor,
            ItemCategory::Material => ItemCategoryDto::Material,
            ItemCategory::Food => ItemCategoryDto::Food,
            ItemCategory::KeyItem => ItemCategoryDto::KeyItem,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwItemDto {
    pub name: String,
    pub quantity: u32,
    pub category: ItemCategoryDto,
}

impl From<(save_engine::botw::BotwItem, save_engine::botw::items::ItemCategory)> for BotwItemDto {
    fn from((item, category): (save_engine::botw::BotwItem, save_engine::botw::items::ItemCategory)) -> Self {
        BotwItemDto { name: item.name, quantity: item.quantity, category: category.into() }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ItemModifierDto {
    pub modifier: u32,
    pub value: u32,
}

impl From<save_engine::botw::ItemModifier> for ItemModifierDto {
    fn from(m: save_engine::botw::ItemModifier) -> Self {
        ItemModifierDto { modifier: m.modifier, value: m.value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwHorseDto {
    pub name: Option<String>,
    pub saddle: Option<String>,
    pub reins: Option<String>,
    pub horse_type: String,
}

impl From<save_engine::botw::BotwHorse> for BotwHorseDto {
    fn from(h: save_engine::botw::BotwHorse) -> Self {
        BotwHorseDto { name: h.name, saddle: h.saddle, reins: h.reins, horse_type: h.horse_type }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModifierCategoryDto {
    Weapon,
    Bow,
    Shield,
}

impl From<ModifierCategoryDto> for save_engine::botw::ModifierCategory {
    fn from(c: ModifierCategoryDto) -> Self {
        match c {
            ModifierCategoryDto::Weapon => save_engine::botw::ModifierCategory::Weapon,
            ModifierCategoryDto::Bow => save_engine::botw::ModifierCategory::Bow,
            ModifierCategoryDto::Shield => save_engine::botw::ModifierCategory::Shield,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotwState {
    pub version: String,
    pub modded: bool,
    pub rupees: u32,
    pub mons: u32,
    pub max_hearts: u32,
    pub max_stamina: u32,
    pub relic_gerudo: u32,
    pub relic_goron: u32,
    pub relic_rito: u32,
    pub korok_seed_counter: u32,
    pub defeated_hinox_counter: u32,
    pub defeated_talus_counter: u32,
    pub defeated_molduga_counter: u32,
    pub playtime_seconds: u32,
    pub motorcycle: Option<bool>,
    pub player_position: (f32, f32, f32),
    pub horse_position: (f32, f32, f32),
    pub map: String,
    pub map_type: String,
    pub items: Vec<BotwItemDto>,
    pub weapon_modifiers: Vec<ItemModifierDto>,
    pub bow_modifiers: Vec<ItemModifierDto>,
    pub shield_modifiers: Vec<ItemModifierDto>,
    pub horses: Vec<BotwHorseDto>,
    pub completism: Vec<CompletismCategoryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkState {
    pub version: String,
    pub modded: bool,
    pub max_life: u32,
    pub current_rupees: u32,
    pub max_stamina: u32,
    pub max_energy: u32,
    pub playtime: u32,
    pub horse_inn_member_point: u32,
    pub save_pos: (f32, f32, f32),
    pub sequence_current_banc: String,
    pub pouch_weapon_valid_num: u32,
    pub pouch_bow_valid_num: u32,
    pub pouch_shield_valid_num: u32,
    pub pouch_weapons: Vec<TotkWeaponDto>,
    pub pouch_bows: Vec<TotkBowDto>,
    pub pouch_shields: Vec<TotkShieldDto>,
    pub armor: Vec<TotkArmorDto>,
    pub arrows: Vec<TotkArrowDto>,
    pub materials: Vec<TotkMaterialDto>,
    pub key_items: Vec<TotkKeyItemDto>,
    pub devices: Vec<TotkDeviceDto>,
    pub food: Vec<TotkFoodDto>,
    pub horses: Vec<TotkHorseDto>,
    pub completism: Vec<CompletismCategoryDto>,
    pub autobuilds: Vec<TotkAutoBuildEntryDto>,
    pub map_pins: Vec<TotkMapPinDto>,
    pub map_markers: Vec<TotkMapMarkerDto>,
    pub teleporters: Vec<TotkTeleporterDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletismMetricDto {
    pub verb: String,
    pub value: usize,
    pub max: usize,
}

impl From<save_engine::completism::CompletismMetric> for CompletismMetricDto {
    fn from(m: save_engine::completism::CompletismMetric) -> Self {
        CompletismMetricDto { verb: m.verb.to_string(), value: m.value, max: m.max }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletismCategoryDto {
    pub id: String,
    pub label: String,
    pub metrics: Vec<CompletismMetricDto>,
}

impl From<save_engine::completism::CompletismCategory> for CompletismCategoryDto {
    fn from(c: save_engine::completism::CompletismCategory) -> Self {
        CompletismCategoryDto {
            id: c.id.to_string(),
            label: c.label.to_string(),
            metrics: c.metrics.into_iter().map(CompletismMetricDto::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkMapPinDto {
    pub icon: u32,
    pub x: f32,
    pub y: f32,
    pub layer: u32,
}

impl From<save_engine::totk::mapdata::MapPinEntry> for TotkMapPinDto {
    fn from(e: save_engine::totk::mapdata::MapPinEntry) -> Self {
        TotkMapPinDto { icon: e.icon, x: e.x, y: e.y, layer: e.layer }
    }
}

impl From<TotkMapPinDto> for save_engine::totk::mapdata::MapPinEntry {
    fn from(d: TotkMapPinDto) -> Self {
        save_engine::totk::mapdata::MapPinEntry { icon: d.icon, x: d.x, y: d.y, layer: d.layer }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkMapMarkerDto {
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<save_engine::totk::mapdata::MapMarkerEntry> for TotkMapMarkerDto {
    fn from(e: save_engine::totk::mapdata::MapMarkerEntry) -> Self {
        TotkMapMarkerDto { color: e.color, x: e.x, y: e.y, z: e.z }
    }
}

impl From<TotkMapMarkerDto> for save_engine::totk::mapdata::MapMarkerEntry {
    fn from(d: TotkMapMarkerDto) -> Self {
        save_engine::totk::mapdata::MapMarkerEntry { color: d.color, x: d.x, y: d.y, z: d.z }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkTeleporterDto {
    pub index: i32,
    pub pos: (f32, f32, f32),
    pub rot: (f32, f32, f32),
}

impl From<save_engine::totk::mapdata::TeleporterEntry> for TotkTeleporterDto {
    fn from(e: save_engine::totk::mapdata::TeleporterEntry) -> Self {
        TotkTeleporterDto { index: e.index, pos: e.pos, rot: e.rot }
    }
}

impl From<TotkTeleporterDto> for save_engine::totk::mapdata::TeleporterEntry {
    fn from(d: TotkTeleporterDto) -> Self {
        save_engine::totk::mapdata::TeleporterEntry { index: d.index, pos: d.pos, rot: d.rot }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkAutoBuildEntryDto {
    pub index: i32,
    pub combined_actor_info: Vec<u8>,
    pub camera_pos: (f32, f32, f32),
    pub camera_at: (f32, f32, f32),
    pub is_favorite: bool,
}

impl From<save_engine::totk::autobuilder::AutoBuildEntry> for TotkAutoBuildEntryDto {
    fn from(e: save_engine::totk::autobuilder::AutoBuildEntry) -> Self {
        TotkAutoBuildEntryDto {
            index: e.index,
            combined_actor_info: e.combined_actor_info,
            camera_pos: e.camera_pos,
            camera_at: e.camera_at,
            is_favorite: e.is_favorite,
        }
    }
}

impl From<TotkAutoBuildEntryDto> for save_engine::totk::autobuilder::AutoBuildEntry {
    fn from(d: TotkAutoBuildEntryDto) -> Self {
        save_engine::totk::autobuilder::AutoBuildEntry {
            index: d.index,
            combined_actor_info: d.combined_actor_info,
            camera_pos: d.camera_pos,
            camera_at: d.camera_at,
            is_favorite: d.is_favorite,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkWeaponDto {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
    pub record_extra_durability: i32,
}

impl From<save_engine::totk::pouch::WeaponEntry> for TotkWeaponDto {
    fn from(e: save_engine::totk::pouch::WeaponEntry) -> Self {
        TotkWeaponDto {
            id: e.id,
            durability: e.durability,
            modifier: e.modifier,
            modifier_value: e.modifier_value,
            fuse_id: e.fuse_id,
            fuse_durability: e.fuse_durability,
            extra_durability: e.extra_durability,
            record_extra_durability: e.record_extra_durability,
        }
    }
}

impl From<TotkWeaponDto> for save_engine::totk::pouch::WeaponEntry {
    fn from(d: TotkWeaponDto) -> Self {
        save_engine::totk::pouch::WeaponEntry {
            id: d.id,
            durability: d.durability,
            modifier: d.modifier,
            modifier_value: d.modifier_value,
            fuse_id: d.fuse_id,
            fuse_durability: d.fuse_durability,
            extra_durability: d.extra_durability,
            record_extra_durability: d.record_extra_durability,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkBowDto {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
}

impl From<save_engine::totk::pouch::BowEntry> for TotkBowDto {
    fn from(e: save_engine::totk::pouch::BowEntry) -> Self {
        TotkBowDto { id: e.id, durability: e.durability, modifier: e.modifier, modifier_value: e.modifier_value }
    }
}

impl From<TotkBowDto> for save_engine::totk::pouch::BowEntry {
    fn from(d: TotkBowDto) -> Self {
        save_engine::totk::pouch::BowEntry { id: d.id, durability: d.durability, modifier: d.modifier, modifier_value: d.modifier_value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkShieldDto {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
}

impl From<save_engine::totk::pouch::ShieldEntry> for TotkShieldDto {
    fn from(e: save_engine::totk::pouch::ShieldEntry) -> Self {
        TotkShieldDto {
            id: e.id,
            durability: e.durability,
            modifier: e.modifier,
            modifier_value: e.modifier_value,
            fuse_id: e.fuse_id,
            fuse_durability: e.fuse_durability,
            extra_durability: e.extra_durability,
        }
    }
}

impl From<TotkShieldDto> for save_engine::totk::pouch::ShieldEntry {
    fn from(d: TotkShieldDto) -> Self {
        save_engine::totk::pouch::ShieldEntry {
            id: d.id,
            durability: d.durability,
            modifier: d.modifier,
            modifier_value: d.modifier_value,
            fuse_id: d.fuse_id,
            fuse_durability: d.fuse_durability,
            extra_durability: d.extra_durability,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkArmorDto {
    pub id: String,
    pub dye_color: u32,
}

impl From<save_engine::totk::pouch::ArmorEntry> for TotkArmorDto {
    fn from(e: save_engine::totk::pouch::ArmorEntry) -> Self {
        TotkArmorDto { id: e.id, dye_color: e.dye_color }
    }
}

impl From<TotkArmorDto> for save_engine::totk::pouch::ArmorEntry {
    fn from(d: TotkArmorDto) -> Self {
        save_engine::totk::pouch::ArmorEntry { id: d.id, dye_color: d.dye_color }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkArrowDto {
    pub id: String,
    pub quantity: i32,
}

impl From<save_engine::totk::pouch::ArrowEntry> for TotkArrowDto {
    fn from(e: save_engine::totk::pouch::ArrowEntry) -> Self {
        TotkArrowDto { id: e.id, quantity: e.quantity }
    }
}

impl From<TotkArrowDto> for save_engine::totk::pouch::ArrowEntry {
    fn from(d: TotkArrowDto) -> Self {
        save_engine::totk::pouch::ArrowEntry { id: d.id, quantity: d.quantity }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkMaterialDto {
    pub id: String,
    pub quantity: i32,
    pub get_order: i32,
    pub use_order: i32,
}

impl From<save_engine::totk::pouch::MaterialEntry> for TotkMaterialDto {
    fn from(e: save_engine::totk::pouch::MaterialEntry) -> Self {
        TotkMaterialDto { id: e.id, quantity: e.quantity, get_order: e.get_order, use_order: e.use_order }
    }
}

impl From<TotkMaterialDto> for save_engine::totk::pouch::MaterialEntry {
    fn from(d: TotkMaterialDto) -> Self {
        save_engine::totk::pouch::MaterialEntry { id: d.id, quantity: d.quantity, get_order: d.get_order, use_order: d.use_order }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkKeyItemDto {
    pub id: String,
    pub quantity: i32,
}

impl From<save_engine::totk::pouch::KeyItemEntry> for TotkKeyItemDto {
    fn from(e: save_engine::totk::pouch::KeyItemEntry) -> Self {
        TotkKeyItemDto { id: e.id, quantity: e.quantity }
    }
}

impl From<TotkKeyItemDto> for save_engine::totk::pouch::KeyItemEntry {
    fn from(d: TotkKeyItemDto) -> Self {
        save_engine::totk::pouch::KeyItemEntry { id: d.id, quantity: d.quantity }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkDeviceDto {
    pub id: String,
    pub quantity: i32,
    pub use_order: i32,
}

impl From<save_engine::totk::pouch::DeviceEntry> for TotkDeviceDto {
    fn from(e: save_engine::totk::pouch::DeviceEntry) -> Self {
        TotkDeviceDto { id: e.id, quantity: e.quantity, use_order: e.use_order }
    }
}

impl From<TotkDeviceDto> for save_engine::totk::pouch::DeviceEntry {
    fn from(d: TotkDeviceDto) -> Self {
        save_engine::totk::pouch::DeviceEntry { id: d.id, quantity: d.quantity, use_order: d.use_order }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkFoodDto {
    pub id: String,
    pub quantity: i32,
    pub hearts_heal: i32,
    pub effect: u32,
    pub effect_multiplier: i32,
    pub effect_time: i32,
    pub price: i32,
    pub recipe: [String; 5],
}

impl From<save_engine::totk::pouch::FoodEntry> for TotkFoodDto {
    fn from(e: save_engine::totk::pouch::FoodEntry) -> Self {
        TotkFoodDto {
            id: e.id,
            quantity: e.quantity,
            hearts_heal: e.hearts_heal,
            effect: e.effect,
            effect_multiplier: e.effect_multiplier,
            effect_time: e.effect_time,
            price: e.price,
            recipe: e.recipe,
        }
    }
}

impl From<TotkFoodDto> for save_engine::totk::pouch::FoodEntry {
    fn from(d: TotkFoodDto) -> Self {
        save_engine::totk::pouch::FoodEntry {
            id: d.id,
            quantity: d.quantity,
            hearts_heal: d.hearts_heal,
            effect: d.effect,
            effect_multiplier: d.effect_multiplier,
            effect_time: d.effect_time,
            price: d.price,
            recipe: d.recipe,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkHorseDto {
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

impl From<save_engine::totk::horse::HorseEntry> for TotkHorseDto {
    fn from(e: save_engine::totk::horse::HorseEntry) -> Self {
        TotkHorseDto {
            id: e.id,
            name: e.name,
            mane: e.mane,
            saddle: e.saddle,
            rein: e.rein,
            bond: e.bond,
            bond_checked: e.bond_checked,
            stats_strength: e.stats_strength,
            stats_speed: e.stats_speed,
            stats_stamina: e.stats_stamina,
            stats_pull: e.stats_pull,
            horse_type: e.horse_type,
            color_type: e.color_type,
            foot_type: e.foot_type,
            amiibo_uid_hash: e.amiibo_uid_hash,
            room_id: e.room_id,
            icon_pattern: e.icon_pattern,
            icon_eye_color: e.icon_eye_color,
            icon_primary_color: e.icon_primary_color,
            icon_secondary_color: e.icon_secondary_color,
            icon_nose_color: e.icon_nose_color,
            icon_hair_primary_color: e.icon_hair_primary_color,
            icon_hair_secondary_color: e.icon_hair_secondary_color,
        }
    }
}

impl From<TotkHorseDto> for save_engine::totk::horse::HorseEntry {
    fn from(d: TotkHorseDto) -> Self {
        save_engine::totk::horse::HorseEntry {
            id: d.id,
            name: d.name,
            mane: d.mane,
            saddle: d.saddle,
            rein: d.rein,
            bond: d.bond,
            bond_checked: d.bond_checked,
            stats_strength: d.stats_strength,
            stats_speed: d.stats_speed,
            stats_stamina: d.stats_stamina,
            stats_pull: d.stats_pull,
            horse_type: d.horse_type,
            color_type: d.color_type,
            foot_type: d.foot_type,
            amiibo_uid_hash: d.amiibo_uid_hash,
            room_id: d.room_id,
            icon_pattern: d.icon_pattern,
            icon_eye_color: d.icon_eye_color,
            icon_primary_color: d.icon_primary_color,
            icon_secondary_color: d.icon_secondary_color,
            icon_nose_color: d.icon_nose_color,
            icon_hair_primary_color: d.icon_hair_primary_color,
            icon_hair_secondary_color: d.icon_hair_secondary_color,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkCaptionDto {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub is_autosave: bool,
    pub thumbnail_jpeg: Vec<u8>,
}

impl From<save_engine::totk::caption::CaptionInfo> for TotkCaptionDto {
    fn from(c: save_engine::totk::caption::CaptionInfo) -> Self {
        TotkCaptionDto {
            year: c.year,
            month: c.month,
            day: c.day,
            hour: c.hour,
            minute: c.minute,
            is_autosave: c.is_autosave,
            thumbnail_jpeg: c.thumbnail_jpeg,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkAbilityDto {
    pub id: String,
    pub label: String,
    pub unlocked: bool,
}

impl From<save_engine::totk::abilities::AbilityState> for TotkAbilityDto {
    fn from(a: save_engine::totk::abilities::AbilityState) -> Self {
        TotkAbilityDto { id: a.id, label: a.label, unlocked: a.unlocked }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotkHashRowDto {
    pub hash: u32,
    pub name: Option<String>,
    pub type_name: String,
    pub editable: bool,
    pub value: f64,
}

impl From<save_engine::totk::hashbrowser::HashRow> for TotkHashRowDto {
    fn from(r: save_engine::totk::hashbrowser::HashRow) -> Self {
        TotkHashRowDto {
            hash: r.hash,
            name: r.name.map(str::to_string),
            type_name: r.type_name.to_string(),
            editable: r.editable,
            value: r.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OpenResult {
    Botw { state: BotwState },
    Totk { state: TotkState },
    Caption { info: TotkCaptionDto },
}
