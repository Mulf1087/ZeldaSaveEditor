import { useEffect } from "react";
import styles from "./Toast.module.css";

interface Props {
  message: string;
  onDismiss: () => void;
  durationMs?: number;
}

export default function Toast({ message, onDismiss, durationMs = 2500 }: Props) {
  useEffect(() => {
    const timer = setTimeout(onDismiss, durationMs);
    return () => clearTimeout(timer);
  }, [message, durationMs, onDismiss]);

  return <div className={styles.toast}>{message}</div>;
}
