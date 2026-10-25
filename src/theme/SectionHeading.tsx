import type { ReactNode } from "react";
import SectionWave from "./SectionWave";
import styles from "./SectionHeading.module.css";

interface Props {
  title: string;
  motif?: ReactNode;
  level?: "h3" | "h4";
}

export default function SectionHeading({ title, motif, level = "h3" }: Props) {
  const Heading = level;
  return (
    <div className={styles.wrap}>
      <div className={styles.row}>
        {motif && <span className={styles.motif}>{motif}</span>}
        <Heading className={styles.title}>{title}</Heading>
      </div>
      <SectionWave />
    </div>
  );
}
