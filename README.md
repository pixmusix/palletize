<img width="3780" height="1033" alt="Palletizebanner" src="https://github.com/user-attachments/assets/fa01ad95-ec99-4d81-8951-1b336b33ebeb" />

Simple bruteforce framework for 3D bin packing problem.
- Provides two types: Cartons to be packed, and Pallet which accepts cartons using a greedy, layer based algorithm.
- Implementation can be handled by the user. Alternatively a helper function is provided in prelude which should cover typical use cases.

Quick and dirty appoximations fast enough for real world application.

## Dependencies
- Built on [`uom`](https://crates.io/crates/uom) for type-safe lengths and masses.
- Impls Error with [thiserror](https://github.com/dtolnay/thiserror)

## Quick Start

```bash
cargo add palletize
```

```rust
// Example, pack two items where the weights don't matter.
let mut things_to_pack = vec![
    Carton::new(cm(40.0), cm(30.0), cm(20.0), None),
    Carton::new(cm(25.0), cm(25.0), cm(25.0), None),
];

let dim_of_bin = Dims {
    length: cm(100.0),
    width: cm(100.0),
    height: cm(100.0),
};

let shipment: Vec<Pallet> = packit(&amp;mut items, dims, None);
```
```rust
// Example, catching an overweight pallet.
let mut pallet = Pallet::from_dims(Dims {
     length: cm(100.0), width: cm(100.0), height: cm(100.0),
});
pallet.max_weight = Some(kg(20.0));

let heavy_box = Carton::new(cm(40.0), cm(30.0), cm(20.0), Some(kg(15.0)));
assert!(pallet.add(heavy_box).is_ok());

let another_box = Carton::new(cm(30.0), cm(30.0), cm(30.0), Some(kg(10.0)));
let result = pallet.add(another_box);
assert!(matches!(result, Err(Mispack::Overweight)));
```

## Further Reading
- [layer-based heuristic for 3D Bin Packing](https://pdf.sciencedirectassets.com/280416/1-s2.0-S1319157822X00104/1-s2.0-S1319157821001749/main.pdf?X-Amz-Security-Token=IQoJb3JpZ2luX2VjEAwaCXVzLWVhc3QtMSJHMEUCIErSdu8GRd%2BsGZJVFLeFyhp%2BTsoVHq9GCkgUFf9PWUxfAiEA2RRzZyRvcxsGZ61Spv3g47YdnsE%2BMvaDNUi%2Bocqu9HoquwUI1f%2F%2F%2F%2F%2F%2F%2F%2F%2F%2FARAFGgwwNTkwMDM1NDY4NjUiDMUSsSi67OYFaVjUvCqPBbuINjAZth%2BdngBF0IJ6Yf1IgOnUeKkunidDoNuiLgUmzWbZcO6tY7eiBB8fI%2FXFIZtZBDovvdphyDyN4mEiyQaKtZ%2FZTpl9ZZgAQ14B9AXZUt5vSfFdIF6ufeRr7lPIKaaManbJDLmnlGrE4%2FA12HpVbY23LwblDF0Rvx86OfMqXMaQBBaNQdda7ckEK2Grn%2B3rnJbhSqqPcOmW%2Fzk3L1Pu3JUrcFL4ZM1lRCvf36Rexkg8avthyBCMdkMG5FmbRNVqF49ycmoraHIF2NhsY1Hz48D%2BavAO60Lpmcw91LDdzh7v5vvmIbFeufadCNXQzxDzaDb9pD5zXkxg4ZcBvEGKl3WXDpxlPJvx2AZVd3XaazpB0KMUwIc5VPbqSzS4je%2FTg%2FoMG824%2FddBkdoSNJhlKWogAm%2BuGIMhgsSmnRbNh21gC44V8lwRmfhzXj8s%2FiofKlFkvGAtxmKhi8%2BRTm1JAAKzT3A9NFBnv%2BbEBNIPrD95O9b9u%2BB%2FsKLG66AaOworoaupDufZpBuAp4ESOHxroDsvW3GdgeNgDB6rLSbzvi7gjsvfZRVp%2FPnAE0dclvxrVpvJZqykem3I5bD8%2FnE12PCgo5jD1YP5osuSg4jQ%2Fd4CxePId5kWYrFHxUQ5i7yMK2TOeePTXhoh3WkZsKfyCfzTT%2FkhfOssb3BIs11dpigxDZKpns%2BgKZ2zMisyeSy7StSnLna7NSL1HA6lWppo3Pa9OZRlq4rAZ6LMl75pm1yTbMd9dyYXkA7PTw1W7XCItGJ3jhXxPxFElU45nvwpgpjn%2F6XlJuM6SOTkh2eH99WOAz83H9eFV0bplHKDUxCkSjg1hX3Epo1KRsOH0E9VYQ65NTyZnTcwHmiIztkw5oCYzgY6sQFJOCWKjl6SrNqIx73mMwo0274erxhTbqcypJvb920BFLMHSAszFvLs3ZsJHdMMM2b1Ri4Zo55M5y2E4NmOUCZasyAdV%2BZ48vOr47MZ8MOTV8nFWKLVmNJxxV5fgHmby7cY0EMJlVdl%2B8TEwWxFuj7JYzIOeN60CyIoe831BCxW%2F%2BGWyjizvupAD%2B2mLbPCTnZ3xU6vHrFslwFGPXSJDY8utSwnmsPHaJHufrWWhAI%2BIFk%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20260327T044407Z&X-Amz-SignedHeaders=host&X-Amz-Expires=300&X-Amz-Credential=ASIAQ3PHCVTY3YS2MQVL%2F20260327%2Fus-east-1%2Fs3%2Faws4_request&X-Amz-Signature=22cf393c0d6a76213790108274ce75ddf4a81012867cec210634494910230eb7&hash=59193bc25bd72dd9f4b19f526479a6e82ad8f2f0c57406d042dc916885d3f8e4&host=68042c943591013ac2b2430a89b270f6af2c76d8dfd086a07176afe7c76c2c61&pii=S1319157821001749&tid=spdf-644aef64-61b7-4859-bf12-c60275d0cf38&sid=c4b897655f3b1741623844c6db8cbccef781gxrqa&type=client&tsoh=d3d3LnNjaWVuY2VkaXJlY3QuY29t&rh=d3d3LnNjaWVuY2VkaXJlY3QuY29t&ua=1b115c0b540c03000b02&rr=9e2bbcef58771713&cc=au)
- [A fast optimization approach for a complex real-life 3D Multiple Bin Size Bin Packing Problem](https://www.sciencedirect.com/science/article/abs/pii/S0377221725003844)
- [2D Packing an Intro](https://arxiv.org/html/2508.13347v1)

## Alternatives
https://github.com/topics/bin-packing?o=desc&s=

