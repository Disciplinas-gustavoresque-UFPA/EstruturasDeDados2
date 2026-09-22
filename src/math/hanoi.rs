const EMPTY_START_TOWER_ERR: &str = "hanoi: 'start' tower must have at least one disc.";
const FILLED_END_TOWER_ERR: &str = "hanoi: 'end' tower must be empty.";
const UNSORTED_START_TOWER_ERR: &str = "hanoi: 'start' tower must be sorted.";

pub fn hanoi<T: PartialOrd>(start: &mut Vec<T>, end: &mut Vec<T>) {
    assert!(!start.is_empty(), "{EMPTY_START_TOWER_ERR}");
    assert!(end.is_empty(), "{FILLED_END_TOWER_ERR}");
    assert!(start.is_sorted(), "{UNSORTED_START_TOWER_ERR}");

    let mut aux: Vec<T> = vec![];
    inner_hanoi(start.len(), start, end, &mut aux);
}

fn inner_hanoi<T: PartialOrd>(
    num_discs: usize,
    start: &mut Vec<T>,
    end: &mut Vec<T>,
    aux: &mut Vec<T>,
) {
    if num_discs == 1 {
        let top = start.remove(0);
        end.insert(0, top);
        
    } else {
        inner_hanoi(num_discs - 1, start, aux, end);
        let top = start.remove(0);
        end.insert(0, top);
        
        inner_hanoi(num_discs - 1, aux, end, start);
    }
}

#[cfg(test)]
mod tests {

    use super::hanoi;

    const EMPTY_TOWER: Vec<i64> = vec![];

    #[test]
    #[should_panic(expected="hanoi: 'start' tower must have at least one disc.")]
    fn test_empty_start_tower() {
        let mut start: Vec<i64> = EMPTY_TOWER.clone();
        let mut end: Vec<i64> = EMPTY_TOWER.clone();
        hanoi(&mut start, &mut end);
    }

    #[test]
    #[should_panic(expected="hanoi: 'end' tower must be empty.")]
    fn test_filled_end_tower() {
        let prop_tower: Vec<i64> = (1..=1).step_by(1).collect(); 
        let mut start: Vec<i64> = prop_tower.clone();
        let mut end: Vec<i64> = prop_tower.clone();
        hanoi(&mut start, &mut end);
    }

    #[test]
    #[should_panic(expected="hanoi: 'start' tower must be sorted.")]
    fn test_unsorted_characters_start_tower() {
        let mut start = vec!["2", "1"];
        let mut end: Vec<&str> = vec![];
        hanoi(&mut start, &mut end);
    }

    #[test]
    #[should_panic(expected="hanoi: 'start' tower must be sorted.")]
    fn test_unsorted_numbers_start_tower() {
        let mut start = vec![2, 1];
        let mut end: Vec<i32> = vec![];
        hanoi(&mut start, &mut end);
    }

    #[test]
    fn test_one_to_ten_discs() {

        /*
            For n, where 1 <= n <= 10, create a prop_tower with 'n' discs, clone it to a start tower.
            Then, create an empty end tower where the discs from start tower will be moved. Run the hanoi algorithm.
            After that, assert that the end tower has the same discs at the exact positions of the prop_tower.
            Also assert that start tower is empty.
         */

        for num in 1..10 {
            let prop_tower: Vec<i64> = (1..=num).step_by(1).collect(); 
            let mut start: Vec<i64> = prop_tower.clone(); 
            let mut end: Vec<i64> = EMPTY_TOWER.clone();
            hanoi(&mut start, &mut end);
            assert_eq!(start, EMPTY_TOWER);
            assert_eq!(end, prop_tower);
        }        
    }
}
