# An Intro to OSC

**Tutorial Info**

- Author: [madskjeldgaard](https://madskjeldgaard.dk)
- Required Knowledge:
    - [Anatomy of a splatter App](/tutorials/basics/anatomy-of-a-splatter-app.md)
- Reading Time: 5 minutes
---

## What is OSC?

Open Sound Control or [OSC](http://opensoundcontrol.org/) is a protocol for communicating between different pieces of software and/or computers. It is based on network technology and offers a flexible way to share control data between processes with a high level of precision, either internally on your local machine or through a network connection.

In splatter it's possible to both send and receive OSC data, allowing you to control other software or let splatter be controlled by other software.

## Setting up OSC

To use OSC in splatter, it is necessary to add the `splatter_osc` crate as a dependency in your splatter project.

Open up your `Cargo.toml` file at the root of your splatter project and add the following line under the `[dependencies]` tag:

```toml
splatter_osc = "0.15.0"
```

The value in the quotes is the version of the OSC package. At the time of writing this, `"0.15.0"` is the latest version.

To get the latest version of the osc library, execute `cargo search splatter_osc` on the command line and read the resulting version from there.

To use the crate in your splatter-projects you can add a use-statement at the top of your `main.rs` file to import the OSC-functionality.

```rust,no_run
# #![allow(unused_imports)]
use splatter_osc as osc;
# fn main() {}
```
