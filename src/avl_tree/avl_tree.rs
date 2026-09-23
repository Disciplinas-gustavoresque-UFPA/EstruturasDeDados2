use std::{boxed::Box, cmp::Ordering, mem::replace};
pub enum AvlTreeErr{
    InvalidInsertion
}

#[derive(Clone, Debug)]
pub(crate) struct AvlNode<T>{
    value: T,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>
}

pub struct AvlTree<T>{
    root: Option<Box<AvlNode<T>>>
}

impl<T: Ord + Eq + Clone> AvlNode<T>{
    pub(crate) fn new(
        value: T
    ) -> Self{
        Self {
            value,
            left: None,
            right: None
        }
    }

    fn get_balance_factor(&self) -> i8{
        let mut left: i64 = 0;
        let mut right: i64 = 0;

        if let Some(ref lnode) = self.left {
            left = lnode.get_height();
        }
        if let Some(ref rnode) = self.right {
            right = rnode.get_height();
        }

        (right - left) as i8
    }

    fn get_height(&self) -> i64{
        let mut left: i64 = 0;
        let mut right: i64 = 0;

        if let Some(ref lnode) = self.left {
            left = lnode.get_height();
        }
        if let Some(ref rnode) = self.right {
            right = rnode.get_height();
        }

        return if left > right {left + 1} else {right + 1};
    }

    fn rotate_right(&mut self){
        if self.left.is_none() { return; }

        let mut left = 
            self
            .left
            .take()
            .unwrap();

        let left_right = 
            left
            .right
            .take();
    
        let mut old_root = replace(
            self,
             *left
        );

        old_root.left = left_right;
        self.right = Some(
            Box::new(
                old_root
            )
        );
    }

    fn rotate_left(&mut self){
        if self.right.is_none() { return;}

        let mut right = 
            self
            .right
            .take()
            .unwrap();

        let right_left = 
            right
            .left
            .take();
    
        let mut old_root = replace(
            self,
             *right
        );

        old_root.right = right_left;

        self.left = Some(
            Box::new(
                old_root
            )
        );
    }

    pub(crate) fn auto_balancing(&mut self){
        match self.get_balance_factor() {
            -2 if let Some(ref mut left) = self.left => {

                if left.get_balance_factor() == 1 {
                    left.rotate_left();
                }

                self.rotate_right();
            },
            2 if let Some(ref mut right) = self.right => {

                if  right.get_balance_factor() == -1 {
                    right.rotate_right();
                }

                self.rotate_left();
            },
            _ => {}
        }
    }


    pub(crate) fn insert(
        &mut self, 
        val: T
    ) -> Result<(), AvlTreeErr>{

        if self.value == val {
            return Err(AvlTreeErr::InvalidInsertion);
        }

        if 
            self.value > val && 
            let Some(ref mut left) = self.left
        {
            left.insert(val)?;
        }   
        else if self.value > val {
            self.left = Some(
                Box::new(
                    AvlNode::new(val)
                )
            );
        }
        else if 
            self.value < val && 
            let Some(ref mut right) = self.right
        {
            right.insert(val)?;
        }   
        else{
            self.right = Some(
                Box::new(
                    AvlNode::new(val)
                )
            );
        }

        self.auto_balancing();
        
        Ok(())
    }

    pub(crate) fn search(
        &self, 
        val: T
    ) -> bool{
        match self.value.cmp(&val) {
            Ordering::Equal => true,
            Ordering::Greater => {
                if let Some(ref left) = self.left {
                    left.search(val)
                } 
                else{
                    false
                }
            },
            Ordering::Less => {
                if let Some(ref right) = self.right {
                    right.search(val)
                } 
                else{
                    false
                }
            }
        }
    }

    pub(crate) fn get_all_balance_factors(
        &self, 
        vec: &mut Vec<i8>
    ){
        if let Some(ref left) = self.left {
            left.get_all_balance_factors(vec);
        }

        vec.push(self.get_balance_factor());

        if let Some(ref right) = self.right {
            right.get_all_balance_factors(vec);
        }
    }



}


impl<T: Ord + Eq + Clone>  AvlTree<T>{
    pub fn new() -> Self{
        Self {
            root: None
        }
    }

    pub fn insert(
        &mut self, 
        val: T
    ) -> Result<(), AvlTreeErr>{
        if let Some(ref mut root) = self.root {
            let res = root.insert(val);
            root.auto_balancing();
            res
        }
        else {
            self.root = Some(
                Box::new(
                    AvlNode::new(val)
                )
            );
            Ok(())
        }
    }

    pub fn search(
        &self, 
        val: T
    ) -> bool{
        if let Some(ref root) = self.root {
            root.search(val) 
        }
        else {
            false
        }
    }

    pub fn get_all_balance_factors(&self) -> Vec<i8>{
        let mut vec = Vec::new();
        if let Some(ref root) = self.root {
            root.get_all_balance_factors(&mut vec);
        }
        vec
    }

}

#[macro_export]
macro_rules! avl_tree {
    ($($val:expr),* $(,)?) => {
        {
            let mut avl_tree = AvlTree::new();
            $(
                assert!( avl_tree.insert($val).is_ok() );    
            )*
            avl_tree
        }
    };
}

#[cfg(test)]
mod avl_tree_tests{
    use crate::avl_tree::avl_tree::AvlTree;

    #[test]
    fn test_avl_insertion(){

        let avl_tree = avl_tree!(1,2,3,4,5);

        for bfactors in avl_tree.get_all_balance_factors() {
            assert!(bfactors.abs() <= 1);
        }
    }

    #[test]
    fn test_avl_search(){
        let avl_tree = avl_tree!(1,2,3,4,5,6,7,8,9,10);

        for val in 1..11{
            assert!(avl_tree.search(val));
        }

    }

}