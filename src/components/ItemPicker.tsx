import { useEffect, useLayoutEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { createPortal } from "react-dom";
import styles from "./ItemPicker.module.css";

// On a touch device, autofocusing the search input pops the on-screen keyboard open immediately,
// which can cover the dropdown it's meant to filter. Desktop keeps it, since typing to filter
// without clicking first is the point there and there's no keyboard to fight.
const isCoarsePointer = window.matchMedia("(pointer: coarse)").matches;

interface Props {
  value: string;
  items: string[];
  /** Renders an item's icon at the given pixel size. Games differ (TOTK <img>, BOTW sprite sheet),
   * so the caller supplies the rendering. */
  renderIcon: (id: string, size: number) => ReactNode;
  nameFor: (id: string) => string | undefined;
  onCommit: (id: string) => void;
}

/** Category-restricted item chooser: a trigger showing the current item's icon + name, and a
 * searchable dropdown of the same category's items (icon + name), matching marcrobledo's own
 * filterable item picker. A native <select> can't render per-option images, hence the custom
 * combobox. */
export default function ItemPicker({ value, items, renderIcon, nameFor, onCommit }: Props) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [active, setActive] = useState(0);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);
  const activeElRef = useRef<HTMLDivElement | null>(null);
  const activeNeedsScrollRef = useRef(false);
  const [panelPos, setPanelPos] = useState({ top: 0, left: 0 });

  const label = (id: string) => {
    const name = nameFor(id);
    return name ? `${name} (${id})` : id;
  };

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return items;
    return items.filter((id) => label(id).toLowerCase().includes(q));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [query, items]);

  useEffect(() => {
    setActive((a) => Math.min(a, Math.max(0, filtered.length - 1)));
  }, [filtered.length]);

  // Scrolls the active row into view, but only when it became active via keyboard navigation.
  // Following mouse/touch hover here fights the user: on Android, scrolling the list itself can
  // synthesize hover-like events on rows the finger passes over, so an unconditional scroll-follow
  // snaps the list back toward whichever row that was, mid-scroll, every time.
  useEffect(() => {
    if (activeNeedsScrollRef.current) {
      activeElRef.current?.scrollIntoView({ block: "nearest" });
    }
  }, [active]);

  // Portaled to <body> and positioned with fixed coordinates so tables with overflow-x/y
  // scrolling (see .tableWrap) don't clip the dropdown, and clamped to the viewport (flipping
  // above the trigger, or sliding left) so it doesn't run off the window either. Runs in a layout
  // effect, before paint, so the panel never flashes at its unpositioned (0,0) default.
  useLayoutEffect(() => {
    if (!open) return;
    const margin = 8;
    const reposition = () => {
      const triggerRect = triggerRef.current?.getBoundingClientRect();
      if (!triggerRect) return;
      const panelRect = panelRef.current?.getBoundingClientRect();
      const panelWidth = panelRect?.width ?? 300;
      const panelHeight = panelRect?.height ?? 0;

      let left = triggerRect.left;
      left = Math.min(left, window.innerWidth - margin - panelWidth);
      left = Math.max(margin, left);

      let top = triggerRect.bottom + 2;
      if (top + panelHeight > window.innerHeight - margin) {
        const above = triggerRect.top - 2 - panelHeight;
        top = above >= margin ? above : Math.max(margin, window.innerHeight - margin - panelHeight);
      }

      setPanelPos({ top, left });
    };
    // Scroll is captured (true) so scrolling any ancestor of the trigger repositions the panel,
    // but that also catches the panel's own option list scrolling internally, which shouldn't
    // move the panel at all: skip those so scrolling the results doesn't fight itself.
    const onScroll = (e: Event) => {
      if (panelRef.current?.contains(e.target as Node)) return;
      reposition();
    };
    reposition();
    window.addEventListener("scroll", onScroll, true);
    window.addEventListener("resize", reposition);
    return () => {
      window.removeEventListener("scroll", onScroll, true);
      window.removeEventListener("resize", reposition);
    };
  }, [open, filtered.length]);

  // Click-outside and tab-away close. (A blur handler doesn't work here: the portaled panel isn't
  // a DOM descendant of .picker, and autoFocus on the search input fires blur before panelRef
  // attaches. focusin covers keyboard-driven focus moves the way blur/relatedTarget used to.)
  useEffect(() => {
    if (!open) return;
    const isOutside = (target: Node) =>
      !triggerRef.current?.contains(target) && !panelRef.current?.contains(target);
    const onPointerDown = (e: MouseEvent) => {
      if (isOutside(e.target as Node)) setOpen(false);
    };
    const onFocusIn = (e: FocusEvent) => {
      if (isOutside(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", onPointerDown);
    document.addEventListener("focusin", onFocusIn);
    return () => {
      document.removeEventListener("mousedown", onPointerDown);
      document.removeEventListener("focusin", onFocusIn);
    };
  }, [open]);

  const openDropdown = () => {
    setQuery("");
    const idx = items.indexOf(value);
    activeNeedsScrollRef.current = true;
    setActive(idx >= 0 ? idx : 0);
    setOpen(true);
  };
  const commit = (id: string) => {
    if (id !== value) onCommit(id);
    setOpen(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      activeNeedsScrollRef.current = true;
      setActive((a) => Math.min(a + 1, filtered.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeNeedsScrollRef.current = true;
      setActive((a) => Math.max(a - 1, 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (filtered[active]) commit(filtered[active]);
    } else if (e.key === "Escape") {
      e.preventDefault();
      setOpen(false);
    }
  };

  return (
    <div className={styles.picker}>
      <button
        ref={triggerRef}
        type="button"
        className={styles.trigger}
        onClick={() => (open ? setOpen(false) : openDropdown())}
      >
        <span className={styles.triggerIcon}>{renderIcon(value, 20)}</span>
        <span className={styles.triggerLabel}>{label(value)}</span>
        <span className={styles.caret}>▾</span>
      </button>
      {open &&
        createPortal(
          <div
            ref={panelRef}
            className={styles.panel}
            style={{ position: "fixed", top: panelPos.top, left: panelPos.left }}
          >
            <input
              autoFocus={!isCoarsePointer}
              className={styles.search}
              placeholder="Search..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={onKeyDown}
            />
            <div className={styles.results}>
              {filtered.map((id, i) => (
                <div
                  key={id}
                  className={i === active ? styles.optionActive : styles.option}
                  onMouseDown={(e) => {
                    e.preventDefault();
                    commit(id);
                  }}
                  onMouseEnter={() => {
                    activeNeedsScrollRef.current = false;
                    setActive(i);
                  }}
                  ref={i === active ? activeElRef : undefined}
                >
                  <span className={styles.optionIcon}>{renderIcon(id, 28)}</span>
                  <span className={styles.optionLabel}>{label(id)}</span>
                </div>
              ))}
              {filtered.length === 0 && <div className={styles.empty}>No matches</div>}
            </div>
          </div>,
          document.body,
        )}
    </div>
  );
}
