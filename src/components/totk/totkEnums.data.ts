import type { SelectOption } from "../Select";
import { DYE_COLOR_NAMES } from "./totkIcons";

// value = hearts * 4 (PlayerStatus.MaxLife stores quarter-hearts), 1-40 hearts matches the
// real editor's select-max-hearts range.
export const HEARTS_OPTIONS: SelectOption[] = Array.from({ length: 40 }, (_, i) => {
  const hearts = i + 1;
  return { value: hearts * 4, label: `${hearts} heart${hearts === 1 ? "" : "s"}` };
});

// PlayerStatus.MaxStamina stores the raw bit pattern of an f32 wheel count (not a plain int),
// literal values transcribed from the real editor's select-max-stamina options.
export const STAMINA_OPTIONS: SelectOption[] = [
  { value: 1148846080, label: "1 wheel" },
  { value: 1150681088, label: "1 wheel+1/5" },
  { value: 1152319488, label: "1 wheel+2/5" },
  { value: 1153957888, label: "1 wheel+3/5" },
  { value: 1155596288, label: "1 wheel+4/5" },
  { value: 1157234688, label: "2 wheels" },
  { value: 1158250496, label: "2 wheels+1/5" },
  { value: 1159069696, label: "2 wheels+2/5" },
  { value: 1159888896, label: "2 wheels+3/5" },
  { value: 1160708096, label: "2 wheels+4/5" },
  { value: 1161527296, label: "3 wheels" },
  { value: 1342177279, label: "Infinite" },
];

// Battery.MaxEnergy is a plain linear scale, 3000-48000 step 1000, matching select-max-battery.
export const BATTERY_OPTIONS: SelectOption[] = Array.from({ length: 46 }, (_, i) => {
  const value = 3000 + i * 1000;
  return { value, label: String(value) };
});

interface ModifierOption extends SelectOption {
  iconName?: string;
}

// Equipment.OPTIONS_MODIFIERS from the real source, values are murmur3_32 hashes of the
// modifier name strings (computed via totk::murmur3::hash32, same convention as every other
// TOTK Enum field). iconName matches the vendored public/icons/totk/bonus/<name>.svg files.
export const WEAPON_MODIFIER_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "No bonus" },
  { value: 0xd5cad39b, label: "Durability +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "Durability ++", iconName: "DurabilityUpPlus" },
  { value: 0xa9384c6c, label: "Attack +", iconName: "weapons_AttackUp" },
  { value: 0xdad10617, label: "Attack ++", iconName: "weapons_AttackUpPlus" },
  { value: 0xd0efac53, label: "Critical Hit +", iconName: "FinishBlow" },
  { value: 0x9659c804, label: "Throw ++", iconName: "LongThrow" },
];

export const BOW_MODIFIER_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "No bonus" },
  { value: 0xd5cad39b, label: "Durability +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "Durability ++", iconName: "DurabilityUpPlus" },
  { value: 0xa9384c6c, label: "Attack +", iconName: "bows_AttackUp" },
  { value: 0xdad10617, label: "Attack ++", iconName: "bows_AttackUpPlus" },
  { value: 0x7d505bc4, label: "Quick Shot", iconName: "RapidFire" },
  { value: 0x934069cd, label: "Arrow Shot x5", iconName: "FiveWay" },
];

export const SHIELD_MODIFIER_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "No bonus" },
  { value: 0xd5cad39b, label: "Durability +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "Durability ++", iconName: "DurabilityUpPlus" },
  { value: 0x37eae30f, label: "Block +", iconName: "GuardUp" },
  { value: 0x0b3c94e5, label: "Block ++", iconName: "GuardUpPlus" },
];

// Item.FOOD_EFFECTS from the real source, same murmur3_32 hash convention as above.
export const FOOD_EFFECT_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "None" },
  { value: 0x1df7a011, label: "Heat Resistance", iconName: "ResistHot" },
  { value: 0x11383afd, label: "Flame Guard", iconName: "ResistBurn" },
  { value: 0x9b6d98fb, label: "Cold Resistance", iconName: "ResistCold" },
  { value: 0x183cd822, label: "Shock Resistance", iconName: "ResistElectric" },
  { value: 0x67866c6d, label: "Swim Speed Up", iconName: "SwimSpeedUp" },
  { value: 0xdc7faf6e, label: "Climb Speed Up", iconName: "ClimbSpeedUp" },
  { value: 0xa9384c6c, label: "Attack Up", iconName: "AttackUp" },
  { value: 0x4a3e58f6, label: "Cold Weather Attack Up", iconName: "AttackUpCold" },
  { value: 0x4c6a85d2, label: "Hot Weather Attack Up", iconName: "AttackUpHot" },
  { value: 0xff347a38, label: "Stormy Weather Attack Up", iconName: "AttackUpThunderstorm" },
  { value: 0x74141898, label: "Stealth Up", iconName: "QuietnessUp" },
  { value: 0x9add92a3, label: "Sand Speed Up", iconName: "SandMoveUp" },
  { value: 0x33261e44, label: "Snow Speed Up", iconName: "SnowMoveUp" },
  { value: 0xa0a00c0e, label: "Defense Up", iconName: "DefenseUp" },
  { value: 0xb3f6b87a, label: "Speed Up", iconName: "AllSpeed" },
  { value: 0x4d1e8af4, label: "Gloom Resistance", iconName: "MiasmaGuard" },
  { value: 0xc1db0965, label: "Extra Heart", iconName: "LifeMaxUp" },
  { value: 0xe9a30056, label: "Stamina Recovery", iconName: "StaminaRecover" },
  { value: 0x60d8315d, label: "Extra Stamina", iconName: "ExStaminaMaxUp" },
  { value: 0x03459853, label: "Gloom Recovery", iconName: "LifeRepair" },
  { value: 0x6775f470, label: "Skydive Mobility Up", iconName: "DivingMobilityUp" },
  { value: 0x2b0cb1e9, label: "Slip Resistance", iconName: "NotSlippy" },
  { value: 0x4939dca1, label: "Glow", iconName: "LightEmission" },
];

export const DYE_COLOR_OPTIONS: SelectOption[] = Object.entries(DYE_COLOR_NAMES)
  .map(([hash, name]) => ({ value: Number(hash), label: name }))
  .sort((a, b) => (a.label === "None" ? -1 : b.label === "None" ? 1 : a.label.localeCompare(b.label)));

export function bonusIconUrl(iconName: string | undefined): string | undefined {
  return iconName ? `/icons/totk/bonus/${iconName}.svg` : undefined;
}

function findIconName(options: ModifierOption[], value: number): string | undefined {
  return options.find((o) => o.value === value)?.iconName;
}

export function weaponModifierIconUrl(modifier: number): string | undefined {
  return bonusIconUrl(findIconName(WEAPON_MODIFIER_OPTIONS, modifier));
}
export function bowModifierIconUrl(modifier: number): string | undefined {
  return bonusIconUrl(findIconName(BOW_MODIFIER_OPTIONS, modifier));
}
export function shieldModifierIconUrl(modifier: number): string | undefined {
  return bonusIconUrl(findIconName(SHIELD_MODIFIER_OPTIONS, modifier));
}
export function foodEffectIconUrl(effect: number): string | undefined {
  return bonusIconUrl(findIconName(FOOD_EFFECT_OPTIONS, effect));
}
