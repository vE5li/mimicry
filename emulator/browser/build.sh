#!/bin/bash
wasm-pack build --target no-modules &&
mkdir -p wasm &&
cp pkg/mimicry_emulator_linux_bg.wasm wasm/emulator.wasm &&
cp pkg/mimicry_emulator_linux.js wasm/emulator.js
