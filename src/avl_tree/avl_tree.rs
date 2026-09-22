use std::boxed::Box;
pub enum AvlTreeErr{
    InvalidInsertion
}

#[derive(Clone, Debug)]
struct Node<T>{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

pub struct AvlTree<T>{
    root: Option<Box<Node<T>>>
}

impl<T: PartialOrd + PartialEq + Clone> Node<T>{
    fn new(
        value: T, 
        left: Option<Box<Node<T>>>, 
        right: Option<Box<Node<T>>>
    ) -> Self{
        Self {
            value,
            left,
            right
        }
    }

    fn get_balance_factor(&self) -> i64{
        let mut left: i64 = 0;
        let mut right: i64 = 0;

        if let Some(ref lnode) = self.left {
            left = lnode.get_height();
        }
        if let Some(ref rnode) = self.right {
            right = rnode.get_height();
        }

        left - right
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

}

impl <T: PartialOrd + PartialEq + Clone> AvlTree<T>{
    pub fn new() -> Self{
        Self { 
            root: None 
        }
    }

    fn rotate_right(
        root: &mut Option<Box<Node<T>>>
    ){
        let mut new_root = 
            root
            .as_ref()
            .unwrap()
            .left
            .clone();

        let new_left = 
            new_root
            .as_ref()
            .unwrap()
            .right
            .clone();

        new_root
        .as_mut()
        .unwrap()
        .right = 
            root
            .clone();

        new_root
        .as_mut()
        .unwrap()
        .right
        .as_mut()
        .unwrap()
        .left = new_left;

        *root = new_root;
    }

    fn rotate_left(
        root: &mut Option<Box<Node<T>>>
    ){
        let mut new_root = 
            root
            .as_ref()
            .unwrap()
            .right
            .clone();

        let new_right = 
            new_root
            .as_ref()
            .unwrap()
            .left
            .clone();

        new_root
        .as_mut()
        .unwrap()
        .left = 
            root
            .clone();

        new_root
        .as_mut()
        .unwrap()
        .left
        .as_mut()
        .unwrap()
        .right = new_right;

        *root = new_root;
    }

    fn recursive_insertion(
        root: &mut Option<Box<Node<T>>>, 
        val: T
    ) -> Result<(), AvlTreeErr>{
        
        if 
            root
            .is_none() || 
            root
            .as_ref()
            .unwrap()
            .value == val 
        {
            return Err(AvlTreeErr::InvalidInsertion);
        }

        if root.as_ref().unwrap().as_ref().value > val {
            if root.as_ref().unwrap().left.is_none() {
                root.as_mut().unwrap().left = Some(
                    Box::new(
                        Node::new(val, None, None)
                    )
                );
            }
            else {
                AvlTree::recursive_insertion(&mut  root.as_mut().unwrap().left, val)?;
            }
        }
        else {
            if root.as_ref().unwrap().right.is_none() {
                root.as_mut().unwrap().right = Some(
                    Box::new(
                        Node::new(val, None, None)
                    )
                );
            }
            else {
                AvlTree::recursive_insertion(&mut root.as_mut().unwrap().right, val)?;
            }
        }

        // left left 
        if 
            root.as_ref().unwrap().get_balance_factor() > 1 && 
            root.as_ref().unwrap().left.as_ref().unwrap().get_balance_factor() >= 0 
        {
            AvlTree::rotate_right(root);
        }
        
        // left right
        if 
            root.as_ref().unwrap().get_balance_factor() > 1 && 
            root.as_ref().unwrap().left.as_ref().unwrap().get_balance_factor() < 0
        {
            AvlTree::rotate_left(&mut root.as_mut().unwrap().left);
            AvlTree::rotate_right(root);
        }

        // right right 
        if 
            root.as_ref().unwrap().get_balance_factor() < -1 &&
            root.as_ref().unwrap().right.as_ref().unwrap().get_balance_factor() <= 0
        {
            AvlTree::rotate_left(root);
        }

        // right left
        if 
            root.as_ref().unwrap().get_balance_factor() < -1 &&
            root.as_ref().unwrap().right.as_ref().unwrap().get_balance_factor() > 0
        {
            AvlTree::rotate_right(&mut root.as_mut().unwrap().right);
        }

        Ok(())        
    }

    pub fn recursive_search(
        node: &Option<Box<Node<T>>>, 
        val: T
    ) -> bool{
        if node.is_none() {
            return false;
        }

        let curr_node = node.as_ref().unwrap();

        if curr_node.value == val {
            true
        } 
        else if curr_node.value > val {
            AvlTree::recursive_search(&curr_node.left, val)
        }
        else {
            AvlTree::recursive_search(&curr_node.right, val)
        }

    }

    pub fn search(
        &self, val: T 
    ) -> bool{
        AvlTree::recursive_search(&self.root, val)
    }

    pub fn insert(
        &mut self, val: T
    ) -> Result<(), AvlTreeErr>{

        if self.root.is_none() {
            self.root = Some(
                Box::new(
                    Node::new(
                        val, 
                        None,
                            None
                    )
                )
            )
        }
        else {
            AvlTree::recursive_insertion(
                &mut self
                .root, 
                val
            )?;
        }

        Ok(())
    }

}

#[cfg(test)]
mod avl_tree_tests{
    use crate::avl_tree::avl_tree::AvlTree;

    macro_rules! add_test {
        ($tree:expr => [$($value:expr),* $(,)?]) => {
            $(
                let _ = $tree.insert($value);
                assert!($tree.search($value));
                assert!( $tree.root.as_ref().unwrap().get_balance_factor().abs() <= 1 );
            )*
        };
    }

    #[test]
    fn test_avl_insertion1(){

        let mut avl_tree = AvlTree::<u8>::new();

        add_test!(
            avl_tree => [20,19,18,17,16,15]
        );
    }

    #[test]
    fn test_avl_insertion2(){
        let mut avl_tree = AvlTree::<u8>::new();

        add_test!(
            avl_tree => [1,2,3,4,5,6,7,8,9,10]
        );
    }


    #[test]
    fn test_invalid_avl_insertion(){
        let mut avl_tree = AvlTree::<u8>::new();
        let _ = avl_tree.insert(19);
        let err = avl_tree.insert(19);

        assert!( err.is_err() );
    }

}