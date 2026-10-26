import type { BotwItem, BotwState } from "../../api";
import BotwStatsForm from "./BotwStatsForm";
import { BotwCategoryTable } from "./BotwItemsTable";
import { BOTW_CATEGORY_ITEMS } from "./botwCategoryItems.data";
import BotwHorsesTable from "./BotwHorsesTable";
import BotwCompletionismPanel from "./BotwCompletionismPanel";
import TabShell from "../../theme/TabShell";
import { LeafIcon, ChestIcon } from "../../theme/icons";
import {
  HomeSprite,
  SwordSprite,
  BowSprite,
  ShieldSprite,
  ShirtSprite,
  AppleSprite,
  StarSprite,
  HorseSprite,
} from "../../theme/TabSprite";
import styles from "./BotwView.module.css";

// BOTW files store bows and arrows in one "bow" category; arrow ids are the ammo entries that don't
// use the "Weapon_" prefix, so this predicate cleanly splits the two into their own sections.
const isBotwArrow = (item: BotwItem) => !item.name.startsWith("Weapon_");
const BOW_PICKER = BOTW_CATEGORY_ITEMS.bow.filter((id) => id.startsWith("Weapon_"));
const ARROW_PICKER = BOTW_CATEGORY_ITEMS.bow.filter((id) => !id.startsWith("Weapon_"));

interface Props {
  state: BotwState;
  onError: (message: string) => void;
  onRefresh: () => void;
}

export default function BotwView({ state, onError, onRefresh }: Props) {
  return (
    <div className={styles.view}>
      <TabShell
        tabs={[
          {
            id: "stats",
            label: "角色",
            icon: <HomeSprite />,
            content: <BotwStatsForm state={state} onError={onError} />,
          },
          {
            id: "weapons",
            label: "武器",
            icon: <SwordSprite />,
            content: (
              <BotwCategoryTable
                title="武器"
                valueLabel="耐久度"
                items={state.items}
                category="weapon"
                modifiers={state.weapon_modifiers}
                onError={onError}
              />
            ),
          },
          {
            id: "bows",
            label: "弓箭",
            icon: <BowSprite />,
            content: (
              <>
                <BotwCategoryTable
                  title="弓"
                  valueLabel="耐久度"
                  items={state.items}
                  category="bow"
                  subFilter={(item) => !isBotwArrow(item)}
                  pickerItems={BOW_PICKER}
                  modifiers={state.bow_modifiers}
                  onError={onError}
                />
                <BotwCategoryTable
                  title="箭矢"
                  valueLabel="数量"
                  items={state.items}
                  category="bow"
                  subFilter={isBotwArrow}
                  pickerItems={ARROW_PICKER}
                  onError={onError}
                />
              </>
            ),
          },
          {
            id: "shields",
            label: "盾牌",
            icon: <ShieldSprite />,
            content: (
              <BotwCategoryTable
                title="盾牌"
                valueLabel="耐久度"
                items={state.items}
                category="shield"
                modifiers={state.shield_modifiers}
                onError={onError}
              />
            ),
          },
          {
            id: "armor",
            label: "防具",
            icon: <ShirtSprite />,
            content: (
              <BotwCategoryTable
                title="防具"
                valueLabel="染色"
                items={state.items}
                category="armor"
                onError={onError}
              />
            ),
          },
          {
            id: "materials",
            label: "材料",
            icon: <LeafIcon />,
            content: (
              <BotwCategoryTable
                title="材料"
                valueLabel="数量"
                items={state.items}
                category="material"
                onError={onError}
              />
            ),
          },
          {
            id: "food",
            label: "料理",
            icon: <AppleSprite />,
            content: (
              <BotwCategoryTable title="料理" valueLabel="数量" items={state.items} category="food" onError={onError} />
            ),
          },
          {
            id: "key_items",
            label: "重要物品",
            icon: <ChestIcon />,
            content: (
              <BotwCategoryTable
                title="重要物品"
                valueLabel="数量"
                items={state.items}
                category="key_item"
                onError={onError}
              />
            ),
          },
          {
            id: "horses",
            label: "马匹",
            icon: <HorseSprite />,
            content: <BotwHorsesTable horses={state.horses} onError={onError} />,
          },
          {
            id: "completionism",
            label: "完成度",
            icon: <StarSprite />,
            content: <BotwCompletionismPanel state={state} onError={onError} />,
          },
        ]}
      />
    </div>
  );
}
