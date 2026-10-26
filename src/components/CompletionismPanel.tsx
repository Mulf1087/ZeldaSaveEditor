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

const VERB_LABELS: Record<string, string> = { found: "已发现", activated: "已激活", cleared: "已通关", carried: "已护送", defeated: "已击败", visited: "已到访", completed: "已完成", unlocked: "已解锁" };
const CATEGORY_LABELS: Record<string, string> = { towers: "鸟望台", shrines: "神庙", lightroots: "破魔之根", koroks: "克洛格", strayed_koroks: "迷路的克洛格", bubbuls: "魔犹伊", locations: "地点", caves: "洞窟", wells: "井", chasms: "地底入口", hinox: "西诺克斯", talus: "岩石巨人", molduga: "莫尔德拉吉克", flux: "方块魔像", frox: "巨霸迦马", gleeok: "古栗欧克", sage_wills: "贤者的遗志", old_maps: "旧地图", addison: "卡邦达告示牌", schema_stones: "设计图石板", yiga_schematics: "依盖队设计图", compendium: "海拉鲁图鉴" };
function verbLabel(verb: string): string { return VERB_LABELS[verb] ?? verb; }

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
      {showLabel && <div className={styles.metricLabel}>{verbLabel(metric.verb)}：</div>}
      <div className={styles.metricValue}>
        {metric.value}
        <small>/{metric.max}</small>
      </div>
      <div className={styles.track}>
        <div className={styles.fill} data-complete={complete} style={{ width: `${percent}%` }} />
      </div>
      {!complete && (
        <button type="button" className={styles.setButton} onClick={apply}>
          全部设为{verbLabel(metric.verb)}
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
      <SectionHeading title="完成度" motif={motif} />
      <div className={styles.grid}>
        {categories.map((category) => (
          <div key={category.id} className={styles.card}>
            <div className={styles.cardTitle}>{CATEGORY_LABELS[category.id] ?? category.label}</div>
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
