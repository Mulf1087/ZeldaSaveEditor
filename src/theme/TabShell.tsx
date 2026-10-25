import { useState, type ReactNode } from "react";
import WavyFrame from "./WavyFrame";
import styles from "./TabShell.module.css";

export interface TabDef {
  id: string;
  label: string;
  icon: ReactNode;
  content: ReactNode;
}

interface Props {
  tabs: TabDef[];
}

export default function TabShell({ tabs }: Props) {
  const [activeId, setActiveId] = useState(tabs[0]?.id);
  const activeTab = tabs.find((t) => t.id === activeId) ?? tabs[0];

  return (
    <div>
      <div className={styles.tabBar} role="tablist">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            role="tab"
            aria-selected={tab.id === activeTab?.id}
            className={`${styles.tab} ${tab.id === activeTab?.id ? styles.active : ""}`}
            onClick={() => setActiveId(tab.id)}
          >
            <WavyFrame />
            <span className={styles.label}>
              {tab.icon}
              {tab.label}
            </span>
          </button>
        ))}
      </div>
      <div className={styles.content}>{activeTab?.content}</div>
    </div>
  );
}
