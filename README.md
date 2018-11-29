# Webview-Minesweeper

## About
This is an implementation of the game Minesweeper using [Elm](https://elm-lang.org/) for the UI & [Rust](https://www.rust-lang.org/en-US/) for the Backend.
It is totally overkill to have a Rust backend using Elm's ports to do I/O between the two but this is meant to be a simple project to test out I/O between Elm & Rust and also expirement with other ideas.

## Expirements
### Horizontal & Vertical
Type aliases don't work for preventing you from using variables when you shouldn't so you can still add Horizontal & Vertical if they're defined as type aliases.
So I needed to make Horizontal & Vertical actually different Types to prevent x-axis variables from being used for y-axis variables. It's probably a bit overkill though and I needed to derive `clone` + `copy` otherwise it's really annoying.

Whats also neat is that both `common/mod.rs` & `engine/minesweeper.rs` have impls for Horizontal & Vertical so it's easy to hang utility methods off of the Types for scopes that need them.

## [WebView](https://github.com/Boscop/web-view)
It seems to be a neat way to have a UI that can easily be on different operating systems (as long as they have a browser basically). It worked really well but I'm definitely not a fan of using JSON-encoded &str for interop although it is real easy to decode the data on the Elm side but it'd still be just as easy to decode `Value` as opposed to `String`.

## Issues
### Performance
Well the Rust backend handles fine if you ignore serialization costs. It will take <1s for the Minesweeper game to create a 1000x1000 field but 12 seconds to serialize it so that it can call `toFrontEnd` with that data.

### UI
This is really just me being lazy. I don't feel like coming up with CSS and whatnot so that the tr/td's all line up despite having changing content. I'd probably use collage's in Elm in the future anyways so everything would be inside a `<canvas />` so the sizes of each tile would be constant but its more work than tr/tds and Elm wasn't the focus of this.

### Cannot borrow something mutably because already borrowed
#### initialize_internal_field's was_bomb_placed
Yeah it looks a bit weird how this variable is used because it'd really just make more logical sense to do the statements based on this variable where the variable was set but `self.internal_field.get_mut` & `self.mutate_neighbors` requires `&mut self` so I need them to not be used in the same scope.