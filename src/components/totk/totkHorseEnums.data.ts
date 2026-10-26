// Horse enum tables ported from marcrobledo's zelda-totk.class.horse.js.
// mane/saddle/rein are stored as murmur3 hashes of the name string; the hex values here were
// computed once with the engine's own totk::murmur3::hash32 (the "None" hash 0xb6eede09 matches
// the dye "None" hash, cross-checking the algorithm). Speed/pull/stamina store a small index
// whose star count is offset by one (index 1 = two stars).
import type { SelectOption } from "../Select";

export const MANE_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "无" },
  { value: 0xb93d9e3b, label: "普通鬃毛" },
  { value: 0x3a84d601, label: "鬃毛 01" },
  { value: 0x0bffd92a, label: "鬃毛 02" },
  { value: 0xe8125091, label: "鬃毛 03" },
  { value: 0xfdb103b2, label: "鬃毛 04" },
  { value: 0x75677ada, label: "鬃毛 05" },
  { value: 0x9cbf81f2, label: "鬃毛 06" },
  { value: 0x8140f2f9, label: "鬃毛 07" },
  { value: 0xd749201c, label: "鬃毛 08" },
  { value: 0xac2a896d, label: "鬃毛 09" },
  { value: 0x87d9391f, label: "鬃毛 10" },
  { value: 0xd6a61738, label: "鬃毛 11" },
  { value: 0x12dd95d6, label: "鬃毛 12" },
  { value: 0x9cd4f27b, label: "巨马黑鬃毛" },
  { value: 0x55365b10, label: "巨马白鬃毛" },
  { value: 0xbad4c4a9, label: "驴鬃毛" },
];

export const SADDLE_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "无" },
  { value: 0x8573ae34, label: "马鞍 00" },
  { value: 0x04c6c17b, label: "马鞍 01" },
  { value: 0x47d0c84e, label: "马鞍 02" },
  { value: 0xaeab565a, label: "马鞍 03" },
  { value: 0xcf167805, label: "马鞍 04" },
  { value: 0x6e2db559, label: "马鞍 05" },
  { value: 0x7feaa5c0, label: "马鞍 06" },
  { value: 0xb926ed8b, label: "马鞍 07" },
  { value: 0xf1435392, label: "马鞍 00L" },
  { value: 0x8c5bd272, label: "马鞍 00S" },
];

export const REIN_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "无" },
  { value: 0x1864234b, label: "缰绳 00" },
  { value: 0x094f807a, label: "缰绳 01" },
  { value: 0xe54abe55, label: "缰绳 02" },
  { value: 0x0200441d, label: "缰绳 03" },
  { value: 0x85610de7, label: "缰绳 04" },
  { value: 0xbdc6a58b, label: "缰绳 05" },
  { value: 0x79c2c72f, label: "缰绳 06" },
  { value: 0x4dbf2061, label: "缰绳 00L" },
  { value: 0xe8fe6ab7, label: "缰绳 00S" },
];

// Speed and Pull: index 1..4 shown as two to five stars (Horse.OPTIONS_STATS).
export const STAT_STAR_OPTIONS: SelectOption[] = [
  { value: 1, label: "★★" },
  { value: 2, label: "★★★" },
  { value: 3, label: "★★★★" },
  { value: 4, label: "★★★★★" },
];

// Stamina: index 2..5 shown as two to five stars, plus 0 = Infinite (Horse.OPTIONS_STATS_STAMINA).
export const STAMINA_STAR_OPTIONS: SelectOption[] = [
  { value: 2, label: "★★" },
  { value: 3, label: "★★★" },
  { value: 4, label: "★★★★" },
  { value: 5, label: "★★★★★" },
  { value: 0, label: "无限" },
];

// Selectable horse ids (Horse.AVAILABILITY), used by the Id picker to swap the current horse.
export const HORSE_AVAILABILITY: string[] = [
  "GameRomHorse",
  "GameRomHorse00",
  "GameRomHorse01",
  "GameRomHorse02",
  "GameRomHorse03",
  "GameRomHorse04",
  "GameRomHorse05",
  "GameRomHorse06",
  "GameRomHorse07",
  "GameRomHorse08",
  "GameRomHorse09",
  "GameRomHorse10",
  "GameRomHorse11",
  "GameRomHorse12",
  "GameRomHorse13",
  "GameRomHorse14",
  "GameRomHorse15",
  "GameRomHorse16",
  "GameRomHorse17",
  "GameRomHorse18",
  "GameRomHorse19",
  "GameRomHorse20",
  "GameRomHorse21",
  "GameRomHorse22",
  "GameRomHorse23",
  "GameRomHorse25",
  "GameRomHorse26",
  "GameRomHorseEpona",
  "GameRomHorseZelda",
  "GameRomHorse00L",
  "GameRomHorse01L",
  "GameRomHorseGold",
  "GameRomHorseSpPattern",
  "GameRomHorseBone",
  "GameRomHorseBone_AllDay",
  "GameRomHorseForStreetVender",
  "GameRomHorseNushi",
];
