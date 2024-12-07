# partial_sort

[![](https://img.shields.io/crates/v/partial_sort.svg)](https://crates.io/crates/partial_sort)
[![](https://img.shields.io/crates/d/partial_sort.svg)](https://crates.io/crates/partial_sort)
[![](https://docs.rs/partial_sort/badge.svg)](https://docs.rs/partial_sort/)
[![](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml/badge.svg)](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml)

partial_sort is Rust version of [std::partial_sort](https://en.cppreference.com/w/cpp/algorithm/partial_sort)

## Usage

```rust
use partial_sort::PartialSort;

fn main() {
    let mut vec = vec![4, 4, 3, 3, 1, 1, 2, 2];
    vec.partial_sort(4, |a, b| a.cmp(b));
    assert_eq!(&vec[0..4], &[1, 1, 2, 2]);
}
```


## Benches

First we compare what happens when sorting the entire vector.

Bench env:

```
$ uname -a
Linux arch 6.8.7-arch1-1 #1 SMP PREEMPT_DYNAMIC Wed, 17 Apr 2024 15:20:28 +0000 x86_64 GNU/Linux

$ cat /proc/cpuinfo | grep '\-Core' | head -1
model name      : AMD Ryzen 9 5950X 16-Core Processor
```

Bench results:

```
nth_select sort 10000 limit 20          time:   [8.6016 µs 8.6123 µs 8.6236 µs]
partial sort 10000 limit 20             time:   [5.3522 µs 5.3565 µs 5.3610 µs]   // 1.6x faster
partial sort 10000 limit 200            time:   [15.736 µs 15.749 µs 15.763 µs]
partial sort 10000 limit 2000           time:   [111.97 µs 112.07 µs 112.18 µs]
partial sort 10000 limit 10000          time:   [219.21 µs 222.58 µs 225.85 µs]
stdsort 10000                           time:   [81.795 µs 82.108 µs 82.383 µs]
unstable stdsort 10000                  time:   [65.339 µs 65.355 µs 65.373 µs]
heapsort 10000                          time:   [241.86 µs 242.09 µs 242.29 µs]
partial reverse sort 10000 limit 20     time:   [5.4574 µs 5.4696 µs 5.4843 µs]
stdsort reverse 10000                   time:   [82.680 µs 82.751 µs 82.822 µs]
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
