use palletize::prelude::*;
use uom::si::length::centimeter;

fn cm(val: f64) -> Length {
    Length::new::<centimeter>(val)
}

#[test]
fn dims_default_is_zero() {
    let d = Dims::default();
    assert_eq!(d.length, Length::ZERO);
    assert_eq!(d.width, Length::ZERO);
    assert_eq!(d.height, Length::ZERO);
}

#[test]
fn dims_volume() {
    let d = Dims { length: cm(10.0), width: cm(20.0), height: cm(30.0) };
    let expected = cm(10.0) * cm(20.0) * cm(30.0);
    assert_eq!(d.volume(), expected);
}

#[test]
fn dims_fits_within_at_origin() {
    let item = Dims { length: cm(5.0), width: cm(5.0), height: cm(5.0) };
    let bounds = Dims { length: cm(10.0), width: cm(10.0), height: cm(10.0) };
    let origin = Coords::default();
    assert!(item.fits_within(&origin, &bounds));
}

#[test]
fn dims_does_not_fit_within() {
    let item = Dims { length: cm(11.0), width: cm(5.0), height: cm(5.0) };
    let bounds = Dims { length: cm(10.0), width: cm(10.0), height: cm(10.0) };
    let origin = Coords::default();
    assert!(!item.fits_within(&origin, &bounds));
}

#[test]
fn dims_fits_exactly() {
    let item = Dims { length: cm(10.0), width: cm(10.0), height: cm(10.0) };
    let bounds = item;
    let origin = Coords::default();
    assert!(item.fits_within(&origin, &bounds));
}

