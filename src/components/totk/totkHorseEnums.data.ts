// Horse enum tables ported from marcrobledo's zelda-totk.class.horse.js.
// mane/saddle/rein are stored as murmur3 hashes of the name string; the hex values here were
// computed once with the engine's own totk::murmur3::hash32 (the "None" hash 0xb6eede09 matches
// the dye "None" hash, cross-checking the algorithm). Speed/pull/stamina store a small index
// whose star count is offset by one (index 1 = two stars).
import type { SelectOption } from "../Select";

export const MANE_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "None" },
  { value: 0xb93d9e3b, label: "Normal Mane" },
  { value: 0x3a84d601, label: "Mane 01" },
  { value: 0x0bffd92a, label: "Mane 02" },
  { value: 0xe8125091, label: "Mane 03" },
  { value: 0xfdb103b2, label: "Mane 04" },
  { value: 0x75677ada, label: "Mane 05" },
  { value: 0x9cbf81f2, label: "Mane 06" },
  { value: 0x8140f2f9, label: "Mane 07" },
  { value: 0xd749201c, label: "Mane 08" },
  { value: 0xac2a896d, label: "Mane 09" },
  { value: 0x87d9391f, label: "Mane 10" },
  { value: 0xd6a61738, label: "Mane 11" },
  { value: 0x12dd95d6, label: "Mane 12" },
  { value: 0x9cd4f27b, label: "Giant black mane" },
  { value: 0x55365b10, label: "Giant white mane" },
  { value: 0xbad4c4a9, label: "Donkey mane" },
];

export const SADDLE_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "None" },
  { value: 0x8573ae34, label: "Saddle 00" },
  { value: 0x04c6c17b, label: "Saddle 01" },
  { value: 0x47d0c84e, label: "Saddle 02" },
  { value: 0xaeab565a, label: "Saddle 03" },
  { value: 0xcf167805, label: "Saddle 04" },
  { value: 0x6e2db559, label: "Saddle 05" },
  { value: 0x7feaa5c0, label: "Saddle 06" },
  { value: 0xb926ed8b, label: "Saddle 07" },
  { value: 0xf1435392, label: "Saddle 00L" },
  { value: 0x8c5bd272, label: "Saddle 00S" },
];

export const REIN_OPTIONS: SelectOption[] = [
  { value: 0xb6eede09, label: "None" },
  { value: 0x1864234b, label: "Reins 00" },
  { value: 0x094f807a, label: "Reins 01" },
  { value: 0xe54abe55, label: "Reins 02" },
  { value: 0x0200441d, label: "Reins 03" },
  { value: 0x85610de7, label: "Reins 04" },
  { value: 0xbdc6a58b, label: "Reins 05" },
  { value: 0x79c2c72f, label: "Reins 06" },
  { value: 0x4dbf2061, label: "Reins 00L" },
  { value: 0xe8fe6ab7, label: "Reins 00S" },
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
  { value: 0, label: "Infinite" },
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
