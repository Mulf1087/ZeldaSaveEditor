<div align="center">

<img src="src-tauri/icons/icon.png" alt="ZeldaSaveEditor icon" width="112" />

# ZeldaSaveEditor · 塞尔达存档编辑器

适用于 macOS 的《塞尔达传说：旷野之息》与《塞尔达传说：王国之泪》存档查看和编辑工具。

</div>

## 下载

从 [Releases](https://github.com/Mulf1087/ZeldaSaveEditor/releases) 下载 Apple Silicon 版 DMG。打开磁盘映像，将“塞尔达存档编辑器.app”拖入“应用程序”。此个人构建采用本地临时签名，未经过 Apple 公证。

## 功能

- 自动查找本机 MacNativeGames 中两款游戏的当前存档。
- 列出 `~/Documents/Games` 顶层名称包含“存档”的 ZIP 内的各个 `.sav` 栏位，也能手动打开独立 `.sav`。
- 查看并修改角色数值、物品、马匹、地图与完成度等数据；部分物品标识仍为英文。
- 保存当前 `.sav` 时，首次写入前保留 `.original`，每次写入前保留 `.bak`；通过临时文件替换原文件。
- 编辑 ZIP 栏位时生成新的 ZIP 到 `~/Documents/Games/存档编辑器输出/`，不覆盖原 ZIP，也不会自动导入游戏。

**编辑当前存档前请完全退出游戏，并保留备份。** 本项目已针对开发者本机两款游戏的当前存档和历史 ZIP 完成解析、编辑回环及导出测试；尚未在游戏中加载修改后的存档验证。界面显示的 v1.8、v1.4.x 等指存档**格式**，不等于游戏更新版本。

详细操作见 [使用说明](使用说明.md)。

## 从源码构建

需要 macOS、Node.js、Rust 和 [Tauri v2 的系统依赖](https://v2.tauri.app/start/prerequisites/)。

```bash
npm ci
cargo test -p save-engine -p zeldasaveeditor
npm run tauri build -- --bundles app
```

生成的应用位于 `target/release/bundle/macos/塞尔达存档编辑器.app`。本仓库只保留上游公开测试夹具，不包含本机用户存档、游戏文件、密钥或固件。

## 来源与许可

本项目基于 [Inkapa/zelda-save-editor](https://github.com/Inkapa/zelda-save-editor) 的 MIT 许可源码开发，保留了原作者许可文本 [LICENSE](LICENSE)。存档格式研究源自 [marcrobledo/savegame-editors](https://github.com/marcrobledo/savegame-editors)；物品图标也来自该项目，其 BOTW 素材注明来源为 [The Spriters Resource](https://www.spriters-resource.com/wii_u/thelegendofzeldabreathofthewild/)。本工具与 Nintendo 无关联。

ZeldaSaveEditor is a macOS save viewer and editor for Breath of the Wild and Tears of the Kingdom. It builds on Inkapa's MIT-licensed Zelda save editor and preserves that license. See the Chinese instructions above for save locations, backup behavior, and build steps.
