import type { TotkHorse } from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import TotkMotif from "../../theme/motifs/TotkMotif";
import { TOTK_ITEM_NAMES } from "./totkItemNames.data";
import {
  MANE_OPTIONS,
  SADDLE_OPTIONS,
  REIN_OPTIONS,
  STAT_STAR_OPTIONS,
  STAMINA_STAR_OPTIONS,
  HORSE_AVAILABILITY,
} from "./totkHorseEnums.data";

function num(key: keyof TotkHorse, label: string): ColumnDef<TotkHorse> {
  return { key, label, type: "number", parse: (raw) => Number(raw) as TotkHorse[keyof TotkHorse] };
}

// Appearance-only fields (icon_pattern, icon_eye_color, the five RGB tuples,
// amiibo_uid_hash) are left out of this table. They're cosmetic icon rendering data, not
// stats a save editor user would typically want to hand-edit. Add columns for them if that
// changes. horse_type/color_type/foot_type stay plain number inputs, matching marcrobledo (which
// derives horse_type from the id and exposes color/foot as ranged ints, not enums).
const columns: ColumnDef<TotkHorse>[] = [
  {
    key: "id",
    label: "物品 ID",
    type: "itempicker",
    // Horses have no per-id icon (marcrobledo draws them procedurally from color data), so the
    // picker shows just name + id.
    picker: { items: HORSE_AVAILABILITY, renderIcon: () => null, nameFor: (id) => TOTK_ITEM_NAMES[id] },
  },
  { key: "name", label: "名称", type: "text" },
  { key: "mane", label: "鬃毛", type: "select", options: MANE_OPTIONS },
  { key: "saddle", label: "马鞍", type: "select", options: SADDLE_OPTIONS },
  { key: "rein", label: "缰绳", type: "select", options: REIN_OPTIONS },
  num("bond", "亲密度 (0-100)"),
  { key: "bond_checked", label: "已确认亲密度", type: "checkbox" },
  num("stats_strength", "力量 (100-350)"),
  { key: "stats_speed", label: "速度", type: "select", options: STAT_STAR_OPTIONS },
  { key: "stats_stamina", label: "精力", type: "select", options: STAMINA_STAR_OPTIONS },
  { key: "stats_pull", label: "牵引力", type: "select", options: STAT_STAR_OPTIONS },
  num("horse_type", "类型"),
  num("color_type", "颜色"),
  num("foot_type", "蹄型"),
  num("room_id", "房间 ID"),
];

interface Props {
  horses: TotkHorse[];
  onError: (message: string) => void;
}

export default function TotkHorsesTable({ horses, onError }: Props) {
  return (
    <EditableEntryTable
      title="马匹"
      entries={horses}
      columns={columns}
      setter={api.setHorses}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
      // Collapsed cards would otherwise show only the row number; surface the horse's given name
      // (falling back to its breed) so a collapsed horse is identifiable at a glance.
      nameFor={(h) => {
        const breed = TOTK_ITEM_NAMES[h.id];
        if (h.name && breed) return `${h.name} (${breed})`;
        return h.name || breed || h.id;
      }}
    />
  );
}
