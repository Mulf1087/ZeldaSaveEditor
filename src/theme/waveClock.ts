type Subscriber = (t: number) => void;

const subs = new Set<Subscriber>();
let rafId: number | null = null;
let t = 0;
let lastMs: number | null = null;

function frame(now: number) {
  if (lastMs !== null) {
    t += (now - lastMs) / 1000;
  }
  lastMs = now;
  for (const fn of subs) fn(t);
  if (subs.size > 0) {
    rafId = requestAnimationFrame(frame);
  } else {
    rafId = null;
    lastMs = null;
  }
}

export function subscribe(fn: Subscriber): () => void {
  if (window.matchMedia?.("(prefers-reduced-motion: reduce)").matches) {
    fn(0);
    return () => {};
  }

  subs.add(fn);
  if (rafId === null) {
    rafId = requestAnimationFrame(frame);
  }
  return () => {
    subs.delete(fn);
  };
}
