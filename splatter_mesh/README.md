# splatter_mesh

An API for composing **Mesh**s shared between `splatter` crates.

**Mesh**s may be composed of different sets of channels including position,
color, texture-coordinate and normals. Note that this is quite a low-level
representation. For a higher-level, graphics-related mesh API, see the
`splatter::draw` module and the `draw.mesh()` API.
