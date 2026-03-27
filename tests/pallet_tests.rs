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
fn pallet_add_single_item() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(100.0), width: cm(100.0), height: cm(100.0),
    });
    let item = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    assert!(p.add(item).is_ok());
    assert_eq!(p.item_count(), 1);
}

#[test]
fn pallet_add_item_too_large() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(10.0), width: cm(10.0), height: cm(10.0),
    });
    let item = Carton::new(cm(20.0), cm(20.0), cm(20.0), None);
    let err = p.add(item).unwrap_err();
    assert!(matches!(err, Mispack::ItemTooLarge));
}

#[test]
fn pallet_add_item_too_large_considers_rotation() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(30.0), width: cm(10.0), height: cm(10.0),
    });
    // 5×5×25 can be rotated to fit 25×5×5
    let item = Carton::new(cm(5.0), cm(5.0), cm(25.0), None);
    assert!(p.add(item).is_ok());
}

#[test]
fn pallet_add_does_not_fit() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(10.0), width: cm(10.0), height: cm(10.0),
    });
    let item = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    assert!(p.add(item).is_ok());

    // Pallet is now full
    let item2 = Carton::new(cm(1.0), cm(1.0), cm(1.0), None);
    let err = p.add(item2).unwrap_err();
    assert!(matches!(err, Mispack::DoesNotFit));
}

#[test]
fn pallet_fill_exactly() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(20.0), width: cm(10.0), height: cm(10.0),
    });
    let a = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    let b = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    assert!(p.add(a).is_ok());
    assert!(p.add(b).is_ok());
    assert_eq!(p.item_count(), 2);
}

#[test]
fn pallet_squash_shrinks_to_contents_exact() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(100.0), width: cm(100.0), height: cm(100.0),
    });
    let item = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    p.add(item).unwrap();
    p.squash();

    assert_eq!(p.dims.length.get::<centimeter>(), 10.0);
    assert_eq!(p.dims.width.get::<centimeter>(), 10.0);
    assert_eq!(p.dims.height.get::<centimeter>(), 10.0);
}

#[test]
fn pallet_squash_shrinks_to_contents_volume() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(100.0), width: cm(100.0), height: cm(100.0),
    });
    let item = Carton::new(cm(15.0), cm(20.0), cm(37.0), None);
    p.add(item).unwrap();
    p.squash();

    assert_eq!(p.get_volume(), cm(15.0) * cm(20.0) * cm(37.0));
}

#[test]
fn pallet_squash_empty_is_zero() {
    let p = Pallet::from_dims(Dims {
        length: cm(100.0), width: cm(100.0), height: cm(100.0),
    }).squashed();
    assert_eq!(p.dims, Dims::default());
}

#[test]
fn pallet_no_weight_limit_accepts_any_mass() {
    let mut p = Pallet::from_dims(Dims {
        length: cm(100.0), width: cm(100.0), height: cm(100.0),
    });

    for _ in 0..100 {
        let mass = rand::random_range(0.0..f64::MAX);
        let item = Carton::new(cm(1.0), cm(1.0), cm(1.0), Some(kg(mass)));
        // No weight limit — every mass must be accepted
        assert!(
            p.add(item).is_ok(),
            "Failed to add item with mass {mass} kg to unlimited pallet"
        );
    }
}

#[test]
fn pallet_weight_limit_rejects_single_heavy_item() {
    let mut p = Pallet::new(
        cm(100.0), cm(100.0), cm(100.0), Some(kg(5.0))
    );

    let item = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(100.0)));
    let err = p.add(item).unwrap_err();
    assert!(matches!(err, Mispack::ItemTooHeavy));
}

#[test]
fn pallet_weight_limit_cumulative_overflow() {
    let mut p = Pallet::new(
        cm(100.0), cm(100.0), cm(100.0), Some(kg(15.0))
    );

    let a = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(8.0)));
    let b = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(8.0)));

    assert!(p.add(a).is_ok()); // 8 kg — fine
    let err = p.add(b).unwrap_err(); // 8 + 8 = 16 > 15
    assert!(matches!(err, Mispack::Overweight));
}

#[test]
fn pallet_weight_limit_exact_capacity() {
    let mut p = Pallet::new(
        cm(100.0), cm(100.0), cm(100.0), Some(kg(20.0))
    );

    let a = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(10.0)));
    let b = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(10.0)));

    assert!(p.add(a).is_ok()); // 10 kg
    assert!(p.add(b).is_ok()); // 10 + 10 = 20 — exactly at limit
}

#[test]
fn pallet_weight_limit_item_without_mass_allowed() {
    let mut p = Pallet::new(
        cm(100.0), cm(100.0), cm(100.0), Some(kg(10.0))
    );

    // Item has no mass — treated as zero, should be allowed
    let item = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);
    assert!(p.add(item).is_ok());
}

#[test]
fn pallet_mixed_mass_and_no_mass_items() {
    let mut p = Pallet::new(
        cm(100.0), cm(100.0), cm(100.0), Some(kg(20.0))
    );

    let weighted = Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(15.0)));
    let weightless = Carton::new(cm(10.0), cm(10.0), cm(10.0), None);

    assert!(p.add(weighted).is_ok());  // 15 kg
    assert!(p.add(weightless).is_ok()); // 15 + 0 = 15 — fine
}
