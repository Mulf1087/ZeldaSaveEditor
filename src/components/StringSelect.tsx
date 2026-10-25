import styles from "../theme/controls.module.css";

/** Like Select, but for string-valued fields (BOTW map/map-type). Keeps the current value visible
 * even if it isn't one of the known options, so an unrecognized value isn't silently replaced. */
export default function StringSelect({
  label,
  value,
  options,
  onCommit,
  onError,
}: {
  label: string;
  value: string;
  options: string[];
  onCommit: (value: string) => Promise<void>;
  onError: (message: string) => void;
}) {
  const withCurrent = options.includes(value) ? options : [value, ...options];
  return (
    <label className={styles.field}>
      {label}:{" "}
      <select
        className={styles.input}
        value={value}
        onChange={(e) => onCommit(e.target.value).catch((err) => onError(String(err)))}
      >
        {withCurrent.map((o) => (
          <option key={o} value={o}>
            {o}
          </option>
        ))}
      </select>
    </label>
  );
}
