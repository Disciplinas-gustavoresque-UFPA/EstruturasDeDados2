/// Represents a point in the K-dimensional geological space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub coordinates: Vec<f64>,
    pub physics_loss: Option<f64>, 
}

impl Point {
    /// Calculates the squared Euclidean distance between two points.
    /// Omitting the square root (sqrt) massively optimizes performance during the search.
    pub fn squared_distance(&self, other: &Point) -> f64 {
        self.coordinates.iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }
}

/// A node within the tree managed by Arena Allocation (indices instead of mutable pointers).
#[derive(Debug)]
pub struct KdNode {
    pub point: Point,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

/// The main KD-Tree structure.
#[derive(Debug)]
pub struct KdTree {
    pub arena: Vec<KdNode>,
    pub root: Option<usize>,
    pub dimensions: usize,
}

impl KdTree {
    pub fn new(dimensions: usize) -> Self {
        KdTree {
            arena: Vec::new(),
            root: None,
            dimensions,
        }
    }

    pub fn size(&self) -> usize {
        self.arena.len()
    }

    /// Inserts a new point into the KD-Tree.
    /// The spatial partitioning axis alternates at each depth level.
    pub fn insert(&mut self, point: Point) {
        let new_node = KdNode {
            point: point.clone(),
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
            let current_point = &self.arena[current_idx].point;
            let go_left = point.coordinates[axis] < current_point.coordinates[axis];

            if go_left {
                match self.arena[current_idx].left {
                    Some(left_idx) => {
                        current_idx = left_idx;
                    }
                    None => {
                        self.arena[current_idx].left = Some(new_index);
                        break;
                    }
                }
            } else {
                match self.arena[current_idx].right {
                    Some(right_idx) => {
                        current_idx = right_idx;
                    }
                    None => {
                        self.arena[current_idx].right = Some(new_index);
                        break;
                    }
                }
            }
            depth += 1;
        }
    }

    /// Finds the nearest neighbor to a given target point.
    pub fn nearest(&self, target: &Point) -> Option<Point> {
        if self.root.is_none() {
            return None;
        }

        let mut best_idx = self.root.unwrap();
        let mut best_dist = f64::MAX;

        self.nearest_recursive(self.root.unwrap(), target, 0, &mut best_idx, &mut best_dist);

        Some(self.arena[best_idx].point.clone())
    }

    /// Recursive helper for the nearest neighbor search with backtracking.
    fn nearest_recursive(
        &self,
        current_idx: usize,
        target: &Point,
        depth: usize,
        best_idx: &mut usize,
        best_dist: &mut f64,
    ) {
        let axis = depth % self.dimensions;
        let current_node = &self.arena[current_idx];
        let current_point = &current_node.point;

        // Calculate the squared distance from the target to the current node
        let dist = target.squared_distance(current_point);

        // Update the best known point if the current one is closer
        if dist < *best_dist {
            *best_dist = dist;
            *best_idx = current_idx;
        }

        // Determine which branch to search first based on the current axis
        let go_left = target.coordinates[axis] < current_point.coordinates[axis];
        
        let (first_branch, second_branch) = if go_left {
            (current_node.left, current_node.right)
        } else {
            (current_node.right, current_node.left)
        };

        // Plunge down the most promising branch first
        if let Some(next_idx) = first_branch {
            self.nearest_recursive(next_idx, target, depth + 1, best_idx, best_dist);
        }

        // Backtracking: Check if the other side of the splitting plane could contain a closer point.
        let axis_dist = (target.coordinates[axis] - current_point.coordinates[axis]).powi(2);
        
        if axis_dist < *best_dist {
            if let Some(next_idx) = second_branch {
                self.nearest_recursive(next_idx, target, depth + 1, best_idx, best_dist);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdtree_insertion() {
        let mut tree = KdTree::new(2);

        // Insert points (simulating 2D geological coordinates)
        tree.insert(Point { coordinates: vec![3.0, 6.0], physics_loss: None });
        tree.insert(Point { coordinates: vec![17.0, 15.0], physics_loss: None });
        tree.insert(Point { coordinates: vec![13.0, 15.0], physics_loss: None });
        tree.insert(Point { coordinates: vec![6.0, 12.0], physics_loss: None });
        tree.insert(Point { coordinates: vec![9.0, 1.0], physics_loss: None });

        assert_eq!(tree.size(), 5);

        // Validate root
        let root_idx = tree.root.unwrap();
        let root_node = &tree.arena[root_idx];
        assert_eq!(root_node.point.coordinates, vec![3.0, 6.0]);
        
        // Validate right child placement (17.0 > 3.0 on X axis)
        assert!(root_node.right.is_some());
        let right_child = &tree.arena[root_node.right.unwrap()];
        assert_eq!(right_child.point.coordinates, vec![17.0, 15.0]);
    }

    #[test]
    fn test_nearest_neighbor_search() {
        let mut tree = KdTree::new(2);
        
        // Build a known spatial distribution
        let points = vec![
            vec![2.0, 3.0],
            vec![5.0, 4.0],
            vec![9.0, 6.0],
            vec![4.0, 7.0],
            vec![8.0, 1.0],
            vec![7.0, 2.0],
        ];

        for coords in points {
            tree.insert(Point { coordinates: coords, physics_loss: None });
        }

        // Scenario 1: Exact match (Target is already in the mesh)
        let target_exact = Point { coordinates: vec![9.0, 6.0], physics_loss: None };
        let nearest_exact = tree.nearest(&target_exact).unwrap();
        assert_eq!(nearest_exact.coordinates, vec![9.0, 6.0]);

        // Scenario 2: Spatial proximity 
        // Target (9.0, 2.0) is mathematically closest to (8.0, 1.0)
        let target_close = Point { coordinates: vec![9.0, 2.0], physics_loss: None };
        let nearest_close = tree.nearest(&target_close).unwrap();
        assert_eq!(nearest_close.coordinates, vec![8.0, 1.0]);

        // Scenario 3: Cross-boundary backtracking check
        // Target (3.0, 4.5) is closer to (2.0, 3.0) than (5.0, 4.0).
        // This forces the algorithm to correctly evaluate the splitting plane logic.
        let target_boundary = Point { coordinates: vec![3.0, 4.5], physics_loss: None };
        let nearest_boundary = tree.nearest(&target_boundary).unwrap();
        assert_eq!(nearest_boundary.coordinates, vec![2.0, 3.0]);
    }
}