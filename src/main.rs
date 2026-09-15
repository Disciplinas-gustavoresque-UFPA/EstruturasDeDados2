use crate::utils::max;
use crate::utils::min;
use crate::utils::find;

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

    let find_inputs: Vec<i32> = vec![
        1,
        -2,
        4,
        3,
        i32::MAX,
        -2,
        1,
        0,
        0,
        i32::MIN,
        i32::MIN 
    ];

    for (target, input) in find_inputs.iter().zip(inputs) {
        let found_integer: Option<usize> = find(&input, &target);
        match found_integer {
            Some(x) => outputs.push(format!("{x}")),
            None => outputs.push(format!("Target [{target}] was not found."))
        }
    }

    // printing find results
    for output in &outputs {
        println!("{output}");
    }
    outputs.clear();

}