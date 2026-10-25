import type { SelectOption } from "../Select";

// value = hearts * 4 (MAX_HEARTS stores quarter-hearts), 1-30 hearts matches the real editor's
// select-max-hearts range.
export const HEARTS_OPTIONS: SelectOption[] = Array.from({ length: 30 }, (_, i) => {
  const hearts = i + 1;
  return { value: hearts * 4, label: `${hearts} heart${hearts === 1 ? "" : "s"}` };
});

// MAX_STAMINA stores the raw bit pattern of an f32 wheel count, same convention (and same
// literal values) as TOTK's stamina field, but BOTW's editor caps at 3 wheels with no
// *Infinite option.
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
];

// BOTW_Data.MODIFIERS from the real source: one shared list reused for weapons, bows, and
// shields alike (the "(Weapon only)"/"(Bow only)"/"(Shield only)" prefixes are just a hint to
// the user about which categories a flag is meaningful for, the source doesn't actually filter
// the dropdown per category). Values are the raw bitflags/star-tier combos stored directly, not
// hashes, unlike TOTK's modifier enum.
export const MODIFIER_OPTIONS: SelectOption[] = [
  { value: 0x00000000, label: "(none)" },
  { value: 0x00000001, label: "Attack up" },
  { value: 0x80000001, label: "Attack up ★" },
  { value: 0x00000002, label: "Durability up" },
  { value: 0x80000002, label: "Durability up ★" },
  { value: 0x00000004, label: "Critical hit up" },
  { value: 0x80000004, label: "Critical hit up ★" },
  { value: 0x00000008, label: "(Weapon only) Long throw" },
  { value: 0x80000008, label: "(Weapon only) Long throw ★" },
  { value: 0x00000010, label: "(Bow only) Five-Shot Burst" },
  { value: 0x80000010, label: "(Bow only) Five-Shot Burst ★" },
  { value: 0x00000020, label: "(Bow only) AddZoomRapid [unused]" },
  { value: 0x80000020, label: "(Bow only) AddZoomRapid ★ [unused]" },
  { value: 0x00000040, label: "(Bow only) Quick shot" },
  { value: 0x80000040, label: "(Bow only) Quick shot ★" },
  { value: 0x00000080, label: "(Shield only) Shield surf up" },
  { value: 0x80000080, label: "(Shield only) Shield surf up ★" },
  { value: 0x00000100, label: "(Shield only) Shield guard up" },
  { value: 0x80000100, label: "(Shield only) Shield guard up ★" },
];

// BOTW_Data.DYE_COLORS: armor's dye slot is a plain 0-15 index (not a name hash like TOTK), plus
// a special "locked color" sentinel value for armor that can't be dyed.
export const DYE_COLOR_OPTIONS: SelectOption[] = [
  { value: 0, label: "-default-" },
  { value: 1, label: "Blue" },
  { value: 2, label: "Red" },
  { value: 3, label: "Yellow" },
  { value: 4, label: "White" },
  { value: 5, label: "Black" },
  { value: 6, label: "Purple" },
  { value: 7, label: "Green" },
  { value: 8, label: "Light Blue" },
  { value: 9, label: "Navy" },
  { value: 10, label: "Orange" },
  { value: 11, label: "Peach" },
  { value: 12, label: "Crimson" },
  { value: 13, label: "Light Yellow" },
  { value: 14, label: "Brown" },
  { value: 15, label: "Gray" },
  { value: 0xffffffff, label: "locked color" },
];
