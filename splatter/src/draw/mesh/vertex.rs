use crate::color;
use crate::geom::{Point2, Point3, Vec3};
use crate::mesh::vertex::{WithColor, WithTexCoords};

pub type Point = Point3;
pub type Color = color::LinSrgba;
pub type TexCoords = Point2;
pub type Normal = Vec3;
pub type ColoredPoint = WithColor<Point, Color>;
pub type ColoredPoint2 = WithColor<Point2, Color>;

/// The vertex type produced by the **draw::Mesh**'s inner **MeshType**.
pub type Vertex = WithTexCoords<WithColor<Point, Color>, TexCoords>;

/// The number of channels in the color type.
pub const COLOR_CHANNEL_COUNT: usize = 4;

pub const DEFAULT_VERTEX_COLOR: Color = color::Alpha {
    color: color::rgb::Rgb {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        standard: std::marker::PhantomData,
    },
    alpha: 1.0,
};

/// Simplified constructor for a **draw::mesh::Vertex**.
pub fn new(point: Point, color: Color, tex_coords: TexCoords) -> Vertex {
    WithTexCoords {
        tex_coords,
        vertex: WithColor {
            color,
            vertex: point,
        },
    }
}

/// Default texture coordinates, for the case where a type is not textured.
pub fn default_tex_coords() -> TexCoords {
    [0.0; 2].into()
}

/// A type that converts an iterator yielding colored points to an iterator yielding **Vertex**s.
///
/// Default values are used for tex_coords.
#[derive(Clone, Debug)]
pub struct IterFromColoredPoints<I> {
    colored_points: I,
}

impl<I> IterFromColoredPoints<I> {
    /// Produce an iterator that converts an iterator yielding colored points to an iterator
    /// yielding **Vertex**s.
    ///
    /// The default value of `(0.0, 0.0)` is used for tex_coords.
    pub fn new<P>(colored_points: P) -> Self
    where
        P: IntoIterator<IntoIter = I, Item = WithColor<Point, Color>>,
        I: Iterator<Item = WithColor<Point, Color>>,
    {
        let colored_points = colored_points.into_iter();
        IterFromColoredPoints { colored_points }
    }
}

impl<I> Iterator for IterFromColoredPoints<I>
where
    I: Iterator<Item = WithColor<Point, Color>>,
{
    type Item = Vertex;
    fn next(&mut self) -> Option<Self::Item> {
        self.colored_points.next().map(|vertex| {
            let tex_coords = default_tex_coords();
            let vertex = WithTexCoords { tex_coords, vertex };
            vertex
        })
    }
}

/// A type that converts an iterator yielding points to an iterator yielding **Vertex**s.
///
/// The given `default_color` is used to color every vertex.
///
/// The default value of `(0.0, 0.0)` is used for tex_coords.
#[derive(Clone, Debug)]
pub struct IterFromPoints<I> {
    points: I,
    default_color: Color,
}

/// A type that converts an iterator yielding 2D points to an iterator yielding **Vertex**s.
///
/// The `z` position for each vertex will be `0.0`.
///
/// The given `default_color` is used to color every vertex.
///
/// The default value of `(0.0, 0.0)` is used for tex_coords.
#[derive(Clone, Debug)]
pub struct IterFromPoint2s<I> {
    points: I,
    default_color: Color,
}

impl<I> IterFromPoints<I> {
    /// Produce an iterator that converts an iterator yielding points to an iterator yielding
    /// **Vertex**s.
    ///
    /// The given `default_color` is used to color every vertex.
    ///
    /// The default value of `(0.0, 0.0)` is used for tex_coords.
    pub fn new<P>(points: P, default_color: Color) -> Self
    where
        P: IntoIterator<IntoIter = I, Item = Point>,
        I: Iterator<Item = Point3>,
    {
        let points = points.into_iter();
        IterFromPoints {
            points,
            default_color,
        }
    }
}

impl<I> IterFromPoint2s<I> {
    /// A type that converts an iterator yielding 2D points to an iterator yielding **Vertex**s.
    ///
    /// The `z` position for each vertex will be `0.0`.
    ///
    /// The given `default_color` is used to color every vertex.
    ///
    /// The default value of `(0.0, 0.0)` is used for tex_coords.
    pub fn new<P>(points: P, default_color: Color) -> Self
    where
        P: IntoIterator<IntoIter = I, Item = Point2>,
        I: Iterator<Item = Point2>,
    {
        let points = points.into_iter();
        IterFromPoint2s {
            points,
            default_color,
        }
    }
}

impl<I> Iterator for IterFromPoints<I>
where
    I: Iterator<Item = Point>,
{
    type Item = Vertex;
    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|vertex| {
            let color = self.default_color;
            let vertex = WithColor { vertex, color };
            let tex_coords = default_tex_coords();
            let vertex = WithTexCoords { vertex, tex_coords };
            vertex
        })
    }
}

impl<I> Iterator for IterFromPoint2s<I>
where
    I: Iterator<Item = Point2>,
{
    type Item = Vertex;
    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|p| {
            let vertex = p.extend(0.0);
            let color = self.default_color;
            let vertex = WithColor { vertex, color };
            let tex_coords = default_tex_coords();
            let vertex = WithTexCoords { vertex, tex_coords };
            vertex
        })
    }
}
