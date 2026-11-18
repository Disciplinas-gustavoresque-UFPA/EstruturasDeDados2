use std::{
    boxed::Box, 
    cmp::{
        self, 
        Ordering
    }, 
    mem::replace, 
    unreachable
};

pub enum AvlTreeErr{
    InvalidInsertion
}

#[derive(Clone, Debug)]
pub(crate) struct AvlNode<T>{
    value: T,
    height: usize,
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
            height: 1,
            left: None,
            right: None
        }
    }

    fn get_balance_factor(&self) -> i8{
        let mut left: i64 = 0;
        let mut right: i64= 0;

        if let Some(ref lnode) = self.left {
            left = lnode.get_height() as i64;
        }
        if let Some(ref rnode) = self.right {
            right = rnode.get_height() as i64;
        }

        (right - left) as i8
    }

    fn get_height(&self) -> usize{
        let mut left = 0;
        let mut right = 0;

        if let Some(ref lnode) = self.left {
            left = lnode.get_height();
        }
        if let Some(ref rnode) = self.right {
            right = rnode.get_height();
        }

        cmp::max(left, right) + 1
    }

    fn rotate_right(&mut self){
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

        self.height = self.get_height();
        let right = self.right.as_mut().unwrap();
        let new_height_right = right.get_height();
        right.height = new_height_right;

    }

    fn rotate_left(&mut self){
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

        self.height = self.get_height();
        let left = self.left.as_mut().unwrap();
        let new_height_left = left.get_height();
        left.height = new_height_left;

    }

    pub(crate) fn auto_balancing(&mut self){
        match self.get_balance_factor() {
            /* A implementação do match arm junto da expressão if let Some(...)
             * não é estável e foi removida da implementação.
             * #51114 <https://github.com/rust-lang/rust/issues/51114>
             */
            -2 => {
                /* Nos dois casos o else vai para unreachable!(), pois 
                 * para retornar -2 ou 2 é necessário o filho esquerdo/direito
                 * existir.
                 */
                let Some(ref mut left) = self.left else{
                    unreachable!();
                };

                if left.get_balance_factor() == 1 {
                    left.rotate_left();
                }

                self.rotate_right();
            },
            2 => {
                let Some(ref mut right) = self.right else {
                    unreachable!()
                };

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
        

        self.height = self.get_height();

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

    fn get_lower_node(
        &self
    ) -> T{
        if let Some(ref left) = self.left {
            left.get_lower_node()
        }
        else {
            self.value.clone()
        }
    }

    pub(crate) fn remove(
        &mut self, 
        val: T
    ) -> Option<T>{
        if 
            self.value == val
        {   
            if self.left.is_none() || self.right.is_none() {
                if let Some(left) = self.left.take() {
                    let _ = replace(self, *left);
                }
                else if let Some(right) = self.right.take() {
                    let _ = replace(self, *right);
                }
            }
            else {
                let Some(mut right) = self.right.take() else{
                    unreachable!()
                };

                let lower = right.get_lower_node();
                right.remove(lower.clone());
                
                if 
                    right.value == lower && 
                    let Some(rright) = right.right.take()
                {
                    let _ = replace(&mut right, rright);
                    self.right = Some(right);
                }
                else if right.value != lower {
                    self.right = Some(right);
                }

                self.value = lower;
            }

            self.auto_balancing();

            self.height = self.get_height();

            Some(val)
        }
        else if 
            self.value > val &&
            let Some(mut left) = self.left.take()
        {
            let res = left.remove(val);
            if 
                let Some(ref vres) = res &&
                *vres != left.value
            {
                self.left = Some(left);
            }
            else if res.is_none() {
                self.left = Some(left)
            }

            self.auto_balancing();

            self.height = self.get_height();

            res
        } 
        else if 
            self.value < val && 
            let Some(mut right) = self.right.take()
        {
            let res = right.remove(val);
            if 
                let Some(ref vres) = res &&
                *vres != right.value 
            {
                self.right = Some(right);
            }
            else if res.is_none() {
                self.right = Some(right);
            }

            self.auto_balancing();

            self.height = self.get_height();

            res
        }
        else {
            None
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

    pub fn remove(
        &mut self,
        val: T
    ) -> Option<T>{
        if let Some(mut root) = self.root.take() {
            let res = root.remove(val);

            root.auto_balancing();
            
            if 
                let Some(ref vres) = res &&
                root.value != *vres
            {
                self.root = Some(root);
            }
            /* Antes desse else if, caso o valor não estivesse na árvore
             * a condição let Some(...) retornaria false e a sub árvore
             * seria apagada de forma incorreta.
             */
            else if res.is_none() {
                self.root = Some(root);
            }

            res
        }
        else{
            None
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
    use crate::avl_tree::avl_tree::{AvlTree, AvlTreeErr};

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

    /* Remove todos os nós da árvore enquanto checa o balanceamento
     * a cada remoção.
     */
    #[test]
    fn test_avl_remove_1(){
        let mut avl_tree = avl_tree!(1,2,3,4,5);

        for val in (1..=5).rev(){
            let res = avl_tree.remove(val);
            assert!(
                res.is_some() &&
                !avl_tree.search(val)
            );
            for bfactors in avl_tree.get_all_balance_factors() {
                assert!(bfactors.abs() <= 1);
            }
        }
    }

    /* Remove apenas a raiz da árvore para testar se o wrapper da
     * função "remove" na AvlTree não quebra a implementação da função
     * "remove" da AvlNode.
     */
    #[test]
    fn test_avl_remove_2(){
        /* A raiz da árvore é 2 verificada via Debug */
        let mut avl_tree = avl_tree!(1,2,3,4,5);

        let res = avl_tree.remove(2);
        assert!(
            res.is_some() &&
            !avl_tree.search(2)
        );
        for bfactors in avl_tree.get_all_balance_factors() {
            assert!(bfactors.abs() <= 1);
        }  
    }

    /* Verifica se ao remover um valor inexistente a integridade
     * da árvore se mantém.
     */
    #[test]
    fn test_avl_remove_3(){
        let mut avl_tree = avl_tree!(1,2,3,4,5);

        let res = avl_tree.remove(10);
        assert!(
            res.is_none()
        );
        
        for v in 1..=5 {
            assert!(
                avl_tree.search(v)
            );
        }

        for bfactors in avl_tree.get_all_balance_factors() {
            assert!(bfactors.abs() <= 1);
        }
    }

    #[test]
    fn test_empty_tree_operations() {
        let mut avl_tree: AvlTree<i32> = AvlTree::new();

        assert!(!avl_tree.search(1));
        assert!(avl_tree.remove(1).is_none());
        assert!(avl_tree.get_all_balance_factors().is_empty());
    }

    #[test]
    fn test_single_element_tree() {
        let mut avl_tree = AvlTree::new();
        assert!(avl_tree.insert(42).is_ok());

        assert!(avl_tree.search(42));
        assert!(!avl_tree.search(0));
        assert_eq!(avl_tree.get_all_balance_factors(), vec![0]);

        let removed = avl_tree.remove(42);
        assert_eq!(removed, Some(42));
        assert!(!avl_tree.search(42));
        assert!(avl_tree.get_all_balance_factors().is_empty());

        // Garante que é possível reinserir após esvaziar a árvore
        assert!(avl_tree.insert(100).is_ok());
        assert!(avl_tree.search(100));
        assert_eq!(avl_tree.get_all_balance_factors(), vec![0]);
    }

    #[test]
    fn test_duplicate_insertion_error() {
        let mut avl_tree = avl_tree!(10, 5, 15, 3, 7);

        // Duplicata na raiz, em nós internos e em folhas
        for &dup in &[10, 5, 15, 3, 7] {
            let res = avl_tree.insert(dup);
            assert!(matches!(res, Err(AvlTreeErr::InvalidInsertion)));
        }

        // Quantidade de nós e fatores de balanceamento devem permanecer intactos
        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 5);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_rotation_ll_descending_insertion() {
        // Inserção decrescente força rotações simples à direita (caso LL)
        let avl_tree = avl_tree!(30, 20, 10);

        for &val in &[10, 20, 30] {
            assert!(avl_tree.search(val));
        }
        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_rotation_rr_ascending_insertion() {
        // Inserção crescente força rotações simples à esquerda (caso RR)
        let avl_tree = avl_tree!(10, 20, 30);

        for &val in &[10, 20, 30] {
            assert!(avl_tree.search(val));
        }
        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_rotation_lr_double_rotation() {
        // Caso Left-Right (rotação dupla esquerda-direita)
        let avl_tree = avl_tree!(30, 10, 20);

        for &val in &[10, 20, 30] {
            assert!(avl_tree.search(val));
        }
        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_rotation_rl_double_rotation() {
        // Caso Right-Left (rotação dupla direita-esquerda)
        let avl_tree = avl_tree!(10, 30, 20);

        for &val in &[10, 20, 30] {
            assert!(avl_tree.search(val));
        }
        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_large_ascending_and_descending_insertions() {
        let mut asc_tree = AvlTree::new();
        let mut desc_tree = AvlTree::new();

        for i in 1..=50 {
            assert!(asc_tree.insert(i).is_ok());
            for bf in asc_tree.get_all_balance_factors() {
                assert!(bf.abs() <= 1);
            }
        }

        for i in (1..=50).rev() {
            assert!(desc_tree.insert(i).is_ok());
            for bf in desc_tree.get_all_balance_factors() {
                assert!(bf.abs() <= 1);
            }
        }

        assert_eq!(asc_tree.get_all_balance_factors().len(), 50);
        assert_eq!(desc_tree.get_all_balance_factors().len(), 50);

        for i in 1..=50 {
            assert!(asc_tree.search(i));
            assert!(desc_tree.search(i));
        }
    }

    #[test]
    fn test_search_non_existent_boundaries() {
        let avl_tree = avl_tree!(10, 20, 30, 40, 50);

        assert!(!avl_tree.search(0));   // Menor que o mínimo
        assert!(!avl_tree.search(15));  // Intermediário inexistente
        assert!(!avl_tree.search(25));  // Intermediário inexistente
        assert!(!avl_tree.search(35));  // Intermediário inexistente
        assert!(!avl_tree.search(60));  // Maior que o máximo
    }

    #[test]
    fn test_remove_leaf_node() {
        let mut avl_tree = avl_tree!(20, 10, 30);

        assert_eq!(avl_tree.remove(10), Some(10));
        assert!(!avl_tree.search(10));
        assert!(avl_tree.search(20));
        assert!(avl_tree.search(30));

        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 2);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_remove_node_with_one_child() {
        // Árvore: 20 na raiz, 10 à esquerda (com filho 5), 30 à direita (com filho 40)
        let mut avl_tree = avl_tree!(20, 10, 30, 5, 40);

        // Remove nó 10 (possui apenas filho esquerdo 5)
        assert_eq!(avl_tree.remove(10), Some(10));
        assert!(!avl_tree.search(10));
        for &v in &[20, 30, 5, 40] {
            assert!(avl_tree.search(v));
        }

        // Remove nó 30 (possui apenas filho direito 40)
        assert_eq!(avl_tree.remove(30), Some(30));
        assert!(!avl_tree.search(30));
        for &v in &[20, 5, 40] {
            assert!(avl_tree.search(v));
        }

        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_remove_node_with_two_children_deep_successor() {
        let values = [50, 25, 75, 10, 35, 60, 85, 55, 65];
        let mut avl_tree = AvlTree::new();
        for &v in &values {
            assert!(avl_tree.insert(v).is_ok());
        }

        // Remove 50 (possui dois filhos; sucessor in-order é 55, mais profundo na subárvore direita)
        assert_eq!(avl_tree.remove(50), Some(50));
        assert!(!avl_tree.search(50));

        for &v in &[25, 75, 10, 35, 60, 85, 55, 65] {
            assert!(avl_tree.search(v));
        }

        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), values.len() - 1);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_remove_two_children_unbalanced_subtrees() {
        // Raiz 20 com subárvore esquerda de altura 2 (10 -> 5) e direita de altura 1 (30).
        // Ao remover 20, o sucessor (30) é removido da direita, desencadeando rotação.
        let mut avl_tree = avl_tree!(20, 10, 30, 5);

        assert_eq!(avl_tree.remove(20), Some(20));
        assert!(!avl_tree.search(20));

        // Todos os demais nós (5, 10, 30) devem continuar presentes e acessíveis via busca
        for &v in &[5, 10, 30] {
            assert!(avl_tree.search(v));
        }

        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 3);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_remove_all_ascending_order() {
        let mut avl_tree = avl_tree!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

        for val in 1..=10 {
            assert_eq!(avl_tree.remove(val), Some(val));
            assert!(!avl_tree.search(val));

            // Verifica que todos os valores restantes ainda estão presentes na árvore
            for remaining in (val + 1)..=10 {
                assert!(avl_tree.search(remaining));
            }

            let bfactors = avl_tree.get_all_balance_factors();
            assert_eq!(bfactors.len(), (10 - val) as usize);
            for bf in bfactors {
                assert!(bf.abs() <= 1);
            }
        }
    }

    #[test]
    fn test_remove_rebalancing_rotations() {
        // Árvore que exige rebalanceamento (incluindo quando o irmão tem fator 0) após remoção
        let mut avl_tree = avl_tree!(10, 5, 20, 15, 25);

        assert_eq!(avl_tree.remove(5), Some(5));
        assert!(!avl_tree.search(5));

        for &v in &[10, 15, 20, 25] {
            assert!(avl_tree.search(v));
        }

        let bfactors = avl_tree.get_all_balance_factors();
        assert_eq!(bfactors.len(), 4);
        for bf in bfactors {
            assert!(bf.abs() <= 1);
        }
    }

    #[test]
    fn test_generic_string_elements() {
        let mut avl_tree = avl_tree!(
            String::from("delta"),
            String::from("alfa"),
            String::from("arcanjo"),
            String::from("bravo"),
            String::from("echo")
        );

        assert!(avl_tree.search(String::from("arcanjo")));
        assert!(!avl_tree.search(String::from("asmodeus")));

        assert_eq!(
            avl_tree.remove(String::from("arcanjo")),
            Some(String::from("arcanjo"))
        );
        assert!(!avl_tree.search(String::from("arcanjo")));

        for bf in avl_tree.get_all_balance_factors() {
            assert!(bf.abs() <= 1);
        }
    }

    fn assert_node_heights_consistent<T: Ord + Eq + Clone>(
        node: &crate::avl_tree::avl_tree::AvlNode<T>,
    ) -> usize {
        let left_h = node
            .left
            .as_ref()
            .map_or(0, |l| assert_node_heights_consistent(l));
        let right_h = node
            .right
            .as_ref()
            .map_or(0, |r| assert_node_heights_consistent(r));

        let expected_height = std::cmp::max(left_h, right_h) + 1;
        assert_eq!(node.get_height(), expected_height);
        assert!(
            (right_h as isize - left_h as isize).abs() <= 1,
            "Subárvore desbalanceada: altura esq = {}, altura dir = {}",
            left_h,
            right_h
        );
        assert_eq!(
            node.height, expected_height,
            "O campo `height` do nó ({}) diverge da altura real calculada ({})",
            node.height, expected_height
        );
        expected_height
    }

    #[test]
    fn test_tree_height_after_insert_and_remove() {
        let mut avl_tree = AvlTree::new();

        // 1 nó -> altura esperada = 1
        assert!(avl_tree.insert(1).is_ok());
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 1);

        // 2 nós -> altura esperada = 2
        assert!(avl_tree.insert(2).is_ok());
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 2);

        // 3 nós em sequência crescente (1, 2, 3) -> rotação à esquerda mantém altura = 2
        assert!(avl_tree.insert(3).is_ok());
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 2);

        // 4 a 7 nós (1..=7) -> rotações formam árvore completa de altura = 3
        for val in 4..=7 {
            assert!(avl_tree.insert(val).is_ok());
            assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 3);
        }

        // 8º nó -> altura deve subir para 4
        assert!(avl_tree.insert(8).is_ok());
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 4);

        // Removendo o 8º nó -> altura deve voltar para 3
        assert_eq!(avl_tree.remove(8), Some(8));
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 3);

        // Removendo nós até restarem apenas 3 elementos -> altura deve cair para 2
        for val in [7, 6, 5, 4] {
            assert_eq!(avl_tree.remove(val), Some(val));
        }
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 2);

        // Removendo até restar 1 elemento -> altura deve cair para 1
        assert_eq!(avl_tree.remove(3), Some(3));
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 2);
        assert_eq!(avl_tree.remove(2), Some(2));
        assert_eq!(avl_tree.root.as_ref().unwrap().get_height(), 1);

        // Removendo o último elemento -> árvore vazia (root = None)
        assert_eq!(avl_tree.remove(1), Some(1));
        assert!(avl_tree.root.is_none());
    }

    #[test]
    fn test_node_height_field_after_insert_and_remove() {
        let mut avl_tree = AvlTree::new();

        for val in 1..=15 {
            assert!(avl_tree.insert(val).is_ok());
            if let Some(ref root) = avl_tree.root {
                assert_node_heights_consistent(root);
            }
        }

        for val in (1..=15).rev() {
            assert_eq!(avl_tree.remove(val), Some(val));
            if let Some(ref root) = avl_tree.root {
                assert_node_heights_consistent(root);
            }
        }
    }
}