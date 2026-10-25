import { useEffect, useRef, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { OpenResult } from "./api";
import type { LocatedSave } from "./api/commands";
import * as api from "./api";
import {
  getRecentFiles,
  recordRecentFile,
  removeRecentFile,
  type RecentFile,
} from "./api/recentFiles";
import BotwView from "./components/botw/BotwView";
import TotkView from "./components/totk/TotkView";
import CaptionView from "./components/totk/CaptionView";
import ErrorBanner from "./components/ErrorBanner";
import CloseConfirmModal from "./components/CloseConfirmModal";
import Toast from "./components/Toast";
import { useThemeAttributes } from "./theme/useThemeAttributes";
import BotwMotif from "./theme/motifs/BotwMotif";
import TotkMotif from "./theme/motifs/TotkMotif";
import SectionWave from "./theme/SectionWave";
import styles from "./App.module.css";

const GAME_SUFFIX: Record<OpenResult["kind"], string> = {
  botw: " (BOTW)",
  totk: " (TOTK)",
  caption: " (TOTK)",
};

function App() {
  const [loaded, setLoaded] = useState<OpenResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [savedToast, setSavedToast] = useState<string | null>(null);
  const [confirmClose, setConfirmClose] = useState(false);
  const [pendingOpen, setPendingOpen] = useState<(() => Promise<void>) | null>(null);
  const [recents, setRecents] = useState<RecentFile[]>(getRecentFiles);
  const [localSaves, setLocalSaves] = useState<LocatedSave[]>([]);
  const [openedSource, setOpenedSource] = useState<"archive" | "file">("file");
  const dirty = api.useDirty();
  const editable = loaded?.kind === "botw" || loaded?.kind === "totk";
  useThemeAttributes(loaded?.kind === "caption" ? "totk" : loaded?.kind ?? "neutral");

  useEffect(() => {
    api.discoverLocalSaves().then(setLocalSaves).catch((err) => setError(String(err)));
  }, []);

  useEffect(() => {
    const suffix = loaded ? GAME_SUFFIX[loaded.kind] : "";
    getCurrentWindow().setTitle(`塞尔达存档编辑器${suffix}${dirty ? " *" : ""}`);
  }, [loaded, dirty]);

  useEffect(() => {
    const win = getCurrentWindow();
    const unlisten = win.onCloseRequested((event) => {
      if (api.isDirty()) {
        event.preventDefault();
        setConfirmClose(true);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const recordIfEditable = async (result: OpenResult) => {
    if (result.kind !== "botw" && result.kind !== "totk") return;
    const path = await api.currentPath();
    if (!path) return;
    // Android hands back a percent-encoded content:// URI rather than a plain path; decode that
    // case so a name like "game_data.sav" splits out instead of "primary%3ASaves%2Fgame_data.sav".
    // Desktop paths are left alone, since a literal "%" in a folder name isn't percent-encoding.
    let decoded = path;
    if (/^[a-z][a-z0-9+.-]*:\/\//i.test(path)) {
      try {
        decoded = decodeURIComponent(path);
      } catch {
        // malformed escape sequence; fall back to the raw path
      }
    }
    const name = decoded.split(/[\\/]/).pop() ?? decoded;
    setRecents(recordRecentFile({ path, kind: result.kind, name }));
  };

  const openNow = async () => {
    try {
      const result = await api.openSave();
      setLoaded(result);
      setOpenedSource("file");
      setError(null);
      await recordIfEditable(result);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const openRecentNow = async (file: RecentFile) => {
    try {
      const result = await api.openSaveAt(file.path);
      setLoaded(result);
      setOpenedSource("file");
      setError(null);
      await recordIfEditable(result);
    } catch (err) {
      // The file may have been moved or deleted; drop it from the list.
      setError((err as api.ShellError).message ?? String(err));
      setRecents(removeRecentFile(file.path));
    }
  };

  const openLocalNow = async (file: LocatedSave) => {
    try {
      const result = file.source === "archive" && file.entry
        ? await api.openArchiveSave(file.path, file.entry)
        : await api.openSaveAt(file.path);
      setLoaded(result);
      setOpenedSource(file.source === "archive" ? "archive" : "file");
      setError(null);
      await recordIfEditable(result);
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleOpenLocal = async (file: LocatedSave) => {
    if (api.isDirty()) {
      setPendingOpen(() => () => openLocalNow(file));
      return;
    }
    await openLocalNow(file);
  };

  // Opening a different file replaces whatever's currently loaded, so it needs the same
  // unsaved-changes guard as closing the window: without it, picking a new file silently
  // discards an in-progress edit with no warning.
  const handleOpen = async () => {
    if (api.isDirty()) {
      setPendingOpen(() => openNow);
      return;
    }
    await openNow();
  };

  const handleOpenRecent = async (file: RecentFile) => {
    if (api.isDirty()) {
      setPendingOpen(() => () => openRecentNow(file));
      return;
    }
    await openRecentNow(file);
  };

  const showLibrary = async () => {
    api.clearDirty();
    setLoaded(null);
    setError(null);
    setLocalSaves(await api.discoverLocalSaves());
  };

  const handleShowLibrary = async () => {
    if (api.isDirty()) {
      setPendingOpen(() => showLibrary);
      return;
    }
    try {
      await showLibrary();
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleSave = async () => {
    try {
      if (openedSource === "archive") {
        const output = await api.exportArchiveCopy();
        setSavedToast(`已导出新的 ZIP：${output}`);
      } else {
        await api.saveSave();
        setSavedToast("已保存，原文件旁已生成备份");
      }
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const handleSaveAs = async () => {
    try {
      await api.saveAs();
      setOpenedSource("file");
      setSavedToast("已另存为单独的 .sav 文件");
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const refreshCurrent = async () => {
    if (!loaded || loaded.kind === "caption") return;
    try {
      if (loaded.kind === "botw") {
        setLoaded({ kind: "botw", state: await api.getBotwState() });
      } else {
        setLoaded({ kind: "totk", state: await api.getTotkState() });
      }
    } catch (err) {
      setError((err as api.ShellError).message ?? String(err));
    }
  };

  const refreshCurrentRef = useRef(refreshCurrent);
  refreshCurrentRef.current = refreshCurrent;

  // Every mutating command (any edit, anywhere in the app) fires this after it succeeds, so the
  // UI always reflects the true backend state. Without it, a controlled input like a <select>
  // has nothing but the stale prop to render, so it visually snaps back right after the user
  // picks a new value.
  useEffect(() => {
    return api.onStateChanged(() => refreshCurrentRef.current());
  }, []);

  const closeForReal = () => {
    getCurrentWindow().destroy();
  };

  return (
    <div className={styles.app}>
      <h1 className={styles.title}>
        塞尔达存档编辑器
        {loaded && <span className={styles.gameSuffix}>{GAME_SUFFIX[loaded.kind]}</span>}
      </h1>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />
      <div className={styles.toolbar}>
        {loaded && <button className={styles.button} onClick={handleShowLibrary}>存档列表</button>}
        <button className={styles.button} onClick={handleOpen}>
          打开 .sav…
        </button>
        <button className={styles.button} onClick={handleSave} disabled={!editable}>
          {openedSource === "archive" ? "导出修改后的 ZIP" : "保存"}
        </button>
        <button className={styles.button} onClick={handleSaveAs} disabled={!editable}>
          另存为 .sav…
        </button>
        <button className={styles.button} onClick={refreshCurrent} disabled={!editable}>
          刷新
        </button>
      </div>
      {(loaded?.kind === "botw" || loaded?.kind === "totk") && (
        <p className={styles.versionLine}>
          {loaded.kind === "botw" ? "旷野之息" : "王国之泪"} · 存档格式版本{" "}
          {loaded.state.version}
          {loaded.state.modded ? "（修改版存档）" : ""}
          {openedSource === "archive" ? " · 保存时会生成新 ZIP，原压缩包保持不变" : ""}
        </p>
      )}
      {loaded?.kind === "botw" && (
        <BotwView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {loaded?.kind === "totk" && (
        <TotkView state={loaded.state} onError={setError} onRefresh={refreshCurrent} />
      )}
      {loaded?.kind === "caption" && <CaptionView info={loaded.info} />}
      {!loaded && (
        <div className={styles.hero}>
          <div className={styles.heroMotifs}>
            <BotwMotif />
            <TotkMotif />
          </div>
          <p className={styles.heroTagline}>旷野之息 · 王国之泪</p>
          <div className={styles.heroWave}>
            <SectionWave />
          </div>
          <button className={styles.heroButton} onClick={handleOpen}>
            打开其他 .sav 文件…
          </button>
          <p className={styles.heroHint}>编辑当前游戏存档前，请先完全退出游戏。</p>
          {localSaves.length > 0 && (
            <section className={styles.localLibrary}>
              <h2>这台 Mac 上找到的存档</h2>
              <p>直接打开当前存档，或从历史 ZIP 中选择一个栏位。</p>
              {localSaves.filter((file) => file.source === "live").map((file) => (
                <button key={file.path} className={styles.localItem} onClick={() => handleOpenLocal(file)}>
                  <span>{file.label}</span><small>当前存档</small>
                </button>
              ))}
              <details className={styles.archiveList}>
                <summary>历史压缩存档 · {localSaves.filter((file) => file.source === "archive").length} 个栏位</summary>
                {localSaves.filter((file) => file.source === "archive").map((file) => (
                  <button key={`${file.path}:${file.entry}`} className={styles.localItem} onClick={() => handleOpenLocal(file)}>
                    <span>{file.label}</span><small>ZIP 副本</small>
                  </button>
                ))}
              </details>
            </section>
          )}
          {recents.length > 0 && (
            <div className={styles.recents}>
              <p className={styles.recentsTitle}>最近打开</p>
              {recents.map((file) => (
                <button
                  key={file.path}
                  className={styles.recentItem}
                  onClick={() => handleOpenRecent(file)}
                  title={file.path}
                >
                  <span className={styles.recentName}>{file.name}</span>
                  <span className={styles.recentKind}>{file.kind.toUpperCase()}</span>
                  <span className={styles.recentPath}>{file.path}</span>
                </button>
              ))}
            </div>
          )}
        </div>
      )}
      {savedToast && <Toast message={savedToast} onDismiss={() => setSavedToast(null)} />}
      {pendingOpen && (
        <CloseConfirmModal
          message="当前修改尚未保存。返回或打开其他存档前，要先保存吗？"
          onSave={async () => {
            try {
              if (openedSource === "archive") await api.exportArchiveCopy();
              else await api.saveSave();
              const run = pendingOpen;
              setPendingOpen(null);
              await run();
            } catch (err) {
              setError((err as api.ShellError).message ?? String(err));
              setPendingOpen(null);
            }
          }}
          onDiscard={async () => {
            const run = pendingOpen;
            setPendingOpen(null);
            await run();
          }}
          onCancel={() => setPendingOpen(null)}
        />
      )}
      {confirmClose && (
        <CloseConfirmModal
          onSave={async () => {
            try {
              if (openedSource === "archive") await api.exportArchiveCopy();
              else await api.saveSave();
              closeForReal();
            } catch (err) {
              setError((err as api.ShellError).message ?? String(err));
              setConfirmClose(false);
            }
          }}
          onDiscard={closeForReal}
          onCancel={() => setConfirmClose(false)}
        />
      )}
    </div>
  );
}

export default App;
