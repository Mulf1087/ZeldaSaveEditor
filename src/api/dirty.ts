import { useSyncExternalStore } from "react";

let dirty = false;
const listeners = new Set<() => void>();

export function markDirty(): void {
  if (dirty) return;
  dirty = true;
  listeners.forEach((l) => l());
}

export function clearDirty(): void {
  if (!dirty) return;
  dirty = false;
  listeners.forEach((l) => l());
}

export function isDirty(): boolean {
  return dirty;
}

export function subscribeDirty(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function useDirty(): boolean {
  return useSyncExternalStore(subscribeDirty, isDirty);
}

// Fired after every successful mutating command, so the app can refetch state and every
// component showing that state (dropdowns, checkboxes, ...) reflects the true backend value
// instead of the stale prop it rendered with before the edit. Plain inputs mask this with their
// own local typed-text state, but a controlled <select>/checkbox has nothing to fall back to,
// so without this it visually snaps back to the old value right after the user picks a new one.
const stateChangeListeners = new Set<() => void>();

export function notifyStateChanged(): void {
  stateChangeListeners.forEach((l) => l());
}

export function onStateChanged(listener: () => void): () => void {
  stateChangeListeners.add(listener);
  return () => stateChangeListeners.delete(listener);
}
