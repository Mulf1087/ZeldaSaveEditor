import { useEffect } from "react";

export type Game = "botw" | "totk" | "neutral";

export function useThemeAttributes(game: Game): void {
  useEffect(() => {
    document.documentElement.dataset.game = game;
  }, [game]);
}
