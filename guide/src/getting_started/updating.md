# Updating splatter

You can update to a new version of splatter by editing your `Cargo.toml` file to
use the new crate. For version 0.12 add the line

```toml
splatter = "0.12"
```

Then within the splatter directory run the following to update all dependencies:

```bash
cargo update
```

## Updating Rust.

From time to time, a splatter update might require features from a newer version
of rustc. For example, splatter 0.12 is known to require at least rustc 1.35.0. In
these cases, you can update your rust toolchain to the latest version by running
the following:

```bash
rustup update
```
