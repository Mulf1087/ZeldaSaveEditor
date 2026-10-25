import { useEffect, useState } from "react";
import * as api from "../../api";
import type { TotkAbility } from "../../api";
import { totkIconUrl } from "./totkIcons";
import styles from "./TotkKeyAbilities.module.css";

/**
 * The six unlockable key-item abilities (Ultrahand, Fuse, Ascend, Recall, Autobuild, Amiibo),
 * shown as icon toggles above the Key Items list. Each toggle flips the pair of lockstep flags
 * the game tracks per ability; state is fetched once on mount and updated optimistically on click.
 */
export default function TotkKeyAbilities({ onError }: { onError: (msg: string) => void }) {
  const [abilities, setAbilities] = useState<TotkAbility[]>([]);

  useEffect(() => {
    api.getTotkAbilities().then(setAbilities).catch((err) => {
      onError((err as api.ShellError).message ?? String(err));
    });
  }, [onError]);

  const toggle = async (ability: TotkAbility) => {
    const next = !ability.unlocked;
    try {
      await api.setTotkAbility(ability.id, next);
      setAbilities((prev) => prev.map((a) => (a.id === ability.id ? { ...a, unlocked: next } : a)));
    } catch (err) {
      onError((err as api.ShellError).message ?? String(err));
    }
  };

  if (abilities.length === 0) return null;

  return (
    <div className={styles.panel}>
      {abilities.map((ability) => (
        <button
          key={ability.id}
          type="button"
          className={`${styles.toggle} ${ability.unlocked ? styles.on : ""}`}
          onClick={() => toggle(ability)}
          aria-pressed={ability.unlocked}
          title={ability.unlocked ? `${ability.label} (unlocked)` : `${ability.label} (locked)`}
        >
          <img className={styles.icon} src={totkIconUrl("keyItem", `Obj_${ability.id}`)} alt="" />
          <span className={styles.label}>{ability.label}</span>
        </button>
      ))}
    </div>
  );
}
