# partial_sort

[![Build Status](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml/badge.svg)](https://github.com/sundy-li/partial_sort/actions/workflows/Build.yml)
[![](http://meritbadge.herokuapp.com/partial_sort)](https://crates.io/crates/partial_sort)
[![](https://img.shields.io/crates/d/partial_sort.svg)](https://crates.io/crates/partial_sort)
[![](https://img.shields.io/crates/dv/partial_sort.svg)](https://crates.io/crates/partial_sort)
[![](https://docs.rs/partial_sort/badge.svg)](https://docs.rs/partial_sort/)


partial_sort is Rust version of [std::partial_sort](https://en.cppreference.com/w/cpp/algorithm/partial_sort)

## Usage

```rust 
use partial_sort::PartialSort;

fn main() {
    let mut vec = vec![4, 4, 3, 3, 1, 1, 2, 2];
    vec.partial_sort(4, |a, b| a.cmp(b));
    println!("{:?}", vec);
}

```


## benches
First we compare what happens when sorting the entire vector:

```
test benches::c_heap_bench          ... bench:   3,109,923 ns/iter (+/- 1,142,674)
test benches::c_partial_10000_bench ... bench:   2,052,967 ns/iter (+/- 84,947)
test benches::c_partial_1000_bench  ... bench:   2,075,428 ns/iter (+/- 661,595)
test benches::c_partial_100_bench   ... bench:     331,775 ns/iter (+/- 46,151)
test benches::c_partial_10_bench    ... bench:      36,194 ns/iter (+/- 14,282)
test benches::c_standard_bench      ... bench:   3,022,585 ns/iter (+/- 160,972)
```


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)