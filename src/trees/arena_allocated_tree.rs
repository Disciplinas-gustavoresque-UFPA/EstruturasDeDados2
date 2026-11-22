// Um alias de tipo torna o código mais legível e seguro
pub type NodeId = usize;

pub struct ArenaTree<T> {
    nodes: Vec<Node<T>>,
}

pub struct Node<T> {
    pub data: T,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

impl<T> ArenaTree<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    // Cria um novo nó na arena e retorna seu índice (NodeId)
    pub fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();
        self.nodes.push(Node {
            data,
            parent: None,
            children: Vec::new(),
        });
        next_index
    }

    // Conecta um nó filho a um nó pai
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
    }

    // Acesso seguro a um nó pelo índice
    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }
}
