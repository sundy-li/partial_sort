//! # partial_sort
//!
//! [![](https://img.shields.io/crates/d/partial_sort.svg)](https://crates.io/crates/partial_sort)
//! [![](https://docs.rs/partial_sort/badge.svg)](https://docs.rs/partial_sort/)
//! [![](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml/badge.svg)](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml)
//!
//! partial_sort is Rust version of [std::partial_sort](https://en.cppreference.com/w/cpp/algorithm/partial_sort)
//!
//! ```shell
//! cargo add partial_sort
//! ```
//!
//! ## Example
//!
//! ```rust
//! # use partial_sort::PartialSort;
//!
//! let mut vec = vec![4, 4, 3, 3, 1, 1, 2, 2];
//! vec.partial_sort(4, |a, b| a.cmp(b));
//! assert_eq!(&vec[0..4], &[1, 1, 2, 2]);
//! ```

#![crate_type = "lib"]
#![crate_name = "partial_sort"]

use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::{mem, ptr};

pub trait PartialSort {
    type Item;

    /// Rearranges elements such that the range `[0, last)` contains the smallest `last` elements
    /// in the range `[0, n)` in ascending order.
    fn partial_sort<F>(&mut self, last: usize, cmp: F)
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
    assert!(last <= v.len());

    make_heap(v, last, &mut is_less);

    for i in last..v.len() {
        if is_less(&v[i], &v[0]) {
            v.swap(0, i);
            adjust_heap(v, 0, last, &mut is_less);
        }
    }

    sort_heap(v, last, &mut is_less);
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
    assert!(len <= v.len());
    assert!(hole_index < v.len());

    let mut left_child = hole_index * 2 + 1;

    // SAFETY: Reading from a reference is always valid. The original memory
    // location is now conceptually moved-from. At the end of the function,
    // or if `is_less()` panics at any point, `hole` is dropped and fills
    // the moved-from location with a valid element.
    let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(&v[hole_index]) });
    let mut hole = InsertionHole {
        src: &*tmp,
        dest: &mut v[hole_index],
    };

    while left_child < len {
        if left_child + 1 < len {
            left_child += usize::from(is_less(
                unsafe { v.get_unchecked(left_child) }, // SAFETY: left_child < len
                unsafe { v.get_unchecked(left_child + 1) }, // SAFETY: left_child + 1 < len
            ));
        }

        // SAFETY: left_child (even incremented) is still in bounds.
        if !is_less(&*tmp, unsafe { v.get_unchecked(left_child) }) {
            break;
        }

        // SAFETY: Source and destination are references. Now the location
        // at index left_child is conceptually moved-from and `hole` is updated
        // accordingly. At the end of the function, or if `is_less()` panics
        // at any point, `hole` is dropped and fills the moved-from location
        // with a valid element.
        unsafe {
            ptr::copy_nonoverlapping(
                v.get_unchecked(left_child), // SAFETY: still in bounds
                hole.dest,
                1,
            );
        }
        hole.dest = &mut v[left_child];

        left_child = left_child * 2 + 1;
    }

    // When dropped, copies from `src` into `dest`. Adapted from
    // `std::sort_by()`.
    struct InsertionHole<T> {
        src: *const T,
        dest: *mut T,
    }

    impl<T> Drop for InsertionHole<T> {
        fn drop(&mut self) {
            // SAFETY: `self.src` and `self.dest` have been created from
            // references.
            unsafe {
                ptr::copy_nonoverlapping(self.src, self.dest, 1);
            }
        }
    }
}

#[inline]
fn sort_heap<T, F>(v: &mut [T], mut last: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    while last > 1 {
        last -= 1;
        v.swap(0, last);
        adjust_heap(v, 0, last, is_less);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::cmp::Ordering;
    use std::fmt;
    use std::sync::Arc;

    use crate::PartialSort;

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
        let mut before: Vec<&str> = vec![
            "a", "cat", "mat", "on", "sat", "the", "xxx", "xxxx", "fdadfdsf",
        ];
        let last = 6;
        let mut d = before.clone();
        d.sort();

        before.partial_sort(last, |a, b| a.cmp(b));
        assert_eq!(&d[0..last], &before.as_slice()[0..last]);
    }

    #[test]
    fn sorted_ref_test() {
        trait TModel: fmt::Debug + Send + Sync {
            fn size(&self) -> usize;
        }

        struct ModelFoo {
            size: usize,
        }

        impl TModel for ModelFoo {
            fn size(&self) -> usize {
                self.size
            }
        }
        impl fmt::Debug for ModelFoo {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "ModelFoo[{}]", self.size)?;
                Ok(())
            }
        }

        struct ModelBar {
            size: usize,
        }

        impl TModel for ModelBar {
            fn size(&self) -> usize {
                self.size
            }
        }
        impl fmt::Debug for ModelBar {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "ModelBar[{}]", self.size)?;
                Ok(())
            }
        }

        type ModelRef = Arc<dyn TModel>;

        /// Compare two `Array`s based on the ordering defined in [ord](crate::array::ord).
        fn cmp_model(a: &dyn TModel, b: &dyn TModel) -> Ordering {
            a.size().cmp(&b.size())
        }

        let mut before: Vec<(i32, ModelRef)> = vec![
            (1i32, Arc::new(ModelBar { size: 100 })),
            (1i32, Arc::new(ModelFoo { size: 99 })),
            (1i32, Arc::new(ModelFoo { size: 101 })),
            (1i32, Arc::new(ModelBar { size: 104 })),
            (1i32, Arc::new(ModelBar { size: 10 })),
            (1i32, Arc::new(ModelBar { size: 24 })),
            (1i32, Arc::new(ModelBar { size: 34 })),
            (1i32, Arc::new(ModelBar { size: 114 })),
        ];

        let last = 6;
        let mut d = before.clone();
        d.sort_by(|a, b| cmp_model(a.1.as_ref(), b.1.as_ref()));

        before.partial_sort(last, |a, b| cmp_model(a.1.as_ref(), b.1.as_ref()));

        d[0..last].iter().zip(&before[0..last]).for_each(|(a, b)| {
            assert_eq!(a.0, b.0);
            assert_eq!(a.1.size(), b.1.size());
        });
    }

    /// creates random initial vectors, partial sorts then and
    /// verifies the result against std's `sort`.
    #[test]
    fn sorted_random_u64_test() {
        let mut rng = rand::thread_rng();
        let vec_size = 1025;
        let partial_size = (rng.gen::<u64>() % vec_size) as usize;
        let mut data = (0u64..vec_size)
            .map(|_| rng.gen::<u64>())
            .collect::<Vec<u64>>();
        let mut d = data.clone();
        d.sort();

        data.partial_sort(partial_size, |a, b| a.cmp(b));
        assert_eq!(&d[0..partial_size], &data.as_slice()[0..partial_size]);
    }

    #[test]
    #[ignore]
    fn sorted_expensive_random_u64_test() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let vec_size = 1025;
            let partial_size = (rng.gen::<u64>() % vec_size) as usize;
            let mut data = (0u64..vec_size)
                .map(|_| rng.gen::<u64>())
                .collect::<Vec<u64>>();
            let mut d = data.clone();
            d.sort();

            data.partial_sort(partial_size, |a, b| a.cmp(b));
            assert_eq!(&d[0..partial_size], &data.as_slice()[0..partial_size]);
        }
    }
}
