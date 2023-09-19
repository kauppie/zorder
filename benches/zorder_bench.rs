use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zorder::{
    array_u16_index_of, array_u8_index_of, bmi2, coord_of, coord_of_64, generic_array_index_of,
    index_of, index_of_64, index_of_64_dual_pass,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("index_of", |b| b.iter(|| index_of(black_box((2374, 8761)))));

    c.bench_function("array_u8_index_of", |b| {
        b.iter(|| array_u8_index_of(black_box([237u8, 87u8])))
    });

    c.bench_function("array_u16_index_of", |b| {
        b.iter(|| array_u16_index_of(black_box([2374u16, 8761u16])))
    });

    c.bench_function("generic_array_index_of", |b| {
        b.iter(|| generic_array_index_of(black_box([2374u16, 8761u16])))
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
