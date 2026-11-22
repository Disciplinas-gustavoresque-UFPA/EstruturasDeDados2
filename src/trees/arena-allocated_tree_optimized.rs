use crate::trees::arena_allocated_tree::NodeId;

pub struct CompactNode<T> {
    pub data: T,
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub prev_sibling: Option<NodeId>, // opcional, facilita remoção O(1)
}
