// BOTW map / map-type option lists, ported from marcrobledo's zelda-botw.js `select('pos-map', ...)`
// and `select('pos-maptype', ...)`. Both fields are raw strings in the save.

export const MAP_TYPE_OPTIONS: string[] = ["?", "MainField", "MainFieldDungeon"];

function buildMapOptions(): string[] {
  const maps = ["?"];
  // Surface grid A-1 .. J-8.
  for (let i = 0; i < 10; i++) {
    for (let j = 0; j < 8; j++) {
      maps.push(`${String.fromCharCode(65 + i)}-${j + 1}`);
    }
  }
  // Shrines Dungeon000 .. Dungeon119 (zero-padded to three digits).
  for (let i = 0; i < 120; i++) {
    maps.push(`Dungeon${String(i).padStart(3, "0")}`);
  }
  maps.push("RemainsElectric", "RemainsFire", "RemainsWater", "RemainsWind");
  return maps;
}

export const MAP_OPTIONS: string[] = buildMapOptions();
