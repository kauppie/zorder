use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zorder::bmi2;

fn bench_bmi2(c: &mut Criterion) {
    if let Some(support_token) = bmi2::HardwareSupportToken::new() {
        c.bench_function("bmi2::array_index_of_u32", |b| {
            b.iter(|| bmi2::index_of(black_box([2312347u32, 76971888u32]), support_token))
        });

        c.bench_function("bmi2::array_coord_of_u32", |b| {
            b.iter(|| bmi2::coord_of::<_, 2>(black_box(231234776971888u64), support_token))
        });

        c.bench_function("bmi2::array_index_of_u16", |b| {
            b.iter(|| bmi2::index_of(black_box([2374u16, 8761u16]), support_token))
        });

        c.bench_function("bmi2::array_coord_of_u16", |b| {
            b.iter(|| bmi2::coord_of::<_, 2>(black_box(23748761u32), support_token))
        });

        c.bench_function("bmi2::array_index_of_u16_dim3", |b| {
            b.iter(|| bmi2::index_of(black_box([23123u16, 1888u16, 11237u16]), support_token))
        });

        c.bench_function("bmi2::array_coord_of_u16_dim3", |b| {
            b.iter(|| bmi2::coord_of::<_, 3>(black_box(23123188811237u64), support_token))
        });

        c.bench_function("bmi2::array_index_of_u8", |b| {
            b.iter(|| bmi2::index_of(black_box([237u8, 76u8]), support_token))
        });

        c.bench_function("bmi2::array_coord_of_u8", |b| {
            b.iter(|| bmi2::coord_of::<_, 2>(black_box(23776u16), support_token))
        });

        c.bench_function("bmi2::array_index_of_u8_dim3", |b| {
            b.iter(|| bmi2::index_of(black_box([23u8, 18u8, 112u8]), support_token))
        });

        c.bench_function("bmi2::array_coord_of_u8_dim3", |b| {
            b.iter(|| bmi2::coord_of::<_, 3>(black_box(2318112u32), support_token))
        });
    } else {
        panic!("failed to benchmark: bmi2 feature is not detected");
    }
}

criterion_group!(benches, bench_bmi2);
criterion_main!(benches);
