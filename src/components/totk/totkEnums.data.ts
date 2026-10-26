import type { SelectOption } from "../Select";
import { DYE_COLOR_NAMES } from "./totkIcons";

// value = hearts * 4 (PlayerStatus.MaxLife stores quarter-hearts), 1-40 hearts matches the
// real editor's select-max-hearts range.
export const HEARTS_OPTIONS: SelectOption[] = Array.from({ length: 40 }, (_, i) => {
  const hearts = i + 1;
  return { value: hearts * 4, label: `${hearts} 颗心` };
});

// PlayerStatus.MaxStamina stores the raw bit pattern of an f32 wheel count (not a plain int),
// literal values transcribed from the real editor's select-max-stamina options.
export const STAMINA_OPTIONS: SelectOption[] = [
  { value: 1148846080, label: "1 圈" },
  { value: 1150681088, label: "1 圈又 1/5" },
  { value: 1152319488, label: "1 圈又 2/5" },
  { value: 1153957888, label: "1 圈又 3/5" },
  { value: 1155596288, label: "1 圈又 4/5" },
  { value: 1157234688, label: "2 圈" },
  { value: 1158250496, label: "2 圈又 1/5" },
  { value: 1159069696, label: "2 圈又 2/5" },
  { value: 1159888896, label: "2 圈又 3/5" },
  { value: 1160708096, label: "2 圈又 4/5" },
  { value: 1161527296, label: "3 圈" },
  { value: 1342177279, label: "无限" },
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
  { value: 0xb6eede09, label: "无附加属性" },
  { value: 0xd5cad39b, label: "耐久度提升 +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "耐久度提升 ++", iconName: "DurabilityUpPlus" },
  { value: 0xa9384c6c, label: "攻击力提升 +", iconName: "weapons_AttackUp" },
  { value: 0xdad10617, label: "攻击力提升 ++", iconName: "weapons_AttackUpPlus" },
  { value: 0xd0efac53, label: "终结暴击 +", iconName: "FinishBlow" },
  { value: 0x9659c804, label: "远距离投掷 ++", iconName: "LongThrow" },
];

export const BOW_MODIFIER_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "无附加属性" },
  { value: 0xd5cad39b, label: "耐久度提升 +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "耐久度提升 ++", iconName: "DurabilityUpPlus" },
  { value: 0xa9384c6c, label: "攻击力提升 +", iconName: "bows_AttackUp" },
  { value: 0xdad10617, label: "攻击力提升 ++", iconName: "bows_AttackUpPlus" },
  { value: 0x7d505bc4, label: "快速射击", iconName: "RapidFire" },
  { value: 0x934069cd, label: "五连射击", iconName: "FiveWay" },
];

export const SHIELD_MODIFIER_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "无附加属性" },
  { value: 0xd5cad39b, label: "耐久度提升 +", iconName: "DurabilityUp" },
  { value: 0xb2c943ee, label: "耐久度提升 ++", iconName: "DurabilityUpPlus" },
  { value: 0x37eae30f, label: "防御力提升 +", iconName: "GuardUp" },
  { value: 0x0b3c94e5, label: "防御力提升 ++", iconName: "GuardUpPlus" },
];

// Item.FOOD_EFFECTS from the real source, same murmur3_32 hash convention as above.
export const FOOD_EFFECT_OPTIONS: ModifierOption[] = [
  { value: 0xb6eede09, label: "无" },
  { value: 0x1df7a011, label: "耐热", iconName: "ResistHot" },
  { value: 0x11383afd, label: "防火", iconName: "ResistBurn" },
  { value: 0x9b6d98fb, label: "耐寒", iconName: "ResistCold" },
  { value: 0x183cd822, label: "耐电", iconName: "ResistElectric" },
  { value: 0x67866c6d, label: "游泳速度提升", iconName: "SwimSpeedUp" },
  { value: 0xdc7faf6e, label: "攀爬速度提升", iconName: "ClimbSpeedUp" },
  { value: 0xa9384c6c, label: "攻击力提升", iconName: "AttackUp" },
  { value: 0x4a3e58f6, label: "寒冷时攻击力提升", iconName: "AttackUpCold" },
  { value: 0x4c6a85d2, label: "炎热时攻击力提升", iconName: "AttackUpHot" },
  { value: 0xff347a38, label: "雷雨时攻击力提升", iconName: "AttackUpThunderstorm" },
  { value: 0x74141898, label: "安静性提升", iconName: "QuietnessUp" },
  { value: 0x9add92a3, label: "沙上移动速度提升", iconName: "SandMoveUp" },
  { value: 0x33261e44, label: "雪上移动速度提升", iconName: "SnowMoveUp" },
  { value: 0xa0a00c0e, label: "防御力提升", iconName: "DefenseUp" },
  { value: 0xb3f6b87a, label: "移动速度提升", iconName: "AllSpeed" },
  { value: 0x4d1e8af4, label: "瘴气防护", iconName: "MiasmaGuard" },
  { value: 0xc1db0965, label: "额外心心", iconName: "LifeMaxUp" },
  { value: 0xe9a30056, label: "精力恢复", iconName: "StaminaRecover" },
  { value: 0x60d8315d, label: "额外精力", iconName: "ExStaminaMaxUp" },
  { value: 0x03459853, label: "瘴气恢复", iconName: "LifeRepair" },
  { value: 0x6775f470, label: "滑翔机动力提升", iconName: "DivingMobilityUp" },
  { value: 0x2b0cb1e9, label: "防滑", iconName: "NotSlippy" },
  { value: 0x4939dca1, label: "发光", iconName: "LightEmission" },
];

const DYE_COLOR_LABELS: Record<string, string> = {
  None: "未染色", Black: "黑色", Blue: "蓝色", Brown: "棕色", Crimson: "深红色",
  Gray: "灰色", Green: "绿色", LightBlue: "浅蓝色", LightYellow: "浅黄色",
  Navy: "深蓝色", Orange: "橙色", Pink: "粉色", Purple: "紫色",
  Red: "红色", White: "白色", Yellow: "黄色",
};
export const DYE_COLOR_OPTIONS: SelectOption[] = Object.entries(DYE_COLOR_NAMES)
  .map(([hash, name]) => ({ value: Number(hash), label: DYE_COLOR_LABELS[name] ?? name }))
  .sort((a, b) => (a.value === 0xb6eede09 ? -1 : b.value === 0xb6eede09 ? 1 : a.label.localeCompare(b.label, "zh")));

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
