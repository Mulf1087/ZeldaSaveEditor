import type { CompletismCategory } from "../../api";
import * as api from "../../api";
import BotwMotif from "../../theme/motifs/BotwMotif";
import CompletionismPanel from "../CompletionismPanel";

interface Props {
  state: { completism: CompletismCategory[] };
  onError: (message: string) => void;
}

export default function BotwCompletionismPanel({ state, onError }: Props) {
  return (
    <CompletionismPanel
      categories={state.completism}
      setAll={api.setBotwCompletism}
      motif={<BotwMotif />}
      onError={onError}
    />
  );
}
