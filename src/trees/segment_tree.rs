pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    /// Constrói a Segment Tree a partir de um slice.
    /// Complexidade: O(n) tempo e espaço.
    pub fn build(arr: &[i64]) -> Self{
        let n = arr.len();
        let mut tree = vec![0i64; 4 * n];
        if n > 0{
            Self::build_recursive(&mut tree, arr, 1, 0, n-1);
        }

        SegmentTree{n, tree}
    }

    fn build_recursive(tree: &mut Vec<i64>, arr: &[i64], node: usize, start: usize, end: usize){
        if start == end {
            tree[node] = arr[start];
        } else{
            let mid = (start + end) / 2;
            Self::build_recursive(tree, arr, 2 * node, start, mid);
            Self::build_recursive(tree, arr, 2 * node + 1, mid + 1, end);
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
    }

    /// Retorna a soma do intervalo [l, r] (inclusivo em ambos os extremos).
    /// Complexidade: O(log n).
    pub fn query(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "índices fora do intervalo válido");
        self.query_recursive(1, 0, self.n - 1, l, r)
    }

    fn query_recursive(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if end < l || start > r {
            return 0; // completamente fora
        }
        if l <= start && end <= r {
            return self.tree[node]; // completamente dentro
        }
        let mid = (start + end) / 2;
        let left  = self.query_recursive(2 * node,     start,   mid, l, r);
        let right = self.query_recursive(2 * node + 1, mid + 1, end, l, r);
        left + right
    }

    /// Atualiza o elemento na posição i para o valor val.
    /// Complexidade: O(log n).
    pub fn update(&mut self, i: usize, val: i64) {
        assert!(i < self.n, "índice fora do intervalo válido");
        self.update_recursive(1, 0, self.n - 1, i, val);
    }

    fn update_recursive(&mut self, node: usize, start: usize, end: usize, i: usize, val: i64) {
        if start == end {
            self.tree[node] = val; // folha encontrada
        } else {
            let mid = (start + end) / 2;
            if i <= mid {
                self.update_recursive(2 * node,     start,   mid, i, val);
            } else {
                self.update_recursive(2 * node + 1, mid + 1, end, i, val);
            }
            // recalcula o nó atual ao subir na recursão
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── construção ──────────────────────────────────────────────────────────

    #[test]
    fn build_um_elemento() {
        let st = SegmentTree::build(&[42]);
        assert_eq!(st.query(0, 0), 42);
    }

    #[test]
    fn build_nao_potencia_de_dois() {
        // n=5 força nós internos com filhos "vazios" e testa o 4*n de folga
        let st = SegmentTree::build(&[10, 20, 30, 40, 50]);
        assert_eq!(st.query(0, 4), 150);
    }

    // ── query ───────────────────────────────────────────────────────────────

    #[test]
    fn query_intervalo_inteiro() {
        let st = SegmentTree::build(&[2, 4, 3, 1, 6, 5]);
        assert_eq!(st.query(0, 5), 21);
    }

    #[test]
    fn query_elemento_unico_em_cada_extremo() {
        let st = SegmentTree::build(&[2, 4, 3, 1, 6, 5]);
        assert_eq!(st.query(0, 0), 2);
        assert_eq!(st.query(5, 5), 5);
    }

    #[test]
    fn query_cruzando_ponto_de_divisao() {
        // [1,4] cruza o mid da raiz — força o caso parcialmente sobreposto
        let st = SegmentTree::build(&[2, 4, 3, 1, 6, 5]);
        assert_eq!(st.query(1, 4), 14);
    }

    #[test]
    fn query_valores_negativos_e_mistos() {
        let st = SegmentTree::build(&[-3, 2, -1, 4, -2]);
        assert_eq!(st.query(0, 4), 0);
        assert_eq!(st.query(1, 3), 5);
    }

    #[test]
    fn query_todas_as_combinacoes() {
        // compara a segment tree com soma direta para todo [l, r] possível
        let arr = [2i64, 4, 3, 1, 6, 5];
        let st = SegmentTree::build(&arr);
        for l in 0..arr.len() {
            for r in l..arr.len() {
                let esperado: i64 = arr[l..=r].iter().sum();
                assert_eq!(st.query(l, r), esperado, "query({l},{r}) falhou");
            }
        }
    }

    // ── update ──────────────────────────────────────────────────────────────

    #[test]
    fn update_propaga_para_todos_os_ancestrais() {
        let mut st = SegmentTree::build(&[1, 1, 1, 1, 1, 1]);
        st.update(5, 100);
        assert_eq!(st.query(0, 5), 105);
        assert_eq!(st.query(3, 5), 102);
        assert_eq!(st.query(5, 5), 100);
    }

    #[test]
    fn update_nao_afeta_indices_adjacentes() {
        let mut st = SegmentTree::build(&[10, 20, 30, 40, 50]);
        st.update(2, 0);
        assert_eq!(st.query(0, 1), 30);
        assert_eq!(st.query(3, 4), 90);
    }

    #[test]
    fn dois_updates_no_mesmo_indice() {
        let mut st = SegmentTree::build(&[1, 2, 3]);
        st.update(1, 100);
        st.update(1, 200);
        assert_eq!(st.query(1, 1), 200);
        assert_eq!(st.query(0, 2), 204);
    }

    #[test]
    fn update_seguido_de_todas_as_queries() {
        // após update, todas as queries que tocam o índice alterado devem refletir a mudança
        let arr = [1i64, 2, 3, 4, 5];
        let mut st = SegmentTree::build(&arr);
        let mut arr_ref = arr.to_vec();
        st.update(2, 99);
        arr_ref[2] = 99;
        for l in 0..arr_ref.len() {
            for r in l..arr_ref.len() {
                let esperado: i64 = arr_ref[l..=r].iter().sum();
                assert_eq!(st.query(l, r), esperado, "após update, query({l},{r}) falhou");
            }
        }
    }
}