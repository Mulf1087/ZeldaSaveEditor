import { ICON_COLS, ICONS, SHEET_DIMENSIONS, REPEAT_ARMOR_ICONS_1, REPEAT_ARMOR_ICONS_2 } from "./botwIcons.data";

export const botwUnknownIconUrl = "/icons/botw/_blank.webp";

interface Resolved {
  sheet: string;
  slot: string;
}

// Ports `zelda-botw.icons.js`'s `_getItemIcon` classification: turns a save's raw item id into
// (sprite sheet, slot id within that sheet). Order matters, each branch's regex is checked in
// the same sequence as the source so earlier, more specific patterns win over the catch-all.
function classify(itemNameId: string): Resolved | null {
  let match: RegExpMatchArray | null;

  if ((match = itemNameId.match(/^Armor_([0-9]{3})_(Head|Upper|Lower)(_Dye[0-9]{2})?/))) {
    const sheet = "Armor_" + match[2];
    let slot = match[1];
    if (match[2] === "Lower" && slot === "140") slot = "141";
    const repeatIndex = REPEAT_ARMOR_ICONS_1.indexOf(slot);
    if (repeatIndex >= 0) slot = REPEAT_ARMOR_ICONS_2[Math.floor(repeatIndex / 4)];
    return { sheet, slot };
  }

  if ((match = itemNameId.match(/^Item_(Chilled|ChilledFish|Enemy|Material|Meat|Roast|RoastFish)_([0-9]{2})/))) {
    return { sheet: "Item_" + match[1], slot: match[2] };
  }

  if (itemNameId === "Item_Enemy_Put_57") {
    return { sheet: "Item_Enemy", slot: "Put_57" };
  }

  const normalized = itemNameId
    .replace("Animal_Insect_", "Item_InsectGet_")
    .replace("Mushroom_", "MushroomGet_")
    .replace("Plant_", "PlantGet_");
  if ((match = normalized.match(/^Item_(FishGet|Fruit|InsectGet|MushroomGet|Ore|PlantGet)_([A-Z][A-B]?)/))) {
    const sheet = "Item_" + match[1];
    let slot = match[2];
    if ((slot === "K" || slot === "O" || slot === "Z") && itemNameId.startsWith("Animal_Insect_")) {
      slot = "Normal" + slot;
    }
    return { sheet, slot };
  }

  if ((match = itemNameId.match(/^Item_Cook_([A-P]_[0-9]{2})/))) {
    return { sheet: "Item_Cook", slot: match[1] };
  }

  if ((match = itemNameId.match(/^Weapon_(Sword|Lsword|Spear|Bow|Shield)_([0-9]{3})/))) {
    return { sheet: "Weapon_" + match[1], slot: match[2] };
  }

  return { sheet: "Other", slot: itemNameId };
}

export interface BotwIconStyle {
  /** Fixed-size outer slot, flex-centers the (possibly narrower/shorter) inner cell. */
  wrapper: { width: number; height: number; display: "flex"; alignItems: "center"; justifyContent: "center" };
  /**
   * The actual sliced icon, sized to exactly one scaled cell. Sizing the box itself to the cell
   * (rather than leaving it at `displaySize` and only shifting `background-position`) is what
   * keeps neighboring cells from ever showing through: a `displaySize`-wide box is wider than a
   * narrow cell, so any background-position shift still leaves slivers of the next cell visible
   * inside the leftover width. A box that IS the cell's own size has no leftover width to leak
   * through in the first place.
   */
  cell: {
    backgroundImage: string;
    backgroundSize: string;
    backgroundPosition: string;
    backgroundRepeat: "no-repeat";
    width: number;
    height: number;
  };
}

/**
 * Style pair for rendering the given item's icon at `displaySize` pixels, sliced from its sprite
 * sheet. Sheets are a 16-column grid, but not every sheet has square cells: single-row sheets
 * (fruits, mushrooms, ore, meat, fish...) are narrower per cell than they are tall (see
 * `SHEET_DIMENSIONS`). Scaling proportionally to fit the larger of the two cell dimensions into
 * `displaySize` shows the whole icon instead of cropping it to a forced square.
 */
export function botwIconStyle(itemNameId: string, dyeColor: number | undefined, displaySize: number): BotwIconStyle | null {
  const resolved = classify(itemNameId);
  if (!resolved) return null;

  const slots = ICONS[resolved.sheet];
  let index = slots ? slots.indexOf(resolved.slot) : -1;
  if (index === -1) return null;

  if (itemNameId.startsWith("Armor_") && dyeColor && dyeColor <= 15) {
    index += dyeColor;
  }

  const dims = SHEET_DIMENSIONS[resolved.sheet];
  if (!dims) return null;
  const [sheetW, sheetH] = dims;
  const rows = Math.ceil(slots.length / ICON_COLS);
  // Multi-row sheets are a real 16-wide grid (confirmed against the vendored art directly), but
  // a single-row sheet isn't padded out to 16 columns, it's packed to exactly however many icons
  // it holds (e.g. Item_Fruit is 12 icons across the sheet's full width, not 12 icons + 4 empty
  // slots). Math.min(ICON_COLS, slots.length) picks the right divisor for both cases: it's
  // ICON_COLS whenever there are enough slots to fill a row, and the real (smaller) count
  // otherwise.
  const cellW = sheetW / Math.min(ICON_COLS, slots.length);
  const cellH = sheetH / rows;
  const scale = displaySize / Math.max(cellW, cellH);
  const scaledCellW = cellW * scale;
  const scaledCellH = cellH * scale;

  const col = index % ICON_COLS;
  const row = Math.floor(index / ICON_COLS);
  return {
    wrapper: { width: displaySize, height: displaySize, display: "flex", alignItems: "center", justifyContent: "center" },
    cell: {
      backgroundImage: `url(/icons/botw/${resolved.sheet}.webp)`,
      backgroundSize: `${sheetW * scale}px ${sheetH * scale}px`,
      backgroundPosition: `${-(col * scaledCellW)}px ${-(row * scaledCellH)}px`,
      backgroundRepeat: "no-repeat",
      width: scaledCellW,
      height: scaledCellH,
    },
  };
}
