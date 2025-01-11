#!/bin/bash
wasm-pack build --target web
mv -f ./pkg/art_verse.js ./app/art_verse.js
mv -f ./pkg/art_verse_bg.wasm ./app/art_verse_bg.wasm
