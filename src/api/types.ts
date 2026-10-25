export type BotwItemCategory = "weapon" | "bow" | "shield" | "armor" | "material" | "food" | "key_item";

export interface BotwItem {
  name: string;
  quantity: number;
  category: BotwItemCategory;
}

export interface ItemModifier {
  modifier: number;
  value: number;
}

export interface BotwHorse {
  name: string | null;
  saddle: string | null;
  reins: string | null;
  horse_type: string;
}

export type ModifierCategory = "weapon" | "bow" | "shield";

export interface BotwState {
  version: string;
  modded: boolean;
  rupees: number;
  mons: number;
  max_hearts: number;
  max_stamina: number;
  relic_gerudo: number;
  relic_goron: number;
  relic_rito: number;
  korok_seed_counter: number;
  defeated_hinox_counter: number;
  defeated_talus_counter: number;
  defeated_molduga_counter: number;
  playtime_seconds: number;
  motorcycle: boolean | null;
  player_position: [number, number, number];
  horse_position: [number, number, number];
  map: string;
  map_type: string;
  items: BotwItem[];
  weapon_modifiers: ItemModifier[];
  bow_modifiers: ItemModifier[];
  shield_modifiers: ItemModifier[];
  horses: BotwHorse[];
  completism: CompletismCategory[];
}

export interface TotkWeapon {
  id: string;
  durability: number;
  modifier: number;
  modifier_value: number;
  fuse_id: string;
  fuse_durability: number;
  extra_durability: number;
  record_extra_durability: number;
}

export interface TotkBow {
  id: string;
  durability: number;
  modifier: number;
  modifier_value: number;
}

export interface TotkShield {
  id: string;
  durability: number;
  modifier: number;
  modifier_value: number;
  fuse_id: string;
  fuse_durability: number;
  extra_durability: number;
}

export interface TotkArmor {
  id: string;
  dye_color: number;
}

export interface TotkArrow {
  id: string;
  quantity: number;
}

export interface TotkMaterial {
  id: string;
  quantity: number;
  get_order: number;
  use_order: number;
}

export interface TotkKeyItem {
  id: string;
  quantity: number;
}

export interface TotkDevice {
  id: string;
  quantity: number;
  use_order: number;
}

export interface TotkFood {
  id: string;
  quantity: number;
  hearts_heal: number;
  effect: number;
  effect_multiplier: number;
  effect_time: number;
  price: number;
  recipe: [string, string, string, string, string];
}

export interface TotkHorse {
  id: string;
  name: string;
  mane: number;
  saddle: number;
  rein: number;
  bond: number;
  bond_checked: boolean;
  stats_strength: number;
  stats_speed: number;
  stats_stamina: number;
  stats_pull: number;
  horse_type: number;
  color_type: number;
  foot_type: number;
  amiibo_uid_hash: number;
  room_id: number;
  icon_pattern: number;
  icon_eye_color: number;
  icon_primary_color: [number, number, number];
  icon_secondary_color: [number, number, number];
  icon_nose_color: [number, number, number];
  icon_hair_primary_color: [number, number, number];
  icon_hair_secondary_color: [number, number, number];
}

export interface TotkAutoBuildEntry {
  index: number;
  combined_actor_info: number[];
  camera_pos: [number, number, number];
  camera_at: [number, number, number];
  is_favorite: boolean;
}

export interface TotkMapPin {
  icon: number;
  x: number;
  y: number;
  layer: number;
}

export interface TotkMapMarker {
  color: number;
  x: number;
  y: number;
  z: number;
}

export interface TotkTeleporter {
  index: number;
  pos: [number, number, number];
  rot: [number, number, number];
}

export interface CompletismMetric {
  /** Past-tense action word, e.g. "found", "activated", "defeated"; also drives the button label. */
  verb: string;
  value: number;
  max: number;
}

export interface CompletismCategory {
  id: string;
  label: string;
  metrics: CompletismMetric[];
}

export interface TotkState {
  version: string;
  modded: boolean;
  max_life: number;
  current_rupees: number;
  max_stamina: number;
  max_energy: number;
  playtime: number;
  horse_inn_member_point: number;
  save_pos: [number, number, number];
  sequence_current_banc: string;
  pouch_weapon_valid_num: number;
  pouch_bow_valid_num: number;
  pouch_shield_valid_num: number;
  pouch_weapons: TotkWeapon[];
  pouch_bows: TotkBow[];
  pouch_shields: TotkShield[];
  armor: TotkArmor[];
  arrows: TotkArrow[];
  materials: TotkMaterial[];
  key_items: TotkKeyItem[];
  devices: TotkDevice[];
  food: TotkFood[];
  horses: TotkHorse[];
  completism: CompletismCategory[];
  autobuilds: TotkAutoBuildEntry[];
  map_pins: TotkMapPin[];
  map_markers: TotkMapMarker[];
  teleporters: TotkTeleporter[];
}

export interface TotkHashRow {
  hash: number;
  name: string | null;
  type_name: string;
  editable: boolean;
  value: number;
}

export interface TotkAbility {
  id: string;
  label: string;
  unlocked: boolean;
}

export interface TotkCaption {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  is_autosave: boolean;
  thumbnail_jpeg: number[];
}

export type OpenResult =
  | { kind: "botw"; state: BotwState }
  | { kind: "totk"; state: TotkState }
  | { kind: "caption"; info: TotkCaption };

export interface ShellError {
  kind: string;
  message: string;
}
