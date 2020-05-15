use crate::{
    algorithms::{Bounded, Closest, ClosestPoint, Translate},
    Arc, BoundingBox, Line, Point, Vector,
};
use specs::prelude::*;

// for rustdoc links
#[allow(unused_imports)]
use crate::components::Layer;

/// Something which can be drawn on the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawingObject {
    pub geometry: Geometry,
    /// The [`Layer`] this [`DrawingObject`] is attached to.
    pub layer: Entity,
}

impl Component for DrawingObject {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

/// The geometry of a [`DrawingObject`].
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Geometry {
    Line(Line),
    Arc(Arc),
    Point(Point),
}

impl ClosestPoint for Geometry {
    fn closest_point(&self, target: Point) -> Closest {
        match self {
            Geometry::Point(p) => p.closest_point(target),
            Geometry::Line(l) => l.closest_point(target),
            Geometry::Arc(a) => a.closest_point(target),
        }
    }
}

impl ClosestPoint for DrawingObject {
    fn closest_point(&self, target: Point) -> Closest {
        self.geometry.closest_point(target)
    }
}

impl Bounded for Geometry {
    fn bounding_box(&self) -> BoundingBox {
        match self {
            Geometry::Line(line) => line.bounding_box(),
            Geometry::Arc(arc) => arc.bounding_box(),
            Geometry::Point(point) => point.bounding_box(),
        }
    }
}

impl Translate for Geometry {
    fn translate(&mut self, displacement: Vector) {
        match self {
            Geometry::Point(ref mut point) => point.translate(displacement),
            Geometry::Line(ref mut line) => line.translate(displacement),
            Geometry::Arc(ref mut arc) => arc.translate(displacement),
        }
    }
}

impl Translate for DrawingObject {
    fn translate(&mut self, displacement: Vector) {
        self.geometry.translate(displacement);
    }
}
