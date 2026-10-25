import { BOTW_ITEM_NAMES } from "./botwItemNames.data";

// Real source id lists (`BOTW_Data.HORSE_TYPES`/`HORSE_TYPES_UNTAMMED`/`HORSE_SADDLES`/
// `HORSE_REINS`). These are raw string ids stored directly in the save (unlike TOTK's
// hash-encoded enums), and the source's own dropdown falls back to showing the raw id when no
// friendly translation exists (most of the plain numbered horse types have none), so this does
// the same via the already-vendored `BOTW_ITEM_NAMES` table rather than a separate list.
const HORSE_TYPE_IDS = [
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
  "GameRomHorseEpona",
  "GameRomHorseZelda",
  "GameRomHorse00L",
];

// Only valid for horse slot 5, the untamed/currently-ridden-wild-horse slot.
const HORSE_TYPE_UNTAMED_IDS = ["GameRomHorseNushi", "GameRomHorseBone"];

const HORSE_SADDLE_IDS = [
  "GameRomHorseSaddle_00",
  "GameRomHorseSaddle_01",
  "GameRomHorseSaddle_02",
  "GameRomHorseSaddle_03",
  "GameRomHorseSaddle_04",
  "GameRomHorseSaddle_05",
  "GameRomHorseSaddle_06",
  "GameRomHorseSaddle_00L",
  "GameRomHorseSaddle_00S",
  "GameRomHorseSaddle_10",
];

const HORSE_REIN_IDS = [
  "GameRomHorseReins_00",
  "GameRomHorseReins_01",
  "GameRomHorseReins_02",
  "GameRomHorseReins_03",
  "GameRomHorseReins_04",
  "GameRomHorseReins_05",
  "GameRomHorseReins_06",
  "GameRomHorseReins_00L",
  "GameRomHorseReins_10",
];

export interface StringOption {
  value: string;
  label: string;
}

function toOptions(ids: string[]): StringOption[] {
  return ids.map((id) => ({ value: id, label: BOTW_ITEM_NAMES[id] ?? id }));
}

export const HORSE_TYPE_OPTIONS = toOptions(HORSE_TYPE_IDS);
export const HORSE_TYPE_UNTAMED_OPTIONS = toOptions([...HORSE_TYPE_IDS, ...HORSE_TYPE_UNTAMED_IDS]);
export const HORSE_SADDLE_OPTIONS = toOptions(HORSE_SADDLE_IDS);
export const HORSE_REIN_OPTIONS = toOptions(HORSE_REIN_IDS);
