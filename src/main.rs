use crate::utils::min;
use crate::utils::max;

pub mod utils;

fn main() {
    let integers: Vec<i32> = vec![4, 1, 2, 3];
    
    let max_integer: Option<i32> = max(&integers);
    match max_integer {
        None => println!("There is no maximum integer."),
        Some(x) => println!("max({x})")
    }

    let min_integer: Option<i32> = min(&integers);
    match min_integer {
        None => print!("There is no minimum integer."),
        Some(x) => print!("min({x})")
    }
}