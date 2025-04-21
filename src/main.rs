// use std::time::Instant;


// mod sorts;
pub mod sorts;
mod utils;


const SIZE: usize = 1_000_000;

fn main() {
    let thing: char = 'n';
    let another: i8 = 3;
    let another2 = true;
    let misaligned = [3, 2, 12, 5, 0, 23, 23, 123, 35, 35, 3, 3, 3, 3, 3, 3];

    println!("{:p} | {:p} | {:p}", &thing, &another, &another2);
    println!("{:p}", &misaligned);

    let mut arr = Vec::with_capacity(SIZE);
    {
        let mut i: usize = 0;
        while i < SIZE {
            arr.push(rand::random_range(0i64..=1_000_000i64));
            i += 1;
        }
    }

    // let mut arr2 = arr.clone();

    // println!("{}", is_sorted(arr.as_slice()));
    // println!("{}", is_sorted(arr2.as_slice()));

    // let now = Instant::now();
    // quick_sort(arr.as_mut_slice(), 0, SIZE - 1);
    // let elapsed = now.elapsed();

    // println!("{}  |  {:.2?} ms", is_sorted(arr.as_slice()), elapsed.as_micros() as f64 / 1000.0);
}
