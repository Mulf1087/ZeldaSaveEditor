import type { ReactNode } from "react";
import type { CompletismCategory, CompletismMetric } from "../api";
import SectionHeading from "../theme/SectionHeading";
import styles from "./CompletionismPanel.module.css";

interface Props {
  categories: CompletismCategory[];
  /** Runs the game's mass-set command for one category metric. */
  setAll: (id: string, metric: number) => Promise<number>;
  motif: ReactNode;
  onError: (message: string) => void;
  /** Category ids that grow the save file when set, so they prompt for confirmation first. */
  confirmIds?: Set<string>;
  confirmMessage?: string;
}

function capitalize(word: string): string {
  return word.charAt(0).toUpperCase() + word.slice(1);
}

function MetricRow({
  category,
  metric,
  index,
  showLabel,
  setAll,
  onError,
  confirmIds,
  confirmMessage,
}: {
  category: CompletismCategory;
  metric: CompletismMetric;
  index: number;
  showLabel: boolean;
  setAll: (id: string, metric: number) => Promise<number>;
  onError: (message: string) => void;
  confirmIds?: Set<string>;
  confirmMessage?: string;
}) {
  const complete = metric.value >= metric.max;
  const percent = metric.max === 0 ? 100 : Math.round((metric.value / metric.max) * 100);
  const apply = () => {
    if (confirmIds?.has(category.id) && confirmMessage && !window.confirm(confirmMessage)) return;
    setAll(category.id, index).catch((err) => onError(String(err)));
  };
  return (
    <div className={styles.metric}>
      {showLabel && <div className={styles.metricLabel}>{capitalize(metric.verb)}:</div>}
      <div className={styles.metricValue}>
        {metric.value}
        <small>/{metric.max}</small>
      </div>
      <div className={styles.track}>
        <div className={styles.fill} data-complete={complete} style={{ width: `${percent}%` }} />
      </div>
      {!complete && (
        <button type="button" className={styles.setButton} onClick={apply}>
          Set all as {metric.verb}
        </button>
      )}
    </div>
  );
}

export default function CompletionismPanel({
  categories,
  setAll,
  motif,
  onError,
  confirmIds,
  confirmMessage,
}: Props) {
  return (
    <div>
      <SectionHeading title="Completionism" motif={motif} />
      <div className={styles.grid}>
        {categories.map((category) => (
          <div key={category.id} className={styles.card}>
            <div className={styles.cardTitle}>{category.label}</div>
            {category.metrics.map((metric, i) => (
              <MetricRow
                key={metric.verb}
                category={category}
                metric={metric}
                index={i}
                showLabel={category.metrics.length > 1}
                setAll={setAll}
                onError={onError}
                confirmIds={confirmIds}
                confirmMessage={confirmMessage}
              />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
