# `zorder`

Crate provides functions to convert 2D coordinates to [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve) indexes and back. Z-order curve, also known as Morton code, is a mapping of 2D coordinates to 1D index which preverses locality. It is cache-efficient way of storing 2D data in 1D array.

## Example

Software implementation:

```rust
use zorder::{index_of, coord_of};

let idx = index_of((1, 1));
assert_eq!(idx, 3);

let coord = coord_of(idx);
assert_eq!(coord, (1, 1));
```

In most cases faster [`bmi2`](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set) based implementation is also available:

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

## Benchmarks

Benchmarked with [criterion](https://github.com/bheisler/criterion.rs) on AMD Ryzen 9 7950X on Fedora 37 VM. You can run the benchmarks yourself with `cargo bench`.

| Function         | Time    |
| ---------------- | ------- |
| `index_of`       | 1.58 ns |
| `coord_of`       | 945 ps  |
| `bmi2::index_of` | 943 ps  |
| `bmi2::coord_of` | 1.11 ns |

The `bmi2` version of `coord_of` is actually slower than the software implementation on this particular CPU. If you use both `index_of` and `coord_of` evenly, or you use `index_of` more often, then you should probably prefer the `bmi2` versions of the functions. Provided that your target architecture is `x86_64` and supports `bmi2` of course.

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
