// Recently-opened save files, persisted in localStorage so the landing screen can offer quick
// reopening. Path-keyed, most-recent-first, capped at MAX.

export interface RecentFile {
  path: string;
  kind: "botw" | "totk";
  name: string;
}

const KEY = "recentFiles";
const MAX = 4;

export function getRecentFiles(): RecentFile[] {
  try {
    const raw = localStorage.getItem(KEY);
    const list = raw ? (JSON.parse(raw) as RecentFile[]) : [];
    return Array.isArray(list) ? list : [];
  } catch {
    return [];
  }
}

export function recordRecentFile(file: RecentFile): RecentFile[] {
  const list = [file, ...getRecentFiles().filter((f) => f.path !== file.path)].slice(0, MAX);
  localStorage.setItem(KEY, JSON.stringify(list));
  return list;
}

export function removeRecentFile(path: string): RecentFile[] {
  const list = getRecentFiles().filter((f) => f.path !== path);
  localStorage.setItem(KEY, JSON.stringify(list));
  return list;
}
