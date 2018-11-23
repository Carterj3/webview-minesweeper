#!/bin/bash

## Scripts home
cd "${0%/*}"

## Build UI
cd minesweeper-ui
elm make src/Main.elm --output=elm.js
cd ..

## Build Backend
cargo run -p minesweeper-backend