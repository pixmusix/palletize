use palletize::prelude::*;
use uom::si::length::centimeter;
use uom::si::mass::kilogram;

fn cm(val: f64) -> Length {
    Length::new::<centimeter>(val)
}

fn kg(val: f64) -> Mass {
    Mass::new::<kilogram>(val)
}

#[test]
fn carton_new_is_unplaced() {
    let c = Carton::new(cm(10.0), cm(20.0), cm(30.0), None);
    assert!(c.coords.is_none());
}

#[test]
fn carton_place_sets_coords() {
    let c = Carton::new(cm(10.0), cm(20.0), cm(30.0), None);
    let placed = c.place(Coords::default());
    assert!(placed.coords.is_some());
    assert_eq!(placed.coords.unwrap(), Coords::default());
}

#[test]
fn carton_orientations_cube_has_one() {
    let cube = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    assert_eq!(cube.orientations().len(), 1);
}

#[test]
fn carton_orientations_all_different_has_six() {
    let c = Carton::new(cm(10.0), cm(20.0), cm(30.0), None);
    assert_eq!(c.orientations().len(), 6);
}

#[test]
fn carton_orientations_two_same_has_three() {
    let c = Carton::new(cm(10.0), cm(10.0), cm(30.0), None);
    assert_eq!(c.orientations().len(), 3);
}

#[test]
fn carton_intersects_overlap() {
    let a = Carton::new(cm(10.0), cm(10.0), cm(10.0), None).place(Coords::default());
    let b = Carton::new(cm(10.0), cm(10.0), cm(10.0), None).place(Coords::default());
    assert!(a.intersects(&b));
}

#[test]
fn carton_no_intersect_side_by_side() {
    let a = Carton::new(cm(10.0), cm(10.0), cm(10.0), None).place(Coords::default());
    let b = Carton::new(cm(10.0), cm(10.0), cm(10.0), None)
        .place(Coords { x: cm(10.0), y: cm(0.0), z: cm(0.0) });
    assert!(!a.intersects(&b));
}

#[test]
fn carton_no_intersect_touching_edge() {
    let a = Carton::new(cm(5.0), cm(5.0), cm(5.0), None).place(Coords::default());
    let b = Carton::new(cm(5.0), cm(5.0), cm(5.0), None)
        .place(Coords { x: cm(5.0), y: cm(0.0), z: cm(0.0) });
    assert!(!a.intersects(&b));
}

#[test]
fn carton_display_unplaced() {
    let c = Carton::new(cm(40.0), cm(30.0), cm(20.0), None);
    let s = format!("{c}");
    assert!(s.contains("40"));
    assert!(s.contains("unplaced"));
}

#[test]
fn carton_display_placed() {
    let c = Carton::new(cm(40.0), cm(30.0), cm(20.0), None).place(Coords::default());
    let s = format!("{c}");
    assert!(s.contains("40"));
    assert!(!s.contains("unplaced"));
}

#[test]
fn carton_without_mass_is_none() {
    let dims = Dims::new(cm(10.0), cm(10.0), cm(10.0));
    let c = Carton::from_dims(dims);
    assert!(c.mass.is_none());
}

#[test]
fn carton_with_mass_is_some() {
    let c = Carton::new(
        cm(10.0),
        cm(10.0),
        cm(10.0),
        Some(kg(5.0))
    );
    assert_eq!(c.mass, Some(kg(5.0)));
}
