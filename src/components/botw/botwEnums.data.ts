import type { SelectOption } from "../Select";

// value = hearts * 4 (MAX_HEARTS stores quarter-hearts), 1-30 hearts matches the real editor's
// select-max-hearts range.
export const HEARTS_OPTIONS: SelectOption[] = Array.from({ length: 30 }, (_, i) => {
  const hearts = i + 1;
  return { value: hearts * 4, label: `${hearts} 颗心` };
});

// MAX_STAMINA stores the raw bit pattern of an f32 wheel count, same convention (and same
// literal values) as TOTK's stamina field, but BOTW's editor caps at 3 wheels with no
// *Infinite option.
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
];

// BOTW_Data.MODIFIERS from the real source: one shared list reused for weapons, bows, and
// shields alike (the "(Weapon only)"/"(Bow only)"/"(Shield only)" prefixes are just a hint to
// the user about which categories a flag is meaningful for, the source doesn't actually filter
// the dropdown per category). Values are the raw bitflags/star-tier combos stored directly, not
// hashes, unlike TOTK's modifier enum.
export const MODIFIER_OPTIONS: SelectOption[] = [
  { value: 0x00000000, label: "（无）" },
  { value: 0x00000001, label: "攻击力提升" },
  { value: 0x80000001, label: "攻击力提升 ★" },
  { value: 0x00000002, label: "耐久度提升" },
  { value: 0x80000002, label: "耐久度提升 ★" },
  { value: 0x00000004, label: "终结暴击" },
  { value: 0x80000004, label: "终结暴击 ★" },
  { value: 0x00000008, label: "（近战武器）远距离投掷" },
  { value: 0x80000008, label: "（近战武器）远距离投掷 ★" },
  { value: 0x00000010, label: "（弓）五连射击" },
  { value: 0x80000010, label: "（弓）五连射击 ★" },
  { value: 0x00000020, label: "（弓）快速瞄准（未使用）" },
  { value: 0x80000020, label: "（弓）快速瞄准 ★（未使用）" },
  { value: 0x00000040, label: "（弓）快速射击" },
  { value: 0x80000040, label: "（弓）快速射击 ★" },
  { value: 0x00000080, label: "（盾）滑盾性能提升" },
  { value: 0x80000080, label: "（盾）滑盾性能提升 ★" },
  { value: 0x00000100, label: "（盾）防御力提升" },
  { value: 0x80000100, label: "（盾）防御力提升 ★" },
];

// BOTW_Data.DYE_COLORS: armor's dye slot is a plain 0-15 index (not a name hash like TOTK), plus
// a special "locked color" sentinel value for armor that can't be dyed.
export const DYE_COLOR_OPTIONS: SelectOption[] = [
  { value: 0, label: "默认" },
  { value: 1, label: "蓝色" },
  { value: 2, label: "红色" },
  { value: 3, label: "黄色" },
  { value: 4, label: "白色" },
  { value: 5, label: "黑色" },
  { value: 6, label: "紫色" },
  { value: 7, label: "绿色" },
  { value: 8, label: "浅蓝色" },
  { value: 9, label: "深蓝色" },
  { value: 10, label: "橙色" },
  { value: 11, label: "桃色" },
  { value: 12, label: "深红色" },
  { value: 13, label: "浅黄色" },
  { value: 14, label: "棕色" },
  { value: 15, label: "灰色" },
  { value: 0xffffffff, label: "不可染色" },
];
