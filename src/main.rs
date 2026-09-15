use crate::utils::min;
use crate::utils::max;

pub mod utils;

fn main() {

    let inputs: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![-1, -2, -3],
        vec![4, 1, 2, 3],
        vec![-4, 1, 2, 3],
        vec![5, 2, 3],
        vec![5, -2, 3],
        vec![5, 4, 3, 2, 1],
        vec![-5, -4, -3, -2, -1],
        vec![i32::MAX],
        vec![i32::MIN],
        vec![i32::MAX, i32::MIN],
        vec![i32::MAX, i32::MIN],
        vec![]
    ];

    let mut outputs: Vec<String> = vec![];
    for input in &inputs {
        let max_integer: Option<i32> = max(&input);
        match max_integer {
            Some(x) => outputs.push(format!("{x}")),
            None => outputs.push("No maximum integers found.".to_string())
        }
    }

    // printing max results
    for output in &outputs {
        println!("{output}");
    }
    outputs.clear();

    for input in &inputs {
        let min_integer: Option<i32> = min(&input);        
        match min_integer {
            Some(x) => outputs.push(format!("{x}")),
            None => outputs.push("No minimum integers found.".to_string())
        }
    }

    // printing min results
    for output in &outputs {
        println!("{output}");
    }
    outputs.clear();
}