export default function TotkMotif() {
  return (
    <svg width="20" height="20" viewBox="0 0 20 20" aria-hidden="true" style={{ display: "block" }}>
      {/* A teardrop: pointed at the top, rounded below, echoing "Tears of the Kingdom" and
          pairing with the BOTW Sheikah-eye diamond (outline plus a filled center). */}
      <path
        d="M10 2 C10 2 15.5 9 15.5 12.5 A5.5 5.5 0 1 1 4.5 12.5 C4.5 9 10 2 10 2 Z"
        fill="none"
        stroke="var(--accent)"
        strokeWidth="1.4"
        strokeLinejoin="round"
      />
      <circle cx="10" cy="12.7" r="2.4" fill="var(--accent)" />
    </svg>
  );
}
