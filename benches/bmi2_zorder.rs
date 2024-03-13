use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zorder::bmi2;

#[cfg(target_arch = "x86_64")]
fn bench_bmi2(c: &mut Criterion) {
    use core::panic;

    if is_x86_feature_detected!("bmi2") {
        c.bench_function("bmi2::array_index_of_u32", |b| {
            b.iter(|| unsafe { bmi2::index_of(black_box([2312347u32, 76971888u32])) })
        });

        c.bench_function("bmi2::array_coord_of_u32", |b| {
            b.iter(|| unsafe { bmi2::coord_of::<_, 2>(black_box(231234776971888u64)) })
        });

        c.bench_function("bmi2::array_index_of_u16", |b| {
            b.iter(|| unsafe { bmi2::index_of(black_box([2374u16, 8761u16])) })
        });

        c.bench_function("bmi2::array_coord_of_u16", |b| {
            b.iter(|| unsafe { bmi2::coord_of::<_, 2>(black_box(23748761u32)) })
        });

        c.bench_function("bmi2::array_index_of_u16_dim3", |b| {
            b.iter(|| unsafe { bmi2::index_of(black_box([23123u16, 1888u16, 11237u16])) })
        });

        c.bench_function("bmi2::array_coord_of_u16_dim3", |b| {
            b.iter(|| unsafe { bmi2::coord_of::<_, 3>(black_box(23123188811237u64)) })
        });

        c.bench_function("bmi2::array_index_of_u8", |b| {
            b.iter(|| unsafe { bmi2::index_of(black_box([237u8, 76u8])) })
        });

        c.bench_function("bmi2::array_coord_of_u8", |b| {
            b.iter(|| unsafe { bmi2::coord_of::<_, 2>(black_box(23776u16)) })
        });
    } else {
        panic!("failed to benchmark: bmi2 feature is not detected");
    }
}

criterion_group!(benches, bench_bmi2);
criterion_main!(benches);
