import type { TotkMapMarker } from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import TotkMotif from "../../theme/motifs/TotkMotif";

// color is a raw hash (MapPinData.Type in the source), same convention as map pin icon/layer.
function hashToText(v: number): string {
  return `0x${(v >>> 0).toString(16).padStart(8, "0")}`;
}
function textToHash(raw: string): number {
  return Number.parseInt(raw, 16) >>> 0;
}

const columns: ColumnDef<TotkMapMarker>[] = [
  {
    key: "color",
    label: "Color",
    format: (v) => hashToText(v as number),
    parse: (raw) => textToHash(raw) as TotkMapMarker["color"],
  },
  { key: "x", label: "X", type: "number", parse: (raw) => Number(raw) as TotkMapMarker["x"] },
  { key: "y", label: "Y", type: "number", parse: (raw) => Number(raw) as TotkMapMarker["y"] },
  { key: "z", label: "Z", type: "number", parse: (raw) => Number(raw) as TotkMapMarker["z"] },
];

interface Props {
  mapMarkers: TotkMapMarker[];
  onError: (message: string) => void;
}

export default function TotkMapMarkersTable({ mapMarkers, onError }: Props) {
  return (
    <EditableEntryTable
      title="Map Markers"
      entries={mapMarkers}
      columns={columns}
      setter={api.setMapMarkers}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
      nameFor={(e) => `(${Math.round(e.x)}, ${Math.round(e.y)}, ${Math.round(e.z)})`}
    />
  );
}
