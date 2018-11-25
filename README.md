# Webview-Minesweeper

## About
This is an implementation of the game Minesweeper using [Elm](https://elm-lang.org/) for the UI & [Rust](https://www.rust-lang.org/en-US/) for the Backend.
It is totally overkill to have a Rust backend using Elm's ports to do I/O between the two but this is meant to be a simple project to test out I/O between Elm & Rust.

## Issues
### Performance
Well the Rust backend handles fine if you ignore serialization costs. It will take <1s for the Minesweeper game to create a 1000x1000 field but 12 seconds to serialize it so that it can call `toFrontEnd` with that data.
I think that [Electron](https://github.com/neon-bindings/neon) wouldn't have this issue though as I think you can just pass Objects to it.

## [WebView](https://github.com/Boscop/web-view)
It seems to be a neat way to have a UI that can easily be on different operating systems (as long as they have a browser basically). I didn't do that much research but it seems to be effectively a lighter weight electron.