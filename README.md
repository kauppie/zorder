# `zorder`

Crate provides functions to convert 2D coordinates to [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve) indexes and back. Z-order curve, also known as Morton code, is a mapping of 2D coordinates to 1D index which preverses locality. It is cache-efficient way of storing 2D data in 1D array.

## Example

```rust
use zorder::{index_of, coord_of};

let idx = index_of((1, 1));
assert_eq!(idx, 3);

let coord = coord_of(idx);
assert_eq!(coord, (1, 1));
```

bmi2 instruction set accelerated version is also available:

```rust
#[cfg(target_arch = "x86_64")]
{
    use zorder::bmi2::{index_of, coord_of};

    if is_x86_feature_detected!("bmi2") {
        let idx = unsafe { index_of((1, 1)) };
        assert_eq!(idx, 3);

        let coord = unsafe { coord_of(idx) };
        assert_eq!(coord, (1, 1));
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
