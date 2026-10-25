import { chromium } from "playwright";

const game = process.argv[2] ?? "neutral"; // botw | totk | neutral
const outPath = process.argv[3] ?? `scratch-${game}.png`;

const mocks = {
  botw: {
    kind: "botw",
    state: {
      rupees: 500,
      mons: 3,
      max_hearts: 30,
      max_stamina: 15,
      relic_gerudo: 1,
      relic_goron: 1,
      relic_rito: 1,
      korok_seed_counter: 100,
      defeated_hinox_counter: 5,
      defeated_talus_counter: 2,
      defeated_molduga_counter: 1,
      playtime_seconds: 3600,
      motorcycle: true,
      player_position: [0, 0, 0],
      horse_position: [0, 0, 0],
      map: "MainField",
      map_type: "Normal",
      items: [{ name: "Weapon_Sword_001", quantity: 1 }],
      weapon_modifiers: [{ modifier: 1, value: 10 }],
      bow_modifiers: [{ modifier: 1, value: 10 }],
      shield_modifiers: [{ modifier: 1, value: 10 }],
      horses: [{ name: "Epona", saddle: "Saddle_01", reins: "Rein_01", horse_type: "HorseNormal1" }],
    },
  },
  totk: {
    kind: "totk",
    state: {
      max_life: 480,
      current_rupees: 999,
      max_stamina: 15,
      max_energy: 1000,
      playtime: 7200,
      horse_inn_member_point: 3,
      save_pos: [0, 0, 0],
      sequence_current_banc: "Banc_001",
      pouch_weapon_valid_num: 8,
      pouch_bow_valid_num: 5,
      pouch_shield_valid_num: 4,
      pouch_weapons: [
        {
          id: "Weapon_Sword_001",
          durability: 100,
          modifier: 0,
          modifier_value: 0,
          fuse_id: "",
          fuse_durability: 0,
          extra_durability: 0,
          record_extra_durability: 0,
        },
      ],
      pouch_bows: [{ id: "Bow_001", durability: 100, modifier: 0, modifier_value: 0 }],
      pouch_shields: [
        {
          id: "Shield_001",
          durability: 100,
          modifier: 0,
          modifier_value: 0,
          fuse_id: "",
          fuse_durability: 0,
          extra_durability: 0,
        },
      ],
      armor: [{ id: "Armor_001", dye_color: 0 }],
      arrows: [{ id: "Arrow_001", quantity: 10 }],
      materials: [{ id: "Material_001", quantity: 5, get_order: 1, use_order: 1 }],
      key_items: [{ id: "KeyItem_001", quantity: 1 }],
      devices: [{ id: "Device_001", quantity: 1, use_order: 1 }],
      food: [
        {
          id: "Food_001",
          quantity: 1,
          hearts_heal: 4,
          effect: 0,
          effect_multiplier: 0,
          effect_time: 0,
          price: 10,
          recipe: ["", "", "", "", ""],
        },
      ],
      horses: [
        {
          id: "Horse_001",
          name: "Storm",
          mane: 0,
          saddle: 0,
          rein: 0,
          bond: 5,
          bond_checked: true,
          stats_strength: 3,
          stats_speed: 3,
          stats_stamina: 3,
          stats_pull: 3,
          horse_type: 0,
          color_type: 0,
          foot_type: 0,
          amiibo_uid_hash: 0,
          room_id: 0,
          icon_pattern: 0,
          icon_eye_color: 0,
          icon_primary_color: [0, 0, 0],
          icon_secondary_color: [0, 0, 0],
          icon_nose_color: [0, 0, 0],
          icon_hair_primary_color: [0, 0, 0],
          icon_hair_secondary_color: [0, 0, 0],
        },
      ],
      shrines_found: 10,
      shrines_cleared: 5,
      koroks_hidden: 100,
      koroks_carried: 2,
      locations_visited: 50,
      defeated_hinox: 3,
      defeated_talus: 2,
      defeated_molduga: 1,
      defeated_bubbuls: 42,
      sage_wills_found: 7,
      old_maps_found: 20,
      addison_completed: 15,
      autobuilds: [
        {
          index: 0,
          combined_actor_info: new Array(6688).fill(0),
          camera_pos: [0, 0, 0],
          camera_at: [0, 0, 0],
          is_favorite: true,
        },
      ],
      map_pins: [{ icon: 0x51b0bed0, x: 100, y: 200, layer: 0x24950135 }],
      map_markers: [{ color: 0x7e3d1e46, x: 0, y: 0, z: 0 }],
      teleporters: [{ index: 0, pos: [0, 0, 0], rot: [0, 0, 0] }],
    },
  },
};

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 1600 } });

await page.addInitScript((mock) => {
  window.__TAURI_INTERNALS__ = {
    invoke: async (cmd) => {
      if (cmd === "open_save") return mock;
      if (cmd === "get_botw_state" || cmd === "get_totk_state") return mock?.state ?? null;
      return null;
    },
    transformCallback: (callback) => {
      const id = Math.floor(Math.random() * 1e9);
      window[`_${id}`] = callback;
      return id;
    },
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { windowLabel: "main", label: "main" },
    },
  };
  window.__TAURI_EVENT_PLUGIN_INTERNALS__ = { unregisterListener: () => {} };
}, mocks[game] ?? null);

await page.goto("http://localhost:1420");

if (game !== "neutral") {
  await page.getByText("Open...").click();
  await page.waitForTimeout(300);
}

await page.screenshot({ path: outPath, fullPage: true });
await browser.close();
console.log(`saved ${outPath}`);
