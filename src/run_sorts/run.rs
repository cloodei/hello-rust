use std::time;
use rand::random_range;

use crate::sorts;
use crate::utils;

const SIZE: usize = 1_000_000;

pub(crate) fn run() {
    let mut arr = Vec::with_capacity(SIZE);
    for _ in 0..SIZE {
        arr.push(random_range(-524_288..1_048_576));
    }

    let mut arr2 = arr.clone();
    let mut start = time::Instant::now();
    arr2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut elapsed = start.elapsed();
    println!("std::sort()  time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    arr2 = arr.clone();
    start = time::Instant::now();
    sorts::heap_sort(arr2.as_mut_slice());
    elapsed = start.elapsed();
    println!("heap_sort()  time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    arr2 = arr.clone();
    start = time::Instant::now();
    sorts::quick_sort(arr2.as_mut_slice());
    elapsed = start.elapsed();
    println!("quick_sort() time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    arr2 = arr.clone();
    start = time::Instant::now();
    sorts::merge_sort(arr2.as_mut_slice());
    elapsed = start.elapsed();
    println!("merge_sort() time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));
}
