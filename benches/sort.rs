extern crate criterion;
extern crate partial_sort;
extern crate rand;

use std::collections::BinaryHeap;

use criterion::{criterion_group, criterion_main, Criterion};
use partial_sort::PartialSort;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn create_vec<T>(size: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut rng = StdRng::seed_from_u64(42);
    (0..size).map(|_| rng.gen::<T>()).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let n = 10000;

    let mut v = create_vec::<u64>(n);
    let mut vv = v.clone();
    vv.sort();

    v.partial_sort(2000, |a, b| a.cmp(b));
    assert_eq!(&vv[0..2000], &v[0..2000]);

    let mut v = create_vec::<u64>(n);
    c.bench_function("partial sort 10000 limit 20", |b| {
        b.iter(|| {
            v.partial_sort(20, |a, b| a.cmp(b));
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("partial sort 10000 limit 200", |b| {
        b.iter(|| {
            v.partial_sort(200, |a, b| a.cmp(b));
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("partial sort 10000 limit 2000", |b| {
        b.iter(|| {
            v.partial_sort(2000, |a, b| a.cmp(b));
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("partial sort 10000 limit 10000", |b| {
        b.iter(|| {
            v.partial_sort(10000, |a, b| a.cmp(b));
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("stdsort 10000", |b| {
        b.iter(|| {
            v.sort_by(|a, b| a.cmp(b));
        })
    });

    c.bench_function("heapsort 10000", |b| {
        b.iter(|| {
            let v = create_vec::<u64>(n);
            let h = BinaryHeap::from(v);
            h.into_sorted_vec();
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("partial reverse sort 10000 limit 20", |b| {
        b.iter(|| {
            v.partial_sort(20, |a, b| a.cmp(b).reverse());
        })
    });

    let mut v = create_vec::<u64>(n);
    c.bench_function("stdsort reverse 10000", |b| {
        b.iter(|| {
            v.sort_by(|a, b| a.cmp(b).reverse());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
