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
use zorder::bmi2::{coord_of, index_of};

if zorder::bmi2::has_hardware_support() {
    let idx = unsafe { index_of([1u16, 1u16]) };
    assert_eq!(idx, 3u32);

    let coord = unsafe { coord_of(idx) };
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
| index_of       | 2         | 16                 | 2.00      |
|                |           | 32                 | 1.50      |
|                |           | 64                 | 1.32      |
|                |           | 128                | 6.34      |
|                | 3         | 32                 | 1.77      |
|                |           | 64                 | 2.23      |
|                |           | 128                | 6.42      |
| coord_of       | 2         | 16                 | 1.59      |
|                |           | 32                 | 1.54      |
|                |           | 64                 | 1.86      |
|                |           | 128                | 3.90      |
|                | 3         | 32                 | 1.93      |
|                |           | 64                 | 2.36      |
|                |           | 128                | 6.11      |
| bmi2::index_of | 2         | 16                 | 1.03      |
|                |           | 32                 | 0.935     |
|                |           | 64                 | 0.994     |
|                | 3         | 32                 | 1.07      |
|                |           | 64                 | 5.17      |
| bmi2::coord_of | 2         | 16                 | 0.947     |
|                |           | 32                 | 0.938     |
|                |           | 64                 | 1.13      |
|                | 3         | 32                 | 1.14      |
|                |           | 64                 | 1.14      |

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
