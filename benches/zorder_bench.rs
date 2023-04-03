use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zorder::{bmi2, coord_of, index_of};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("index_of", |b| b.iter(|| index_of(black_box((2374, 8761)))));

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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
