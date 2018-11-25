#!/bin/bash

## Scripts home
cd "${0%/*}"

## Build UI
cd minesweeper-ui

elm make src/Main.elm --output=elm.js
if [ $? -ne 0 ]; then
    exit 1
fi

cd ..

## Run Backend
RUST_LOG=minesweeper_backend=TRACE cargo run -p minesweeper-backend