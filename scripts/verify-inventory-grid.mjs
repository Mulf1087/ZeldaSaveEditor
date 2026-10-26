import assert from "node:assert/strict";
import { chromium } from "playwright";
import { createServer } from "vite";

const botwWeapons = Array.from({ length: 23 }, (_, index) => ({
  name: "Weapon_Sword_001",
  quantity: 100 + index,
  category: "weapon",
}));
const totkWeapons = Array.from({ length: 23 }, (_, index) => ({
  id: "Weapon_Sword_001",
  durability: 100 + index,
  modifier: 0xb6eede09,
  modifier_value: 0,
  fuse_id: "",
  fuse_durability: 0,
  extra_durability: 0,
  record_extra_durability: 0,
}));

const fixtures = {
  botw: { kind: "botw", state: {
    version: "test", modded: false,
    player_position: [0, 0, 0], horse_position: [0, 0, 0],
    map: "MainField", map_type: "Normal", horses: [],
    items: [...botwWeapons, { name: "Item_Fruit_A", quantity: 42, category: "material" }],
    weapon_modifiers: botwWeapons.map(() => ({ modifier: 0, value: 0 })),
  } },
  totk: { kind: "totk", state: {
    version: "test", modded: false,
    save_pos: [0, 0, 0],
    pouch_weapons: totkWeapons,
    materials: [{ id: "Item_Fruit_A", quantity: 42, get_order: 0, use_order: 0 }],
  } },
};

const server = await createServer({ server: { host: "127.0.0.1", port: 1421, strictPort: true } });
await server.listen();
const browser = await chromium.launch({ channel: "chrome" });

try {
  for (const [game, fixture] of Object.entries(fixtures)) {
    const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
    await page.addInitScript((mock) => {
      window.__inventoryCalls = [];
      window.__inventoryState = structuredClone(mock);
      window.__TAURI_INTERNALS__ = {
        invoke: async (command, args) => {
          const current = window.__inventoryState;
          if (command === "open_save") return current;
          if (command === "get_botw_state" || command === "get_totk_state") return current.state;
          if (command === "discover_local_saves") return [];
          if (command === "current_path") return null;
          if (command === "set_item") current.state.items[args.index] = { ...current.state.items[args.index], name: args.name, quantity: args.quantity };
          if (command === "set_pouch_weapons") current.state.pouch_weapons = args.entries;
          if (command === "remove_item") current.state.items.splice(args.index, 1);
          window.__inventoryCalls.push({ command, args });
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
    }, fixture);
    await page.goto("http://127.0.0.1:1421");
    await page.getByRole("button", { name: "打开 .sav…" }).click();
    await page.getByRole("tab", { name: "武器" }).click();
    const grid = page.getByLabel("武器 物品格");
    assert.match(await grid.innerText(), /旅人之剑/, `${game}: item name should be Chinese`);
    assert.equal(await grid.getByRole("button").count(), 20, `${game}: first page should have 20 slots`);
    await page.getByRole("button", { name: "下一页" }).click();
    assert.equal(await grid.getByRole("button").count(), 3, `${game}: second page should have 3 slots`);
    assert.match(await page.locator("body").innerText(), /2 \/ 2/);
    await grid.getByRole("button").last().click();
    assert.equal(await grid.getByRole("button", { pressed: true }).count(), 1);
    const detail = page.getByLabel("武器 物品属性");
    const durability = detail.locator('[data-label="耐久度"] input');
    assert.equal(await durability.inputValue(), "122");
    await durability.fill("77");
    await durability.blur();
    const calls = await page.evaluate(() => window.__inventoryCalls);
    if (game === "botw") {
      assert(calls.some((call) => call.command === "set_item" && call.args.index === 22 && call.args.quantity === 77));
    } else {
      assert(calls.some((call) => call.command === "set_pouch_weapons" && call.args.entries.length === 23 && call.args.entries[22].durability === 77));
    }
    await page.screenshot({ path: `/tmp/zelda-${game}-inventory-grid.png`, fullPage: true });
    await page.setViewportSize({ width: 390, height: 844 });
    assert.equal(await durability.isVisible(), true, `${game}: selected detail stays editable at narrow width`);
    await page.screenshot({ path: `/tmp/zelda-${game}-inventory-grid-mobile.png`, fullPage: true });
    await page.getByRole("tab", { name: "材料" }).click();
    const materialSlot = page.getByLabel("材料 物品格").getByRole("button").first();
    const quantityBadge = materialSlot.locator('[data-quantity="true"]');
    assert.equal(await quantityBadge.textContent(), "×42");
    const slotRect = await materialSlot.boundingBox();
    const badgeRect = await quantityBadge.boundingBox();
    assert(slotRect && badgeRect);
    assert(badgeRect.y - slotRect.y < 15, `${game}: quantity should be in the top corner`);
    assert(slotRect.x + slotRect.width - badgeRect.x - badgeRect.width < 15, `${game}: quantity should be in the right corner`);
    await page.close();
    console.log(`${game}: pagination, selection, editing, and quantity corner passed`);
  }
} finally {
  await browser.close();
  await server.close();
}
