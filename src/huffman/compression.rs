pub trait HuffmanAnalysis {
    fn get_frequency(&self) -> usize;
    fn next(txt: &str) -> (Self, usize) where Self: Sized;
}

pub struct HeapMin<T: HuffmanAnalysis>{
    vec: Vec<T>
}

pub struct HuffmanNode{
    value: u8,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>
}

pub struct HuffmanTree{
    root: Option<HuffmanNode>
}

impl<T: HuffmanAnalysis + Clone> HeapMin<T>{
    pub fn new(mut vec: Vec<T>) -> Self {
        let size = vec.len();
        for idx in (0..vec.len()/2-1).rev() {
            Self::heapify(&mut vec, idx, size);
        }
        Self { vec }
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.vec.is_empty() {return None}
        let min = self.vec[0].clone();
        let size = self.vec.len() - 1;
        if let Some(last) = self.vec.last().cloned() {
            self.vec[0] = last;
            Self::heapify(&mut self.vec, 0, size);
        }
        self.vec.shrink_to(size);
        Some(min)
    }

    fn heapify(
        vec: &mut Vec<T>, 
        idx: usize, 
        size: usize
    ) {
        let mut smallest = idx;
        let left = idx * 2 + 1;
        let right = idx * 2 + 2;
        if left < size && left < smallest {smallest = left}
        if right < size && right < smallest {smallest = right}
        if smallest != idx {
            let temp = vec[smallest].clone();
            vec[smallest] = vec[idx].clone();
            vec[idx] = temp;
            Self::heapify(vec, smallest, size);
        }
    }

}

impl HuffmanNode {
    fn new_leaf(value: u8) -> Self{
        Self { value, left: None, right: None }
    }

    fn new(
        value: u8, 
        left: Option<Box<HuffmanNode>>, 
        right: Option<Box<HuffmanNode>>
    ) -> Self {
        Self { value, left, right }
    }
}

impl HuffmanTree {
    pub fn new() -> Self{
        Self { root: None }
    }
}

