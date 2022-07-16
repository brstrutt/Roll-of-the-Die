## Roll of the Die (an unimaginatively named game produced during the GMTK game jam 2022)

This game was produced see how easy it is to produce a webapp Game in Rust.
In it you play as a single Die. Your goal is to roll onto a certain set of squares, landing with a specific number on top of the die.
Control the die with arrow keys.

This game was written in Rust, using Trunk to handle all the web side of things (compiling to a working wasm webapp and running a local server to host the webapp) and Bevy as the game engine.
Highly recommend both. This was much more "possible" than I anticipated.

To run it locally:
clone the repo
install Cargo & Trunk
run `trunk serve --open` from within the repo root

(fun fact, if you make any changes to the source: trunk will automatically recompile and refresh the web-app while `trunk serve --open` is running)

Also don't look to closely at the code. It IS gamejame level quality, so it's not exactly "presentable".