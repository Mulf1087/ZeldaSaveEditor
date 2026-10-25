import { useState, type ReactNode } from "react";
import styles from "./HoverPreview.module.css";

/** Wraps a small icon and shows a larger `preview` node near the cursor while hovered. Works for
 * both <img> icons (TOTK) and sprite-sheet div icons (BOTW). */
export default function HoverPreview({
  children,
  preview,
}: {
  children: ReactNode;
  preview: ReactNode;
}) {
  const [at, setAt] = useState<{ x: number; y: number } | null>(null);
  return (
    <span
      className={styles.trigger}
      onMouseEnter={(e) => setAt({ x: e.clientX, y: e.clientY })}
      onMouseMove={(e) => setAt({ x: e.clientX, y: e.clientY })}
      onMouseLeave={() => setAt(null)}
    >
      {children}
      {at && (
        <div className={styles.preview} style={{ left: at.x + 18, top: at.y + 18 }}>
          {preview}
        </div>
      )}
    </span>
  );
}
