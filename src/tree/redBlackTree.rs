use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color{
    Black,
    Red,
}

struct Node<T> {
    value: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
    color: Color
}

struct RedBlackTree<T> {
    root: Option<Box<Node<T>>>
}

impl<T> Node<T>{
    // função para criar um nó novo
    fn new(value: T) -> Self{
        Node{
            value,
            left: None,
            right: None,
            color: Color::Red
        }
    }
}

impl<T:Ord + Display> RedBlackTree<T>{
    fn new() -> Self{
        RedBlackTree {
            root: None
        }
    }

    fn is_red(node: &Option<Box<Node<T>>>) -> bool{
        matches!(node.as_ref(), Some(n) if n.color == Color::Red)
    }

    pub fn insert(&mut self, value: T){
        self.root = Self::insert_rec(self.root.take(), value);
        
        // Toda raiz é negra
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn recolor(mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(l) = node.left.as_mut() {l.color = Color::Black;}
        if let Some(r) = node.right.as_mut() {r.color = Color::Black;}
        node.color = Color::Red;
        node
    }

    fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>>{
        let mut new_root = node.left.take().expect("Filho esquerdo necessário");
        node.left = new_root.right.take();
        new_root.right = Some(node);
        new_root
    }

    fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>> {
        let mut new_root = node.right.take().expect("filho direito necessário");
        node.right = new_root.left.take();
        new_root.left = Some(node);
        new_root
    }

    fn balance(mut node: Box<Node<T>>) -> Box<Node<T>>{
        if node.color == Color::Red{return node;}

        let left_violation = Self::is_red(&node.left)
        && node.left.as_ref().is_some_and(|l| Self::is_red(&l.left) || Self::is_red(&l.right));

        if left_violation {
            if Self::is_red(&node.right) {
                return Self::recolor(node);
            }
            let mut l = node.left.take().unwrap();
            if Self::is_red(&l.right) {
                l = Self::rotate_left(l);
            }
            node.left = Some(l);
            node.color = Color::Red;
            if let Some(l) = node.left.as_mut() { l.color = Color::Black; }
            return Self::rotate_right(node);
        }

        let right_violation = Self::is_red(&node.right)
            && node.right.as_ref().is_some_and(|r| Self::is_red(&r.left) || Self::is_red(&r.right));

        if right_violation {
            if Self::is_red(&node.left) {
                return Self::recolor(node);
            }
            let mut r = node.right.take().unwrap();
            if Self::is_red(&r.left) {
                r = Self::rotate_right(r);
            }
            node.right = Some(r);
            node.color = Color::Red;
            if let Some(r) = node.right.as_mut() { r.color = Color::Black; }
            return Self::rotate_left(node);
        }
        node
    }

    // função de inserção recursiva
    fn insert_rec(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>>{
        match node {
            None => Some(Box::new(Node::new(value))),
            Some(mut cur_n) => {
                match value.cmp(&cur_n.value){
                    Ordering::Less =>{
                        cur_n.left = Self::insert_rec(cur_n.left.take(), value);
                    }
                    Ordering::Greater => {
                        cur_n.right = Self::insert_rec(cur_n.right.take(), value);
                    }
                    Ordering::Equal => {
                        return Some(cur_n);
                    }
                }
                Some(Self::balance(cur_n))
            }
        }
    }

 
    fn cor_letra(c: Color) -> &'static str {
            match c {
                Color::Black => "P",
                Color::Red => "V",
            }
        }

    fn imprimir_arvore(
        no: &Option<Box<Node<T>>>,
        prefixo: String,
        conector: &str
    ) {
        if let Some(n) = no {

            println!(
                "{}{}{}({})",
                prefixo,
                conector,
                n.value,
                Self::cor_letra(n.color)
            );

            let extensao = match conector {
                "" => "",
                "└── " => "    ",
                _ => "│   ",
            };

            let novo_prefixo = format!("{}{}", prefixo, extensao);

            Self::imprimir_arvore(
                &n.left,
                novo_prefixo.clone(),
                "├── "
            );

            Self::imprimir_arvore(
                &n.right,
                novo_prefixo,
                "└── "
            );
        }
    }
}

// testando a arvore
fn main() {
    let mut arvore: RedBlackTree<i32> = RedBlackTree::new();

    for v in [7, 2, 11, 1, 5, 8, 14, 4, 15] {
        arvore.insert(v);
    }

    RedBlackTree::imprimir_arvore(
        &arvore.root,
        String::new(),
        ""
    );
}