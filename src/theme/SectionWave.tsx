import { useEffect, useRef } from "react";
import { subscribe } from "./waveClock";

const WIDTH = 400;
const HEIGHT = 24;
const AMPLITUDE = 7;
const FREQUENCY = 6;
const SPEED = 0.03;
const STEPS = 120;

// Layered sines of different frequency/phase give a looser, hand-drawn ripple instead of a rigid
// triangle wave. Normalized so the combined amplitude stays within AMPLITUDE.
function organicWave(x: number): number {
  return 0.6 * Math.sin(x) + 0.28 * Math.sin(2.3 * x + 1) + 0.12 * Math.sin(4.1 * x + 2);
}

function buildPoints(t: number): string {
  const dtAngle = t * SPEED * 60;
  const parts: string[] = [];
  for (let i = 0; i <= STEPS; i++) {
    const progress = i / STEPS;
    const angle = progress * FREQUENCY * 2 * Math.PI + dtAngle;
    const y = (HEIGHT / 2 + AMPLITUDE * organicWave(angle)).toFixed(2);
    parts.push(`${(progress * WIDTH).toFixed(1)},${y}`);
  }
  return parts.join(" ");
}

export default function SectionWave() {
  const lineRef = useRef<SVGPolylineElement>(null);

  useEffect(() => {
    const el = lineRef.current;
    if (!el) return;
    el.setAttribute("points", buildPoints(0));
    return subscribe((t) => el.setAttribute("points", buildPoints(t)));
  }, []);

  return (
    <svg
      width="100%"
      height={HEIGHT}
      viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
      preserveAspectRatio="none"
      style={{ display: "block" }}
    >
      <polyline ref={lineRef} fill="none" stroke="var(--accent)" strokeWidth={1.5} strokeLinejoin="round" strokeLinecap="round" />
    </svg>
  );
}
