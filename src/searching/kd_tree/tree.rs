pub trait Tree {
    fn new(dimensions: usize) -> Self;
    fn insert(&mut self, coordinates: Vec<f64>);
    fn nearest(&self, coordinates: &Vec<f64>) -> Option<Vec<f64>>;
}

/// Calcula a distância euclidiana ao quadrado entre dois pontos.
/// Omitir a raiz quadrada (sqrt) otimiza drasticamente o desempenho durante a procura.
fn squared_distance(coordinates: &Vec<f64>, other_coordinates: &Vec<f64>) -> f64 {
    coordinates.iter()
        .zip(other_coordinates.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum()
}

/// Um nó dentro da árvore gerido por Alocação de Arena (Arena Allocation).
/// Utiliza índices em vez de ponteiros mutáveis para garantir a segurança de memória em Rust.
#[derive(Debug)]
struct KdNode {
    coordinates: Vec<f64>,
    left: Option<usize>,
    right: Option<usize>,
}

/// A estrutura principal da KD-Tree. Os campos são privados para garantir o encapsulamento.
#[derive(Debug)]
pub struct KdTree {
    arena: Vec<KdNode>,
    root: Option<usize>,
    dimensions: usize,
}

impl KdTree {
    /// Função auxiliar recursiva para a procura do vizinho mais próximo com retrocesso (backtracking).
    fn nearest_recursive(
        &self,
        current_idx: usize,
        coordinates: &Vec<f64>,
        depth: usize,
        best_idx: &mut usize,
        best_dist: &mut f64,
    ) {
        let axis = depth % self.dimensions;
        let current_node = &self.arena[current_idx];
        let current_point = &current_node.coordinates;

        let dist = squared_distance(coordinates, current_point);

        if dist < *best_dist {
            *best_dist = dist;
            *best_idx = current_idx;
        }

        let go_left = coordinates[axis] < current_point[axis];
        
        let (first_branch, second_branch) = if go_left {
            (current_node.left, current_node.right)
        } else {
            (current_node.right, current_node.left)
        };

        if let Some(next_idx) = first_branch {
            self.nearest_recursive(next_idx, coordinates, depth + 1, best_idx, best_dist);
        }

        let axis_dist = (coordinates[axis] - current_point[axis]).powi(2);
        
        if axis_dist < *best_dist {
            if let Some(next_idx) = second_branch {
                self.nearest_recursive(next_idx, coordinates, depth + 1, best_idx, best_dist);
            }
        }
    }
}

impl Tree for KdTree {
    fn new(dimensions: usize) -> Self {
        KdTree {
            arena: Vec::new(),
            root: None,
            dimensions,
        }
    }

    /// Insere um novo ponto na KD-Tree.
    fn insert(&mut self, coordinates: Vec<f64>) {
        let new_node = KdNode {
            coordinates: coordinates.clone(),
            left: None,
            right: None,
        };

        let new_index = self.arena.len();
        self.arena.push(new_node);

        if self.root.is_none() {
            self.root = Some(new_index);
            return;
        }

        let mut current_idx = self.root.unwrap();
        let mut depth = 0;

        loop {
            let axis = depth % self.dimensions;
            let current_point = &self.arena[current_idx].coordinates;
            let go_left = coordinates[axis] < current_point[axis];

            if go_left {
                match self.arena[current_idx].left {
                    Some(left_idx) => current_idx = left_idx,
                    None => {
                        self.arena[current_idx].left = Some(new_index);
                        break;
                    }
                }
            } else {
                match self.arena[current_idx].right {
                    Some(right_idx) => current_idx = right_idx,
                    None => {
                        self.arena[current_idx].right = Some(new_index);
                        break;
                    }
                }
            }
            depth += 1;
        }
    }

    /// Encontra o vizinho mais próximo de um determinado ponto alvo.
    fn nearest(&self, coordinates: &Vec<f64>) -> Option<Vec<f64>> {
        if self.root.is_none() {
            return None;
        }

        let mut best_idx = self.root.unwrap();
        let mut best_dist = f64::MAX;

        self.nearest_recursive(self.root.unwrap(), coordinates, 0, &mut best_idx, &mut best_dist);

        Some(self.arena[best_idx].coordinates.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{KdTree, Tree};

    #[test]
    fn test_kdtree_insertion() {
        let mut tree: KdTree = Tree::new(2);

        tree.insert(vec![3.0, 6.0]);
        tree.insert(vec![17.0, 15.0]);
        tree.insert(vec![13.0, 15.0]);
        tree.insert(vec![6.0, 12.0]);
        tree.insert(vec![9.0, 1.0]);

        assert_eq!(tree.arena.len(), 5);

        let root_idx = tree.root.unwrap();
        let root_node = &tree.arena[root_idx];
        assert_eq!(root_node.coordinates, vec![3.0, 6.0]);
    }

    #[test]
    fn test_nearest_neighbor_search() {
        let mut tree: KdTree = Tree::new(2);
        
        let points = vec![
            vec![2.0, 3.0], vec![5.0, 4.0], vec![9.0, 6.0],
            vec![4.0, 7.0], vec![8.0, 1.0], vec![7.0, 2.0],
        ];

        for coords in points {
            tree.insert(coords);
        }

        let target_exact = vec![9.0, 6.0];
        let nearest_exact = tree.nearest(&target_exact).unwrap();
        assert_eq!(nearest_exact, vec![9.0, 6.0]);
    }
}