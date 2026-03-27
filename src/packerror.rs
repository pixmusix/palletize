use thiserror::Error;

/// Mispack outlines all the ways a pallet can gracefully reject a box.
#[derive(Error, Debug)]
pub enum Mispack {
    #[error("Item dimensions exceed pallet dimensions")]
    ItemTooLarge,

    #[error("Item cannot be placed in the remaining pallet space")]
    DoesNotFit,
    
    #[error("Item cannot be placed without pushing pallet above weight limit")]
    Overweight,

    #[error("Item weight exceeds pallet's weight limit")]
    ItemTooHeavy,
}
