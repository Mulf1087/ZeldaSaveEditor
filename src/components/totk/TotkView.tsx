import { useState } from "react";
import type { TotkState } from "../../api";
import * as api from "../../api";
import {
  TotkWeaponsTable,
  TotkBowsTable,
  TotkShieldsTable,
  TotkArmorTable,
  TotkArrowsTable,
  TotkMaterialsTable,
  TotkKeyItemsTable,
  TotkDevicesTable,
  TotkFoodTable,
} from "./TotkPouchTables";
import TotkKeyAbilities from "./TotkKeyAbilities";
import TotkHorsesTable from "./TotkHorsesTable";
import TotkAutoBuildTable from "./TotkAutoBuildTable";
import TotkMapPinsTable from "./TotkMapPinsTable";
import TotkMapMarkersTable from "./TotkMapMarkersTable";
import TotkTeleportersTable from "./TotkTeleportersTable";
import TotkCompletionismPanel from "./TotkCompletionismPanel";
import TotkHashBrowser from "./TotkHashBrowser";
import SectionHeading from "../../theme/SectionHeading";
import TabShell from "../../theme/TabShell";
import Select from "../Select";
import { HEARTS_OPTIONS, STAMINA_OPTIONS, BATTERY_OPTIONS } from "./totkEnums.data";
import { LeafIcon, ChestIcon, BlueprintIcon, PinIcon, SearchIcon } from "../../theme/icons";
import {
  HomeSprite,
  SwordSprite,
  BowSprite,
  ShieldSprite,
  ShirtSprite,
  GearSprite,
  AppleSprite,
  HorseSprite,
  StarSprite,
} from "../../theme/TabSprite";
import styles from "./TotkView.module.css";

interface Props {
  state: TotkState;
  onError: (message: string) => void;
  onRefresh: () => void;
}

function NumberField({
  label,
  value,
  onCommit,
  onError,
}: {
  label: string;
  value: number;
  onCommit: (val: number) => Promise<void>;
  onError: (message: string) => void;
}) {
  const [text, setText] = useState(String(value));
  return (
    <label className={styles.field}>
      {label}:{" "}
      <input
        className={styles.input}
        type="number"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onBlur={() => onCommit(Number(text)).catch((err) => onError(String(err)))}
      />
    </label>
  );
}

export default function TotkView({ state, onError, onRefresh }: Props) {
  const [posX, setPosX] = useState(String(state.save_pos[0]));
  const [posY, setPosY] = useState(String(state.save_pos[1]));
  const [posZ, setPosZ] = useState(String(state.save_pos[2]));
  const commitPos = () =>
    api
      .setSavePos(Number(posX), Number(posY), Number(posZ))
      .catch((err) => onError(String(err)));

  const statsTab = (
    <div>
      <Select label="最大心数" value={state.max_life} options={HEARTS_OPTIONS} onCommit={api.setMaxLife} onError={onError} />
      <NumberField
        label="当前卢比"
        value={state.current_rupees}
        onCommit={api.setCurrentRupees}
        onError={onError}
      />
      <Select
        label="最大精力"
        value={state.max_stamina}
        options={STAMINA_OPTIONS}
        onCommit={api.setMaxStaminaTotk}
        onError={onError}
      />
      <Select
        label="电池容量"
        value={state.max_energy}
        options={BATTERY_OPTIONS}
        onCommit={api.setMaxEnergy}
        onError={onError}
      />
      <NumberField label="游玩时间" value={state.playtime} onCommit={api.setPlaytime} onError={onError} />
      <NumberField
        label="驿站会员点数"
        value={state.horse_inn_member_point}
        onCommit={api.setHorseInnMemberPoint}
        onError={onError}
      />

      <SectionHeading level="h4" title="林克坐标" />
      <div className={styles.row}>
        <input className={styles.input} type="number" value={posX} onChange={(e) => setPosX(e.target.value)} onBlur={commitPos} />{" "}
        <input className={styles.input} type="number" value={posY} onChange={(e) => setPosY(e.target.value)} onBlur={commitPos} />{" "}
        <input className={styles.input} type="number" value={posZ} onChange={(e) => setPosZ(e.target.value)} onBlur={commitPos} />
      </div>

      <SectionHeading level="h4" title="当前检查点" />
      <input
        className={styles.input}
        type="text"
        defaultValue={state.sequence_current_banc}
        onBlur={(e) => api.setSequenceCurrentBanc(e.target.value).catch((err) => onError(String(err)))}
      />

      <SectionHeading level="h4" title="袋子栏位数量" />
      <NumberField
        label="武器"
        value={state.pouch_weapon_valid_num}
        onCommit={api.setPouchWeaponValidNum}
        onError={onError}
      />
      <NumberField
        label="弓"
        value={state.pouch_bow_valid_num}
        onCommit={api.setPouchBowValidNum}
        onError={onError}
      />
      <NumberField
        label="盾"
        value={state.pouch_shield_valid_num}
        onCommit={api.setPouchShieldValidNum}
        onError={onError}
      />
    </div>
  );

  const mapTab = (
    <div>
      <TotkMapPinsTable mapPins={state.map_pins} onError={onError} />
      <TotkMapMarkersTable mapMarkers={state.map_markers} onError={onError} />
      <TotkTeleportersTable teleporters={state.teleporters} onError={onError} />
    </div>
  );

  return (
    <div className={styles.view}>
      <TabShell
        tabs={[
          { id: "stats", label: "角色", icon: <HomeSprite />, content: statsTab },
          {
            id: "weapons",
            label: "武器",
            icon: <SwordSprite />,
            content: <TotkWeaponsTable weapons={state.pouch_weapons} onError={onError} />,
          },
          {
            id: "bows",
            label: "弓箭",
            icon: <BowSprite />,
            content: (
              <>
                <TotkBowsTable bows={state.pouch_bows} onError={onError} />
                <TotkArrowsTable arrows={state.arrows} onError={onError} />
              </>
            ),
          },
          {
            id: "shields",
            label: "盾牌",
            icon: <ShieldSprite />,
            content: <TotkShieldsTable shields={state.pouch_shields} onError={onError} />,
          },
          {
            id: "armor",
            label: "防具",
            icon: <ShirtSprite />,
            content: <TotkArmorTable armor={state.armor} onError={onError} />,
          },
          {
            id: "materials",
            label: "材料",
            icon: <LeafIcon />,
            content: <TotkMaterialsTable materials={state.materials} onError={onError} />,
          },
          {
            id: "key_items",
            label: "重要物品",
            icon: <ChestIcon />,
            content: (
              <>
                <TotkKeyAbilities onError={onError} />
                <TotkKeyItemsTable keyItems={state.key_items} onError={onError} />
              </>
            ),
          },
          {
            id: "devices",
            label: "左纳乌装置",
            icon: <GearSprite />,
            content: <TotkDevicesTable devices={state.devices} onError={onError} />,
          },
          {
            id: "food",
            label: "料理",
            icon: <AppleSprite />,
            content: <TotkFoodTable food={state.food} onError={onError} />,
          },
          {
            id: "horses",
            label: "马匹",
            icon: <HorseSprite />,
            content: <TotkHorsesTable horses={state.horses} onError={onError} />,
          },
          {
            id: "autobuild",
            label: "蓝图",
            icon: <BlueprintIcon />,
            content: <TotkAutoBuildTable autobuilds={state.autobuilds} onError={onError} />,
          },
          {
            id: "map_pins",
            label: "地图图钉",
            icon: <PinIcon />,
            content: <TotkMapPinsTable mapPins={state.map_pins} onError={onError} />,
          },
          {
            id: "map_markers",
            label: "地图标记",
            icon: <PinIcon />,
            content: <TotkMapMarkersTable mapMarkers={state.map_markers} onError={onError} />,
          },
          {
            id: "teleporters",
            label: "传送点",
            icon: <PinIcon />,
            content: <TotkTeleportersTable teleporters={state.teleporters} onError={onError} />,
          },
          {
            id: "completionism",
            label: "完成度",
            icon: <StarSprite />,
            content: <TotkCompletionismPanel state={state} onError={onError} />,
          },
          {
            id: "hashes",
            label: "高级字段",
            icon: <SearchIcon />,
            content: <TotkHashBrowser onError={onError} />,
          },
        ]}
      />
    </div>
  );
}
