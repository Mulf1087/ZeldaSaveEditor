import { useEffect, useState } from "react";
import type { TotkCaption } from "../../api";
import SectionHeading from "../../theme/SectionHeading";
import TotkMotif from "../../theme/motifs/TotkMotif";
import styles from "./CaptionView.module.css";

interface Props {
  info: TotkCaption;
}

export default function CaptionView({ info }: Props) {
  const [imageUrl, setImageUrl] = useState<string | null>(null);

  useEffect(() => {
    const blob = new Blob([new Uint8Array(info.thumbnail_jpeg)], { type: "image/jpeg" });
    const url = URL.createObjectURL(blob);
    setImageUrl(url);
    return () => URL.revokeObjectURL(url);
  }, [info.thumbnail_jpeg]);

  const capturedAt = new Date(info.year, info.month - 1, info.day, info.hour, info.minute);

  return (
    <div className={styles.view}>
      <SectionHeading title="Save Thumbnail" motif={<TotkMotif />} />
      {imageUrl && <img className={styles.thumbnail} src={imageUrl} alt="Save slot thumbnail" />}
      <p className={styles.meta}>
        {capturedAt.toLocaleString()}
        {info.is_autosave && <span className={styles.autosave}>Autosave</span>}
      </p>
    </div>
  );
}
