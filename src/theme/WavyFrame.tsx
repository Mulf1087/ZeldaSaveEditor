import styles from "./WavyFrame.module.css";

// A hand-drawn-looking wavy rectangle outline, used as the border of the tab buttons instead of a
// straight box. Drawn in a normalized viewBox and stretched to the button with
// preserveAspectRatio="none"; the stroke stays crisp via vector-effect non-scaling-stroke, and the
// waves stretch a little per button width, which reads as organic rather than rigid.
//
// Deliberately minimal: a near-rectangle with just a light undulation (a couple of low-amplitude
// waves per edge) rather than a heavily scalloped border.
//
// The same wave is animated by morphing the path through a full phase cycle, which makes the ripple
// travel slowly around the border. The shape and amplitude are unchanged from the static version;
// only the phase advances, so it reads as the existing outline gently flowing.
const W = 100;
const H = 40;
const PAD = 4;
const AMP = 0.8;
const FRAMES = 12;
const DURATION = "9s";

function edge(
  n: number,
  waves: number,
  point: (p: number, offset: number) => [number, number],
  startAt: number,
  phase: number,
): [number, number][] {
  const pts: [number, number][] = [];
  for (let i = startAt; i <= n; i++) {
    const p = i / n;
    const offset = AMP * Math.sin(p * waves * 2 * Math.PI + phase);
    pts.push(point(p, offset));
  }
  return pts;
}

function wavyRectPath(phase: number): string {
  const iw = W - 2 * PAD;
  const ih = H - 2 * PAD;
  const pts: [number, number][] = [
    ...edge(16, 2, (p, o) => [PAD + p * iw, PAD + o], 0, phase), // top
    ...edge(6, 1, (p, o) => [W - PAD + o, PAD + p * ih], 1, phase), // right
    ...edge(16, 2, (p, o) => [W - PAD - p * iw, H - PAD + o], 1, phase), // bottom
    ...edge(6, 1, (p, o) => [PAD + o, H - PAD - p * ih], 1, phase), // left
  ];
  return "M" + pts.map(([x, y]) => `${x.toFixed(1)},${y.toFixed(1)}`).join(" L") + " Z";
}

// One path per phase step across a full 2*PI cycle; the last frame equals the first (phase 2*PI is
// the same wave as phase 0), so the loop is seamless.
const FRAME_PATHS = Array.from({ length: FRAMES + 1 }, (_, i) => wavyRectPath((i / FRAMES) * 2 * Math.PI));
const PATH = FRAME_PATHS[0];
const VALUES = FRAME_PATHS.join(";");

export default function WavyFrame() {
  return (
    <svg
      className={styles.frame}
      viewBox={`0 0 ${W} ${H}`}
      preserveAspectRatio="none"
      aria-hidden="true"
    >
      <path d={PATH} vectorEffect="non-scaling-stroke">
        <animate attributeName="d" values={VALUES} dur={DURATION} repeatCount="indefinite" calcMode="linear" />
      </path>
    </svg>
  );
}
