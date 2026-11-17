// Implementação da Árvore de Busca Ternária (TST)
// Autor: Juan Carlo Teles Martins

pub struct TstNode {
    ch: char,
    is_final: bool,
    left: Option<Box<TstNode>>,
    mid: Option<Box<TstNode>>,
    right: Option<Box<TstNode>>,
}

impl TstNode {
    fn new(ch: char) -> Self {
        TstNode {
            ch,
            is_final: false,
            left: None,
            mid: None,
            right: None,
        }
    }
}

pub struct TernarySearchTree {
    root: Option<Box<TstNode>>,
}

impl Default for TernarySearchTree {
    fn default() -> Self {
        Self::new()
    }
}

impl TernarySearchTree {
    pub fn new() -> Self {
        TernarySearchTree { root: None }
    }

    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        let chars: Vec<char> = word.chars().collect();
        self.root = Self::insert_node(self.root.take(), &chars, 0);
    }

    fn insert_node(
        node: Option<Box<TstNode>>,
        chars: &[char],
        index: usize,
    ) -> Option<Box<TstNode>> {
        let ch = chars[index];
        let mut current = node.unwrap_or_else(|| Box::new(TstNode::new(ch)));

        // Navegação manual de ponteiros puros (atendendo à exigência do professor)
        if ch < current.ch {
            current.left = Self::insert_node(current.left.take(), chars, index);
        } else if ch > current.ch {
            current.right = Self::insert_node(current.right.take(), chars, index);
        } else {
            if index + 1 < chars.len() {
                current.mid = Self::insert_node(current.mid.take(), chars, index + 1);
            } else {
                current.is_final = true;
            }
        }
        Some(current)
    }

    pub fn search(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }
        let chars: Vec<char> = word.chars().collect();
        Self::search_node(&self.root, &chars, 0)
    }

    fn search_node(node: &Option<Box<TstNode>>, chars: &[char], index: usize) -> bool {
        if let Some(current) = node {
            let ch = chars[index];
            if ch < current.ch {
                Self::search_node(&current.left, chars, index)
            } else if ch > current.ch {
                Self::search_node(&current.right, chars, index)
            } else {
                if index + 1 < chars.len() {
                    Self::search_node(&current.mid, chars, index + 1)
                } else {
                    current.is_final
                }
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_insercao_e_busca_tst() {
        let mut tst = TernarySearchTree::new();

        tst.insert("algoritmo");
        tst.insert("arvore");
        tst.insert("trie");
        tst.insert("rust");

        assert!(tst.search("algoritmo"));
        assert!(tst.search("arvore"));
        assert!(tst.search("trie"));
        assert!(tst.search("rust"));

        assert!(!tst.search("algo")); // Prefixo existe, mas não é palavra final
        assert!(!tst.search("grafos")); // Palavra inexistente
        assert!(!tst.search("")); // String vazia
    }

    #[test]
    fn testa_tst_visual() {
        let mut tst = TernarySearchTree::new();
        println!("\n___ TESTE VISUAL DA ÁRVORE TERNÁRIA (TST) ___");
        let palavras = ["car", "cat", "bug"];
        for p in &palavras {
            println!("-> Inserindo a palavra: '{}'", p);
            tst.insert(p);
        }

        println!(
            "\n-> Buscando palavra existente 'cat': {}",
            tst.search("cat")
        );
        println!("-> Buscando palavra incompleta 'ca': {}", tst.search("ca"));
        println!(
            "-> Buscando palavra inexistente 'carro': {}",
            tst.search("carro")
        );
        println!("_______________________________________________\n");
    }
}
