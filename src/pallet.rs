use super::carton::Carton;
use super::packerror::Mispack;
use super::prims::{Dims, Coords};
use uom::si::f64::{Length, Volume, Mass};
use uom::si::mass::kilogram;
use std::cmp::Ordering;

/// Represents a container of cartons.
#[derive(Debug, Clone)]
pub struct Pallet {
    pub dims: Dims,
    pub max_weight: Option<Mass>,
    items: Vec<Carton>,
}

impl Pallet {
    /// Get new empty pallet
    pub fn new(length: Length, width: Length, height: Length, mass: Option<Mass>) -> Self {
        Self {
            dims: Dims {
                length,
                width,
                height,
            },
            max_weight: mass,
            items: Vec::new(),
        }
    }

    /// Get new empty pallet from Dims
    pub fn from_dims(d: Dims) -> Self {
        Self {
            dims: d,
            max_weight: None,
            items: Vec::new(),
        }
    }

    /// Read-only access to the packed items.
    pub fn items(&self) -> &[Carton] {
        &self.items
    }

    /// Number of items currently on the pallet.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    /// Total volume occupied by the inventory.
    pub fn get_volume(&self) -> Volume {
        self.items.iter().map(|c| c.volume()).sum()
    }
    
    /// Total weight of the inventory.
    pub fn get_weight(&self) -> Mass {
        self.items.iter().map(|c| c.weight()).sum()
    }

    /// Get max weight were None is interpreted as inf.
    pub fn get_weight_limit(&self) -> Mass {
        self.max_weight.unwrap_or(Mass::new::<kilogram>(f64::INFINITY))
    }

    /// Checks whether a carton can physically fit inside this pallet.
    fn item_fits_any_orientation(&self, item: &Carton) -> bool {
        item.orientations().iter().any(|o| {
            o.dims.length <= self.dims.length
                && o.dims.width <= self.dims.width
                && o.dims.height <= self.dims.height
        })
    }

    /// Removes the last packed carton and returns it unplaced.
    pub fn pop(&mut self) -> Option<Carton> {
        self.items.pop().map(|mut item| {
            item.coords = None;
            item
        })
    }

    /// Generate candidate coordinates (extremal points) from existing items.
    /// These do not necessarily represent a space, it depends on the carton being placed.
    fn get_candidate_coords(&self) -> Vec<Coords> {
        // This ensures that at least one corner is provided: (0,0,0).
        let mut corners: Vec<Coords> = vec![Coords::default()];

        for item in &self.items {
            if let Some(c) = item.coords {
                let point = Coords {
                    x: c.x + item.dims.length,
                    y: c.y + item.dims.width,
                    z: c.z + item.dims.height
                };
                corners.push(point);
            }
        }

        Coords::cartesian_product(&corners)
    }

    /// Attempts to place a single item onto the pallet.
    ///
    /// Uses a greedy layer-based strategy: tries the lowest Z first,
    /// then the smallest Y, then the smallest X.
    ///
    /// # Errors
    /// - `Mispack::ItemTooLarge` if no rotation of the item fits the pallet dimensions.
    /// - `Mispack::DoesNotFit` if the item cannot fit in the remaining space.
    /// - `Mispack::ItemTooHeavy` if the item is heavier than the pallet weight limit entirely.
    /// - `Mispack::Overweight` if the item would push the pallet beyond it's weight limit.
    ///
    /// # Example
    /// ```rust
    /// # use palletize::prelude::*;
    /// # use uom::si::length::centimeter;
    /// 
    /// # fn cm(v: f64) -> Length {
    /// #     Length::new::<centimeter>(v)
    /// # }
    /// // Create a pallet
    /// let mut pallet = Pallet::new(cm(100.0), cm(100.0), cm(100.0), None);
    ///
    /// // Create a carton and pack it
    /// let pack_me = Carton::new(cm(40.0), cm(30.0), cm(20.0), None);
    /// assert!(pallet.add(pack_me).is_ok());
    ///
    /// // Oops! This carton is too big for this pallet.
    /// let big = Carton::new(cm(200.0), cm(200.0), cm(200.0), None);
    /// assert!(matches!(pallet.add(big), Err(Mispack::ItemTooLarge)));
    /// ```
    pub fn add(&mut self, item: Carton) -> Result<(), Mispack> {
        // 1. Check if item can ever fit this pallet (considering all rotations)
        if !self.item_fits_any_orientation(&item) {
            return Err(Mispack::ItemTooLarge);
        }

        if self.get_weight_limit() < item.weight() {
            return Err(Mispack::ItemTooHeavy);
        }
        
        if self.get_weight() + item.weight() > self.get_weight_limit() {
            return Err(Mispack::Overweight);
        }
        
        // 2. Get candidate coordinates to search for a gap
        let empty_xyz: Vec<Coords> = self.get_candidate_coords();

        // empty_xyz is order by z axis. First that works is best.
        for empty_space in &empty_xyz {
            // pick the orientation that minimizes height
            let best_placement: Option<Carton> = item
                .orientations()
                .into_iter()
                .filter(|oriented| oriented.dims.fits_within(empty_space, &self.dims))
                .filter(|oriented| {
                    let placed = oriented.place(*empty_space);
                    !self.items.iter().any(|i| i.intersects(&placed))
                })
                .min_by(|a, b| {
                    a.dims.height
                        .partial_cmp(&b.dims.height)
                        .unwrap_or(Ordering::Equal)
                });

            if let Some(shortest) = best_placement {
                self.items.push(shortest.place(*empty_space));                   
                return Ok(());
            }
        }
        
        // 4. No valid rotation or position was found
        Err(Mispack::DoesNotFit)
    }

    /// Shrinks the pallet dimensions to the minimum bounding box of its contents.
    /// Packed items are not moved or modified
    pub fn squash(&mut self) {
        if self.items.is_empty() {
            self.dims = Dims::default();
            return;
        }

        let mut max = Dims::default();

        for item in &self.items {
            if let Some(c) = item.coords {
                let furthest_vertex = Dims {
                    length: c.x + item.dims.length,
                    width: c.y + item.dims.width,
                    height: c.z + item.dims.height,
                };
                if furthest_vertex.length > max.length { max.length = furthest_vertex.length; }
                if furthest_vertex.width > max.width { max.width = furthest_vertex.width; }
                if furthest_vertex.height > max.height { max.height = furthest_vertex.height; }
            }
        }

        self.dims = max;
    }

    /// Squash, but chainable
    pub fn squashed(mut self) -> Self {
            self.squash();
            self
        }
}

/// Make a carton into an empty pallet of the same size.
/// Cartons coordinates are discarded.
/// Useful for recursive solutions since pallets have inventory.
impl From<Carton> for Pallet {
    fn from(carton: Carton) -> Self {
        Pallet {
            dims: carton.dims,
            max_weight: carton.mass,
            items: Vec::new(),
        }
    }
}
