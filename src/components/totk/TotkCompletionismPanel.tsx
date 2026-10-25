import type { CompletismCategory } from "../../api";
import * as api from "../../api";
import TotkMotif from "../../theme/motifs/TotkMotif";
import CompletionismPanel from "../CompletionismPanel";

// These categories record completion by appending GUIDs to the save's discovery list, which grows
// the file rather than flipping an existing flag, so they get a heads-up before running.
const GUID_CONFIRM = new Set(["bubbuls", "sage_wills", "addison"]);
const GUID_MESSAGE =
  "This adds entries to your save's internal tracking data and grows the file slightly, a less common kind of edit than others in this tool. Continue?";

interface Props {
  state: { completism: CompletismCategory[] };
  onError: (message: string) => void;
}

export default function TotkCompletionismPanel({ state, onError }: Props) {
  return (
    <CompletionismPanel
      categories={state.completism}
      setAll={api.setTotkCompletism}
      motif={<TotkMotif />}
      onError={onError}
      confirmIds={GUID_CONFIRM}
      confirmMessage={GUID_MESSAGE}
    />
  );
}
