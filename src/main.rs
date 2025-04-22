use std::time;
use rand::random_range;

pub mod sorts;
pub mod utils;

const SIZE: usize = 1_000_000;

fn main() {
    // let thing: char = 'n';
    // let another: i8 = 3;
    // let another2 = true;
    // let misaligned = [3, 2, 12, 5, 0, 23, 23, 123, 35, 35, 3, 3, 3, 3, 3, 3];

    // println!("{:p} | {:p} | {:p}", &thing, &another, &another2);
    // println!("{:p}", &misaligned);

    let mut arr = Vec::with_capacity(SIZE);
    for _ in 0..20 {
        arr.push(random_range(-10i64..=100));
    }
    {
        let some = arr.as_mut_slice();
        some[0] = 1;
        some[1] = 2;
        some[2] = 3;
        println!("{:?}", some);
    }

    println!("{:?}", arr);

    // let mut arr2 = arr.clone();
    // let mut start = time::Instant::now();
    // arr2.sort();
    // let mut elapsed = start.elapsed();
    // println!("std::sort()  time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    // arr2 = arr.clone();
    // start = time::Instant::now();
    // sorts::heap_sort(arr2.as_mut_slice());
    // elapsed = start.elapsed();
    // println!("heap_sort()  time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    // arr2 = arr.clone();
    // start = time::Instant::now();
    // sorts::quick_sort(arr2.as_mut_slice());
    // elapsed = start.elapsed();
    // println!("quick_sort() time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));

    // arr2 = arr.clone();
    // start = time::Instant::now();
    // sorts::merge_sort(arr2.as_mut_slice());
    // elapsed = start.elapsed();
    // println!("merge_sort() time: {:.2?} ms | {}", elapsed.as_micros() as f64 / 1000.0, utils::is_sorted_strict(arr.as_slice(), arr2.as_slice()));
    // println!("{}", utils::is_sorted(arr2.as_slice()));


    // let mut arr2 = arr.clone();

    // println!("{}", is_sorted(arr.as_slice()));
    // println!("{}", is_sorted(arr2.as_slice()));

    // let now = Instant::now();
    // quick_sort(arr.as_mut_slice(), 0, SIZE - 1);
    // let elapsed = now.elapsed();

    // println!("{}  |  {:.2?} ms", is_sorted(arr.as_slice()), elapsed.as_micros() as f64 / 1000.0);
}
