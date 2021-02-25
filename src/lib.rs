#![crate_type = "lib"]
#![crate_name = "partial_sort"]
#![cfg_attr(feature = "nightly", feature(test))]

use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::ptr;

// use partial_sort::PartialSort;
//
// fn main() {
//     let mut vec = vec![4, 4, 3, 3, 1, 1, 2, 2];
//     vec.partial_sort(4, |a, b| a.cmp(b));
//     println!("{:?}", vec);
// }

pub trait PartialSort {
    type Item;

    fn partial_sort<F>(&mut self, _: usize, _: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;
}

impl<T> PartialSort for [T] {
    type Item = T;

    fn partial_sort<F>(&mut self, last: usize, mut cmp: F)
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        partial_sort(self, last, |a, b| cmp(a, b) == Less);
    }
}

pub fn partial_sort<T, F>(v: &mut [T], last: usize, mut is_less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    debug_assert!(last <= v.len());

    make_heap(v, last, &mut is_less);

    unsafe {
        for i in last..v.len() {
            if is_less(v.get_unchecked(i), v.get_unchecked(0)) {
                v.swap(0, i);
                adjust_heap(v, 0, last, &mut is_less);
            }
        }

        sort_heap(v, last, &mut is_less);
    }
}

#[inline]
fn make_heap<T, F>(v: &mut [T], last: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if last < 2 {
        return;
    }

    let len = last;
    let mut parent = (len - 2) / 2;

    loop {
        adjust_heap(v, parent, len, is_less);
        if parent == 0 {
            return;
        }
        parent -= 1;
    }
}

/// adjust_heap is a shift up adjust op for the heap
#[inline]
fn adjust_heap<T, F>(v: &mut [T], hole_index: usize, len: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let mut hole_index = hole_index;
    let mut left_child = hole_index * 2 + 1;

    unsafe {
        let value = ptr::read(v.get_unchecked(hole_index));

        while left_child < len {
            if left_child + 1 < len
                && is_less(v.get_unchecked(left_child), v.get_unchecked(left_child + 1))
            {
                left_child += 1;
            }

            if is_less(&value, v.get_unchecked(left_child)) {
                ptr::copy_nonoverlapping(&v[left_child], &mut v[hole_index], 1);
                hole_index = left_child;
            } else {
                break;
            }

            left_child = left_child * 2 + 1;
        }

        ptr::copy_nonoverlapping(&value, &mut v[hole_index], 1);
    }
}

#[inline]
unsafe fn sort_heap<T, F>(v: &mut [T], last: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    let mut last = last;
    while last > 1 {
        v.swap(0, last - 1);
        adjust_heap(v, 0, last - 1, is_less);
        last -= 1;
    }
}

#[cfg(test)]
mod tests {
    use PartialSort;

    #[test]
    fn empty_test() {
        let mut before: Vec<u32> = vec![4, 4, 3, 3, 1, 1, 2, 2];
        before.partial_sort(0, |a, b| a.cmp(b));
    }

    #[test]
    fn single_test() {
        let mut before: Vec<u32> = vec![4, 4, 3, 3, 1, 1, 2, 2];
        let last = 6;
        let mut d = before.clone();
        d.sort();

        before.partial_sort(last, |a, b| a.cmp(b));
        assert_eq!(&d[0..last], &before.as_slice()[0..last]);
    }

    #[test]
    fn sorted_strings_test() {
        let mut before: Vec<&str> = vec!["a", "cat", "mat", "on", "sat", "the"];
        let last = 6;
        let mut d = before.clone();
        d.sort();

        before.partial_sort(last, |a, b| a.cmp(b));
        assert_eq!(&d[0..last], &before.as_slice()[0..last]);
    }

    // #[test]
    // fn sorted_random_u64_test() {
    //     for i in (0..100) {
    //         let mut rng = rand::thread_rng();
    //
    //         let vec_size = 102400;
    //         let partial_size = (rng.next_u64() % vec_size) as usize;
    //
    //         let mut data = (0u64..102400).map(|_| rng.next_u64()).collect::<Vec<u64>>();
    //         let mut d = data.clone();
    //         d.sort();
    //
    //         data.partial_sort(partial_size, |a, b| a.cmp(b));
    //         assert_eq!(&d[0..partial_size], &data.as_slice()[0..partial_size]);
    //     }
    // }
}

#[cfg(feature = "nightly")]
#[cfg(test)]
mod benches {
    extern crate rand;

    extern crate test;

    use self::test::{black_box, Bencher};

    use self::rand::distributions::{Distribution, Range};

    use crate::PartialSort;
    use std::collections::BinaryHeap;

    static RANGE: u64 = 1000000;
    static VEC_SIZE: u64 = 50000;

    fn data() -> Vec<u64> {
        let mut rng = rand::thread_rng();
        let between = Range::new(0u64, RANGE);
        (0u64..VEC_SIZE).map(|_| between.sample(&mut rng)).collect()
    }

    #[bench]
    fn c_standard_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let mut numbers = black_box(&input).clone();
            numbers.sort();
        });
    }

    #[bench]
    fn c_partial_10_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let mut numbers = black_box(&input).clone();
            numbers.partial_sort(10, |a, b| a.cmp(b));
        });
    }

    #[bench]
    fn c_partial_100_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let mut numbers = black_box(&input).clone();
            numbers.partial_sort(100, |a, b| a.cmp(b));
        });
    }

    #[bench]
    fn c_partial_1000_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let mut numbers = black_box(&input).clone();
            numbers.partial_sort(1000, |a, b| a.cmp(b));
        });
    }

    #[bench]
    fn c_partial_10000_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let mut numbers = black_box(&input).clone();
            numbers.partial_sort(10000, |a, b| a.cmp(b));
        });
    }

    #[bench]
    fn c_heap_bench(b: &mut Bencher) {
        let input = data();

        b.iter(|| {
            let numbers = black_box(&input).clone();
            let h = BinaryHeap::from(numbers);
            h.into_sorted_vec();
        });
    }
}
