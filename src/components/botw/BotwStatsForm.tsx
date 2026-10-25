import { useState } from "react";
import type { BotwState } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import Select from "../Select";
import StringSelect from "../StringSelect";
import { HEARTS_OPTIONS, STAMINA_OPTIONS } from "./botwEnums.data";
import { MAP_OPTIONS, MAP_TYPE_OPTIONS } from "./botwMapOptions.data";
import styles from "./BotwStatsForm.module.css";

interface Props {
  state: BotwState;
  onError: (message: string) => void;
}

function NumberField({
  label,
  value,
  onCommit,
}: {
  label: string;
  value: number;
  onCommit: (val: number) => Promise<void>;
}) {
  const [text, setText] = useState(String(value));
  return (
    <label className={styles.field}>
      {label}:{" "}
      <input
        className={styles.input}
        type="number"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={() => {
          onCommit(Number(text));
        }}
      />
    </label>
  );
}

export default function BotwStatsForm({ state, onError }: Props) {
  const commit = (fn: (val: number) => Promise<void>) => async (val: number) => {
    try {
      await fn(val);
    } catch (err) {
      onError((err as api.ShellError).message ?? String(err));
    }
  };

  return (
    <div>
      <SectionHeading title="角色数据" motif={<BotwMotif />} />
      <NumberField label="卢比" value={state.rupees} onCommit={commit(api.setRupees)} />
      <NumberField label="莫力（Amiibo）" value={state.mons} onCommit={commit(api.setMons)} />
      <Select
        label="最大心数"
        value={state.max_hearts}
        options={HEARTS_OPTIONS}
        onCommit={commit(api.setMaxHearts)}
        onError={onError}
      />
      <Select
        label="最大精力"
        value={state.max_stamina}
        options={STAMINA_OPTIONS}
        onCommit={commit(api.setMaxStaminaBotw)}
        onError={onError}
      />
      <NumberField
        label="格鲁德神兽技能"
        value={state.relic_gerudo}
        onCommit={commit(api.setRelicGerudo)}
      />
      <NumberField
        label="鼓隆神兽技能"
        value={state.relic_goron}
        onCommit={commit(api.setRelicGoron)}
      />
      <NumberField label="利特神兽技能" value={state.relic_rito} onCommit={commit(api.setRelicRito)} />
      <NumberField
        label="克洛格种子计数"
        value={state.korok_seed_counter}
        onCommit={commit(api.setKorokSeedCounter)}
      />
      <NumberField
        label="已击败西诺克斯"
        value={state.defeated_hinox_counter}
        onCommit={commit(api.setDefeatedHinoxCounter)}
      />
      <NumberField
        label="已击败岩石巨人"
        value={state.defeated_talus_counter}
        onCommit={commit(api.setDefeatedTalusCounter)}
      />
      <NumberField
        label="已击败莫尔德拉吉克"
        value={state.defeated_molduga_counter}
        onCommit={commit(api.setDefeatedMoldugaCounter)}
      />
      <NumberField
        label="游玩时间（秒）"
        value={state.playtime_seconds}
        onCommit={commit(api.setPlaytimeSeconds)}
      />
      {state.motorcycle !== null && (
        <label className={styles.field}>
          已解锁大师摩托：{" "}
          <input
            type="checkbox"
            checked={state.motorcycle}
            onChange={(e) =>
              api.setMotorcycle(e.target.checked).catch((err) => onError(String(err)))
            }
          />
        </label>
      )}
      <SectionHeading level="h4" title="地图" />
      <StringSelect label="地图" value={state.map} options={MAP_OPTIONS} onCommit={api.setMap} onError={onError} />
      <StringSelect
        label="地图类型"
        value={state.map_type}
        options={MAP_TYPE_OPTIONS}
        onCommit={api.setMapType}
        onError={onError}
      />
      <PositionField
        label="林克坐标"
        value={state.player_position}
        onCommit={commit3(api.setPlayerPosition, onError)}
      />
      <PositionField
        label="马匹坐标"
        value={state.horse_position}
        onCommit={commit3(api.setHorsePosition, onError)}
      />
    </div>
  );
}

function commit3(
  fn: (x: number, y: number, z: number) => Promise<void>,
  onError: (message: string) => void,
) {
  return async (x: number, y: number, z: number) => {
    try {
      await fn(x, y, z);
    } catch (err) {
      onError(String(err));
    }
  };
}

function PositionField({
  label,
  value,
  onCommit,
}: {
  label: string;
  value: [number, number, number];
  onCommit: (x: number, y: number, z: number) => Promise<void>;
}) {
  const [xs, setXs] = useState(String(value[0]));
  const [ys, setYs] = useState(String(value[1]));
  const [zs, setZs] = useState(String(value[2]));
  const commit = () => onCommit(Number(xs), Number(ys), Number(zs));
  return (
    <div className={styles.field}>
      {label}:{" "}
      <input className={styles.input} type="number" value={xs} onChange={(e) => setXs(e.target.value)} onBlur={commit} />{" "}
      <input className={styles.input} type="number" value={ys} onChange={(e) => setYs(e.target.value)} onBlur={commit} />{" "}
      <input className={styles.input} type="number" value={zs} onChange={(e) => setZs(e.target.value)} onBlur={commit} />
    </div>
  );
}
