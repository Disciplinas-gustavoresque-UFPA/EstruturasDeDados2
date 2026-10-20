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

        if let Some(ref mut nodes) = self.nodes{
            
            if let Some(next_node) = nodes.get_mut(&curr_ch) {
                
                let can_delete = next_node.delete(ch);
                
                if can_delete {
                    nodes.remove(&curr_ch);
                }

            }
            false
        }   
        else{
            true
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
}
