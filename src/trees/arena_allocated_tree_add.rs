use crate::trees::arena_allocated_tree::NodeId;

pub struct CompactNode<T> {
    pub data: T,
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub prev_sibling: Option<NodeId>,
}

pub struct CompactArenaTree<T> {
    nodes: Vec<CompactNode<T>>,
}

impl<T> CompactArenaTree<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// O(1): insere o novo nó como PRIMEIRO filho do pai.
    /// Retorna `None` se `parent` for um índice inválido.
    pub fn add(&mut self, parent: Option<NodeId>, data: T) -> Option<NodeId> {
        if let Some(p) = parent {
            if p >= self.nodes.len() {
                return None;
            }
        }
        let id = self.nodes.len();
        let old_first = parent.and_then(|p| self.nodes[p].first_child);

        self.nodes.push(CompactNode {
            data,
            parent,
            first_child: None,
            next_sibling: old_first,
            prev_sibling: None,
        });

        if let Some(p) = parent {
            if let Some(f) = old_first {
                self.nodes[f].prev_sibling = Some(id);
            }
            self.nodes[p].first_child = Some(id);
        }
        Some(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_root() {
        let mut tree = CompactArenaTree::new();
        let root = tree.add(None, "raiz").unwrap();
        assert_eq!(root, 0);
        assert_eq!(tree.nodes[root].parent, None);
    }

    #[test]
    fn add_children_links_siblings() {
        let mut tree = CompactArenaTree::new();
        let root = tree.add(None, "raiz").unwrap();
        let a = tree.add(Some(root), "a").unwrap();
        let b = tree.add(Some(root), "b").unwrap();

        // b foi inserido por último, então é o primeiro filho
        assert_eq!(tree.nodes[root].first_child, Some(b));
        assert_eq!(tree.nodes[b].next_sibling, Some(a));
        assert_eq!(tree.nodes[a].prev_sibling, Some(b));
        assert_eq!(tree.nodes[a].parent, Some(root));
    }

    #[test]
    fn add_with_invalid_parent_returns_none() {
        let mut tree: CompactArenaTree<i32> = CompactArenaTree::new();
        assert_eq!(tree.add(Some(42), 1), None);
        assert!(tree.nodes.is_empty()); // nenhum nó órfão criado
    }
}
