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
fn packit_empty_input() {
    let mut items: Vec<Carton> = vec![];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);
    assert!(shipment.is_empty());
}

#[test]
fn packit_single_item() {
    let mut items = vec![Carton::new(cm(10.0), cm(10.0), cm(10.0), None)];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);
    assert_eq!(shipment.len(), 1);
    assert_eq!(shipment[0].item_count(), 1);
    assert!(items.is_empty());
}

#[test]
fn packit_oversized_gets_own_pallet() {
    let mut items = vec![Carton::new(cm(200.0), cm(200.0), cm(200.0), None)];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);
    assert!(shipment.len() >= 1);
    assert!(items.is_empty());
}

#[test]
fn packit_drains_input_vec() {
    let mut items = vec![
        Carton::new(cm(10.0), cm(10.0), cm(10.0), None),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), None),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), None),
    ];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    packit(&mut items, dims, None);
    assert!(items.is_empty());
}

#[test]
fn packit_multiple_pallets() {
    // 8 items that each take a quarter of the pallet floor
    let mut items: Vec<Carton> = (0..9)
        .map(|_| Carton::new(cm(50.0), cm(50.0), cm(50.0), None))
        .collect();
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);
    println!("shipment size {}", shipment.len());
    // 8 of these cubes fill the pallet, so you're gonna need at least 2
    assert!(shipment.len() >= 2);
    assert!(items.is_empty());
}

#[test]
fn packit_prefers_short_orientation() {
    let mut items = vec![Carton::new(cm(10.0), cm(20.0), cm(50.0), None)];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);

    let pallet = &shipment[0];
    pallet.items().iter().for_each(|item| {
        // The item should be placed with its shortest dimension as height
        assert_eq!(item.dims.height.get::<centimeter>(), 10.0);
    });
}

#[test]
fn packit_no_weight_limit_packs_normally() {
    let mut items = vec![
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(50.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(50.0))),
    ];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, None);
    assert_eq!(shipment.len(), 1);
    assert!(items.is_empty());
}

#[test]
fn packit_weight_limit_splits_across_pallets() {
    let mut items = vec![
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(8.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(8.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(8.0))),
    ];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, Some(kg(15.0)));
    assert!(shipment.len() >= 3);
    assert!(items.is_empty());
}

#[test]
fn packit_weight_limit_two_per_pallet() {
    let mut items = vec![
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(5.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(5.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(5.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(5.0))),
    ];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, Some(kg(10.0)));
    assert_eq!(shipment.len(), 2);
    assert!(items.is_empty());
}

#[test]
fn packit_oversized_weight_gets_own_pallet() {
    let mut items = vec![
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(5.0))),
        Carton::new(cm(10.0), cm(10.0), cm(10.0), Some(kg(999.0))),
    ];
    let dims = Dims { length: cm(100.0), width: cm(100.0), height: cm(100.0) };
    let shipment = packit(&mut items, dims, Some(kg(10.0)));
    assert!(shipment.len() >= 2);
    assert!(items.is_empty());
}
