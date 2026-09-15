use crate::utils::min;
use crate::utils::max;

pub mod utils;

fn main() {

    let inputs: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 1, 2, 3],
        vec![5, 2, 3],
        vec![5, 4, 3, 2, 1],
        vec![i32::MAX],
        vec![i32::MIN],
        vec![i32::MAX, i32::MIN],
        vec![]
    ];

    let mut outputs : Vec<Option<i32>> = vec![];

    for input in &inputs {
        let max_integer: Option<i32> = max(&input);
        outputs.push(max_integer);
    
    }

    // printing max results
    for output in &outputs {
        match output {
            None => println!("There is no maximum integer."),
            Some(x) => println!("max({x})")
        }
    }
    outputs.clear();

    for input in &inputs {
        let max_integer: Option<i32> = min(&input);
        outputs.push(max_integer);
    
    }

    // printing min results
    for output in &outputs {
        match output {
            None => println!("There is no minimum integer."),
            Some(x) => println!("min({x})")
        }
    }
    outputs.clear();
}