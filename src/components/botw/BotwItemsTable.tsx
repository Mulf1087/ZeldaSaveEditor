import { useState, type ReactNode } from "react";
import type { BotwItem, BotwItemCategory, ItemModifier, ModifierCategory } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import { botwIconStyle, botwUnknownIconUrl } from "./botwIcons";
import { BOTW_ITEM_NAMES } from "./botwItemNames.data";
import { BOTW_CATEGORY_ITEMS } from "./botwCategoryItems.data";
import { MODIFIER_OPTIONS, DYE_COLOR_OPTIONS } from "./botwEnums.data";
import { withCurrentValue } from "../Select";
import ItemPicker from "../ItemPicker";
import HoverPreview from "../HoverPreview";
import { rowHeaderToggle, guardBounds } from "../totk/EditableEntryTable";
import styles from "./BotwItemsTable.module.css";

const ICON_SIZE = 48;
// The flat item array's capacity; a new item goes in the first empty slot at the end, so adding
// is blocked once every slot is used. Mirrors the engine's own MAX_ITEMS.
const MAX_ITEMS = 420;

// Per-item quantity ceilings, mirroring the real editor: stackables cap at 999, equipment stores
// durability*100 (so ~65535 durability), and the two counter items have their own limits. Used as
// a soft guard, so a first over-limit entry snaps down but a repeat entry still goes through.
function itemMaxQuantity(item: BotwItem): number {
  const id = item.name;
  if (id.endsWith("Arrow") || id.endsWith("Arrow_A") || item.category === "material" || item.category === "food")
    return 999;
  if (item.category === "weapon" || item.category === "bow" || item.category === "shield") return 6553500;
  if (id === "Obj_DungeonClearSeal") return 120;
  if (id === "Obj_KorokNuts") return 900;
  return 0xffffffff;
}

// BOTW icons are sliced from shared sprite sheets (CSS background-position), not one file per item.
function botwSpriteIcon(id: string, size: number, dyeColor?: number): ReactNode {
  const style = botwIconStyle(id, dyeColor, size);
  if (!style)
    return <img src={botwUnknownIconUrl} width={size} height={size} style={{ objectFit: "contain" }} alt="" />;
  return (
    <div style={style.wrapper}>
      <div style={style.cell} />
    </div>
  );
}

function ItemIcon({ item }: { item: BotwItem }) {
  const dyeColor = item.category === "armor" ? item.quantity : undefined;
  return (
    <HoverPreview preview={botwSpriteIcon(item.name, 144, dyeColor)}>
      {botwSpriteIcon(item.name, ICON_SIZE, dyeColor)}
    </HoverPreview>
  );
}

function ModifierCell({
  category,
  categoryIndex,
  modifier,
  onError,
}: {
  category: ModifierCategory;
  categoryIndex: number;
  modifier: ItemModifier;
  onError: (message: string) => void;
}) {
  const [value, setValue] = useState(String(modifier.value));
  const commitModifier = (raw: string) =>
    api.setModifier(category, categoryIndex, Number(raw), modifier.value).catch((err) => onError(String(err)));
  const commitValue = () =>
    api.setModifier(category, categoryIndex, modifier.modifier, Number(value)).catch((err) => onError(String(err)));
  return (
    <>
      <td data-label="Modifier">
        <select
          className={styles.input}
          value={String(modifier.modifier)}
          onChange={(e) => commitModifier(e.target.value)}
        >
          {withCurrentValue(MODIFIER_OPTIONS, modifier.modifier).map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
      </td>
      <td data-label="Modifier Value">
        <input
          className={styles.input}
          type="number"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onBlur={commitValue}
          disabled={modifier.modifier === 0}
        />
      </td>
    </>
  );
}

function ItemRow({
  index,
  item,
  valueLabel,
  modifierCategory,
  modifier,
  categoryIndex,
  pickerItems,
  allowMaxQuantity,
  onError,
}: {
  index: number;
  item: BotwItem;
  valueLabel: string;
  modifierCategory: ModifierCategory | null;
  modifier: ItemModifier | undefined;
  categoryIndex: number;
  pickerItems: string[];
  allowMaxQuantity: boolean;
  onError: (message: string) => void;
}) {
  const [name, setName] = useState(item.name);
  const [quantity, setQuantity] = useState(String(item.quantity));
  const [expanded, setExpanded] = useState(false);
  const toggle = () => setExpanded((v) => !v);
  const commit = () => {
    const guarded = guardBounds(Number(quantity), item.quantity, 0, itemMaxQuantity(item));
    if (guarded !== Number(quantity)) setQuantity(String(guarded));
    api.setItem(index, name, guarded).catch((err) => onError(String(err)));
  };
  const remove = () => api.removeItem(index).catch((err) => onError(String(err)));
  const duplicate = () => api.duplicateItem(index).catch((err) => onError(String(err)));
  const maxQuantity = () => {
    setQuantity("999");
    api.setItem(index, name, 999).catch((err) => onError(String(err)));
  };
  const isDye = valueLabel === "Dye Color";

  return (
    <tr data-collapsed={expanded ? "false" : "true"} onClick={(e) => rowHeaderToggle(e, toggle)}>
      <td data-summary>
        <button
          type="button"
          className={styles.rowToggle}
          aria-expanded={expanded}
          aria-label={expanded ? "Collapse" : "Expand"}
          onClick={(e) => {
            e.stopPropagation();
            toggle();
          }}
        >
          {expanded ? "▾" : "▸"}
        </button>
        <span>{index}</span>
      </td>
      <td data-summary>
        <ItemIcon item={item} />
      </td>
      <td data-summary data-grow>
        {BOTW_ITEM_NAMES[item.name] ?? ""}
      </td>
      <td data-label="Id">
        <ItemPicker
          value={name}
          items={pickerItems}
          renderIcon={(id, size) => botwSpriteIcon(id, size)}
          nameFor={(id) => BOTW_ITEM_NAMES[id]}
          onCommit={(id) => {
            setName(id);
            api.setItem(index, id, Number(quantity)).catch((err) => onError(String(err)));
          }}
        />
      </td>
      <td data-label={valueLabel}>
        {isDye ? (
          <select
            className={styles.input}
            value={quantity}
            onChange={(e) => {
              setQuantity(e.target.value);
              api.setItem(index, name, Number(e.target.value)).catch((err) => onError(String(err)));
            }}
          >
            {withCurrentValue(DYE_COLOR_OPTIONS, Number(quantity)).map((o) => (
              <option key={o.value} value={o.value}>
                {o.label}
              </option>
            ))}
          </select>
        ) : (
          <input
            className={styles.input}
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(e.target.value)}
            onBlur={commit}
          />
        )}
      </td>
      {modifierCategory &&
        (modifier ? (
          <ModifierCell category={modifierCategory} categoryIndex={categoryIndex} modifier={modifier} onError={onError} />
        ) : (
          <>
            <td data-label="Modifier" />
            <td data-label="Modifier Value" />
          </>
        ))}
      <td data-label="Actions">
        <div className={styles.rowActions}>
          {allowMaxQuantity && (
            <button
              type="button"
              className={styles.rowAction}
              title="Max quantity"
              onClick={(e) => {
                e.stopPropagation();
                maxQuantity();
              }}
            >
              999
            </button>
          )}
          <button
            type="button"
            className={styles.rowAction}
            title="Duplicate"
            onClick={(e) => {
              e.stopPropagation();
              duplicate();
            }}
          >
            ⧉
          </button>
          <button
            type="button"
            className={styles.rowAction}
            title="Delete"
            onClick={(e) => {
              e.stopPropagation();
              remove();
            }}
          >
            ✕
          </button>
        </div>
      </td>
    </tr>
  );
}

interface CategoryTableProps {
  title: string;
  valueLabel: string;
  items: BotwItem[];
  category: BotwItemCategory;
  modifiers?: ItemModifier[];
  /** Narrows a category to a sub-set (e.g. bows vs. arrows, which share the "bow" category). */
  subFilter?: (item: BotwItem) => boolean;
  /** Overrides the id picker's option list, so a split section only offers its own sub-set. */
  pickerItems?: string[];
  onError: (message: string) => void;
}

export function BotwCategoryTable({
  title,
  valueLabel,
  items,
  category,
  modifiers,
  subFilter,
  pickerItems,
  onError,
}: CategoryTableProps) {
  const entries = items
    .map((item, index) => ({ item, index }))
    .filter(({ item }) => item.category === category && (!subFilter || subFilter(item)));
  // Modifier columns only apply where modifiers are actually supplied. Bows/weapons/shields pass a
  // modifier list; the arrows section shares the "bow" category but has no modifiers, so it must not
  // render those columns.
  const modifierCategory: ModifierCategory | null =
    modifiers && (category === "weapon" || category === "bow" || category === "shield") ? category : null;
  const pickerOptions = pickerItems ?? BOTW_CATEGORY_ITEMS[category];

  // A new item is appended into the first empty slot at the end of the flat array (index equal to
  // the current item count), mirroring marcrobledo's addItem. The default id is the first entry of
  // this section's own id list so the row lands under the table the button belongs to.
  const addItem = () => {
    const id = pickerOptions[0];
    if (!id) return;
    api.setItem(items.length, id, 1).catch((err) => onError(String(err)));
  };

  return (
    <div>
      <SectionHeading title={`${title} (${entries.length})`} motif={<BotwMotif />} />
      <div className={styles.tableWrap}>
        <table className={styles.table}>
          <thead>
            <tr>
              <th>#</th>
              <th></th>
              <th>Name</th>
              <th>Id</th>
              <th>{valueLabel}</th>
              {modifierCategory && (
                <>
                  <th>Modifier</th>
                  <th>Modifier Value</th>
                </>
              )}
              <th></th>
            </tr>
          </thead>
          <tbody>
            {entries.map(({ item, index }, categoryIndex) => (
              <ItemRow
                key={index}
                index={index}
                item={item}
                valueLabel={valueLabel}
                modifierCategory={modifierCategory}
                modifier={modifiers?.[categoryIndex]}
                categoryIndex={categoryIndex}
                pickerItems={pickerOptions}
                allowMaxQuantity={valueLabel === "Quantity"}
                onError={onError}
              />
            ))}
          </tbody>
        </table>
      </div>
      <button
        type="button"
        className={styles.addButton}
        onClick={addItem}
        disabled={items.length >= MAX_ITEMS}
      >
        + Add {title}
      </button>
    </div>
  );
}
