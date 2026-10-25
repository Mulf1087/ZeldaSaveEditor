// Tab icons cropped from marcrobledo's assets/tabs.png (12 cells, 128px each). The sheet art is a
// flat dark silhouette, so it's used as a CSS mask and filled with the current theme accent, which
// recolors it per game (BOTW blue, TOTK gold) instead of a fixed color.
const CELLS = 12;
const SHEET = "/icons/tabs.webp";

export function TabSprite({ index, size = 20 }: { index: number; size?: number }) {
  const mask = {
    maskImage: `url(${SHEET})`,
    WebkitMaskImage: `url(${SHEET})`,
    maskSize: `${size * CELLS}px ${size}px`,
    WebkitMaskSize: `${size * CELLS}px ${size}px`,
    maskPosition: `${-index * size}px 0`,
    WebkitMaskPosition: `${-index * size}px 0`,
    maskRepeat: "no-repeat",
    WebkitMaskRepeat: "no-repeat",
  } as const;
  return (
    <span
      aria-hidden="true"
      style={{
        display: "inline-block",
        width: size,
        height: size,
        backgroundColor: "var(--accent)",
        flexShrink: 0,
        ...mask,
      }}
    />
  );
}

// Named cells, in sheet order.
export const HomeSprite = () => <TabSprite index={0} />;
export const SwordSprite = () => <TabSprite index={1} />;
export const BowSprite = () => <TabSprite index={2} />;
export const ShieldSprite = () => <TabSprite index={3} />;
export const ShirtSprite = () => <TabSprite index={4} />;
export const AppleSprite = () => <TabSprite index={5} />;
export const PotSprite = () => <TabSprite index={6} />;
export const StarSprite = () => <TabSprite index={7} />;
export const HorseSprite = () => <TabSprite index={8} />;
export const TriforceSprite = () => <TabSprite index={9} />;
export const DomeSprite = () => <TabSprite index={10} />;
export const GearSprite = () => <TabSprite index={11} />;
