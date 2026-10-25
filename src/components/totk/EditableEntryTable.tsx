import { useState, type MouseEvent as ReactMouseEvent, type ReactNode } from "react";
import SectionHeading from "../../theme/SectionHeading";
import { withCurrentValue, type SelectOption } from "../Select";
import ItemIcon from "./ItemIcon";
import ItemPicker from "../ItemPicker";
import styles from "./EditableEntryTable.module.css";

export type { SelectOption };

export interface DatalistDef {
  /** Must be unique within the page; shared by every row's input in this column via the
   * standard HTML `list` attribute, so the option list itself is rendered once per column
   * rather than duplicated per row. */
  id: string;
  options: { value: string; label: string }[];
}

export interface PickerDef {
  /** Ids selectable in this column, restricted to the table's category. */
  items: string[];
  renderIcon: (id: string, size: number) => ReactNode;
  nameFor: (id: string) => string | undefined;
}

/** A one-click field preset shown in a row's actions cell (e.g. "Restore durability"). `apply`
 * returns the whole row with the preset fields changed. */
export interface QuickFill<T> {
  label: string;
  icon: string;
  apply: (entry: T) => T;
}

export interface ColumnDef<T> {
  key: keyof T;
  label: string;
  type?: "text" | "number" | "checkbox" | "readonly" | "select" | "itempicker";
  format?: (value: T[keyof T]) => string;
  parse?: (raw: string) => T[keyof T];
  options?: SelectOption[];
  /** Turns the column into a category-restricted item picker (icon + name dropdown). */
  picker?: PickerDef;
  /** Turns a text column into a searchable autocomplete (native <datalist>) instead of a blind
   * free-text field, for id-shaped values a human can't reasonably type from memory (fuse
   * targets, etc). Still a plain text input underneath, so an unrecognized/custom id can still
   * be typed directly. */
  datalist?: DatalistDef;
  /** Small icon shown next to this cell, derived from the whole row (e.g. a bonus icon
   * matching a sibling `modifier` column's current value). */
  iconFor?: (entry: T) => string | undefined;
  /** Soft bounds for a number cell. May depend on the row (e.g. a per-item durability cap).
   * See `guardBounds` for the clamp-once-then-allow behavior. */
  min?: number | ((entry: T) => number);
  max?: number | ((entry: T) => number);
  /** Greys out and blocks editing when true (e.g. a modifier value with no modifier set). */
  disabled?: (entry: T) => boolean;
}

/** Soft upper/lower guard. The first time a value lands outside [min, max] (the previous value
 * was still inside the bound) it snaps to that bound; a repeat out-of-range entry passes through,
 * so it nudges rather than hard-caps. */
export function guardBounds(value: number, prev: number, min?: number, max?: number): number {
  if (typeof max === "number" && value > max && prev < max) return max;
  if (typeof min === "number" && value < min && prev > min) return min;
  return value;
}

interface CellProps<T> {
  entries: T[];
  index: number;
  column: ColumnDef<T>;
  setter: (entries: T[]) => Promise<void>;
  onError: (message: string) => void;
}

/** Toggles a card's expanded state from a click anywhere on its header. Clicks that land on an
 * editable control (or inside an expanded detail cell) are ignored so editing still works; the
 * chevron button handles its own toggle and stops propagation, so it isn't double-counted. Shared
 * with the BOTW item rows, which have the same card behavior. */
export function rowHeaderToggle(e: ReactMouseEvent, toggle: () => void) {
  if ((e.target as HTMLElement).closest("input, select, textarea, [data-label]")) return;
  toggle();
}

function defaultFormat(value: unknown): string {
  return value === null || value === undefined ? "" : String(value);
}

function replaceAt<T>(entries: T[], index: number, key: keyof T, value: T[keyof T]): T[] {
  return entries.map((e, i) => (i === index ? { ...e, [key]: value } : e));
}

function CellIcon<T>({ column, entry }: { column: ColumnDef<T>; entry: T }) {
  const url = column.iconFor?.(entry);
  if (!url) return null;
  return <img className={styles.bonusIcon} src={url} alt="" />;
}

function TextCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const format = column.format ?? defaultFormat;
  const [text, setText] = useState(format(entries[index][column.key]));
  const entry = entries[index];
  const bound = (b?: number | ((entry: T) => number)) => (typeof b === "function" ? b(entry) : b);
  const commit = () => {
    const parse = column.parse ?? ((raw: string) => raw as unknown as T[keyof T]);
    const parsed = parse(text);
    let toStore = parsed;
    const prev = entry[column.key];
    if (typeof parsed === "number" && typeof prev === "number") {
      const guarded = guardBounds(parsed, prev, bound(column.min), bound(column.max));
      if (guarded !== parsed) setText(String(guarded));
      toStore = guarded as T[keyof T];
    }
    setter(replaceAt(entries, index, column.key, toStore)).catch((err) => onError(String(err)));
  };
  return (
    <td data-label={column.label}>
      <div className={styles.cellRow}>
        <input
          className={styles.input}
          type={column.type === "number" ? "number" : "text"}
          value={text}
          onChange={(e) => setText(e.target.value)}
          onBlur={commit}
          disabled={column.disabled?.(entry)}
          list={column.datalist?.id}
        />
        <CellIcon column={column} entry={entry} />
      </div>
    </td>
  );
}

function SelectCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const rawValue = entries[index][column.key] as unknown as number;
  const options = withCurrentValue(column.options ?? [], rawValue);
  const commit = (raw: string) => {
    const value = Number(raw) as T[keyof T];
    setter(replaceAt(entries, index, column.key, value)).catch((err) => onError(String(err)));
  };
  return (
    <td data-label={column.label}>
      <div className={styles.cellRow}>
        <select className={styles.select} value={String(rawValue)} onChange={(e) => commit(e.target.value)}>
          {options.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
        <CellIcon column={column} entry={entries[index]} />
      </div>
    </td>
  );
}

function PickerCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const picker = column.picker!;
  const value = String(entries[index][column.key] ?? "");
  const commit = (id: string) =>
    setter(replaceAt(entries, index, column.key, id as unknown as T[keyof T])).catch((err) =>
      onError(String(err)),
    );
  return (
    <td data-label={column.label}>
      <ItemPicker
        value={value}
        items={picker.items}
        renderIcon={picker.renderIcon}
        nameFor={picker.nameFor}
        onCommit={commit}
      />
    </td>
  );
}

function CheckboxCell<T>({ entries, index, column, setter, onError }: CellProps<T>) {
  const checked = Boolean(entries[index][column.key]);
  const commit = (next: boolean) => {
    setter(replaceAt(entries, index, column.key, next as unknown as T[keyof T])).catch((err) =>
      onError(String(err)),
    );
  };
  return (
    <td data-label={column.label}>
      <input type="checkbox" checked={checked} onChange={(e) => commit(e.target.checked)} />
    </td>
  );
}

function ReadOnlyCell<T>({ entries, index, column }: CellProps<T>) {
  const format = column.format ?? defaultFormat;
  return <td data-label={column.label}>{format(entries[index][column.key])}</td>;
}

function Cell<T>(props: CellProps<T>) {
  if (props.column.type === "checkbox") return <CheckboxCell {...props} />;
  if (props.column.type === "readonly") return <ReadOnlyCell {...props} />;
  if (props.column.type === "select") return <SelectCell {...props} />;
  if (props.column.type === "itempicker") return <PickerCell {...props} />;
  return <TextCell {...props} />;
}

interface RowProps<T> {
  entries: T[];
  index: number;
  columns: ColumnDef<T>[];
  setter: (entries: T[]) => Promise<void>;
  onError: (message: string) => void;
  iconFor?: (entry: T) => string;
  nameFor?: (entry: T) => string | undefined;
  /** When true, renders per-row delete/duplicate controls. */
  rowActions?: boolean;
  quickFills?: QuickFill<T>[];
}

function EntryRow<T>({ entries, index, columns, setter, onError, iconFor, nameFor, rowActions, quickFills }: RowProps<T>) {
  // Rows start collapsed; the flag only takes effect at narrow widths (see `.table` media query),
  // where a chevron in the header expands the detail fields. At wide widths it's ignored.
  const [expanded, setExpanded] = useState(false);
  const toggle = () => setExpanded((v) => !v);
  const entry = entries[index];
  const remove = () => setter(entries.filter((_, i) => i !== index)).catch((err) => onError(String(err)));
  const duplicate = () =>
    setter([...entries.slice(0, index + 1), entries[index], ...entries.slice(index + 1)]).catch((err) =>
      onError(String(err)),
    );
  const applyFill = (fill: QuickFill<T>) =>
    setter(entries.map((e, i) => (i === index ? fill.apply(e) : e))).catch((err) => onError(String(err)));
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
      {iconFor && (
        <td data-summary>
          <ItemIcon url={iconFor(entry)} />
        </td>
      )}
      {nameFor && (
        <td data-summary data-grow>
          {nameFor(entry) ?? ""}
        </td>
      )}
      {columns.map((c) => (
        <Cell key={String(c.key)} entries={entries} index={index} column={c} setter={setter} onError={onError} />
      ))}
      {rowActions && (
        <td data-label="Actions">
          <div className={styles.rowActions}>
            {quickFills?.map((q) => (
              <button
                key={q.label}
                type="button"
                className={styles.rowAction}
                title={q.label}
                onClick={(e) => {
                  e.stopPropagation();
                  applyFill(q);
                }}
              >
                {q.icon}
              </button>
            ))}
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
      )}
    </tr>
  );
}

interface Props<T> {
  title: string;
  entries: T[];
  columns: ColumnDef<T>[];
  setter: (entries: T[]) => Promise<void>;
  onError: (message: string) => void;
  level?: "h3" | "h4";
  motif?: ReactNode;
  iconFor?: (entry: T) => string;
  nameFor?: (entry: T) => string | undefined;
  /** Renders an "Add" button below the table when provided. */
  onAdd?: () => void;
  /** One-click field presets shown per row (only rendered alongside the delete/duplicate actions). */
  quickFills?: QuickFill<T>[];
}

export default function EditableEntryTable<T>({
  title,
  entries,
  columns,
  setter,
  onError,
  level = "h4",
  motif,
  iconFor,
  nameFor,
  onAdd,
  quickFills,
}: Props<T>) {
  return (
    <div>
      <SectionHeading level={level} motif={motif} title={`${title} (${entries.length})`} />
      {Array.from(new Map(columns.filter((c) => c.datalist).map((c) => [c.datalist!.id, c.datalist!])).values()).map(
        (dl) => (
          <datalist key={dl.id} id={dl.id}>
            {dl.options.map((o) => (
              <option key={o.value} value={o.value}>
                {o.label}
              </option>
            ))}
          </datalist>
        ),
      )}
      <div className={styles.tableWrap}>
        <table className={styles.table}>
          <thead>
            <tr>
              <th>#</th>
              {iconFor && <th></th>}
              {nameFor && <th>Name</th>}
              {columns.map((c) => (
                <th key={String(c.key)}>{c.label}</th>
              ))}
              {onAdd && <th></th>}
            </tr>
          </thead>
          <tbody>
            {entries.map((_, i) => (
              <EntryRow
                key={i}
                entries={entries}
                index={i}
                columns={columns}
                setter={setter}
                onError={onError}
                iconFor={iconFor}
                nameFor={nameFor}
                rowActions={Boolean(onAdd)}
                quickFills={quickFills}
              />
            ))}
          </tbody>
        </table>
      </div>
      {onAdd && (
        <button type="button" className={styles.addButton} onClick={onAdd}>
          + Add {title}
        </button>
      )}
    </div>
  );
}
