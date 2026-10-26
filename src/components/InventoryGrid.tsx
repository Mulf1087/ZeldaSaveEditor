import { useState, type ReactNode } from "react";
import styles from "./InventoryGrid.module.css";

export const INVENTORY_PAGE_SIZE = 20;

export interface InventorySlot {
  key: string | number;
  icon: ReactNode;
  name: string;
  meta?: ReactNode;
  badge?: ReactNode;
  quantityBadge?: boolean;
}

interface Props {
  title: string;
  slots: InventorySlot[];
  renderDetail: (index: number) => ReactNode;
}

export default function InventoryGrid({ title, slots, renderDetail }: Props) {
  const [requestedPage, setRequestedPage] = useState(0);
  const [requestedSelection, setRequestedSelection] = useState(0);
  const pageCount = Math.max(1, Math.ceil(slots.length / INVENTORY_PAGE_SIZE));
  const page = Math.min(requestedPage, pageCount - 1);
  const selected = Math.min(requestedSelection, slots.length - 1);
  const first = page * INVENTORY_PAGE_SIZE;
  const visible = slots.slice(first, first + INVENTORY_PAGE_SIZE);

  const goToPage = (next: number) => {
    setRequestedPage(next);
    setRequestedSelection(next * INVENTORY_PAGE_SIZE);
  };

  return (
    <div className={styles.inventory}>
      <div className={styles.toolbar}>
        <span className={styles.count}>共 {slots.length} 件 · 每页最多 {INVENTORY_PAGE_SIZE} 格</span>
        {pageCount > 1 && (
          <nav className={styles.pager} aria-label={`${title} 分页`}>
            <button type="button" onClick={() => goToPage(page - 1)} disabled={page === 0} aria-label="上一页">‹</button>
            <span aria-live="polite">{page + 1} / {pageCount}</span>
            <button type="button" onClick={() => goToPage(page + 1)} disabled={page === pageCount - 1} aria-label="下一页">›</button>
          </nav>
        )}
      </div>
      {slots.length === 0 ? (
        <p className={styles.empty}>这个分类还没有物品。可使用下方按钮添加。</p>
      ) : (
        <>
          <div className={styles.grid} aria-label={`${title} 物品格`}>
            {visible.map((slot, offset) => {
              const index = first + offset;
              return (
                <button
                  key={slot.key}
                  type="button"
                  className={styles.slot}
                  aria-label={`${slot.name}，第 ${index + 1} 格`}
                  aria-pressed={index === selected}
                  onClick={() => setRequestedSelection(index)}
                >
                  <span className={styles.slotNumber}>{String(index + 1).padStart(2, "0")}</span>
                  <span className={styles.icon}>{slot.icon}</span>
                  {slot.badge && <span className={styles.badge} data-quantity={slot.quantityBadge || undefined}>{slot.badge}</span>}
                  <span className={styles.name} title={slot.name}>{slot.name}</span>
                  {slot.meta && <span className={styles.meta} title={typeof slot.meta === "string" ? slot.meta : undefined}>{slot.meta}</span>}
                </button>
              );
            })}
          </div>
          {selected >= 0 && (
            <section className={styles.detail} aria-label={`${title} 物品属性`}>
              <div className={styles.detailHeading}>
                <span className={styles.detailEyebrow}>已选中 · {String(selected + 1).padStart(2, "0")}</span>
                <strong>{slots[selected].name}</strong>
                <span className={styles.detailHint}>在下方修改此物品的完整属性</span>
              </div>
              {renderDetail(selected)}
            </section>
          )}
        </>
      )}
    </div>
  );
}
