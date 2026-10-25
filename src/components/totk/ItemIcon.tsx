import { type SyntheticEvent } from "react";
import { totkUnknownIconUrl } from "./totkIcons";
import HoverPreview from "../HoverPreview";
import styles from "./ItemIcon.module.css";

function fallback(e: SyntheticEvent<HTMLImageElement>) {
  e.currentTarget.onerror = null;
  e.currentTarget.src = totkUnknownIconUrl;
}

/** TOTK item thumbnail with a larger hover preview. */
export default function ItemIcon({ url, size = 48 }: { url: string; size?: number }) {
  return (
    <HoverPreview preview={<img className={styles.previewImg} src={url} alt="" onError={fallback} />}>
      <img
        className={styles.icon}
        style={{ width: size, height: size }}
        src={url}
        alt=""
        onError={fallback}
      />
    </HoverPreview>
  );
}
