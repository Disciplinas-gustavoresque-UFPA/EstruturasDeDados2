use std::{mem::replace, vec};


#[derive(Clone)]
pub(crate) struct RadixNode{
    key: Vec<char>,
    is_final: bool,
    children: Vec<RadixNode>
}

pub struct RadixTree{
    root: RadixNode
}

impl RadixNode{
    pub(crate) fn new(key: Vec<char>, is_final: bool) -> Self{
        Self { 
            key: key, 
            is_final,
            children: Vec::new()
        }
    }

    pub(crate) fn insert(&mut self, key: &[char]){
        if key.len() == 0 {return;}
        for child in &mut self.children {

            let mut key_idx = 0;

            let mut match_all_chars = false;

            for idx in 0..child.key.len() {
                if key_idx >= key.len() {
                    let mut new_root = RadixNode::new(key.to_vec(), true);
                    let mut remainder = RadixNode::new(child.key[key_idx..].to_vec(), child.is_final);
                    remainder.children = child.children.clone();
                    new_root.children.push(remainder);
                    let _ = replace(child, new_root);
                    return;
                }

                if 
                    child.key[idx] != key[key_idx] 
                {

                    if idx != 0 {
                        let mut new_root = RadixNode::new(key[..idx].to_vec(), false);
                        let mut last_remainder = RadixNode::new(child.key[idx..].to_vec(), child.is_final);
                        let new_remainder = RadixNode::new(key[idx..].to_vec(), true);

                        last_remainder.children = child.children.clone();
                        new_root.children.push(new_remainder);
                        new_root.children.push(last_remainder);

                        let _ = replace(child, new_root);
                        return;
                    }

                    break;
                }

                key_idx += 1;

                if key_idx >= key.len() {
                    let mut new_root = RadixNode::new(key.to_vec(), true);
                    let mut remainder = RadixNode::new(child.key[key_idx..].to_vec(), child.is_final);
                    remainder.children = child.children.clone();
                    new_root.children.push(remainder);
                    let _ = replace(child, new_root);
                    return;
                }

                match_all_chars = idx == child.key.len() - 1;
            }

            if match_all_chars && key_idx == key.len() {return;}
            else if match_all_chars {
                child.insert(&key[key_idx..]);
                return;
            } 

        }

        self.children.push(RadixNode::new(key.to_vec(), true));
    }

    pub(crate) fn search(&self, key: &[char]) -> bool{
        if key.len() == 0 {return false;}

        for child in &self.children {

            let mut key_idx = 0;
            let mut match_all_chars = false;

            for idx in 0..child.key.len() {
                if key_idx >= key.len() {break;}
                if child.key[idx] != key[key_idx] {break;}

                key_idx += 1;

                match_all_chars = idx == child.key.len() - 1; 
            }

            if match_all_chars && key_idx == key.len() {
                return child.is_final;
            }
            else if match_all_chars {
                return child.search(&key[key_idx..].to_vec())
            }
        }

        false
    }

}

impl RadixTree{
    pub fn new() -> Self{
        Self { root: RadixNode::new(vec![], false) }
    }

    pub fn insert(&mut self, key: String){
        if key.len() == 0 {return;}
        let vec: Vec<char> = key.chars().collect();
        self.root.insert(&vec);
    }

    pub fn search(&self, key: String) -> bool{
        if key.len() == 0 {return false;}
        let vec: Vec<char> = key.chars().collect();
        self.root.search(&vec)
    }

}


#[macro_export]
macro_rules! radix_tree{
    ($($val:expr),* $(,)?) => {
        {
            let mut radix_tree = RadixTree::new();
            $(
                radix_tree.insert($val.to_string());
            )*
            radix_tree
        }
    };
}


#[cfg(test)]
mod test{
    use crate::{radix_tree::radix_tree::RadixTree, radix_tree};

    #[test]
    fn test_empty_tree() {
        let mut tree = RadixTree::new();

        assert!(!tree.search(String::from("")));
        assert!(!tree.search(String::from("qualquer")));

        // Inserção de string vazia deve ser ignorada
        tree.insert(String::from(""));
        assert!(!tree.search(String::from("")));
        assert!(tree.root.children.is_empty());
    }

    #[test]
    fn test_single_insertion_and_search() {
        let tree = radix_tree!("algoritmo");
        
        assert!(tree.search(String::from("algoritmo")));
        assert!(!tree.search(String::from("algo")));
        assert!(!tree.search(String::from("algoritmos")));
        assert!(!tree.search(String::from("al")));
        assert!(!tree.search(String::from("algori")));
    }

    #[test]
    fn test_disjoint_keys() {
        let tree = radix_tree!("casa", "bola", "dado", "faca");

        assert!(tree.search(String::from("casa")));
        assert!(tree.search(String::from("bola")));
        assert!(tree.search(String::from("dado")));
        assert!(tree.search(String::from("faca")));

        assert!(!tree.search(String::from("gato")));
        assert!(!tree.search(String::from("cas")));
        assert!(!tree.search(String::from("casas")));
    }

    #[test]
    fn test_split_shared_prefix() {
        // Caso clássico de Radix Tree com múltiplos níveis de divisão (split) de prefixos
        let words = [
            "romane",
            "romanus",
            "romulus",
            "rubens",
            "ruber",
            "rubicon",
            "rubicundus",
        ];
        let tree = radix_tree!(
            "romane",
            "romanus",
            "romulus",
            "rubens",
            "ruber",
            "rubicon",
            "rubicundus",
        );

        for &w in &words {
            assert!(
                tree.search(String::from(w)),
                "Esperava encontrar a palavra inserida: {}",
                w
            );
        }
        
        // Prefixos internos gerados por split (mas não inseridos como palavras) não devem retornar true
        for &prefix in &["r", "ro", "rom", "roman", "rub", "rubi", "rubic"] {
            assert!(
                !tree.search(String::from(prefix)),
                "Não esperava encontrar o prefixo não inserido: {}",
                prefix
            );
        }
    }

    #[test]
    fn test_insert_prefix_then_longer_words() {
        // Insere do menor prefixo para as palavras mais longas
        let tree = radix_tree!("car", "cart", "cartoon", "care");

        assert!(tree.search(String::from("car")));
        assert!(tree.search(String::from("cart")));
        assert!(tree.search(String::from("cartoon")));
        assert!(tree.search(String::from("care")));

        assert!(!tree.search(String::from("c")));
        assert!(!tree.search(String::from("ca")));
        assert!(!tree.search(String::from("cartoo")));
        assert!(!tree.search(String::from("cartoons")));
    }

    #[test]
    fn test_insert_longer_word_then_shorter_prefix() {
        // Insere primeiro a palavra mais longa e depois seus prefixos menores (força split por tamanho)
        let tree = radix_tree!("cartoon", "cart", "car");

        assert!(tree.search(String::from("cartoon")));
        assert!(tree.search(String::from("cart")));
        assert!(tree.search(String::from("car")));

        assert!(!tree.search(String::from("ca")));
        assert!(!tree.search(String::from("cartoo")));
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut tree = radix_tree!("rust", "romane", "romanus");

        // Reinsere palavras já existentes na raiz e em nós internos/folhas
        tree.insert(String::from("rust"));
        tree.insert(String::from("romane"));
        tree.insert(String::from("romanus"));

        assert!(tree.search(String::from("rust")));
        assert!(tree.search(String::from("romane")));
        assert!(tree.search(String::from("romanus")));
    }

    #[test]
    fn test_single_character_keys_and_extensions() {
        let tree = radix_tree!("a", "b", "ab", "abc", "ac");

        for &w in &["a", "b", "ab", "abc", "ac"] {
            assert!(tree.search(String::from(w)));
        }

        assert!(!tree.search(String::from("c")));
        assert!(!tree.search(String::from("abcd")));
    }

    #[test]
    fn test_case_sensitivity_and_unicode() {
        let tree = radix_tree!("Radix", "radix", "árvore", "área");

        assert!(tree.search(String::from("Radix")));
        assert!(tree.search(String::from("radix")));
        assert!(!tree.search(String::from("RADIX")));

        assert!(tree.search(String::from("árvore")));
        assert!(tree.search(String::from("área")));
        assert!(!tree.search(String::from("ár")));
    }
}

