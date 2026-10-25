import styles from "./ErrorBanner.module.css";

interface Props {
  message: string | null;
  onDismiss: () => void;
}

export default function ErrorBanner({ message, onDismiss }: Props) {
  if (!message) return null;
  return (
    <div className={styles.banner}>
      {message}{" "}
      <button className={styles.dismiss} onClick={onDismiss}>
        Dismiss
      </button>
    </div>
  );
}
