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
const TYPE_LABELS: Record<string, string> = {
  Bool: "布尔值", Int: "整数", UInt: "无符号整数", Enum: "枚举值",
  Float: "浮点数", String: "文本", Unknown: "未知类型",
};

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
      <SectionHeading title="高级：哈希字段浏览器" motif={<TotkMotif />} />
      <p className={styles.warning}>
        可直接编辑软件未单独列出的存档字段。这里的修改不经过常规校验，输入错误可能损坏存档。
      </p>
      <input
        className={styles.search}
        type="text"
        placeholder="按字段名称或十六进制哈希搜索…"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      {query.trim().length >= MIN_QUERY_LENGTH && (
        <table className={styles.table}>
          <thead>
            <tr>
              <th>哈希值</th>
              <th>名称</th>
              <th>类型</th>
              <th>数值</th>
            </tr>
          </thead>
          <tbody>
            {matches.map((row) => (
              <tr key={row.hash}>
                <td>{row.hash.toString(16).padStart(8, "0")}</td>
                <td>{row.name ?? "（未知）"}</td>
                <td>{TYPE_LABELS[row.type_name] ?? row.type_name}</td>
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
                    <span className={styles.readonly}>0x{row.value.toString(16)} （只读）</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      {matches.length === MAX_RESULTS && (
        <p className={styles.truncated}>仅显示前 {MAX_RESULTS} 条结果，请缩小搜索范围。</p>
      )}
    </div>
  );
}
