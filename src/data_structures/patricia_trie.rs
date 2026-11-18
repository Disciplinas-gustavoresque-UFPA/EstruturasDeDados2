// Implementação da Patricia Trie (Radix Tree) otimizada
// Foco em Zero-Allocation de Strings auxiliares nos parâmetros (usando fatias de bytes &[u8])
// Autor: Juan Carlo Teles Martins

use std::mem;

pub struct PatriciaNode {
    prefix: Vec<u8>,
    is_final: bool,
    children: Vec<PatriciaNode>,
}

impl PatriciaNode {
    fn new(prefix: Vec<u8>, is_final: bool) -> Self {
        PatriciaNode {
            prefix,
            is_final,
            children: Vec::new(),
        }
    }
}

pub struct PatriciaTrie {
    root: PatriciaNode,
}

impl Default for PatriciaTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl PatriciaTrie {
    pub fn new() -> Self {
        PatriciaTrie {
            root: PatriciaNode::new(Vec::new(), false),
        }
    }

    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        Self::insert_node(&mut self.root, word.as_bytes());
    }

    fn insert_node(node: &mut PatriciaNode, mut word_bytes: &[u8]) {
        let mut current = node;

        'search_loop: while !word_bytes.is_empty() {
            for i in 0..current.children.len() {
                // Isolamos o empréstimo imutável (borrow) num escopo seguro
                let (common_len, child_prefix_len) = {
                    let child_prefix = &current.children[i].prefix;
                    let common_len = word_bytes
                        .iter()
                        .zip(child_prefix.iter())
                        .take_while(|(a, b)| a == b)
                        .count();
                    (common_len, child_prefix.len())
                };

                if common_len > 0 {
                    if common_len == child_prefix_len {
                        current = &mut current.children[i];
                        word_bytes = &word_bytes[common_len..];
                        continue 'search_loop;
                    } else {
                        // Extrai a cópia do prefixo antes de fazer o empréstimo mutável
                        let original_prefix = current.children[i].prefix.clone();
                        let remaining_child_prefix = original_prefix[common_len..].to_vec();

                        let mut split_node =
                            PatriciaNode::new(remaining_child_prefix, current.children[i].is_final);

                        // Agora a mutação é 100% segura para o compilador
                        split_node.children = mem::take(&mut current.children[i].children);

                        current.children[i].prefix = original_prefix[..common_len].to_vec();
                        current.children[i].is_final = common_len == word_bytes.len();
                        current.children[i].children = vec![split_node];

                        if common_len < word_bytes.len() {
                            let new_leaf =
                                PatriciaNode::new(word_bytes[common_len..].to_vec(), true);
                            current.children[i].children.push(new_leaf);
                        }
                        return;
                    }
                }
            }

            current
                .children
                .push(PatriciaNode::new(word_bytes.to_vec(), true));
            return;
        }

        current.is_final = true;
    }

    pub fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }
        Self::search_node(&self.root, word.as_bytes())
    }

    fn search_node(node: &PatriciaNode, mut word_bytes: &[u8]) -> bool {
        let mut current = node;

        'search_loop: while !word_bytes.is_empty() {
            for child in &current.children {
                let child_prefix = &child.prefix;

                if word_bytes.starts_with(child_prefix) {
                    current = child;
                    word_bytes = &word_bytes[child_prefix.len()..];
                    continue 'search_loop;
                }
            }
            return false;
        }

        current.is_final
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_insercao_busca_zero_allocation() {
        let mut patricia = PatriciaTrie::new();

        let palavras = [
            "romane",
            "romanus",
            "romulus",
            "rubens",
            "ruber",
            "rubicon",
            "rubicundus",
        ];
        for p in palavras {
            patricia.insert(p);
        }

        for p in palavras {
            assert!(patricia.search(p));
        }

        assert!(!patricia.search("rom"));
        assert!(!patricia.search("rub"));
        assert!(!patricia.search(""));
    }
}
