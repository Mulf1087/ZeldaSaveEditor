import styles from "./CloseConfirmModal.module.css";

interface Props {
  onSave: () => void;
  onDiscard: () => void;
  onCancel: () => void;
  message?: string;
}

export default function CloseConfirmModal({
  onSave,
  onDiscard,
  onCancel,
  message = "当前修改尚未保存。关闭前要先保存吗？",
}: Props) {
  return (
    <div className={styles.overlay}>
      <div className={styles.panel}>
        <h2 className={styles.title}>未保存的修改</h2>
        <p className={styles.message}>{message}</p>
        <div className={styles.actions}>
          <button className={styles.button} onClick={onSave}>
            保存
          </button>
          <button className={styles.button} onClick={onDiscard}>
            放弃修改
          </button>
          <button className={styles.button} onClick={onCancel}>
            取消
          </button>
        </div>
      </div>
    </div>
  );
}
