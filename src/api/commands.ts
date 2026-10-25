import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { clearDirty, markDirty, notifyStateChanged } from "./dirty";
import type {
  BotwState,
  ModifierCategory,
  OpenResult,
  TotkArmor,
  TotkArrow,
  TotkAutoBuildEntry,
  TotkBow,
  TotkDevice,
  TotkFood,
  TotkHorse,
  TotkKeyItem,
  TotkMapMarker,
  TotkMapPin,
  TotkMaterial,
  TotkHashRow,
  TotkAbility,
  TotkShield,
  TotkState,
  TotkTeleporter,
  TotkWeapon,
} from "./types";

const READONLY_COMMANDS = new Set([
  "get_botw_state",
  "get_totk_state",
  "get_totk_hash_rows",
  "get_totk_abilities",
  "current_path",
  "discover_local_saves",
]);
const OPEN_COMMANDS = new Set(["open_save", "open_save_at", "open_archive_save"]);
const CLEARS_DIRTY_COMMANDS = new Set(["save", "save_as", "export_archive_copy"]);

function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return tauriInvoke<T>(cmd, args).then((result) => {
    if (CLEARS_DIRTY_COMMANDS.has(cmd)) {
      clearDirty();
    } else if (OPEN_COMMANDS.has(cmd)) {
      // caption.sav is a read-only preview: the backend leaves any already-loaded, possibly
      // still-dirty save untouched when the opened file turns out to be one, so only clear
      // dirty here when the open actually replaced the editable save.
      const kind = (result as { kind?: string } | undefined)?.kind;
      if (kind !== "caption") clearDirty();
    } else if (!READONLY_COMMANDS.has(cmd)) {
      markDirty();
      notifyStateChanged();
    }
    return result;
  });
}

export function openSave(): Promise<OpenResult> {
  return invoke("open_save");
}

export function openSaveAt(path: string): Promise<OpenResult> {
  return invoke("open_save_at", { path });
}

export interface LocatedSave {
  source: "live" | "archive";
  game: "botw" | "totk";
  label: string;
  path: string;
  entry: string | null;
}

export function discoverLocalSaves(): Promise<LocatedSave[]> {
  return invoke("discover_local_saves");
}

export function openArchiveSave(path: string, entry: string): Promise<OpenResult> {
  return invoke("open_archive_save", { path, entry });
}

export function exportArchiveCopy(): Promise<string> {
  return invoke("export_archive_copy");
}

export function currentPath(): Promise<string | null> {
  return invoke("current_path");
}

export function saveSave(): Promise<void> {
  return invoke("save");
}

export function saveAs(): Promise<void> {
  return invoke("save_as");
}

// --- BOTW ---

export function getBotwState(): Promise<BotwState> {
  return invoke("get_botw_state");
}

export function setRupees(val: number): Promise<void> {
  return invoke("set_rupees", { val });
}

export function setMons(val: number): Promise<void> {
  return invoke("set_mons", { val });
}

export function setMaxHearts(val: number): Promise<void> {
  return invoke("set_max_hearts", { val });
}

export function setMaxStaminaBotw(val: number): Promise<void> {
  return invoke("set_max_stamina", { val });
}

export function setRelicGerudo(val: number): Promise<void> {
  return invoke("set_relic_gerudo", { val });
}

export function setRelicGoron(val: number): Promise<void> {
  return invoke("set_relic_goron", { val });
}

export function setRelicRito(val: number): Promise<void> {
  return invoke("set_relic_rito", { val });
}

export function setKorokSeedCounter(val: number): Promise<void> {
  return invoke("set_korok_seed_counter", { val });
}

export function setDefeatedHinoxCounter(val: number): Promise<void> {
  return invoke("set_defeated_hinox_counter", { val });
}

export function setDefeatedTalusCounter(val: number): Promise<void> {
  return invoke("set_defeated_talus_counter", { val });
}

export function setDefeatedMoldugaCounter(val: number): Promise<void> {
  return invoke("set_defeated_molduga_counter", { val });
}

export function setPlaytimeSeconds(val: number): Promise<void> {
  return invoke("set_playtime_seconds", { val });
}

export function setMotorcycle(val: boolean): Promise<void> {
  return invoke("set_motorcycle", { val });
}

export function setPlayerPosition(x: number, y: number, z: number): Promise<void> {
  return invoke("set_player_position", { x, y, z });
}

export function setHorsePosition(x: number, y: number, z: number): Promise<void> {
  return invoke("set_horse_position", { x, y, z });
}

export function setMap(value: string): Promise<void> {
  return invoke("set_map", { value });
}

export function setMapType(value: string): Promise<void> {
  return invoke("set_map_type", { value });
}

export function setItem(index: number, name: string, quantity: number): Promise<void> {
  return invoke("set_item", { index, name, quantity });
}

export function removeItem(index: number): Promise<void> {
  return invoke("remove_item", { index });
}

export function duplicateItem(index: number): Promise<void> {
  return invoke("duplicate_item", { index });
}

export function setModifier(
  category: ModifierCategory,
  index: number,
  modifier: number,
  value: number,
): Promise<void> {
  return invoke("set_modifier", { category, index, modifier, value });
}

export function setHorseName(index: number, value: string): Promise<void> {
  return invoke("set_horse_name", { index, value });
}

export function setHorseSaddle(index: number, value: string): Promise<void> {
  return invoke("set_horse_saddle", { index, value });
}

export function setHorseReins(index: number, value: string): Promise<void> {
  return invoke("set_horse_reins", { index, value });
}

export function setHorseType(index: number, value: string): Promise<void> {
  return invoke("set_horse_type", { index, value });
}

export function unlockAllKoroks(): Promise<number> {
  return invoke("unlock_all_koroks");
}

export function unlockAllDefeatedHinox(): Promise<number> {
  return invoke("unlock_all_defeated_hinox");
}

export function unlockAllDefeatedTalus(): Promise<number> {
  return invoke("unlock_all_defeated_talus");
}

export function unlockAllDefeatedMolduga(): Promise<number> {
  return invoke("unlock_all_defeated_molduga");
}

export function unlockAllLocations(): Promise<number> {
  return invoke("unlock_all_locations");
}

/** Sets one BOTW completionism category to complete. Returns how many entries changed. */
export function setBotwCompletism(id: string, metric: number): Promise<number> {
  return invoke("set_botw_completism", { id, metric });
}

// --- TOTK ---

export function getTotkState(): Promise<TotkState> {
  return invoke("get_totk_state");
}

export function setMaxLife(val: number): Promise<void> {
  return invoke("set_max_life", { val });
}

export function setCurrentRupees(val: number): Promise<void> {
  return invoke("set_current_rupees", { val });
}

export function setMaxStaminaTotk(val: number): Promise<void> {
  return invoke("totk_set_max_stamina", { val });
}

export function setMaxEnergy(val: number): Promise<void> {
  return invoke("set_max_energy", { val });
}

export function setPlaytime(val: number): Promise<void> {
  return invoke("set_playtime", { val });
}

export function setHorseInnMemberPoint(val: number): Promise<void> {
  return invoke("set_horse_inn_member_point", { val });
}

export function setSavePos(x: number, y: number, z: number): Promise<void> {
  return invoke("set_save_pos", { x, y, z });
}

export function setSequenceCurrentBanc(value: string): Promise<void> {
  return invoke("set_sequence_current_banc", { value });
}

export function setPouchWeaponValidNum(val: number): Promise<void> {
  return invoke("set_pouch_weapon_valid_num", { val });
}

export function setPouchBowValidNum(val: number): Promise<void> {
  return invoke("set_pouch_bow_valid_num", { val });
}

export function setPouchShieldValidNum(val: number): Promise<void> {
  return invoke("set_pouch_shield_valid_num", { val });
}

export function setPouchWeapons(entries: TotkWeapon[]): Promise<void> {
  return invoke("set_pouch_weapons", { entries });
}

export function setPouchBows(entries: TotkBow[]): Promise<void> {
  return invoke("set_pouch_bows", { entries });
}

export function setPouchShields(entries: TotkShield[]): Promise<void> {
  return invoke("set_pouch_shields", { entries });
}

export function setArmor(entries: TotkArmor[]): Promise<void> {
  return invoke("set_armor", { entries });
}

export function setArrows(entries: TotkArrow[]): Promise<void> {
  return invoke("set_arrows", { entries });
}

export function setMaterials(entries: TotkMaterial[]): Promise<void> {
  return invoke("set_materials", { entries });
}

export function setKeyItems(entries: TotkKeyItem[]): Promise<void> {
  return invoke("set_key_items", { entries });
}

export function setDevices(entries: TotkDevice[]): Promise<void> {
  return invoke("set_devices", { entries });
}

export function setFood(entries: TotkFood[]): Promise<void> {
  return invoke("set_food", { entries });
}

export function setHorses(entries: TotkHorse[]): Promise<void> {
  return invoke("set_horses", { entries });
}

export function setAutobuilds(entries: TotkAutoBuildEntry[]): Promise<void> {
  return invoke("set_autobuilds", { entries });
}

export function setMapPins(entries: TotkMapPin[]): Promise<void> {
  return invoke("set_map_pins", { entries });
}

export function setMapMarkers(entries: TotkMapMarker[]): Promise<void> {
  return invoke("set_map_markers", { entries });
}

export function setTeleporters(entries: TotkTeleporter[]): Promise<void> {
  return invoke("set_teleporters", { entries });
}

export function unlockAllBubbuls(): Promise<number> {
  return invoke("unlock_all_bubbuls");
}

export function unlockAllSageWills(): Promise<number> {
  return invoke("unlock_all_sage_wills");
}

export function unlockAllAddison(): Promise<number> {
  return invoke("unlock_all_addison");
}

/** Sets one TOTK completionism metric to complete. Returns how many entries changed. */
export function setTotkCompletism(id: string, metric: number): Promise<number> {
  return invoke("set_totk_completism", { id, metric });
}

export function getTotkHashRows(): Promise<TotkHashRow[]> {
  return invoke("get_totk_hash_rows");
}

export function setTotkHashField(hash: number, value: number): Promise<void> {
  return invoke("set_totk_hash_field", { hash, value });
}

export function getTotkAbilities(): Promise<TotkAbility[]> {
  return invoke("get_totk_abilities");
}

export function setTotkAbility(id: string, on: boolean): Promise<void> {
  return invoke("set_totk_ability", { id, on });
}
