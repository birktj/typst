use super::*;
use crate::eval::dict;

/// A bounding box
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self { min: p1.min(p2), max: p1.max(p2) }
    }

    pub fn extend(&self, other: &BoundingBox) -> BoundingBox {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

cast_to_value! {
    v: BoundingBox => Value::Dict(dict! {
        "min" => Axes::from(v.min),
        "max" => Axes::from(v.max),
    })
}
