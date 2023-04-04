use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use zorder::{bmi2, coord_of, index_of};

trait IndexType {
    fn index_of(&self, xy: (u16, u16)) -> u32;
    fn coord_of(&self, idx: u32) -> (u16, u16);
}

struct Linear {
    width: u32,
}

impl IndexType for Linear {
    #[inline]
    fn index_of(&self, (x, y): (u16, u16)) -> u32 {
        y as u32 * self.width + x as u32
    }

    #[inline]
    fn coord_of(&self, idx: u32) -> (u16, u16) {
        let x = idx % self.width;
        let y = idx / self.width;
        (x as u16, y as u16)
    }
}

struct ZOrder;

impl IndexType for ZOrder {
    #[inline]
    fn index_of(&self, xy: (u16, u16)) -> u32 {
        index_of(xy)
    }

    #[inline]
    fn coord_of(&self, idx: u32) -> (u16, u16) {
        coord_of(idx)
    }
}

struct ZOrderBmi2;

impl IndexType for ZOrderBmi2 {
    #[inline]
    fn index_of(&self, xy: (u16, u16)) -> u32 {
        unsafe { bmi2::index_of(xy) }
    }

    #[inline]
    fn coord_of(&self, idx: u32) -> (u16, u16) {
        unsafe { bmi2::coord_of(idx) }
    }
}

struct Vec2<I: IndexType> {
    inner: Vec<u64>,
    index_type: I,
}

impl<I: IndexType> Vec2<I> {
    fn new(width: u32, index_type: I) -> Self {
        Self {
            inner: vec![0; (width * width) as usize],
            index_type,
        }
    }

    fn set(&mut self, xy: (u16, u16), value: u64) {
        let idx = self.index_type.index_of(xy);
        self.inner[idx as usize] = value;
    }

    fn sum_neighbors(&self, (x, y): (u16, u16)) -> u64 {
        let mut sum = 0;
        for dx in 0..3 {
            for dy in 0..3 {
                let idx = self.index_type.index_of((x + dx - 1, y + dy - 1));
                sum += self.inner[idx as usize];
            }
        }
        sum
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let pos_and_idx: [((u16, u16), u64); 9] = [
        ((373, 870), 1),
        ((373, 871), 2),
        ((373, 872), 3),
        ((374, 870), 4),
        ((374, 871), 5),
        ((374, 872), 6),
        ((375, 870), 7),
        ((375, 871), 8),
        ((375, 872), 9),
    ];

    fn vec_with_index_type<I: IndexType>(arr: &[((u16, u16), u64); 9], index_type: I) -> Vec2<I> {
        let mut vec2 = Vec2::new(1000, index_type);
        for (xy, val) in arr {
            vec2.set(*xy, *val);
        }
        vec2
    }

    {
        let vec2_linear = vec_with_index_type(&pos_and_idx, Linear { width: 1000 });
        c.bench_function("linear indexing", |b| {
            b.iter(|| {
                for (xy, _) in &pos_and_idx {
                    black_box(vec2_linear.sum_neighbors(*xy));
                }
            })
        });
    }

    {
        let vec2_zorder = vec_with_index_type(&pos_and_idx, ZOrder);
        c.bench_function("zorder indexing", |b| {
            b.iter(|| {
                for (xy, _) in &pos_and_idx {
                    black_box(vec2_zorder.sum_neighbors(*xy));
                }
            })
        });
    }

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            let vec2_bmi2 = vec_with_index_type(&pos_and_idx, ZOrderBmi2);
            c.bench_function("bmi2 indexing", |b| {
                b.iter(|| {
                    for (xy, _) in &pos_and_idx {
                        black_box(vec2_bmi2.sum_neighbors(*xy));
                    }
                })
            });
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
