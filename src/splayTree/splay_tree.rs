#[derive(Debug)]
struct Node<K> {
    key: K,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct SplayTree<K> {
    pub arena: Vec<Node<K>>,
    pub root: Option<usize>,
}

impl<K: Ord + Clone> SplayTree<K> {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            root: None,
        }
    }

    pub fn new_node(&mut self, key: K) -> usize {
        let idx = self.arena.len();
        self.arena.push(Node {
            key,
            left: None,
            right: None,
        });
        return idx;
    }

    fn rot_right(&mut self, x: usize) -> usize {
        let y = self.arena[x].left.unwrap();
        self.arena[x].left = self.arena[y].right;
        self.arena[y].right = Some(x);
        return y;
    }

    fn rot_left(&mut self, x: usize) -> usize {
        let y = self.arena[x].right.unwrap();
        self.arena[x].right = self.arena[y].left;
        self.arena[y].left = Some(x);
        return y;
    }

    fn splay(&mut self, value: K) {
        let Some(mut curr) = self.root else {
            return;
        };

        let mut l_root: Option<usize> = None;
        let mut r_root: Option<usize> = None;

        let mut l_attach: Option<usize> = None;
        let mut r_attach: Option<usize> = None;

        loop {
            if value < self.arena[curr].key {
                let Some(left) = self.arena[curr].left else {
                    break;
                };

                // Zig-zig
                if value < self.arena[left].key {
                    curr = self.rot_right(curr);
                    if self.arena[curr].left.is_none() {
                        break;
                    }
                }

                if let Some(r) = r_attach {
                    self.arena[r].left = Some(curr);
                } else {
                    r_root = Some(curr);
                }
                r_attach = Some(curr);
                curr = self.arena[curr].left.unwrap();
            } else if value > self.arena[curr].key {
                let Some(right) = self.arena[curr].right else {
                    break;
                };

                // Zag-zag
                if value > self.arena[right].key {
                    curr = self.rot_left(curr);
                    if self.arena[curr].right.is_none() {
                        break;
                    }
                }

                if let Some(l) = l_attach {
                    self.arena[l].right = Some(curr);
                } else {
                    l_root = Some(curr);
                }
                l_attach = Some(curr);
                curr = self.arena[curr].right.unwrap();
            } else {
                break;
            }
        }
        if let Some(l) = l_attach {
            self.arena[l].right = self.arena[curr].left;
        } else {
            l_root = self.arena[curr].left;
        }

        if let Some(r) = r_attach {
            self.arena[r].left = self.arena[curr].right;
        } else {
            r_root = self.arena[curr].right;
        }

        self.arena[curr].left = l_root;
        self.arena[curr].right = r_root;

        self.root = Some(curr);
    }
}

#[cfg(test)]
mod tests {
    use super::SplayTree;

    #[test]
    fn test_rot_right() {
        let mut tree = SplayTree::new();

        let root_id = tree.new_node(20);
        let left_id = tree.new_node(10);

        tree.arena[root_id].left = Some(left_id);
        tree.root = Some(root_id);

        let new_sub_root = tree.rot_right(root_id);

        assert_eq!(new_sub_root, left_id);
        assert_eq!(tree.arena[left_id].right, Some(root_id));
        assert_eq!(tree.arena[root_id].left, None);
    }

    #[test]
    fn test_rot_left() {
        let mut tree = SplayTree::new();

        let root_id = tree.new_node(10);
        let right_id = tree.new_node(20);

        tree.arena[root_id].right = Some(right_id);
        tree.root = Some(root_id);

        let new_sub_root = tree.rot_left(root_id);

        assert_eq!(new_sub_root, right_id);
        assert_eq!(tree.arena[right_id].left, Some(root_id));
        assert_eq!(tree.arena[root_id].right, None);
    }

    #[test]
    fn test_splay() {
        let mut tree = SplayTree::new();
        let n20 = tree.new_node(20);
        let n10 = tree.new_node(10);
        let n30 = tree.new_node(30);

        tree.arena[n20].left = Some(n10);
        tree.arena[n20].right = Some(n30);
        tree.root = Some(n20);

        tree.splay(10);

        assert_eq!(tree.root, Some(n10));
        assert_eq!(tree.arena[n10].right, Some(n20));
        assert_eq!(tree.arena[n20].right, Some(n30));
        assert_eq!(tree.arena[n20].left, None);
    }

    #[test]
    fn test_splay_zig_zig() {
        let mut tree = SplayTree::new();
        let n30 = tree.new_node(30);
        let n20 = tree.new_node(20);
        let n10 = tree.new_node(10);

        tree.arena[n30].left = Some(n20);
        tree.arena[n20].left = Some(n10);
        tree.root = Some(n30);

        tree.splay(10);

        assert_eq!(tree.root, Some(n10));
        assert_eq!(tree.arena[n10].right, Some(n20));
        assert_eq!(tree.arena[n20].right, Some(n30));
    }

    #[test]
    fn test_splay_none_value() {
        let mut tree = SplayTree::new();
        let n20 = tree.new_node(20);
        let n10 = tree.new_node(10);
        let n30 = tree.new_node(30);

        tree.arena[n20].left = Some(n10);
        tree.arena[n20].right = Some(n30);
        tree.root = Some(n20);

        tree.splay(25);

        assert_eq!(tree.root, Some(n30));
        assert_eq!(tree.arena[n30].left, Some(n20));
        assert_eq!(tree.arena[n20].left, Some(n10));
    }
}
