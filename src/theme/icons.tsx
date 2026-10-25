const COMMON = { fill: "none", stroke: "var(--accent)", strokeWidth: 1.4, strokeLinecap: "round" as const, strokeLinejoin: "round" as const };

// Hand-drawn line icons for the tabs that have no matching cell in the tabs.webp spritesheet.
// Tabs that do have a sheet cell use the tinted sprites from ./TabSprite instead.

export function PinIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M10 2 C6.5 2 4 4.7 4 8 C4 12 10 18 10 18 C10 18 16 12 16 8 C16 4.7 13.5 2 10 2 Z" {...COMMON} />
      <circle cx="10" cy="8" r="2" fill="var(--accent)" stroke="none" />
    </svg>
  );
}

export function SearchIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <circle cx="8.5" cy="8.5" r="5.5" {...COMMON} />
      <line x1="12.5" y1="12.5" x2="18" y2="18" {...COMMON} />
    </svg>
  );
}

export function ArrowIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <line x1="3" y1="17" x2="17" y2="3" {...COMMON} />
      <path d="M9 3 L17 3 L17 11" {...COMMON} />
    </svg>
  );
}

export function BlueprintIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <rect x="3" y="3" width="14" height="14" {...COMMON} />
      <line x1="3" y1="8" x2="17" y2="8" {...COMMON} />
      <line x1="8" y1="3" x2="8" y2="17" {...COMMON} />
      <circle cx="12.5" cy="12.5" r="1.1" fill="var(--accent)" stroke="none" />
    </svg>
  );
}

export function LeafIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M4 16 C4 8 10 3 17 3 C17 10 12 16 4 16 Z" {...COMMON} />
      <line x1="4" y1="16" x2="12" y2="8" {...COMMON} />
    </svg>
  );
}

export function ChestIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      <path d="M3 8 L3 16 L17 16 L17 8 Z" {...COMMON} />
      <path d="M3 8 C3 5 5 4 10 4 C15 4 17 5 17 8" {...COMMON} />
      <circle cx="10" cy="11" r="1.1" fill="var(--accent)" stroke="none" />
    </svg>
  );
}
