use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zorder::{
    array_coord_of, array_index_of, bmi2, coord_of, coord_of_64, coord_of_dual_pass, index_of,
    index_of_64, index_of_64_dual_pass,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("coord_of_dual_pass", |b| {
        b.iter(|| coord_of_dual_pass(black_box(23748761)))
    });

    c.bench_function("index_of", |b| b.iter(|| index_of(black_box((2374, 8761)))));

    c.bench_function("array_index_of_u64", |b| {
        b.iter(|| array_index_of(black_box([2765132312347u64, 769718192876348788u64])))
    });

    c.bench_function("array_coord_of_u64", |b| {
        b.iter(|| array_coord_of::<_, 2>(black_box(2765132312347769718192876348788u128)))
    });

    c.bench_function("array_index_of_u32", |b| {
        b.iter(|| array_index_of(black_box([2312347u32, 76971888u32])))
    });

    c.bench_function("array_coord_of_u32", |b| {
        b.iter(|| array_coord_of::<_, 2>(black_box(231234776971888u64)))
    });

    c.bench_function("array_index_of_u32_dim3", |b| {
        b.iter(|| array_index_of(black_box([2312347u32, 76971888u32, 411237u32])))
    });

    c.bench_function("array_coord_of_u32_dim3", |b| {
        b.iter(|| array_coord_of::<_, 3>(black_box(231234776971888411237u128)))
    });

    c.bench_function("array_index_of_u16", |b| {
        b.iter(|| array_index_of(black_box([2374u16, 8761u16])))
    });

    c.bench_function("array_coord_of_u16", |b| {
        b.iter(|| array_coord_of::<_, 2>(black_box(23748761u32)))
    });

    c.bench_function("array_index_of_u16_dim3", |b| {
        b.iter(|| array_index_of(black_box([23123u16, 1888u16, 11237u16])))
    });

    c.bench_function("array_coord_of_u16_dim3", |b| {
        b.iter(|| array_coord_of::<_, 3>(black_box(23123188811237u64)))
    });

    c.bench_function("array_index_of_u8", |b| {
        b.iter(|| array_index_of(black_box([237u8, 76u8])))
    });

    c.bench_function("array_coord_of_u8", |b| {
        b.iter(|| array_coord_of::<_, 2>(black_box(23776u16)))
    });

    c.bench_function("coord_of", |b| b.iter(|| coord_of(black_box(23748761))));

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            c.bench_function("bmi2::index_of", |b| {
                b.iter(|| unsafe { bmi2::index_of(black_box((2374, 8761))) })
            });

            c.bench_function("bmi2::coord_of", |b| {
                b.iter(|| unsafe { bmi2::coord_of(black_box(23748761)) })
            });
        }
    }

    c.bench_function("index_of_64", |b| {
        b.iter(|| index_of_64(black_box((23744732, 87611678))))
    });

    c.bench_function("index_of_64_dual_pass", |b| {
        b.iter(|| index_of_64_dual_pass(black_box((23744732, 87611678))))
    });

    c.bench_function("coord_of_64", |b| {
        b.iter(|| coord_of_64(black_box(2374473287611678)))
    });

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            c.bench_function("bmi2::index_of_64", |b| {
                b.iter(|| unsafe { bmi2::index_of_64(black_box((23744732, 87611678))) })
            });

            c.bench_function("bmi2::coord_of_64", |b| {
                b.iter(|| unsafe { bmi2::coord_of_64(black_box(2374473287611678)) })
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
