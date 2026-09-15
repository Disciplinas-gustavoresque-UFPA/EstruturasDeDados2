use std::fmt::Display;

use crate::sorting::bubble_sort;
use crate::sorting::insertion_sort;
use crate::searching::max;
use crate::searching::min;
use crate::searching::find;

pub mod searching;
pub mod sorting;

pub fn vec_to_string<T: Display>(elements: &Vec<T>) -> String {
    let mut msg = String::new();
    let len = elements.len();
    if len > 0 {
        for idx in 0..len - 1 {
            msg.push_str(&elements[idx].to_string());
            msg.push(' ');
        }
        msg.push_str(&elements[len - 1].to_string());
    }
    
    return msg;
}

pub fn print_outputs(message: &str, outputs: &Vec<String>) {
    println!("{message}");
    for output in outputs {
        println!("{output}");
    }
}

fn main() {

    let search_inputs: Vec<(i32, Vec<i32>)> = vec![
        (1,         vec![1, 2, 3]),
        (-2,        vec![-1, -2, -3]),
        (4,         vec![4, 1, 2, 3]),
        (3,         vec![-4, 1, 2, 3]),
        (i32::MAX,  vec![5, 2, 3]),
        (-2,        vec![5, -2, 3]),
        (1,         vec![5, 4, 3, 2, 1]),
        (0,         vec![-5, -4, -3, -2, -1]),
        (0,         vec![i32::MAX]),
        (i32::MIN,  vec![i32::MIN]),
        (i32::MIN,  vec![i32::MAX, i32::MIN]),
        (1,         vec![i32::MAX, i32::MIN]),
        (1,         vec![])
    ];

     let inputs_to_order: Vec<Vec<i32>> = vec![
        vec![],                         // empty
        vec![0],                        // one element
        vec![1, 2, 3],                  // already ordered
        vec![4, 1, 2, 3],
        vec![4, 1, 2, 3, 5],
        vec![1, 2, 3, 0],
        vec![0, 1, 2, 3, 0],
        vec![-1, -1, -1, -1],           // repeated elements
        vec![5, 4, 3, 2, 1],            // reverse order
        vec![1, 2, -1, -2, -3],         
    ];

    let mut outputs: Vec<String> = vec![];

    for (_, input) in &search_inputs {
        let max_integer: Option<i32> = max(input);
        match max_integer {
            Some(x) => outputs.push(format!("max({}): {}", vec_to_string(input), x)),
            None => outputs.push("No maximum integers found.".to_string())
        }
    }

    // printing max results
    print_outputs("\nMAX:", &outputs);
    outputs.clear();

    for (_, input) in &search_inputs {
        let min_integer: Option<i32> = min(input);        
        match min_integer {
            Some(x) => outputs.push(format!("min({}): {}", vec_to_string(input), x)),
            None => outputs.push("No minimum integers found.".to_string())
        }
    }

    // printing min results
    print_outputs("\nMIN:", &outputs);
    outputs.clear();

    for (target, input) in &search_inputs {
        let found_integer: Option<usize> = find(input, target);
        match found_integer {
            Some(x) => outputs.push(format!("Find {} in [{}]: idx {}", target, vec_to_string(input), x)),
            None => outputs.push(format!("Target {} was not found in [{}].", target, vec_to_string(input)))
        }
    }

    // printing find results
    print_outputs("\nFIND:", &outputs);
    outputs.clear();

    for mut input in &inputs_to_order {
        let output = insertion_sort(&mut input);
        outputs.push(format!("[{}]: {}", vec_to_string(input), vec_to_string(&output)));
    }

    print_outputs("\nINSERTION SORT:", &outputs);
    outputs.clear();

    for mut input in &inputs_to_order {
        let output = bubble_sort(&mut input);
        outputs.push(format!("[{}]: {}", vec_to_string(input), vec_to_string(&output)));
    }
    print_outputs("\nBUBBLE SORT:", &outputs);
    outputs.clear(); 

}