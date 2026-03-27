use palletize::prelude::*;
use uom::si::f64::Length;
use uom::si::length::meter;
use rand::prelude::*;

fn m(val: f64) -> Length {
    Length::new::<meter>(val)
}

fn printbar() {
    println!("{}", "-".repeat(50));
}

fn random_carton(rng: &mut impl Rng, min: f64, max: f64) -> Carton {
    Carton::from_dims(Dims {
        length: m(rng.random_range(min..max)),
        width: m(rng.random_range(min..max)),
        height: m(rng.random_range(min..max)),
    })
}

fn main() {
    let mut rng = rand::rng();

    // Define pallet dimensions (standard EUR pallet footprint)
    let pallet_dims = Dims {
        length: m(1.2),
        width: m(0.8),
        height: m(1.6),
    };

    // For this example, let's build some random cartons to pack.
    let mut items: Vec<Carton> = (0..30)
        .map(|_| random_carton(&mut rng, 0.1, 1.0))
        .collect();

    println!("Generated {} random cartons to pack.\n", items.len());
    println!("Pallet size: {:.2}m × {:.2}m × {:.2}m",
        pallet_dims.length.get::<meter>(),
        pallet_dims.width.get::<meter>(),
        pallet_dims.height.get::<meter>(),
    );
    printbar();

    // Let's put the cartons in the pallet.
    let shipment = packit(&mut items, pallet_dims, None);
    println!("\nShipment complete: {} pallets needed.\n", shipment.len());

    // Some feedback.
    for (i, pallet) in shipment.iter().enumerate() {
        let vol = pallet.get_volume().get::<uom::si::volume::cubic_meter>();
        let capacity = pallet.dims.length * pallet.dims.width * pallet.dims.height;
        let cap_val = capacity.get::<uom::si::volume::cubic_meter>();
        let utilization = if cap_val > 0.0 { vol / cap_val * 100.0 } else { 0.0 };

        println!(
            "Pallet {:>2}: {:>2} items | vol: {:.4} m³ | bounds: {:.2}×{:.2}×{:.2}m | utilization: {:.1}%",
            i + 1,
            pallet.item_count(),
            vol,
            pallet.dims.length.get::<meter>(),
            pallet.dims.width.get::<meter>(),
            pallet.dims.height.get::<meter>(),
            utilization,
        );
    }

    // Summary
    let total_items: usize = shipment.iter().map(|p| p.item_count()).sum();
    let total_vol: f64 = shipment
        .iter()
        .map(|p| p.get_volume().get::<uom::si::volume::cubic_meter>())
        .sum();

    println!("{}", "-".repeat(50));
    println!("Total: {} items across {} pallets, {:.4} m³ packed.",
        total_items,
        shipment.len(),
        total_vol,
    );
    println!("Items left to pack: {}", items.len());
}
