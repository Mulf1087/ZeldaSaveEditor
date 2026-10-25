import type { TotkMapPin } from "../../api";
import * as api from "../../api";
import EditableEntryTable, { ColumnDef } from "./EditableEntryTable";
import TotkMotif from "../../theme/motifs/TotkMotif";

// icon/layer are raw hashes (MapPin.ICON_*/MAP_* in the source), same convention as every
// other Enum-typed field in this app: shown and edited as the hex the save format stores.
function hashToText(v: number): string {
  return `0x${(v >>> 0).toString(16).padStart(8, "0")}`;
}
function textToHash(raw: string): number {
  return Number.parseInt(raw, 16) >>> 0;
}

const columns: ColumnDef<TotkMapPin>[] = [
  {
    key: "icon",
    label: "Icon",
    format: (v) => hashToText(v as number),
    parse: (raw) => textToHash(raw) as TotkMapPin["icon"],
  },
  { key: "x", label: "X", type: "number", parse: (raw) => Number(raw) as TotkMapPin["x"] },
  { key: "y", label: "Y", type: "number", parse: (raw) => Number(raw) as TotkMapPin["y"] },
  {
    key: "layer",
    label: "Layer",
    format: (v) => hashToText(v as number),
    parse: (raw) => textToHash(raw) as TotkMapPin["layer"],
  },
];

interface Props {
  mapPins: TotkMapPin[];
  onError: (message: string) => void;
}

export default function TotkMapPinsTable({ mapPins, onError }: Props) {
  return (
    <EditableEntryTable
      title="Map Pins"
      entries={mapPins}
      columns={columns}
      setter={api.setMapPins}
      onError={onError}
      level="h3"
      motif={<TotkMotif />}
      nameFor={(e) => `(${Math.round(e.x)}, ${Math.round(e.y)})`}
    />
  );
}
