import type {
  TotkArmor,
  TotkArrow,
  TotkBow,
  TotkDevice,
  TotkFood,
  TotkKeyItem,
  TotkMaterial,
  TotkShield,
  TotkWeapon,
} from "../../api";
import * as api from "../../api";
import type { SyntheticEvent } from "react";
import EditableEntryTable, { ColumnDef, QuickFill } from "./EditableEntryTable";
import { totkArmorIconUrl, totkIconUrl, totkUnknownIconUrl, type TotkIconCategory } from "./totkIcons";
import { TOTK_ITEM_NAMES } from "./totkItemNames.data";
import { TOTK_CATEGORY_ITEMS } from "./totkCategoryItems.data";
import {
  WEAPON_MODIFIER_OPTIONS,
  BOW_MODIFIER_OPTIONS,
  SHIELD_MODIFIER_OPTIONS,
  FOOD_EFFECT_OPTIONS,
  DYE_COLOR_OPTIONS,
  weaponModifierIconUrl,
  bowModifierIconUrl,
  shieldModifierIconUrl,
  foodEffectIconUrl,
} from "./totkEnums.data";

function nameFor(entry: { id: string }): string | undefined {
  return TOTK_ITEM_NAMES[entry.id];
}

// Durability is a raw hit-count (a fresh weapon reads ~40); overshooting just gives more hits, so
// a single comfortably-full value covers every item without a per-weapon max table. 999 is the
// standard stack cap these editors use for quantities.
const FULL_DURABILITY = 100;
const MAX_QUANTITY = 999;

const maxQuantityFill = <T extends { quantity: number }>(): QuickFill<T> => ({
  label: "Max quantity",
  icon: "999",
  apply: (e) => ({ ...e, quantity: MAX_QUANTITY }),
});

const weaponDurabilityFill: QuickFill<TotkWeapon> = {
  label: "Restore durability",
  icon: "🛠",
  apply: (e) => ({
    ...e,
    durability: FULL_DURABILITY,
    fuse_durability: e.fuse_id ? FULL_DURABILITY : e.fuse_durability,
  }),
};

const bowDurabilityFill: QuickFill<TotkBow> = {
  label: "Restore durability",
  icon: "🛠",
  apply: (e) => ({ ...e, durability: FULL_DURABILITY }),
};

const shieldDurabilityFill: QuickFill<TotkShield> = {
  label: "Restore durability",
  icon: "🛠",
  apply: (e) => ({
    ...e,
    durability: FULL_DURABILITY,
    fuse_durability: e.fuse_id ? FULL_DURABILITY : e.fuse_durability,
  }),
};

// Reverse map id -> category, so a cross-category value (a fuse target can be anything) can be
// given the right icon. First category to claim an id wins; ids repeated across categories are
// rare and the icon folder is the same shape regardless.
const ITEM_CATEGORY: Record<string, TotkIconCategory> = {};
for (const [cat, ids] of Object.entries(TOTK_CATEGORY_ITEMS)) {
  for (const id of ids) if (!(id in ITEM_CATEGORY)) ITEM_CATEGORY[id] = cat as TotkIconCategory;
}

// Fuse targets are cross-category, but in practice you fuse materials far more than anything else,
// so they lead the list; the rest stay selectable. Leading "" is the "no fusion" choice.
const FUSE_ITEMS: string[] = [
  "",
  ...new Set(
    (["material", "weapon", "bow", "shield", "device", "food", "keyItem", "arrow", "armor"] as const).flatMap(
      (c) => TOTK_CATEGORY_ITEMS[c],
    ),
  ),
];

function fuseIcon(id: string, size: number) {
  if (!id) return null;
  const cat = ITEM_CATEGORY[id];
  const url = cat === "armor" ? totkArmorIconUrl(id, 0) : cat ? totkIconUrl(cat, id) : totkUnknownIconUrl;
  return totkIconImg(url, size);
}

function fuseColumn<T extends { fuse_id: string }>(): ColumnDef<T> {
  return {
    key: "fuse_id" as keyof T,
    label: "Fuse Id",
    type: "itempicker",
    picker: {
      items: FUSE_ITEMS,
      renderIcon: fuseIcon,
      nameFor: (id) => (id === "" ? "(none)" : TOTK_ITEM_NAMES[id]),
    },
  };
}

function text<T>(key: keyof T, label: string): ColumnDef<T> {
  return { key, label, type: "text" };
}

function totkIconImg(url: string, size: number) {
  const onError = (e: SyntheticEvent<HTMLImageElement>) => {
    e.currentTarget.onerror = null;
    e.currentTarget.src = totkUnknownIconUrl;
  };
  return <img src={url} width={size} height={size} style={{ objectFit: "contain" }} alt="" onError={onError} />;
}

// A category-restricted item picker for the "Id" column: shows an icon + name dropdown listing
// only that category's items, in place of a blind free-text field.
function idPicker<T extends { id: string }>(category: TotkIconCategory): ColumnDef<T> {
  const urlFor =
    category === "armor"
      ? (id: string) => totkArmorIconUrl(id, 0)
      : (id: string) => totkIconUrl(category, id);
  return {
    key: "id" as keyof T,
    label: "Id",
    type: "itempicker",
    picker: {
      items: TOTK_CATEGORY_ITEMS[category],
      renderIcon: (id, size) => totkIconImg(urlFor(id), size),
      nameFor: (id) => TOTK_ITEM_NAMES[id],
    },
  };
}

// murmur3 hash of "None" (Equipment.OPTIONS_MODIFIERS' no-bonus value); a modifier value is
// meaningless without a modifier, so its input is disabled while this is selected.
const NO_MODIFIER = 0xb6eede09;

interface NumOpts<T> {
  min?: number | ((entry: T) => number);
  max?: number | ((entry: T) => number);
  disabled?: (entry: T) => boolean;
}

function num<T>(key: keyof T, label: string, opts: NumOpts<T> = {}): ColumnDef<T> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as T[keyof T], ...opts };
}

// Append a new entry to a category, defaulting to the next id after the last row (wrapping),
// copying the last row's other fields so the new item is otherwise valid. Mirrors marcrobledo's
// addItem next-id cycling. `blank` is only used when the category is currently empty.
function addEntry<T extends { id: string }>(
  entries: T[],
  category: TotkIconCategory,
  blank: Omit<T, "id">,
  setter: (entries: T[]) => Promise<void>,
  onError: (message: string) => void,
) {
  const items = TOTK_CATEGORY_ITEMS[category];
  const last = entries[entries.length - 1];
  const nextId = last ? items[(items.indexOf(last.id) + 1) % items.length] : items[0];
  const next = { ...(last ?? (blank as T)), id: nextId } as T;
  setter([...entries, next]).catch((err) => onError(String(err)));
}

// Field defaults used only when adding to a currently-empty category (otherwise the last row's
// fields are copied).
const BLANK_WEAPON: Omit<TotkWeapon, "id"> = { durability: 0, modifier: 0, modifier_value: 0, fuse_id: "", fuse_durability: 0, extra_durability: 0, record_extra_durability: 0 };
const BLANK_BOW: Omit<TotkBow, "id"> = { durability: 0, modifier: 0, modifier_value: 0 };
const BLANK_SHIELD: Omit<TotkShield, "id"> = { durability: 0, modifier: 0, modifier_value: 0, fuse_id: "", fuse_durability: 0, extra_durability: 0 };
const BLANK_ARMOR: Omit<TotkArmor, "id"> = { dye_color: 0 };
const BLANK_ARROW: Omit<TotkArrow, "id"> = { quantity: 1 };
const BLANK_MATERIAL: Omit<TotkMaterial, "id"> = { quantity: 1, get_order: 0, use_order: 0 };
const BLANK_KEYITEM: Omit<TotkKeyItem, "id"> = { quantity: 1 };
const BLANK_DEVICE: Omit<TotkDevice, "id"> = { quantity: 1, use_order: 0 };
const BLANK_FOOD: Omit<TotkFood, "id"> = { quantity: 1, hearts_heal: 0, effect: 0, effect_multiplier: 0, effect_time: 0, price: 0, recipe: ["", "", "", "", ""] };

// Fuse durability records cap at 25 for most fusables in the real editor; durability defaults
// vary per weapon but 70 is the fallback the source uses, so it makes a sensible soft ceiling.
const weaponColumns: ColumnDef<TotkWeapon>[] = [
  idPicker<TotkWeapon>("weapon"),
  num("durability", "Durability", { min: 1, max: 70 }),
  { key: "modifier", label: "Modifier", type: "select", options: WEAPON_MODIFIER_OPTIONS },
  {
    ...num("modifier_value", "Modifier Value", { disabled: (e) => e.modifier === NO_MODIFIER }),
    iconFor: (e) => weaponModifierIconUrl(e.modifier),
  },
  fuseColumn<TotkWeapon>(),
  num("fuse_durability", "Fuse Durability", { min: 0, max: 25 }),
  num("extra_durability", "Extra Durability", { min: 0, max: 25 }),
  num("record_extra_durability", "Record Extra Durability", { min: 0, max: 25 }),
];

const bowColumns: ColumnDef<TotkBow>[] = [
  idPicker<TotkBow>("bow"),
  num("durability", "Durability", { min: 1, max: 70 }),
  { key: "modifier", label: "Modifier", type: "select", options: BOW_MODIFIER_OPTIONS },
  {
    ...num("modifier_value", "Modifier Value", { disabled: (e) => e.modifier === NO_MODIFIER }),
    iconFor: (e) => bowModifierIconUrl(e.modifier),
  },
];

const shieldColumns: ColumnDef<TotkShield>[] = [
  idPicker<TotkShield>("shield"),
  num("durability", "Durability", { min: 1, max: 70 }),
  { key: "modifier", label: "Modifier", type: "select", options: SHIELD_MODIFIER_OPTIONS },
  {
    ...num("modifier_value", "Modifier Value", { disabled: (e) => e.modifier === NO_MODIFIER }),
    iconFor: (e) => shieldModifierIconUrl(e.modifier),
  },
  fuseColumn<TotkShield>(),
  num("fuse_durability", "Fuse Durability", { min: 0, max: 25 }),
  num("extra_durability", "Extra Durability", { min: 0, max: 25 }),
];

const armorColumns: ColumnDef<TotkArmor>[] = [
  idPicker<TotkArmor>("armor"),
  { key: "dye_color", label: "Dye Color", type: "select", options: DYE_COLOR_OPTIONS },
];

const arrowColumns: ColumnDef<TotkArrow>[] = [
  idPicker<TotkArrow>("arrow"),
  num("quantity", "Quantity", { min: 0, max: MAX_QUANTITY }),
];

const materialColumns: ColumnDef<TotkMaterial>[] = [
  idPicker<TotkMaterial>("material"),
  num("quantity", "Quantity", { min: 0, max: MAX_QUANTITY }),
  num("get_order", "Get Order"),
  num("use_order", "Use Order"),
];

const keyItemColumns: ColumnDef<TotkKeyItem>[] = [
  idPicker<TotkKeyItem>("keyItem"),
  num("quantity", "Quantity", { min: 0, max: MAX_QUANTITY }),
];

const deviceColumns: ColumnDef<TotkDevice>[] = [
  idPicker<TotkDevice>("device"),
  num("quantity", "Quantity", { min: 0, max: MAX_QUANTITY }),
  num("use_order", "Use Order"),
];

const foodColumns: ColumnDef<TotkFood>[] = [
  idPicker<TotkFood>("food"),
  num("quantity", "Quantity", { min: 0, max: MAX_QUANTITY }),
  num("hearts_heal", "Hearts Heal", { min: -1, max: 160 }),
  { key: "effect", label: "Food Effect", type: "select", options: FOOD_EFFECT_OPTIONS },
  { ...num("effect_multiplier", "Multiplier", { min: 0, max: 250 }), iconFor: (e) => foodEffectIconUrl(e.effect) },
  num("effect_time", "Duration (in seconds)", { min: 0, max: 59999 }),
  num("price", "Price", { min: 0, max: 999999 }),
  {
    key: "recipe",
    label: "Recipe (comma-separated)",
    format: (v) => (v as string[]).join(","),
    parse: (raw) => {
      const parts = raw.split(",").map((s) => s.trim());
      const padded = [parts[0] ?? "", parts[1] ?? "", parts[2] ?? "", parts[3] ?? "", parts[4] ?? ""];
      return padded as TotkFood["recipe"];
    },
  },
];

interface ErrorProp {
  onError: (message: string) => void;
}

export function TotkWeaponsTable({ weapons, onError }: { weapons: TotkWeapon[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Weapons"
      entries={weapons}
      columns={weaponColumns}
      setter={api.setPouchWeapons}
      onError={onError}
      onAdd={() => addEntry(weapons, "weapon", BLANK_WEAPON, api.setPouchWeapons, onError)}
      quickFills={[weaponDurabilityFill]}
      iconFor={(e) => totkIconUrl("weapon", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkBowsTable({ bows, onError }: { bows: TotkBow[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Bows"
      entries={bows}
      columns={bowColumns}
      setter={api.setPouchBows}
      onError={onError}
      onAdd={() => addEntry(bows, "bow", BLANK_BOW, api.setPouchBows, onError)}
      quickFills={[bowDurabilityFill]}
      iconFor={(e) => totkIconUrl("bow", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkShieldsTable({ shields, onError }: { shields: TotkShield[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Shields"
      entries={shields}
      columns={shieldColumns}
      setter={api.setPouchShields}
      onError={onError}
      onAdd={() => addEntry(shields, "shield", BLANK_SHIELD, api.setPouchShields, onError)}
      quickFills={[shieldDurabilityFill]}
      iconFor={(e) => totkIconUrl("shield", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkArmorTable({ armor, onError }: { armor: TotkArmor[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Armor"
      entries={armor}
      columns={armorColumns}
      setter={api.setArmor}
      onError={onError}
      onAdd={() => addEntry(armor, "armor", BLANK_ARMOR, api.setArmor, onError)}
      iconFor={(e) => totkArmorIconUrl(e.id, e.dye_color)}
      nameFor={nameFor}
    />
  );
}

export function TotkArrowsTable({ arrows, onError }: { arrows: TotkArrow[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Arrows"
      entries={arrows}
      columns={arrowColumns}
      setter={api.setArrows}
      onError={onError}
      onAdd={() => addEntry(arrows, "arrow", BLANK_ARROW, api.setArrows, onError)}
      quickFills={[maxQuantityFill<TotkArrow>()]}
      iconFor={(e) => totkIconUrl("arrow", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkMaterialsTable({ materials, onError }: { materials: TotkMaterial[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Materials"
      entries={materials}
      columns={materialColumns}
      setter={api.setMaterials}
      onError={onError}
      onAdd={() => addEntry(materials, "material", BLANK_MATERIAL, api.setMaterials, onError)}
      quickFills={[maxQuantityFill<TotkMaterial>()]}
      iconFor={(e) => totkIconUrl("material", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkKeyItemsTable({ keyItems, onError }: { keyItems: TotkKeyItem[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Key Items"
      entries={keyItems}
      columns={keyItemColumns}
      setter={api.setKeyItems}
      onError={onError}
      onAdd={() => addEntry(keyItems, "keyItem", BLANK_KEYITEM, api.setKeyItems, onError)}
      quickFills={[maxQuantityFill<TotkKeyItem>()]}
      iconFor={(e) => totkIconUrl("keyItem", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkDevicesTable({ devices, onError }: { devices: TotkDevice[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Devices"
      entries={devices}
      columns={deviceColumns}
      setter={api.setDevices}
      onError={onError}
      onAdd={() => addEntry(devices, "device", BLANK_DEVICE, api.setDevices, onError)}
      quickFills={[maxQuantityFill<TotkDevice>()]}
      iconFor={(e) => totkIconUrl("device", e.id)}
      nameFor={nameFor}
    />
  );
}

export function TotkFoodTable({ food, onError }: { food: TotkFood[] } & ErrorProp) {
  return (
    <EditableEntryTable
      title="Food"
      entries={food}
      columns={foodColumns}
      setter={api.setFood}
      onError={onError}
      onAdd={() => addEntry(food, "food", BLANK_FOOD, api.setFood, onError)}
      quickFills={[maxQuantityFill<TotkFood>()]}
      iconFor={(e) => totkIconUrl("food", e.id)}
      nameFor={nameFor}
    />
  );
}
