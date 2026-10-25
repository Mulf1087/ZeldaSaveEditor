#!/usr/bin/env python3
"""Idempotently patches src-tauri/gen/android/app/build.gradle.kts to add a release
signingConfig read from a sibling keystore.properties file.

`tauri android init` regenerates gen/android from scratch (it isn't committed to this
repo), so this runs fresh in CI after every init, right before the signed build. Safe to
run when build.gradle.kts is already patched: it no-ops.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

GRADLE_FILE = Path(__file__).resolve().parent.parent / "src-tauri/gen/android/app/build.gradle.kts"

SIGNING_CONFIG_MARKER = 'create("release") {'
WIRED_RELEASE_MARKER = 'signingConfig = signingConfigs.getByName("release")'


def patch() -> int:
    if not GRADLE_FILE.exists():
        print(f"[setup-android-signing] {GRADLE_FILE} not found", file=sys.stderr)
        return 1

    src = GRADLE_FILE.read_text()
    if SIGNING_CONFIG_MARKER in src and WIRED_RELEASE_MARKER in src:
        print("[setup-android-signing] already patched, no-op")
        return 0

    if "import java.io.FileInputStream" not in src:
        src = re.sub(
            r"((?:import [^\n]+\n)+)",
            r"\1import java.io.FileInputStream\n",
            src,
            count=1,
        )

    signing_block = (
        "    signingConfigs {\n"
        '        create("release") {\n'
        '            val keystorePropertiesFile = rootProject.file("keystore.properties")\n'
        "            val keystoreProperties = Properties()\n"
        "            keystoreProperties.load(FileInputStream(keystorePropertiesFile))\n"
        '            keyAlias = keystoreProperties["keyAlias"] as String\n'
        '            keyPassword = keystoreProperties["keyPassword"] as String\n'
        '            storeFile = file(keystoreProperties["storeFile"] as String)\n'
        '            storePassword = keystoreProperties["storePassword"] as String\n'
        "        }\n"
        "    }\n"
        "    "
    )
    if "signingConfigs" not in src:
        src, n = re.subn(r"    buildTypes \{", signing_block + "buildTypes {", src, count=1)
        if n == 0:
            print("[setup-android-signing] couldn't find 'buildTypes {' to insert signingConfigs before", file=sys.stderr)
            return 1

    if WIRED_RELEASE_MARKER not in src:
        src, n = re.subn(
            r'(getByName\("release"\)\s*\{\s*)',
            r'\1signingConfig = signingConfigs.getByName("release")\n            ',
            src,
            count=1,
        )
        if n == 0:
            print('[setup-android-signing] couldn\'t find getByName("release") { to wire signingConfig into', file=sys.stderr)
            return 1

    if SIGNING_CONFIG_MARKER not in src or WIRED_RELEASE_MARKER not in src:
        print("[setup-android-signing] patch did not produce the expected markers, refusing to write", file=sys.stderr)
        return 1

    GRADLE_FILE.write_text(src)
    print("[setup-android-signing] build.gradle.kts patched")
    return 0


if __name__ == "__main__":
    sys.exit(patch())
