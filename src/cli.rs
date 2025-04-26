use std::{collections::VecDeque, env};


pub fn main() {
    let args: VecDeque<String> = env::args().collect();
    println!("{args:?}");
}
