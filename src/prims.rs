use uom::si::f64::{Length, Volume};
use std::cmp::Ordering;

/// 3D Dimensions with arbitrary units.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Dims {
    pub length: Length,
    pub width: Length,
    pub height: Length,
}

impl Dims {
    /// Get 3D Dimensions
    pub fn new(len: Length, wid: Length, hei: Length) -> Self {
        Dims {
            length: len,
            width: wid,
            height: hei,
        }
    }
    
    /// Checks whether this item, placed at the given coordinate,
    /// is within the given boundaries.
    pub fn fits_within(&self, at: &Coords, bounds: &Dims) -> bool {
        at.x + self.length <= bounds.length
            && at.y + self.width <= bounds.width
            && at.z + self.height <= bounds.height
    }

    /// Get the 3d volume of arbitrary units
    pub fn volume(&self) -> Volume {
        self.length * self.width * self.height
    }
}

/// A 3D vector
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coords {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

impl Coords {
    /// Takes a set of corner points and returns the Cartesian product
    /// of unique x, y, & z.
    ///
    /// # Example
    ///   Input corners:
    ///   1. (0, 0, 0)
    ///   2. (10, 20, 30)
    ///
    ///   Unique corners per axis:
    ///   xs: {0, 10}
    ///   ys: {0, 20}
    ///   zs: {0, 30}
    ///
    ///   Cartesian product (2 × 2 × 2 = 8):
    ///   - (0,  0,  0)
    ///   - (10, 0,  0)
    ///   - (0,  20, 0)
    ///   - (10, 20, 0)
    ///   - (0,  0,  30)
    ///   - (10, 0,  30)
    ///   - (0,  20, 30)
    ///   - (10, 20, 30)
    pub fn cartesian_product(corners: &[Coords]) -> Vec<Coords> {
        let mut xs: Vec<Length> = corners.iter().map(|c| c.x).collect();
        let mut ys: Vec<Length> = corners.iter().map(|c| c.y).collect();
        let mut zs: Vec<Length> = corners.iter().map(|c| c.z).collect();

        xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        zs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        xs.dedup();
        ys.dedup();
        zs.dedup();

        let mut result:Vec<Coords> = Vec::with_capacity(xs.len() * ys.len() * zs.len());
        for &z in &zs {
            for &y in &ys {
                for &x in &xs {
                    result.push(Coords { x, y, z });
                }
            }
        }
        result
    }
}
