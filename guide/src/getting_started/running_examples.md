# Running Examples

The easiest way to get familiar with splatter is to explore the examples.

splatter provides three collections of examples:

| **Path** | **Description** |
| --- | --- |
| [**`examples/`**](https://github.com/splatter-org/splatter/tree/master/examples) | A collection of examples with categorised demonstrations of splatter. |
| [**`generative_design/`**](https://github.com/splatter-org/splatter/tree/master/generative_design) | Examples from [Generative Gestaltung](http://www.generative-gestaltung.de/), ported from p5.js to splatter. |
| [**`nature_of_code/`**](https://github.com/splatter-org/splatter/tree/master/nature_of_code) | Examples from [Nature of Code](https://natureofcode.com/), ported from Processing to splatter. |

To get the examples we can clone the splatter repository.

```bash
git clone https://github.com/splatter-org/splatter
```

If you do not have `git` installed you can press the "Clone or download" button
at the top of this page and then press "Download .zip".

Now, change the current directory to `splatter`.

```bash
cd splatter
```

Run the example using cargo!

```bash
cargo run --release --example draw
```

The `--release` flag means we want to build with optimisations enabled.

The value passed via the `--example` flag matches the `name` property of an
entry within the `[[examples]]` table of the package's `Cargo.toml` file. The
matched entry's `path` property points to the source file to build:

```toml
# Draw
[[example]]
name = "draw"
path = "draw/draw.rs"
```

If we were to look through the nature of code directory and decide we want to
run the following example:

```toml
# Chapter 1 Vectors
[[example]]
name = "1_1_bouncingball_novectors"
path = "chp_01_vectors/1_1_bouncingball_novectors.rs"
```

We could do so with the following:

```bash
cargo run --release --example 1_1_bouncingball_novectors
```

In general, the name of the example will almost always be the file name without
the `.rs` extension.

If you are compiling splatter for the first time you will see cargo download and
build all the necessary dependencies. This might take a while! Luckily, we only
have to wait for this the first time.

![cargo](https://i.imgur.com/5OBNqMB.gif)

Once the example compiles you should see the following window appear.

![draw_HD](https://i.imgur.com/HVVamUI.gif)

To run any of the other examples, replace `draw` with the name of the
desired example.
