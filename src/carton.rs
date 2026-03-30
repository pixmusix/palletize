use super::pallet::Pallet;
use super::prims::{Coords, Dims};
use std::cmp::Ordering;
use uom::si::f64::{Length, Volume, Mass};
use std::fmt;
use uom::si::length::centimeter;

/// A box/container to be packed.
/// Some(coords) signals that the box is placed in a pallet.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Carton {
    pub dims: Dims,
    pub mass: Option<Mass>,
    pub coords: Option<Coords>,
}

impl Carton {
    /// Creates unplaced carton.
    pub fn new(length: Length, width: Length, height: Length, mass: Option<Mass>) -> Self {
        Self {
            dims: Dims { length, width, height },
            mass: mass,
            coords: None,
        }
    }

    /// Unplaced carton from dimensions
    pub fn from_dims(d:Dims) -> Self {
        Self {
            dims: d,
            mass: None,
            coords: None,
        }
    }
    
    /// Assigns coordinates.
    pub fn place(mut self, c: Coords) -> Self {
        self.coords = Some(c);
        self
    }

    /// Calculates the volume.
    pub fn volume(&self) -> Volume {
        self.dims.volume()
    }
    
    /// Get weight where None maps to 0.
    pub fn weight(&self) -> Mass {
        match self.mass {
            Some(m) => m,
            None => Mass::default(),
        }
    }

    /// Retreive copies of this carton in all othogonal rotations
    pub fn orientations(&self) -> Vec<Self> {
        let l: Length = self.dims.length;
        let w: Length = self.dims.width;
        let h: Length = self.dims.height;
        let m: Option<Mass> = self.mass;

        let mut perms = vec![
            (l, w, h, m), (l, h, w, m), (w, l, h, m),
            (w, h, l, m), (h, l, w, m), (h, w, l, m),
        ];

        perms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        perms.dedup();

        perms
            .into_iter()
            .map(|(len, wid, hei, mas)| Self {
                dims: Dims {
                    length: len,
                    width: wid,
                    height: hei,
                },
                mass: mas,
                coords: None,
            })
            .collect()
    }

    /// AABB collision detection between this carton and another carton.
    /// Axis aligned bounding box is a brute force detector for overlapping orthogonal edges.
    /// If either carton is unplaced it returns false
    pub fn intersects(&self, other: &Carton) -> bool {
        if let (Some(c1), Some(c2)) = (self.coords, other.coords) {
            !(
                c1.x + self.dims.length <= c2.x     // is left of
                || c2.x + other.dims.length <= c1.x // is right of
                || c1.y + self.dims.width <= c2.y   // is front of
                || c2.y + other.dims.width <= c1.y  // is behind
                || c1.z + self.dims.height <= c2.z  // is below
                || c2.z + other.dims.height <= c1.z // is above
            )
        } else {
            false // abstractly, I suppose a box with no coordinates can't really intersect
        }
    }
}

impl From<Pallet> for Carton {
    /// Flattens a Pallet into a single unplaced Carton.
    /// Cartons do not contain items, so the Pallets inventory is discarded
    fn from(pallet: Pallet) -> Self {
        Carton {
            dims: pallet.dims,
            mass: Some(pallet.get_weight()),
            coords: None,
        }
    }
}

impl fmt::Display for Carton {
    /// Returns dims and coords as centimeters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.dims.length.get::<centimeter>() as i64;
        let w = self.dims.width.get::<centimeter>() as i64;
        let h = self.dims.height.get::<centimeter>() as i64;

        match self.coords {
            Some(c) => write!(
                f,
                "Carton({}×{}×{}cm) @ ({}, {}, {})cm",
                l, w, h,
                c.x.get::<centimeter>() as i64,
                c.y.get::<centimeter>() as i64,
                c.z.get::<centimeter>() as i64,
            ),
            None => write!(f, "Carton({}×{}×{}cm) unplaced", l, w, h),
        }
    }
}
