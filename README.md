# `zorder` / curve index conversions

This crate provides functions to convert N-dimensional[^1] coordinates to [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve) indexes and back. Z-order curve, also known as Morton code, is a mapping of N-dimensional coordinates to 1D index which preverses locality. It is cache-efficient way of storing N-dimensional data in 1D array.

[^1]: Maximum number of dimensions is limited by the largest unsigned integer type, `u128`, which is able to store 16 8-bit coordinates. `bmi2` based approach is limited to `u64`.

## Examples

### Software implementation

```rust
use zorder::{index_of, coord_of};

let idx = index_of([1u16, 1u16]);
assert_eq!(idx, 3u32);

let coord = coord_of(idx);
assert_eq!(coord, [1u16, 1u16]);
```

### [`bmi2`](https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set) implementation

This should be faster but requires x86 specific instruction set support.

```rust
use zorder::bmi2::{coord_of, coord_of_unchecked, HardwareSupportToken, index_of, index_of_unchecked};

// Safe interface with hardware support token.
let support_token = HardwareSupportToken::new();
if let Some(support_token) = support_token {
  let idx = index_of([1u16, 1u16], support_token);
    assert_eq!(idx, 3u32);

    let coord = coord_of(idx, support_token);
    assert_eq!(coord, [1u16, 1u16]);
}

// Unsafe interface with hardware support check.
// Only works on x86_64 CPUs.
if zorder::bmi2::has_hardware_support() {
    let idx = unsafe { index_of_unchecked([1u16, 1u16]) };
    assert_eq!(idx, 3u32);

    let coord = unsafe { coord_of_unchecked(idx) };
    assert_eq!(coord, [1u16, 1u16]);
}
```

You can validate that your CPU supports `bmi2` with the provided example:

```sh
$ cargo run --example bmi2_support
```

## Benchmarks

Benchmarked with [criterion](https://github.com/bheisler/criterion.rs) using AMD Ryzen 9 7950X on WSL2. Standard `release` profile was used. All results are rounded up to three significant figures.

You can run `cargo bench` to see the results on your machine.

| Function       | Dimension | Index width (bits) | Time (ns) |
| -------------- | --------- | ------------------ | --------- |
| index_of       | 2         | 16  (2 x 8)        | 2.00      |
|                |           | 32  (2 x 16)       | 1.50      |
|                |           | 64  (2 x 32)       | 1.32      |
|                |           | 128 (2 x 64)       | 6.34      |
|                | 3         | 32  (3 x 8)        | 1.77      |
|                |           | 64  (3 x 16)       | 2.23      |
|                |           | 128 (3 x 32)       | 6.42      |
| coord_of       | 2         | 16  (2 x 8)        | 1.59      |
|                |           | 32  (2 x 16)       | 1.54      |
|                |           | 64  (2 x 32)       | 1.86      |
|                |           | 128 (2 x 64)       | 3.90      |
|                | 3         | 32  (3 x 8)        | 1.93      |
|                |           | 64  (3 x 16)       | 2.36      |
|                |           | 128 (3 x 32)       | 6.11      |
| bmi2::index_of | 2         | 16  (2 x 8)        | 1.03      |
|                |           | 32  (2 x 16)       | 0.935     |
|                |           | 64  (2 x 32)       | 0.994     |
|                | 3         | 32  (3 x 8)        | 1.07      |
|                |           | 64  (3 x 16)       | 5.17      |
| bmi2::coord_of | 2         | 16  (2 x 8)        | 0.947     |
|                |           | 32  (2 x 16)       | 0.938     |
|                |           | 64  (2 x 32)       | 1.13      |
|                | 3         | 32  (3 x 8)        | 1.14      |
|                |           | 64  (3 x 16)       | 1.14      |

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
