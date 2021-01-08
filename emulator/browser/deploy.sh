#!/bin/bash
./build.sh
mkdir -p "$1"
cp wasm -r "$1"
cp index.html "$1"/emulator.html
