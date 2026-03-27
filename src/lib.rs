//! # Palletize
//!
//! A boring, brute force, but simple library for the 3D bin packing problem.
//!
//! The bin packing problem is an np-hard optimization task where arbitrary items
//! are packed into finite containers. There are many approaches to this.
//! <https://en.wikipedia.org/wiki/Bin_packing_problem>
//! 
//! This lib offers a layer based approach, where the container is packed through
//! sequential 2D bin packs up the Z axis. This is an reasonable approximation
//! See the Readme for sources on the heuristics.
//! 
//! This lib focus's on sensible types and primitives for variatious use cases.
//! packit() acts as a quickstart, consuming a `Vec<Carton>` and returning `Vec<Pallet>`.

pub mod carton;
pub mod pallet;
pub mod prims;
pub mod packerror;
pub mod prelude;

use prelude::*;
use std::cmp::Ordering;

/// A helper function that feeds cartons into pallets of a given size.
/// Uses as many pallets as necessary to exhaust the Vector.
/// Pallets are returned by in MBB.
/// You can expect a heuristic of ~80%
pub fn packit(items: &mut Vec<Carton>, dims: Dims, max_weight: Option<Mass>) -> Vec<Pallet> {
    let mut shipment: Vec<Pallet> = Vec::new();

    while !items.is_empty() {
        // Sort tallest first (layer-based preference)
        // After sort, shortest is at the end — pop takes from the end
        items.sort_by(|a, b| {
            a.dims.height
                .partial_cmp(&b.dims.height)
                .unwrap_or(Ordering::Equal)
        });

        let mut pallet = Pallet::from_dims(dims);
        pallet.max_weight = max_weight;
        let mut leftover = Vec::new();

        while let Some(item) = items.pop() {
            match pallet.add(item) {
                Ok(_) => {
                    println!("Packed item {}", item);
                }
                Err(Mispack::DoesNotFit) | Err( Mispack::Overweight) => {
                    leftover.push(item);
                }
                Err(Mispack::ItemTooLarge) | Err(Mispack::ItemTooHeavy) => {
                    let mut solo = Pallet::from_dims(item.dims);
                    let _ = solo.add(item);
                    solo.squash();
                    shipment.push(solo);
                }
            }
        }

        // Pallet is full — shrink it and ship it
        pallet.squash();
        shipment.push(pallet);

        // Got left overs? No worries, we loop back for a new pallet.
        *items = leftover;
    }

    shipment
}
