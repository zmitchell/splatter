//! An API for composing **Mesh**s. **Mesh**s may be composed of different sets of channels
//! including position, color, texture-coordinate and normals. Note that this is quite a low-level
//! representation. For a higher-level, graphics-related mesh API, see the `draw` module.

use core::cell::{Ref, RefMut};
use core::cmp;
use core::convert::{TryFrom, TryInto};
use core::ops::{self, Deref, DerefMut};
use splatter_core::geom;

pub mod channel;
pub mod vertex;

pub use self::channel::{Channel, ChannelMut};

// Traits describing meshes with access to certain channels.

/// Mesh types that can be indexed to produce a vertex.
pub trait GetVertex<I> {
    /// The vertex type representing all channels of data within the mesh at a single index.
    type Vertex;
    /// Create a vertex containing all channel properties for the given index.
    fn get_vertex(&self, index: I) -> Option<Self::Vertex>;
}

/// All meshes must contain at least one vertex channel.
pub trait Points {
    /// The vertex type used to represent the location of a vertex.
    type Point;
    /// The channel type containing points.
    type Points: Channel<Element = Self::Point>;
    /// Borrow the vertex channel from the mesh.
    fn points(&self) -> &Self::Points;
}

/// Meshes that contain a channel of indices that describe the edges between points.
pub trait Indices {
    /// The type used to index into the vertex buffer.
    type Index;
    /// The channel type containing indices.
    type Indices: Channel<Element = Self::Index>;
    /// Borrow the index channel from the mesh.
    fn indices(&self) -> &Self::Indices;
}

/// Meshes that contain a channel of colors.
pub trait Colors {
    /// The color type stored within the channel.
    type Color;
    /// The channel type containing colors.
    type Colors: Channel<Element = Self::Color>;
    /// Borrow the color channel from the mesh.
    fn colors(&self) -> &Self::Colors;
}

/// Meshes that contain a channel of texture coordinates.
pub trait TexCoords {
    /// The point type used to represent texture coordinates.
    type TexCoord;
    /// The channel type containing texture coordinates.
    type TexCoords: Channel<Element = Self::TexCoord>;
    /// Borrow the texture coordinate channel from the mesh.
    fn tex_coords(&self) -> &Self::TexCoords;
}

/// Meshes that contain a channel of vertex normals.
pub trait Normals {
    /// The vector type used to represent the normal.
    type Normal;
    /// The channel type containing vertex normals.
    type Normals: Channel<Element = Self::Normal>;
    /// Borrow the normal channel from the mesh.
    fn normals(&self) -> &Self::Normals;
}

/// Meshes that can push vertices of type **V** while keeping all non-index channels the same
/// length before and after the push.
pub trait PushVertex<V> {
    /// Push the given vertex onto the mesh.
    ///
    /// Implementation requires that all non-index channels maintain the same length before and
    /// after a call to this method.
    fn push_vertex(&mut self, vertex: V);
}

/// Meshes that contain an **Indices** channel and can push new indices to it.
pub trait PushIndex {
    /// The inner index type.
    type Index;
    /// Push a new index onto the indices channel.
    fn push_index(&mut self, index: Self::Index);
    /// Extend the **Mesh**'s **Indices** channel with the given indices.
    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = Self::Index>,
    {
        for i in indices {
            self.push_index(i);
        }
    }
}

/// Meshes whose **Indices** channel can be cleared.
pub trait ClearIndices {
    /// Clear all indices from the mesh.
    fn clear_indices(&mut self);
}

/// Meshes whose vertices channels can be cleared.
pub trait ClearVertices {
    /// Clear all vertices from the mesh.
    fn clear_vertices(&mut self);
}

/// Meshes whose indices and vertices buffers may be cleared for re-use.
pub trait Clear: ClearIndices + ClearVertices {
    fn clear(&mut self) {
        self.clear_indices();
        self.clear_vertices();
    }
}

/// Meshes that may be extended from a slice of data.
pub trait ExtendFromSlice<'a> {
    /// The slice type expected via the mesh.
    ///
    /// Note: This may be multiple combined slices if the mesh contains multiple channels of data.
    type Slice: 'a;
    /// Extend the mesh.
    fn extend_from_slice(&mut self, slice: Self::Slice);
}

// Mesh types.

/// The base mesh type with only a single vertex channel.
///
/// Extra channels can be added to the mesh via the `WithIndices`, `WithColors`, `WithTexCoords`
/// and `WithNormals` adaptor types.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MeshPoints<P> {
    points: P,
}

/// A mesh type with an added channel containing indices describing the edges between vertices.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithIndices<M, I> {
    mesh: M,
    indices: I,
}

/// A `Mesh` type with an added channel containing colors.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithColors<M, C> {
    mesh: M,
    colors: C,
}

/// A `Mesh` type with an added channel containing texture coordinates.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithTexCoords<M, T> {
    mesh: M,
    tex_coords: T,
}

/// A `Mesh` type with an added channel containing vertex normals.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithNormals<M, N> {
    mesh: M,
    normals: N,
}

// **GetVertex** implementations.

impl<'a, M, I> GetVertex<I> for &'a M
where
    M: GetVertex<I>,
{
    type Vertex = M::Vertex;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        (**self).get_vertex(index)
    }
}

impl<'a, M, I> GetVertex<I> for &'a mut M
where
    M: GetVertex<I>,
{
    type Vertex = M::Vertex;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        (**self).get_vertex(index)
    }
}

impl<'a, M, I> GetVertex<I> for Ref<'a, M>
where
    M: GetVertex<I>,
{
    type Vertex = M::Vertex;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        (**self).get_vertex(index)
    }
}

impl<'a, M, I> GetVertex<I> for RefMut<'a, M>
where
    M: GetVertex<I>,
{
    type Vertex = M::Vertex;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        (**self).get_vertex(index)
    }
}

impl<P, I> GetVertex<I> for MeshPoints<P>
where
    P: Channel,
    P::Element: Clone,
    I: TryInto<usize>,
{
    type Vertex = P::Element;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        let index = index
            .try_into()
            .unwrap_or_else(|_err| panic!("index out of range of valid `usize` values"));
        self.points.channel().get(index).cloned()
    }
}

impl<M, I, Ix> GetVertex<Ix> for WithIndices<M, I>
where
    M: GetVertex<Ix>,
{
    type Vertex = M::Vertex;
    fn get_vertex(&self, index: Ix) -> Option<Self::Vertex> {
        self.mesh.get_vertex(index)
    }
}

impl<M, C, I> GetVertex<I> for WithColors<M, C>
where
    M: GetVertex<I>,
    C: Channel,
    C::Element: Clone,
    I: Copy + TryInto<usize>,
{
    type Vertex = vertex::WithColor<M::Vertex, C::Element>;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        self.mesh.get_vertex(index).and_then(|vertex| {
            let index: usize = index
                .try_into()
                .unwrap_or_else(|_err| panic!("index out of range of valid usize values"));
            self.colors.channel().get(index).map(|color: &C::Element| {
                let color = color.clone();
                vertex::WithColor { vertex, color }
            })
        })
    }
}

impl<M, T, I> GetVertex<I> for WithTexCoords<M, T>
where
    M: GetVertex<I>,
    T: Channel,
    T::Element: Clone,
    I: Copy + TryInto<usize>,
{
    type Vertex = vertex::WithTexCoords<M::Vertex, T::Element>;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        self.mesh.get_vertex(index).and_then(|vertex| {
            let index: usize = index
                .try_into()
                .unwrap_or_else(|_err| panic!("index out of range of valid usize values"));
            self.tex_coords.channel().get(index).map(|tex_coords| {
                let tex_coords = tex_coords.clone();
                vertex::WithTexCoords { vertex, tex_coords }
            })
        })
    }
}

impl<M, N, I> GetVertex<I> for WithNormals<M, N>
where
    M: GetVertex<I>,
    N: Channel,
    N::Element: Clone,
    I: Copy + TryInto<usize>,
{
    type Vertex = vertex::WithNormal<M::Vertex, N::Element>;
    fn get_vertex(&self, index: I) -> Option<Self::Vertex> {
        self.mesh.get_vertex(index).and_then(|vertex| {
            let index: usize = index
                .try_into()
                .unwrap_or_else(|_err| panic!("index out of range of valid usize values"));
            self.normals.channel().get(index).map(|normal| {
                let normal = normal.clone();
                vertex::WithNormal { vertex, normal }
            })
        })
    }
}

// **Points** implementations.

impl<P> Points for MeshPoints<P>
where
    P: Channel,
{
    type Point = P::Element;
    type Points = P;
    fn points(&self) -> &Self::Points {
        &self.points
    }
}

impl<'a, M> Points for &'a M
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        (**self).points()
    }
}

impl<'a, M> Points for &'a mut M
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        (**self).points()
    }
}

impl<'a, M> Points for Ref<'a, M>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        (**self).points()
    }
}

impl<'a, M> Points for RefMut<'a, M>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        (**self).points()
    }
}

impl<M, I> Points for WithIndices<M, I>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        self.mesh.points()
    }
}

impl<M, C> Points for WithColors<M, C>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        self.mesh.points()
    }
}

impl<M, T> Points for WithTexCoords<M, T>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        self.mesh.points()
    }
}

impl<M, N> Points for WithNormals<M, N>
where
    M: Points,
{
    type Point = M::Point;
    type Points = M::Points;
    fn points(&self) -> &Self::Points {
        self.mesh.points()
    }
}

// **Indices** implementations.

impl<M, I> Indices for WithIndices<M, I>
where
    I: Channel,
{
    type Index = I::Element;
    type Indices = I;
    fn indices(&self) -> &Self::Indices {
        &self.indices
    }
}

impl<'a, M> Indices for &'a M
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        (**self).indices()
    }
}

impl<'a, M> Indices for &'a mut M
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        (**self).indices()
    }
}

impl<'a, M> Indices for Ref<'a, M>
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        (**self).indices()
    }
}

impl<'a, M> Indices for RefMut<'a, M>
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        (**self).indices()
    }
}

impl<M, C> Indices for WithColors<M, C>
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        self.mesh.indices()
    }
}

impl<M, T> Indices for WithTexCoords<M, T>
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        self.mesh.indices()
    }
}

impl<M, N> Indices for WithNormals<M, N>
where
    M: Indices,
{
    type Index = M::Index;
    type Indices = M::Indices;
    fn indices(&self) -> &Self::Indices {
        self.mesh.indices()
    }
}

// **Colors** implementations.

impl<M, C> Colors for WithColors<M, C>
where
    C: Channel,
{
    type Color = C::Element;
    type Colors = C;
    fn colors(&self) -> &Self::Colors {
        &self.colors
    }
}

impl<'a, M> Colors for &'a M
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        (**self).colors()
    }
}

impl<'a, M> Colors for &'a mut M
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        (**self).colors()
    }
}

impl<'a, M> Colors for Ref<'a, M>
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        (**self).colors()
    }
}

impl<'a, M> Colors for RefMut<'a, M>
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        (**self).colors()
    }
}

impl<M, I> Colors for WithIndices<M, I>
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        self.mesh.colors()
    }
}

impl<M, T> Colors for WithTexCoords<M, T>
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        self.mesh.colors()
    }
}

impl<M, N> Colors for WithNormals<M, N>
where
    M: Colors,
{
    type Color = M::Color;
    type Colors = M::Colors;
    fn colors(&self) -> &Self::Colors {
        self.mesh.colors()
    }
}

// **TexCoords** implementations.

impl<M, T> TexCoords for WithTexCoords<M, T>
where
    T: Channel,
{
    type TexCoord = T::Element;
    type TexCoords = T;
    fn tex_coords(&self) -> &Self::TexCoords {
        &self.tex_coords
    }
}

impl<'a, M> TexCoords for &'a M
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        (**self).tex_coords()
    }
}

impl<'a, M> TexCoords for &'a mut M
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        (**self).tex_coords()
    }
}

impl<'a, M> TexCoords for Ref<'a, M>
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        (**self).tex_coords()
    }
}

impl<'a, M> TexCoords for RefMut<'a, M>
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        (**self).tex_coords()
    }
}

impl<M, I> TexCoords for WithIndices<M, I>
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        self.mesh.tex_coords()
    }
}

impl<M, C> TexCoords for WithColors<M, C>
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        self.mesh.tex_coords()
    }
}

impl<M, N> TexCoords for WithNormals<M, N>
where
    M: TexCoords,
{
    type TexCoord = M::TexCoord;
    type TexCoords = M::TexCoords;
    fn tex_coords(&self) -> &Self::TexCoords {
        self.mesh.tex_coords()
    }
}

// **Normals** implementations.

impl<M, N> Normals for WithNormals<M, N>
where
    M: Points,
    N: Channel,
{
    type Normal = N::Element;
    type Normals = N;
    fn normals(&self) -> &Self::Normals {
        &self.normals
    }
}

impl<'a, M> Normals for &'a M
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        (**self).normals()
    }
}

impl<'a, M> Normals for &'a mut M
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        (**self).normals()
    }
}

impl<'a, M> Normals for Ref<'a, M>
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        (**self).normals()
    }
}

impl<'a, M> Normals for RefMut<'a, M>
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        (**self).normals()
    }
}

impl<M, I> Normals for WithIndices<M, I>
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        self.mesh.normals()
    }
}

impl<M, C> Normals for WithColors<M, C>
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        self.mesh.normals()
    }
}

impl<M, T> Normals for WithTexCoords<M, T>
where
    M: Normals,
{
    type Normal = M::Normal;
    type Normals = M::Normals;
    fn normals(&self) -> &Self::Normals {
        self.mesh.normals()
    }
}

// PushVertex implementations for each mesh type where the channels are **Vec**s.

impl<'a, M, V> PushVertex<V> for &'a mut M
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: V) {
        (**self).push_vertex(v)
    }
}

impl<'a, M, V> PushVertex<V> for RefMut<'a, M>
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: V) {
        (**self).push_vertex(v)
    }
}

impl<V> PushVertex<V> for MeshPoints<Vec<V>> {
    fn push_vertex(&mut self, v: V) {
        self.points.push(v);
    }
}

impl<M, I, V> PushVertex<V> for WithIndices<M, Vec<I>>
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: V) {
        self.mesh.push_vertex(v);
    }
}

impl<M, V, C> PushVertex<vertex::WithColor<V, C>> for WithColors<M, Vec<C>>
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: vertex::WithColor<V, C>) {
        let vertex::WithColor { vertex, color } = v;
        self.colors.push(color);
        self.mesh.push_vertex(vertex);
    }
}

impl<M, V, T> PushVertex<vertex::WithTexCoords<V, T>> for WithTexCoords<M, Vec<T>>
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: vertex::WithTexCoords<V, T>) {
        let vertex::WithTexCoords { vertex, tex_coords } = v;
        self.tex_coords.push(tex_coords);
        self.mesh.push_vertex(vertex);
    }
}

impl<M, V, N> PushVertex<vertex::WithNormal<V, N>> for WithNormals<M, Vec<N>>
where
    M: PushVertex<V>,
{
    fn push_vertex(&mut self, v: vertex::WithNormal<V, N>) {
        let vertex::WithNormal { vertex, normal } = v;
        self.normals.push(normal);
        self.mesh.push_vertex(vertex);
    }
}

// PushIndex implementations for meshes.

impl<'a, M> PushIndex for &'a mut M
where
    M: PushIndex,
{
    type Index = M::Index;
    fn push_index(&mut self, index: Self::Index) {
        (**self).push_index(index);
    }
    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = Self::Index>,
    {
        (**self).extend_indices(indices);
    }
}

impl<'a, M> PushIndex for RefMut<'a, M>
where
    M: PushIndex,
{
    type Index = M::Index;
    fn push_index(&mut self, index: Self::Index) {
        (**self).push_index(index);
    }
    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = Self::Index>,
    {
        (**self).extend_indices(indices);
    }
}

impl<M, I> PushIndex for WithIndices<M, Vec<I>> {
    type Index = I;

    fn push_index(&mut self, index: Self::Index) {
        self.indices.push(index);
    }

    fn extend_indices<It>(&mut self, indices: It)
    where
        It: IntoIterator<Item = Self::Index>,
    {
        self.indices.extend(indices);
    }
}

impl<M, C> PushIndex for WithColors<M, C>
where
    M: PushIndex,
{
    type Index = M::Index;

    fn push_index(&mut self, index: Self::Index) {
        self.mesh.push_index(index);
    }

    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = Self::Index>,
    {
        self.mesh.extend_indices(indices);
    }
}

impl<M, T> PushIndex for WithTexCoords<M, T>
where
    M: PushIndex,
{
    type Index = M::Index;

    fn push_index(&mut self, index: Self::Index) {
        self.mesh.push_index(index);
    }

    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = Self::Index>,
    {
        self.mesh.extend_indices(indices);
    }
}

impl<M, N> PushIndex for WithNormals<M, N>
where
    M: PushIndex,
{
    type Index = M::Index;

    fn push_index(&mut self, index: M::Index) {
        self.mesh.push_index(index);
    }

    fn extend_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator<Item = M::Index>,
    {
        self.mesh.extend_indices(indices);
    }
}

// **ClearIndices** implementations

impl<'a, M> ClearIndices for &'a mut M
where
    M: ClearIndices,
{
    fn clear_indices(&mut self) {
        (**self).clear_indices();
    }
}

impl<'a, M> ClearIndices for RefMut<'a, M>
where
    M: ClearIndices,
{
    fn clear_indices(&mut self) {
        (**self).clear_indices();
    }
}

impl<M, I> ClearIndices for WithIndices<M, Vec<I>> {
    fn clear_indices(&mut self) {
        self.indices.clear();
    }
}

impl<M, C> ClearIndices for WithColors<M, C>
where
    M: ClearIndices,
{
    fn clear_indices(&mut self) {
        self.mesh.clear_indices();
    }
}

impl<M, T> ClearIndices for WithTexCoords<M, T>
where
    M: ClearIndices,
{
    fn clear_indices(&mut self) {
        self.mesh.clear_indices();
    }
}

impl<M, N> ClearIndices for WithNormals<M, N>
where
    M: ClearIndices,
{
    fn clear_indices(&mut self) {
        self.mesh.clear_indices();
    }
}

// **ClearVertices** implementations

impl<'a, M> ClearVertices for &'a mut M
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        (**self).clear_vertices()
    }
}

impl<'a, M> ClearVertices for RefMut<'a, M>
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        (**self).clear_vertices()
    }
}

impl<V> ClearVertices for MeshPoints<Vec<V>> {
    fn clear_vertices(&mut self) {
        self.points.clear();
    }
}

impl<M, I> ClearVertices for WithIndices<M, Vec<I>>
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        self.mesh.clear_vertices();
        self.indices.clear();
    }
}

impl<M, C> ClearVertices for WithColors<M, Vec<C>>
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        self.mesh.clear_vertices();
        self.colors.clear();
    }
}

impl<M, T> ClearVertices for WithTexCoords<M, Vec<T>>
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        self.mesh.clear_vertices();
        self.tex_coords.clear();
    }
}

impl<M, N> ClearVertices for WithNormals<M, Vec<N>>
where
    M: ClearVertices,
{
    fn clear_vertices(&mut self) {
        self.mesh.clear_vertices();
        self.normals.clear();
    }
}

// **ExtendFromSlice** implementations

impl<'a, P> ExtendFromSlice<'a> for MeshPoints<Vec<P>>
where
    P: 'a + Clone,
{
    type Slice = &'a [P];
    fn extend_from_slice(&mut self, slice: Self::Slice) {
        self.points.extend_from_slice(slice);
    }
}

impl<'a, M, I> ExtendFromSlice<'a> for WithIndices<M, Vec<I>>
where
    M: ExtendFromSlice<'a>,
    I: 'a + Clone,
{
    type Slice = (&'a [I], M::Slice);
    fn extend_from_slice(&mut self, slice: Self::Slice) {
        let (slice, inner) = slice;
        self.mesh.extend_from_slice(inner);
        self.indices.extend_from_slice(slice);
    }
}

impl<'a, M, C> ExtendFromSlice<'a> for WithColors<M, Vec<C>>
where
    M: ExtendFromSlice<'a>,
    C: 'a + Clone,
{
    type Slice = (&'a [C], M::Slice);
    fn extend_from_slice(&mut self, slice: Self::Slice) {
        let (slice, inner) = slice;
        self.mesh.extend_from_slice(inner);
        self.colors.extend_from_slice(slice);
    }
}

impl<'a, M, T> ExtendFromSlice<'a> for WithTexCoords<M, Vec<T>>
where
    M: ExtendFromSlice<'a>,
    T: 'a + Clone,
{
    type Slice = (&'a [T], M::Slice);
    fn extend_from_slice(&mut self, slice: Self::Slice) {
        let (slice, inner) = slice;
        self.mesh.extend_from_slice(inner);
        self.tex_coords.extend_from_slice(slice);
    }
}

impl<'a, M, N> ExtendFromSlice<'a> for WithNormals<M, Vec<N>>
where
    M: ExtendFromSlice<'a>,
    N: 'a + Clone,
{
    type Slice = (&'a [N], M::Slice);
    fn extend_from_slice(&mut self, slice: Self::Slice) {
        let (slice, inner) = slice;
        self.mesh.extend_from_slice(inner);
        self.normals.extend_from_slice(slice);
    }
}

// **Clear** implementation for all meshes.

impl<T> Clear for T where T: ClearIndices + ClearVertices {}

// **Default** implementations for all meshes.

impl<P> Default for MeshPoints<P>
where
    P: Default,
{
    fn default() -> Self {
        let points = Default::default();
        MeshPoints { points }
    }
}

impl<M, I> Default for WithIndices<M, I>
where
    M: Default,
    I: Default,
{
    fn default() -> Self {
        let mesh = Default::default();
        let indices = Default::default();
        WithIndices { mesh, indices }
    }
}

impl<M, C> Default for WithColors<M, C>
where
    M: Default,
    C: Default,
{
    fn default() -> Self {
        let mesh = Default::default();
        let colors = Default::default();
        WithColors { mesh, colors }
    }
}

impl<M, T> Default for WithTexCoords<M, T>
where
    M: Default,
    T: Default,
{
    fn default() -> Self {
        let mesh = Default::default();
        let tex_coords = Default::default();
        WithTexCoords { mesh, tex_coords }
    }
}

impl<M, N> Default for WithNormals<M, N>
where
    M: Default,
    N: Default,
{
    fn default() -> Self {
        let mesh = Default::default();
        let normals = Default::default();
        WithNormals { mesh, normals }
    }
}

// Deref implementations for the mesh adaptor types to their inner mesh.

impl<M, I> Deref for WithIndices<M, I> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl<M, I> DerefMut for WithIndices<M, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl<M, C> Deref for WithColors<M, C> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl<M, C> DerefMut for WithColors<M, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl<M, T> Deref for WithTexCoords<M, T> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl<M, T> DerefMut for WithTexCoords<M, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl<M, N> Deref for WithNormals<M, N> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl<M, N> DerefMut for WithNormals<M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

// Mesh length functions.

/// Get the number of vertices in the mesh.
pub fn raw_vertex_count<M>(mesh: M) -> usize
where
    M: Points,
{
    mesh.points().channel().len()
}

/// The number of vertices that would be yielded by a **Vertices** iterator for the given mesh.
pub fn vertex_count<M>(mesh: M) -> usize
where
    M: Indices,
{
    mesh.indices().channel().len()
}

/// The number of triangles that would be yielded by a **Triangles** iterator for the given mesh.
pub fn triangle_count<M>(mesh: M) -> usize
where
    M: Indices,
{
    vertex_count(mesh) / geom::tri::NUM_VERTICES as usize
}

// Mesh constructors.

/// Create a simple base mesh from the given channel of vertex points.
pub fn from_points<P>(points: P) -> MeshPoints<P>
where
    P: Channel,
{
    MeshPoints { points }
}

/// Combine the given mesh with the given channel of vertex indices.
pub fn with_indices<M, I, Ix>(mesh: M, indices: I) -> WithIndices<M, I>
where
    M: GetVertex<Ix>,
    I: Channel<Element = Ix>,
{
    WithIndices { mesh, indices }
}

/// Combine the given mesh with the given channel of vertex colors.
///
/// **Panics** if the length of the **colors** channel differs from **points**.
pub fn with_colors<M, C>(mesh: M, colors: C) -> WithColors<M, C>
where
    M: Points,
    C: Channel,
{
    assert_eq!(raw_vertex_count(&mesh), colors.channel().len());
    WithColors { mesh, colors }
}

/// Combine the given mesh with the given channel of vertex texture coordinates.
///
/// **Panics** if the length of the **tex_coords** channel differs from **points**.
pub fn with_tex_coords<M, T>(mesh: M, tex_coords: T) -> WithTexCoords<M, T>
where
    M: Points,
    T: Channel,
{
    assert_eq!(raw_vertex_count(&mesh), tex_coords.channel().len());
    WithTexCoords { mesh, tex_coords }
}

/// Combine the given mesh with the given **Normals** channel.
///
/// **Panics** if the length of the **normals** channel differs from **points**.
pub fn with_normals<M, N>(mesh: M, normals: N) -> WithNormals<M, N>
where
    M: Points,
    N: Channel,
{
    assert_eq!(raw_vertex_count(&mesh), normals.channel().len());
    WithNormals { mesh, normals }
}

// Mesh mutation functions.

/// Push the given vertex to the given `mesh`.
///
/// The lengths of all non-index channels within the mesh should remain equal before and after a
/// call to this function.
pub fn push_vertex<M, V>(mut mesh: M, vertex: V)
where
    M: PushVertex<V>,
{
    mesh.push_vertex(vertex);
}

/// Extend the given **mesh** with the given sequence of **vertices**.
///
/// The lengths of all non-index channels within the mesh should remain equal before and after a
/// call to this function.
pub fn extend_vertices<M, I>(mut mesh: M, vertices: I)
where
    M: PushVertex<I::Item>,
    I: IntoIterator,
{
    for v in vertices {
        push_vertex(&mut mesh, v);
    }
}

/// Push the given index to the given `mesh`.
pub fn push_index<M>(mut mesh: M, index: M::Index)
where
    M: PushIndex,
{
    mesh.push_index(index);
}

/// Extend the given mesh with the given indices.
pub fn extend_indices<M, I>(mut mesh: M, indices: I)
where
    M: PushIndex,
    I: IntoIterator<Item = M::Index>,
{
    mesh.extend_indices(indices);
}

/// Clear all vertices from the mesh.
pub fn clear_vertices<M>(mut mesh: M)
where
    M: ClearVertices,
{
    mesh.clear_vertices();
}

/// Clear all indices from the mesh.
pub fn clear_indices<M>(mut mesh: M)
where
    M: ClearIndices,
{
    mesh.clear_indices();
}

/// Clear all vertices and indices from the mesh.
pub fn clear<M>(mut mesh: M)
where
    M: Clear,
{
    mesh.clear();
}

// Mesh iterators.

/// An iterator yielding the raw vertices (with combined channels) of a mesh.
///
/// Requires that the mesh implements **GetVertex**.
///
/// Returns `None` when the inner mesh first returns `None` for a call to **GetVertex::get_vertex**.
#[derive(Clone, Debug)]
pub struct RawVertices<M> {
    range: ops::Range<usize>,
    mesh: M,
}

/// An iterator yielding vertices in the order specified via the mesh's **Indices** channel.
///
/// Requires that the mesh implements **Indices** and **GetVertex**.
///
/// Returns `None` when a vertex has been yielded for every index in the **Indices** channel.
///
/// **Panics** if the **Indices** channel produces an index that is out of bounds of the mesh's
/// vertices.
#[derive(Clone, Debug)]
pub struct Vertices<M> {
    index_range: ops::Range<usize>,
    mesh: M,
}

/// An iterator yielding triangles in the order specified via the mesh's **Indices** channel.
///
/// Requires that the mesh implements **Indices** and **GetVertex**.
///
/// **Panics** if the **Indices** channel produces an index that is out of bounds of the mesh's
pub type Triangles<M> = geom::tri::IterFromVertices<Vertices<M>>;

/// An iterator yielding the raw vertices (with combined channels) of a mesh.
///
/// Requires that the inner mesh implements **GetVertex**.
///
/// Returns `None` when the inner mesh first returns `None` for a call to **GetVertex::get_vertex**.
pub fn raw_vertices<M>(mesh: M) -> RawVertices<M>
where
    M: Points,
{
    let len = raw_vertex_count(&mesh);
    let range = 0..len;
    RawVertices { range, mesh }
}

/// Produce an iterator yielding vertices in the order specified via the mesh's **Indices**
/// channel.
///
/// Requires that the mesh implements **Indices** and **GetVertex**.
///
/// Returns `None` when a vertex has been yielded for every index in the **Indices** channel.
///
/// **Panics** if the **Indices** channel produces an index that is out of bounds of the mesh's
/// vertices.
pub fn vertices<M, I>(mesh: M) -> Vertices<M>
where
    M: Indices<Index = I> + GetVertex<I>,
    I: TryFrom<usize>,
{
    let len = mesh.indices().channel().len();
    let index_range = 0..len;
    Vertices { index_range, mesh }
}

/// Produce an iterator yielding triangles for every three vertices yielded in the order specified
/// via the mesh's **Indices** channel.
///
/// Requires that the mesh implements **Indices** and **GetVertex**.
///
/// Returns `None` when there are no longer enough vertex indices to produce a triangle.
///
/// **Panics** if the **Indices** channel produces an index that is out of bounds of the mesh's
/// vertices.
pub fn triangles<M, I>(mesh: M) -> Triangles<M>
where
    M: Indices<Index = I> + GetVertex<I>,
    I: Copy + TryFrom<usize>,
{
    geom::tri::iter_from_vertices(vertices(mesh))
}

// The error message produced when the `Vertices` iterator panics due to an out of bound index.
const NO_VERTEX_FOR_INDEX: &str =
    "no vertex for the index produced by the mesh's indices channel";

impl<M> RawVertices<M> {
    /// Specify a range of raw vertices to yield.
    pub fn range(mut self, range: ops::Range<usize>) -> Self {
        self.range = range;
        self
    }
}

impl<M> Vertices<M>
where
    M: Indices,
{
    /// Specify the range of vertex indices to yield vertices from.
    pub fn index_range(mut self, range: ops::Range<usize>) -> Self {
        self.index_range = range;
        self
    }

    /// Convert this iterator yielding vertices into an iterator yielding triangles for every three
    /// vertices yielded.
    pub fn triangles<I>(self) -> Triangles<M>
    where
        M: Indices<Index = I> + GetVertex<I>,
        I: Copy,
    {
        geom::tri::iter_from_vertices(self)
    }
}

impl<M> Iterator for RawVertices<M>
where
    M: GetVertex<usize>,
{
    type Item = M::Vertex;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(vertex) = self.range.next().and_then(|i| self.mesh.get_vertex(i)) {
            return Some(vertex);
        }
        None
    }
}

impl<M, I> Iterator for Vertices<M>
where
    M: Indices<Index = I> + GetVertex<I>,
    I: Copy,
{
    type Item = M::Vertex;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.index_range.next() {
            if let Some(&index) = self.mesh.indices().channel().get(i) {
                let vertex = self.mesh.get_vertex(index).expect(NO_VERTEX_FOR_INDEX);
                return Some(vertex);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<M> ExactSizeIterator for RawVertices<M>
where
    M: GetVertex<usize>,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<M, I> DoubleEndedIterator for Vertices<M>
where
    M: Indices<Index = I> + GetVertex<I>,
    I: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.index_range.next_back() {
            if let Some(&index) = self.mesh.indices().channel().get(i) {
                let vertex = self.mesh.get_vertex(index).expect(NO_VERTEX_FOR_INDEX);
                return Some(vertex);
            }
        }
        None
    }
}

impl<M, I> ExactSizeIterator for Vertices<M>
where
    M: Indices<Index = I> + GetVertex<I>,
    I: Copy,
{
    fn len(&self) -> usize {
        let indices_len = self.mesh.indices().channel().len();
        let remaining_indices = indices_len - self.index_range.start;
        let range_len = self.index_range.len();
        cmp::min(remaining_indices, range_len)
    }
}
