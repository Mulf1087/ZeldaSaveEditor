import { useState } from "react";
import type { BotwHorse } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import BotwMotif from "../../theme/motifs/BotwMotif";
import {
  HORSE_TYPE_OPTIONS,
  HORSE_TYPE_UNTAMED_OPTIONS,
  HORSE_SADDLE_OPTIONS,
  HORSE_REIN_OPTIONS,
  type StringOption,
} from "./botwHorseEnums.data";
import styles from "./BotwHorsesTable.module.css";

function withCurrentValue(options: StringOption[], value: string): StringOption[] {
  if (options.some((o) => o.value === value)) return options;
  return [{ value, label: `Custom (${value})` }, ...options];
}

function HorseSelect({
  value,
  options,
  onCommit,
  onError,
}: {
  value: string;
  options: StringOption[];
  onCommit: (val: string) => Promise<void>;
  onError: (message: string) => void;
}) {
  return (
    <select
      className={styles.input}
      value={value}
      onChange={(e) => onCommit(e.target.value).catch((err) => onError(String(err)))}
    >
      {withCurrentValue(options, value).map((o) => (
        <option key={o.value} value={o.value}>
          {o.label}
        </option>
      ))}
    </select>
  );
}

interface Props {
  horses: BotwHorse[];
  onError: (message: string) => void;
}

function HorseRow({
  index,
  horse,
  onError,
}: {
  index: number;
  horse: BotwHorse;
  onError: (message: string) => void;
}) {
  const [name, setName] = useState(horse.name ?? "");
  const hasNamedSlots = horse.name !== null;
  const typeOptions = index === 5 ? HORSE_TYPE_UNTAMED_OPTIONS : HORSE_TYPE_OPTIONS;

  return (
    <tr>
      <td>{index}</td>
      <td>
        <HorseSelect
          value={horse.horse_type}
          options={typeOptions}
          onCommit={(val) => api.setHorseType(index, val)}
          onError={onError}
        />
      </td>
      <td>
        {hasNamedSlots && (
          <input
            className={styles.input}
            value={name}
            onChange={(e) => setName(e.target.value)}
            onBlur={() => api.setHorseName(index, name).catch((err) => onError(String(err)))}
          />
        )}
      </td>
      <td>
        {hasNamedSlots && horse.saddle !== null && (
          <HorseSelect
            value={horse.saddle}
            options={HORSE_SADDLE_OPTIONS}
            onCommit={(val) => api.setHorseSaddle(index, val)}
            onError={onError}
          />
        )}
      </td>
      <td>
        {hasNamedSlots && horse.reins !== null && (
          <HorseSelect
            value={horse.reins}
            options={HORSE_REIN_OPTIONS}
            onCommit={(val) => api.setHorseReins(index, val)}
            onError={onError}
          />
        )}
      </td>
    </tr>
  );
}

export default function BotwHorsesTable({ horses, onError }: Props) {
  return (
    <div>
      <SectionHeading title="Horses" motif={<BotwMotif />} />
      <table className={styles.table}>
        <thead>
          <tr>
            <th>#</th>
            <th>Type</th>
            <th>Name</th>
            <th>Saddle</th>
            <th>Reins</th>
          </tr>
        </thead>
        <tbody>
          {horses.map((horse, i) => (
            <HorseRow key={i} index={i} horse={horse} onError={onError} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
