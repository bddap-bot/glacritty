# Glacritty

A fork of [Alacritty](https://github.com/alacritty/alacritty) that adds
`window.background_shader`. The fork's diff is kept small so it rebases on
upstream cheaply; nothing from here goes to upstream.

## Where the fork differs

- `alacritty/src/renderer/background.rs` and `alacritty/res/background.v.glsl`:
  compiling and drawing the shader.
- `alacritty/src/renderer/mod.rs`, `alacritty/src/display/mod.rs`,
  `alacritty/src/window_context.rs`: loading, drawing before the cells, and
  per-frame redraw while the shader reads `iTime`.
- `alacritty/src/config/{mod,window}.rs`: the option, its path resolution, and
  watching the shader file for hot reload.
- The name: `BIN_NAME`/`DEFAULT_NAME` in `alacritty/src/config/window.rs` (config
  and IPC socket paths, window class) and `[[bin]]` in `alacritty/Cargo.toml`.
  The crate stays `alacritty` so upstream's workspace, CI and packaging files
  apply unchanged.
- `extra/shaders/`, this file and the top of `README.md`.

## Rebasing on upstream

```sh
git remote add upstream https://github.com/alacritty/alacritty.git  # once
git fetch upstream
git rebase upstream/master
cargo build --release && cargo test && cargo clippy --all-targets
cargo +nightly fmt -- --check
```

Then run `target/release/glacritty` with an animated and a still example shader:
the animated one moves, the still one leaves the process idle, and a broken edit
to the shader file shows its compile error while the previous shader keeps
drawing. Push with `--force-with-lease`.

## Code style

Comments are deleted unless they state a why the code cannot show.
