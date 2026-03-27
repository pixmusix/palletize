use palletize::prelude::*;
use uom::si::length::centimeter;

fn cm(val: f64) -> Length {
    Length::new::<centimeter>(val)
}

#[test]
fn coords_origin_is_zero() {
    let o = Coords::default();
    assert_eq!(o.x, Length::ZERO);
    assert_eq!(o.y, Length::ZERO);
    assert_eq!(o.z, Length::ZERO);
}

#[test]
fn cartesian_product_single_point() {
    let corners = vec![Coords::default()];
    let result = Coords::cartesian_product(&corners);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Coords::default());
}

#[test]
fn cartesian_product_two_points() {
    let corners = vec![
        Coords::default(),
        Coords { x: cm(10.0), y: cm(20.0), z: cm(30.0) },
    ];
    let result = Coords::cartesian_product(&corners);
    // 2 x-values × 2 y-values × 2 z-values = 8
    assert_eq!(result.len(), 8);
}

#[test]
fn cartesian_product_deduplicates() {
    let corners = vec![
        Coords { x: cm(10.0), y: cm(20.0), z: cm(30.0) },
        Coords { x: cm(10.0), y: cm(20.0), z: cm(30.0) },
    ];
    let result = Coords::cartesian_product(&corners);
    assert_eq!(result.len(), 1);
}

#[test]
fn cartesian_product_empty_input() {
    let corners: Vec<Coords> = vec![];
    let result = Coords::cartesian_product(&corners);
    assert!(result.is_empty());
}
