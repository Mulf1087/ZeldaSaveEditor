import { armorBaseId } from "./totkArmorFamilies.data";

const CATEGORY_DIRS = {
  weapon: "weapons",
  bow: "bows",
  shield: "shields",
  armor: "armors",
  arrow: "arrows",
  material: "materials",
  keyItem: "key",
  device: "devices",
  food: "food",
} as const;

export type TotkIconCategory = keyof typeof CATEGORY_DIRS;

export const totkUnknownIconUrl = "/icons/totk/unknown.webp";

export function totkIconUrl(category: TotkIconCategory, id: string): string {
  return `/icons/totk/${CATEGORY_DIRS[category]}/${id}.webp`;
}

// murmur3("<name>") of each dye color, computed with the engine's own hash function
// (totk::murmur3::hash32) since armor's dye_color field stores a name hash, not an index.
export const DYE_COLOR_NAMES: Record<number, string> = {
  0xb6eede09: "None",
  0x6cbc3cb4: "Black",
  0xe2911aba: "Blue",
  0xb364bb2c: "Brown",
  0xf8bdf528: "Crimson",
  0x762266bf: "Gray",
  0x7c9b6ddb: "Green",
  0x01666931: "LightBlue",
  0xdf26c6da: "LightYellow",
  0x0adfd3a1: "Navy",
  0x619ec353: "Orange",
  0xeaf26a09: "Pink",
  0x7f0ae256: "Purple",
  0x6e1a9181: "Red",
  0x4402060c: "White",
  0xc03f6678: "Yellow",
};

export function totkArmorIconUrl(id: string, dyeColor: number): string {
  const baseId = armorBaseId(id);
  const colorName = DYE_COLOR_NAMES[dyeColor];
  if (!colorName || colorName === "None") return totkIconUrl("armor", baseId);
  return `/icons/totk/armors/dye/${baseId}_${colorName}.webp`;
}
