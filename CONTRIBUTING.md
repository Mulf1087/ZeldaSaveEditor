# Contributing to ZeldaSaveEditor

This is a macOS focused fork of [Inkapa/zelda-save-editor](https://github.com/Inkapa/zelda-save-editor). Please report the game, save-format version, exact action, and observed result when filing an issue. Do not attach personal saves, Nintendo game files, firmware, or keys to public issues.

## Development

```bash
npm ci
cargo test -p save-engine -p zeldasaveeditor
npm run tauri dev
```

Verify changes to binary save parsing against an appropriate fixture, and preserve a no-edit byte-for-byte round trip. For UI changes, check the packaged macOS app as well as the TypeScript build. Keep source and application names consistent with `ZeldaSaveEditor` / `塞尔达存档编辑器`.

The MIT license and upstream attribution in [README.md](README.md) must remain with redistributions.
