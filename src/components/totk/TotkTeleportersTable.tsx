import type { TotkTeleporter } from "../../api";
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

const columns: ColumnDef<TotkTeleporter>[] = [
  { key: "index", label: "Index", type: "number", parse: (raw) => Number(raw) as TotkTeleporter["index"] },
  {
    key: "pos",
    label: "Position (x,y,z)",
    format: (v) => vec3ToText(v as [number, number, number]),
    parse: (raw) => textToVec3(raw) as TotkTeleporter["pos"],
  },
  {
    key: "rot",
    label: "Rotation (x,y,z)",
    format: (v) => vec3ToText(v as [number, number, number]),
    parse: (raw) => textToVec3(raw) as TotkTeleporter["rot"],
  },
];

interface Props {
  teleporters: TotkTeleporter[];
  onError: (message: string) => void;
}

export default function TotkTeleportersTable({ teleporters, onError }: Props) {
  return (
    <EditableEntryTable
      title="Teleporters"
      entries={teleporters}
      columns={columns}
      setter={api.setTeleporters}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
      nameFor={(e) => `(${e.pos.map(Math.round).join(", ")})`}
    />
  );
}
