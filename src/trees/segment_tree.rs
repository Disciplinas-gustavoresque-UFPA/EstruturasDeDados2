pub struct SegmentTree {
    n: usize,
    // nós em i128 para que somas intermediárias nunca causem overflow;
    // a conversão para i64 acontece apenas no resultado da query
    tree: Vec<i128>,
}

impl SegmentTree {
    /// Constrói a Segment Tree a partir de um slice.
    /// Complexidade: O(n) tempo e espaço.
    pub fn build(arr: &[i64]) -> Self{
        let n = arr.len();
        let mut tree = vec![0i128; 4 * n];
        if n > 0{
            Self::build_recursive(&mut tree, arr, 1, 0, n-1);
        }

        SegmentTree{n, tree}
    }

    fn build_recursive(tree: &mut Vec<i128>, arr: &[i64], node: usize, start: usize, end: usize){
        if start == end {
            tree[node] = arr[start] as i128;
        } else{
            let mid = (start + end) / 2;
            Self::build_recursive(tree, arr, 2 * node, start, mid);
            Self::build_recursive(tree, arr, 2 * node + 1, mid + 1, end);
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }
    }

    /// Retorna a soma do intervalo [l, r] (inclusivo em ambos os extremos).
    /// Complexidade: O(log n).
    ///
    /// Causa panic se os índices forem inválidos ou se a soma não couber em i64.
    pub fn query(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "índices fora do intervalo válido");
        let soma = self.query_recursive(1, 0, self.n - 1, l, r);
        i64::try_from(soma).expect("a soma do intervalo excede os limites de i64")
    }

    fn query_recursive(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i128 {
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
            self.tree[node] = val as i128; // folha encontrada
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

    // ── casos de borda e erros ──────────────────────────────────────────────

    #[test]
    fn build_array_vazio() {
        let _st = SegmentTree::build(&[]);
    }

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn query_em_array_vazio_causa_panic() {
        let st = SegmentTree::build(&[]);
        st.query(0, 0);
    }

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn query_com_l_maior_que_r_causa_panic() {
        let st = SegmentTree::build(&[1, 2, 3]);
        st.query(2, 1);
    }

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn query_com_r_fora_do_array_causa_panic() {
        let st = SegmentTree::build(&[1, 2, 3]);
        st.query(0, 3);
    }

    #[test]
    #[should_panic(expected = "índice fora do intervalo válido")]
    fn update_com_i_fora_do_array_causa_panic() {
        let mut st = SegmentTree::build(&[1, 2, 3]);
        st.update(3, 10);
    }

    #[test]
    fn tamanhos_pequenos_todas_as_queries() {
        // tamanhos 1 a 8 cobrem os casos pequenos que costumam esconder erros de fronteira
        for n in 1..=8 {
            let arr: Vec<i64> = (1..=n as i64).map(|x| x * 3 - 7).collect();
            let st = SegmentTree::build(&arr);
            for l in 0..n {
                for r in l..n {
                    let esperado: i64 = arr[l..=r].iter().sum();
                    assert_eq!(st.query(l, r), esperado, "n={n}, query({l},{r})");
                }
            }
        }
    }

    // ── overflow ────────────────────────────────────────────────────────────

    #[test]
    fn somas_intermediarias_maiores_que_i64_nao_causam_overflow() {
        // o nó [0, 1] guarda i64::MAX + 1, mas a soma total cabe em i64
        let st = SegmentTree::build(&[i64::MAX, 1, -1]);
        assert_eq!(st.query(0, 2), i64::MAX);
        assert_eq!(st.query(0, 0), i64::MAX);
    }

    #[test]
    #[should_panic(expected = "a soma do intervalo excede os limites de i64")]
    fn soma_maior_que_i64_causa_panic() {
        let st = SegmentTree::build(&[i64::MAX, 1]);
        st.query(0, 1);
    }

    // ── testes aleatórios e de escala ───────────────────────────────────────

    /// Gerador pseudoaleatório simples (xorshift), para não adicionar dependências.
    struct Xorshift(u64);

    impl Xorshift {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        fn intervalo(&mut self, limite: usize) -> usize {
            (self.next() % limite as u64) as usize
        }

        fn valor(&mut self) -> i64 {
            (self.next() % 2001) as i64 - 1000
        }
    }

    #[test]
    fn operacoes_aleatorias_comparadas_com_vec() {
        // semente fixa: o teste é reproduzível
        let mut rng = Xorshift(0x2545_F491_4F6C_DD1D);
        for n in [1, 2, 3, 4, 5, 17, 64, 100] {
            let mut referencia: Vec<i64> = (0..n).map(|_| rng.valor()).collect();
            let mut st = SegmentTree::build(&referencia);
            for _ in 0..2_000 {
                if rng.next() % 2 == 0 {
                    let i = rng.intervalo(n);
                    let val = rng.valor();
                    st.update(i, val);
                    referencia[i] = val;
                } else {
                    let a = rng.intervalo(n);
                    let b = rng.intervalo(n);
                    let (l, r) = (a.min(b), a.max(b));
                    let esperado: i64 = referencia[l..=r].iter().sum();
                    assert_eq!(st.query(l, r), esperado, "n={n}, query({l},{r})");
                }
            }
        }
    }

    #[test]
    fn array_grande() {
        let n = 1_000_000;
        let arr: Vec<i64> = (0..n as i64).collect();
        let mut st = SegmentTree::build(&arr);
        // soma de 0 até n-1 = n(n-1)/2
        assert_eq!(st.query(0, n - 1), (n as i64) * (n as i64 - 1) / 2);
        assert_eq!(st.query(n / 2, n / 2), (n / 2) as i64);
        st.update(n - 1, 0);
        assert_eq!(st.query(0, n - 1), (n as i64) * (n as i64 - 1) / 2 - (n as i64 - 1));
    }
}