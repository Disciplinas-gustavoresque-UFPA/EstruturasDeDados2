use std::{
    collections::HashMap,
    str::Chars
};


struct TrieNode{    
    is_final: bool,
    nodes: Option<HashMap<char, Box<TrieNode>>>
}

pub struct Trie{
    root: Option<TrieNode>
}

impl TrieNode{
    pub(crate) fn new() -> Self{
        Self{
            is_final: false,
            nodes: None
        }
    }

    pub(crate) fn insert<'a>(
        &mut self, 
        st: &'a str
    ){

        let mut curr = self;

        for ch in st.chars(){
            let new_curr;
            
            if let Some(ref mut nodes) = curr.nodes{
                if nodes.contains_key(&ch) {
                    new_curr = Some(
                        nodes
                        .get_mut(&ch)
                        .unwrap()
                    );
                }
                else {
                    nodes.insert(ch, Box::new(TrieNode::new()));
                    new_curr = Some(
                        nodes
                        .get_mut(&ch)
                        .unwrap()
                    );
                }
            }
            else {
                let mut nodes = HashMap::new();
                
                nodes.insert(ch, Box::new(TrieNode::new()));
                
                curr.nodes = Some(nodes);
                
                new_curr = Some( 
                    curr
                    .nodes
                    .as_mut()
                    .unwrap()
                    .get_mut(&ch)
                    .unwrap()
                );
            }

            curr = new_curr.unwrap();

        }

        curr.is_final = true;

    }

    pub(crate) fn search<'a>(
        &self, 
        st: &'a str
    ) -> bool{
        let mut curr = self;
        
        for ch in st.chars(){
            if 
                let Some(ref nodes) = curr.nodes &&
                nodes.contains_key(&ch)
            {
                curr = &nodes[&ch];
            }
            else {
                return false;
            }
        }

        curr.is_final
    }

    fn delete<'a>(
        &mut self, 
        mut ch: Chars<'a>
    ) -> bool{
        let curr_ch = ch.next();
      
        if curr_ch.is_none(){ 
            self.is_final = false;
            return self.nodes.is_none();
        }

        let curr_ch = curr_ch.unwrap();

        if let Some(mut nodes) = self.nodes.take() {
            
            if let Some(next_node) = nodes.get_mut(&curr_ch) {
                
                let can_delete = next_node.delete(ch);
                
                if can_delete {
                    nodes.remove(&curr_ch);
                }

            }

            if !nodes.is_empty() {
                self.nodes = Some(nodes);
            }

            self.nodes.is_none() && !self.is_final
        }   
        else{
            false
        }
    }
}

impl Trie{
    pub fn new() -> Self{
        Self{
            root: None
        }
    }

    pub fn insert<'a>(
        &mut self, 
        st: &'a str
    ){
        if let Some(ref mut root ) = self.root{
            root.insert(st);
        }
        else{
            let mut root = TrieNode::new();
            root.insert(st);
            self.root = Some(root);
        }
    }

    pub fn search<'a>(
        &self, 
        st:&'a str
    ) -> bool{
        if let Some(ref root) = self.root{
            root.search(st)
        }
        else{
            false
        }
    }

    pub fn delete<'a>(
        &mut self,
        st: &'a str
    ){
        if let Some(mut root) = self.root.take(){
            if !root.delete(st.chars()) {
                self.root = Some(root);
            }
        }
    }


}

#[macro_export]
macro_rules! trie {
    ($($st:literal),* $(,)?) => {
        {
            let mut trie = Trie::new();
            $(
                trie.insert($st);
            )*
            trie
        }
    };
}

#[cfg(test)]
mod trie_tests{
    use crate::trie::trie::Trie;

    #[test]
    fn test_insert(){
        
        let trie = trie!("hello","hello world", "good", "good morning");
        assert!(
            trie.search("hello") &&
            trie.search("hello world") &&
            trie.search("good") &&
            trie.search("good morning")
        );
        
    }

    #[test]
    fn test_invalid_insert(){
        let trie = trie!("hello", "good");
        assert!(
            !trie.search("hello world") &&
            !trie.search("good, ") &&
            !trie.search("okay")
        );
    }

    #[test]
    fn test_remove(){
        let mut trie = trie!("he", "helo");

        trie.delete("helo");

        assert!(
            !trie.search("helo")
        );

        trie.delete("he");

        assert!(
            !trie.search("he")
        );

    }

    #[test]
    fn test_empty_trie() {
        let mut trie = Trie::new();

        assert!(!trie.search(""));
        assert!(!trie.search("qualquer"));

        // Remover de uma Trie vazia não deve causar panic
        trie.delete("inexistente");
        assert!(!trie.search("inexistente"));
        assert!(trie.root.is_none());
    }

    #[test]
    fn test_prefix_not_marked_as_word() {
        let trie = trie!("algoritmo", "estrutura");

        // Prefixos parciais não foram inseridos como palavras completas
        assert!(!trie.search("a"));
        assert!(!trie.search("algo"));
        assert!(!trie.search("algoritm"));
        assert!(!trie.search("estru"));

        // Palavras maiores que as inseridas também não devem existir
        assert!(!trie.search("algoritmos"));
        assert!(!trie.search("estruturas"));

        // As palavras exatas devem existir
        assert!(trie.search("algoritmo"));
        assert!(trie.search("estrutura"));
    }

    #[test]
    fn test_delete_prefix_preserves_longer_words() {
        let mut trie = trie!("he", "hell", "hello", "heavy");

        // Remove o prefixo curto "he" e verifica que as palavras maiores continuam intactas
        trie.delete("he");
        assert!(!trie.search("he"));
        assert!(trie.search("hell"));
        assert!(trie.search("hello"));
        assert!(trie.search("heavy"));

        // Remove o prefixo intermediário "hell"
        trie.delete("hell");
        assert!(!trie.search("hell"));
        assert!(trie.search("hello"));
        assert!(trie.search("heavy"));
    }

    #[test]
    fn test_delete_longer_word_preserves_shorter_prefix() {
        let mut trie = trie!("car", "cart", "cartoon");

        trie.delete("cartoon");
        assert!(!trie.search("cartoon"));
        assert!(trie.search("car"));
        assert!(trie.search("cart"));

        trie.delete("cart");
        assert!(!trie.search("cart"));
        assert!(trie.search("car"));
    }

    #[test]
    fn test_delete_branch_preserves_siblings() {
        let mut trie = trie!("cat", "car", "cab", "dog", "dot");

        trie.delete("car");
        assert!(!trie.search("car"));
        assert!(trie.search("cat"));
        assert!(trie.search("cab"));
        assert!(trie.search("dog"));
        assert!(trie.search("dot"));

        trie.delete("dog");
        assert!(!trie.search("dog"));
        assert!(trie.search("dot"));
    }

    #[test]
    fn test_delete_non_existent_words() {
        let mut trie = trie!("hello", "world");

        // Tenta deletar palavra totalmente inexistente
        trie.delete("rust");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        // Tenta deletar prefixo que não é palavra final
        trie.delete("hel");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        // Tenta deletar palavra com 1 caractere a mais que uma folha existente
        trie.delete("helloo");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        // Tenta deletar palavra com vários caracteres a mais que uma folha existente
        trie.delete("helloooo");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("hel"));
    }

    #[test]
    fn test_duplicate_insertion_and_reinsertion() {
        let mut trie = Trie::new();

        trie.insert("teste");
        trie.insert("teste");
        assert!(trie.search("teste"));

        // Uma única remoção deve desmarcar a palavra mesmo após inserção duplicada
        trie.delete("teste");
        assert!(!trie.search("teste"));

        // Reinserção após remoção
        trie.insert("teste");
        assert!(trie.search("teste"));
    }

    #[test]
    fn test_empty_string_operations() {
        let mut trie = Trie::new();

        trie.insert("");
        assert!(trie.search(""));
        assert!(!trie.search("a"));

        trie.insert("a");
        assert!(trie.search(""));
        assert!(trie.search("a"));

        trie.delete("");
        assert!(!trie.search(""));
        assert!(trie.search("a"));
    }

    #[test]
    fn test_unicode_and_case_sensitivity() {
        let mut trie = trie!("Rust", "rust", "árvore", "ação", "🦀");

        assert!(trie.search("Rust"));
        assert!(trie.search("rust"));
        assert!(!trie.search("RUST"));

        assert!(trie.search("árvore"));
        assert!(trie.search("ação"));
        assert!(trie.search("🦀"));

        trie.delete("árvore");
        assert!(!trie.search("árvore"));
        assert!(trie.search("ação"));
    }

    #[test]
    fn test_physical_node_pruning_after_deleting_all_words() {
        let mut trie = trie!("abc", "abd");

        trie.delete("abc");
        assert!(!trie.search("abc"));
        assert!(trie.search("abd"));

        trie.delete("abd");
        assert!(!trie.search("abd"));

        // Quando todas as palavras são removidas da Trie, os nós órfãos e a raiz devem ser limpos
        assert!(trie.root.is_none());
    }
}
