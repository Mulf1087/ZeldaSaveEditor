import type { TotkAutoBuildEntry } from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import TotkMotif from "../../theme/motifs/TotkMotif";

function vec3ToText(v: [number, number, number]): string {
  return v.join(",");
}

function textToVec3(raw: string): [number, number, number] {
  const parts = raw.split(",").map((s) => Number(s.trim()));
  return [parts[0] ?? 0, parts[1] ?? 0, parts[2] ?? 0];
}

// combined_actor_info is a 6688-byte binary blob per slot, not something to hand-edit
// as a table cell. Shown read-only as a byte count instead.
const columns: ColumnDef<TotkAutoBuildEntry>[] = [
  { key: "index", label: "序号", type: "number", parse: (raw) => Number(raw) as TotkAutoBuildEntry["index"] },
  {
    key: "camera_pos",
    label: "相机位置 (x,y,z)",
    format: (v) => vec3ToText(v as [number, number, number]),
    parse: (raw) => textToVec3(raw) as TotkAutoBuildEntry["camera_pos"],
  },
  {
    key: "camera_at",
    label: "相机目标 (x,y,z)",
    format: (v) => vec3ToText(v as [number, number, number]),
    parse: (raw) => textToVec3(raw) as TotkAutoBuildEntry["camera_at"],
  },
  { key: "is_favorite", label: "收藏", type: "checkbox" },
  {
    key: "combined_actor_info",
    label: "数据大小",
    type: "readonly",
    format: (v) => `${(v as number[]).length} bytes`,
  },
];

interface Props {
  autobuilds: TotkAutoBuildEntry[];
  onError: (message: string) => void;
}

export default function TotkAutoBuildTable({ autobuilds, onError }: Props) {
  return (
    <EditableEntryTable
      title="蓝图草稿"
      entries={autobuilds}
      columns={columns}
      setter={api.setAutobuilds}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
      // No natural name for a draft; surface its favourite state so a collapsed card isn't just "#".
      nameFor={(e) => (e.is_favorite ? "★ 收藏的蓝图" : "蓝图草稿")}
    />
  );
}
