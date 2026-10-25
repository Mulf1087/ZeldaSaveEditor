import styles from "../theme/controls.module.css";

export interface SelectOption {
  value: number;
  label: string;
}

/** If `value` isn't one of `options` (a modded/edge-case save, or a value outside the
 * enumerated range like a completionist save's `max_life`), inject a synthetic option holding
 * it, so the select shows the true current value instead of silently snapping to the first
 * option and risking overwriting it with the wrong value on the next change. */
export function withCurrentValue(options: SelectOption[], value: number): SelectOption[] {
  if (options.some((o) => o.value === value)) return options;
  return [{ value, label: `Custom (${value})` }, ...options];
}

interface Props {
  label: string;
  value: number;
  options: SelectOption[];
  onCommit: (value: number) => Promise<void>;
  onError: (message: string) => void;
}

export default function Select({ label, value, options, onCommit, onError }: Props) {
  return (
    <label className={styles.field}>
      {label}:{" "}
      <select
        className={styles.input}
        value={String(value)}
        onChange={(e) => onCommit(Number(e.target.value)).catch((err) => onError(String(err)))}
      >
        {withCurrentValue(options, value).map((o) => (
          <option key={o.value} value={o.value}>
            {o.label}
          </option>
        ))}
      </select>
    </label>
  );
}
