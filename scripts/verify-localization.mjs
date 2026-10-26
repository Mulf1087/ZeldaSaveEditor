import assert from "node:assert/strict";
import { BOTW_ITEM_NAMES } from "../src/components/botw/botwItemNames.data.ts";
import { BOTW_CATEGORY_ITEMS } from "../src/components/botw/botwCategoryItems.data.ts";
import { TOTK_ITEM_NAMES } from "../src/components/totk/totkItemNames.data.ts";
import { TOTK_CATEGORY_ITEMS } from "../src/components/totk/totkCategoryItems.data.ts";

for (const [game, names, categories] of [
  ["旷野之息", BOTW_ITEM_NAMES, BOTW_CATEGORY_ITEMS],
  ["王国之泪", TOTK_ITEM_NAMES, TOTK_CATEGORY_ITEMS],
]) {
  for (const [id, name] of Object.entries(names)) {
    assert.match(name, /[\u3400-\u9fff]/, `${game} ${id} 缺少中文名称`);
    assert.doesNotMatch(name, /undefined|\bBug\b|\bDLC\b/, `${game} ${id} 译名未完成`);
  }
  for (const [category, ids] of Object.entries(categories)) {
    for (const id of ids) {
      assert.ok(names[id], `${game} ${category} ${id} 缺少译名`);
    }
  }
  console.log(`${game}：${Object.keys(names).length} 个中文名称，所有分类选项均有译名`);
}

assert.equal(BOTW_ITEM_NAMES.Weapon_Sword_070, "大师之剑");
assert.equal(TOTK_ITEM_NAMES.Weapon_Sword_070, "大师之剑");
