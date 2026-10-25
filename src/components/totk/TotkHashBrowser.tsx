import { useEffect, useMemo, useState } from "react";
import type { TotkHashRow } from "../../api";
import * as api from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";
import styles from "./TotkHashBrowser.module.css";

interface Props {
  onError: (message: string) => void;
}

const MAX_RESULTS = 300;
const MIN_QUERY_LENGTH = 2;

export default function TotkHashBrowser({ onError }: Props) {
  const [rows, setRows] = useState<TotkHashRow[] | null>(null);
  const [query, setQuery] = useState("");

  useEffect(() => {
    api.getTotkHashRows().then(setRows).catch((err) => onError(String(err)));
  }, [onError]);

  const matches = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!rows || q.length < MIN_QUERY_LENGTH) return [];
    return rows
      .filter((r) => (r.name?.toLowerCase().includes(q) ?? false) || r.hash.toString(16).includes(q))
      .slice(0, MAX_RESULTS);
  }, [rows, query]);

  const commit = (row: TotkHashRow, text: string) => {
    const value = Number(text);
    if (Number.isNaN(value)) return;
    api.setTotkHashField(row.hash, value).catch((err) => onError(String(err)));
  };

  return (
    <div className={styles.view}>
      <SectionHeading title="Advanced: Hash Browser" motif={<TotkMotif />} />
      <p className={styles.warning}>
        Raw save-format access, for editing fields the app doesn't otherwise expose. Edits here
        skip the app's normal validation and can corrupt your save if the value is wrong.
      </p>
      <input
        className={styles.search}
        type="text"
        placeholder="Search by field name or hex hash..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      {query.trim().length >= MIN_QUERY_LENGTH && (
        <table className={styles.table}>
          <thead>
            <tr>
              <th>Hash</th>
              <th>Name</th>
              <th>Type</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            {matches.map((row) => (
              <tr key={row.hash}>
                <td>{row.hash.toString(16).padStart(8, "0")}</td>
                <td>{row.name ?? "(unknown)"}</td>
                <td>{row.type_name}</td>
                <td>
                  {row.editable ? (
                    row.type_name === "Bool" ? (
                      <input
                        type="checkbox"
                        defaultChecked={row.value !== 0}
                        onChange={(e) => commit(row, e.target.checked ? "1" : "0")}
                      />
                    ) : (
                      <input
                        className={styles.input}
                        type="number"
                        defaultValue={row.value}
                        onBlur={(e) => commit(row, e.target.value)}
                      />
                    )
                  ) : (
                    <span className={styles.readonly}>0x{row.value.toString(16)} (read-only)</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      {matches.length === MAX_RESULTS && (
        <p className={styles.truncated}>Showing the first {MAX_RESULTS} matches, refine your search for more.</p>
      )}
    </div>
  );
}
