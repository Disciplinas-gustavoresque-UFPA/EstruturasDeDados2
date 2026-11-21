//! Fenwick Tree (Binary Indexed Tree) para somas de prefixo e de intervalo.
//!
//! A estrutura guarda um vetor `tree` de n + 1 posições, indexado a partir
//! de 1. Cada posição `i` armazena a soma de um bloco do array original que
//! termina em `i` e tem tamanho `lsb(i)`, o bit menos significativo de `i`.
//! Por exemplo, `lsb(12) = lsb(0b1100) = 4`, então `tree[12]` guarda a soma
//! das posições 9 a 12.
//!
//! - Para somar um prefixo, acumula-se `tree[i]` e remove-se o bit menos
//!   significativo (`i -= lsb(i)`) até chegar a 0.
//! - Para atualizar uma posição, soma-se o valor em `tree[i]` e avança-se
//!   para o próximo bloco que a contém (`i += lsb(i)`) até passar de n.
//!
//! Em ambos os casos, cada passo remove ou acrescenta um bit de `i`, então são
//! no máximo ⌊log₂ n⌋ + 1 passos.

pub struct FenwickTree {
    n: usize,
    // tree[0] não é usado; tree[1..=n] guardam os blocos.
    // Valores em i128 para que somas intermediárias nunca causem overflow;
    // a conversão para i64 acontece apenas no resultado das consultas.
    tree: Vec<i128>,
}

impl FenwickTree {
    /// Cria uma Fenwick Tree com `n` elementos iguais a zero.
    /// Complexidade: O(n).
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            tree: vec![0; n + 1],
        }
    }

    /// Constrói a Fenwick Tree a partir de um slice.
    ///
    /// Em vez de chamar `add` para cada elemento (O(n log n)), cada bloco
    /// `tree[i]`, depois de completo, é somado uma única vez ao bloco "pai"
    /// `i + lsb(i)`, que o contém.
    /// Complexidade: O(n).
    pub fn from_slice(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![0i128; n + 1];

        for i in 1..=n {
            // os blocos menores que compõem tree[i] já foram somados nele
            tree[i] += arr[i - 1] as i128;
            let parent = i + Self::lsb(i);
            if parent <= n {
                tree[parent] += tree[i];
            }
        }

        FenwickTree { n, tree }
    }

    /// Retorna a quantidade de elementos.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Retorna `true` se a árvore não tiver elementos.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Soma `delta` ao elemento da posição `i`.
    /// Complexidade: O(log n).
    pub fn add(&mut self, i: usize, delta: i64) {
        assert!(i < self.n, "índice fora do intervalo válido");
        self.add_raw(i + 1, delta as i128);
    }

    /// Altera o elemento da posição `i` para `val`.
    ///
    /// O valor atual é obtido pela soma do intervalo `[i, i]`, e a diferença
    /// é aplicada com `add`. Assim, não é preciso guardar uma cópia do array.
    /// Complexidade: O(log n).
    pub fn set(&mut self, i: usize, val: i64) {
        assert!(i < self.n, "índice fora do intervalo válido");
        let atual = self.prefix_sum_raw(i + 1) - self.prefix_sum_raw(i);
        self.add_raw(i + 1, val as i128 - atual);
    }

    /// Retorna a soma do intervalo `[0, i]`.
    /// Complexidade: O(log n).
    ///
    /// Causa panic se o índice for inválido ou se a soma não couber em i64.
    pub fn prefix_sum(&self, i: usize) -> i64 {
        assert!(i < self.n, "índice fora do intervalo válido");
        Self::to_i64(self.prefix_sum_raw(i + 1))
    }

    /// Retorna a soma do intervalo `[l, r]` (inclusivo em ambos os extremos).
    ///
    /// Calculada como a diferença entre dois prefixos. A subtração é feita em
    /// i128, antes da conversão, para que prefixos maiores que i64 não causem
    /// erro quando a soma do intervalo cabe em i64.
    /// Complexidade: O(log n).
    ///
    /// Causa panic se os índices forem inválidos ou se a soma não couber em i64.
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "índices fora do intervalo válido");
        Self::to_i64(self.prefix_sum_raw(r + 1) - self.prefix_sum_raw(l))
    }

    /// Isola o bit menos significativo de `i` (ex.: 12 = 0b1100 → 4 = 0b100).
    /// Em complemento de dois, `-i` inverte todos os bits acima do menor bit 1.
    // A operação é escrita de forma explícita, e não com `isolate_lowest_one()`,
    // por ser a base da estrutura e para manter compatibilidade com versões
    // mais antigas do Rust.
    #[allow(clippy::manual_isolate_lowest_one)]
    fn lsb(i: usize) -> usize {
        i & i.wrapping_neg()
    }

    /// Soma `delta` em todos os blocos que contêm a posição `idx` (a partir de 1).
    fn add_raw(&mut self, mut idx: usize, delta: i128) {
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += Self::lsb(idx);
        }
    }

    /// Retorna a soma dos `count` primeiros elementos, sem conversão para i64.
    /// Com `count = 0`, retorna 0, o que evita tratar `l − 1` quando `l = 0`.
    fn prefix_sum_raw(&self, mut count: usize) -> i128 {
        let mut soma = 0;
        while count > 0 {
            soma += self.tree[count];
            count -= Self::lsb(count);
        }
        soma
    }

    /// Converte uma soma para i64, causando panic se ela não couber no tipo.
    fn to_i64(soma: i128) -> i64 {
        i64::try_from(soma).expect("a soma do intervalo excede os limites de i64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── construção ──────────────────────────────────────────────────────────

    #[test]
    fn new_cria_arvore_com_zeros() {
        let ft = FenwickTree::new(5);
        assert_eq!(ft.len(), 5);
        assert!(!ft.is_empty());
        assert_eq!(ft.range_sum(0, 4), 0);
    }

    #[test]
    fn arvore_vazia() {
        let ft = FenwickTree::new(0);
        assert_eq!(ft.len(), 0);
        assert!(ft.is_empty());

        let ft = FenwickTree::from_slice(&[]);
        assert!(ft.is_empty());
    }

    #[test]
    fn um_elemento() {
        let mut ft = FenwickTree::from_slice(&[42]);
        assert_eq!(ft.prefix_sum(0), 42);
        assert_eq!(ft.range_sum(0, 0), 42);
        ft.add(0, 8);
        assert_eq!(ft.range_sum(0, 0), 50);
        ft.set(0, -1);
        assert_eq!(ft.range_sum(0, 0), -1);
    }

    #[test]
    fn lsb_isola_o_bit_menos_significativo() {
        assert_eq!(FenwickTree::lsb(1), 1); // 0b1
        assert_eq!(FenwickTree::lsb(6), 2); // 0b110
        assert_eq!(FenwickTree::lsb(8), 8); // 0b1000
        assert_eq!(FenwickTree::lsb(12), 4); // 0b1100
    }

    #[test]
    fn from_slice_equivale_a_adds_sucessivos() {
        // a construção O(n) deve gerar exatamente o mesmo vetor interno
        // que n chamadas de add sobre uma árvore zerada
        for n in 0..=20 {
            let arr: Vec<i64> = (0..n as i64).map(|x| x * x - 5).collect();
            let construida = FenwickTree::from_slice(&arr);
            let mut incremental = FenwickTree::new(n);
            for (i, &x) in arr.iter().enumerate() {
                incremental.add(i, x);
            }
            assert_eq!(construida.tree, incremental.tree, "n={n}");
        }
    }

    // ── consultas ───────────────────────────────────────────────────────────

    #[test]
    fn tamanhos_pequenos_todas_as_queries() {
        // tamanhos 1 a 8 cobrem os casos pequenos que costumam esconder erros de fronteira
        for n in 1..=8 {
            let arr: Vec<i64> = (1..=n as i64).map(|x| x * 3 - 7).collect();
            let ft = FenwickTree::from_slice(&arr);
            for l in 0..n {
                assert_eq!(
                    ft.prefix_sum(l),
                    arr[..=l].iter().sum::<i64>(),
                    "n={n}, prefix_sum({l})"
                );
                for r in l..n {
                    let esperado: i64 = arr[l..=r].iter().sum();
                    assert_eq!(ft.range_sum(l, r), esperado, "n={n}, range_sum({l},{r})");
                }
            }
        }
    }

    #[test]
    fn valores_negativos_e_mistos() {
        let ft = FenwickTree::from_slice(&[-3, 2, -1, 4, -2]);
        assert_eq!(ft.range_sum(0, 4), 0);
        assert_eq!(ft.range_sum(1, 3), 5);
        assert_eq!(ft.prefix_sum(2), -2);
    }

    // ── atualizações ────────────────────────────────────────────────────────

    #[test]
    fn add_repetido_no_mesmo_indice() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3, 4]);
        ft.add(2, 10);
        ft.add(2, -4);
        assert_eq!(ft.range_sum(2, 2), 9);
        assert_eq!(ft.range_sum(0, 3), 16);
    }

    #[test]
    fn set_repetido_no_mesmo_indice() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3, 4]);
        ft.set(1, 100);
        ft.set(1, 200);
        assert_eq!(ft.range_sum(1, 1), 200);
        assert_eq!(ft.range_sum(0, 3), 208);
    }

    #[test]
    fn set_depois_de_add() {
        // set deve substituir o valor, não importando os adds anteriores
        let mut ft = FenwickTree::new(3);
        ft.add(1, 5);
        ft.add(1, 5);
        ft.set(1, 3);
        assert_eq!(ft.range_sum(0, 2), 3);
    }

    #[test]
    fn update_nao_afeta_indices_adjacentes() {
        let mut ft = FenwickTree::from_slice(&[10, 20, 30, 40, 50]);
        ft.set(2, 0);
        assert_eq!(ft.range_sum(0, 1), 30);
        assert_eq!(ft.range_sum(3, 4), 90);
    }

    // ── índices inválidos ───────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn range_sum_com_l_maior_que_r_causa_panic() {
        let ft = FenwickTree::from_slice(&[1, 2, 3]);
        ft.range_sum(2, 1);
    }

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn range_sum_com_r_fora_do_array_causa_panic() {
        let ft = FenwickTree::from_slice(&[1, 2, 3]);
        ft.range_sum(0, 3);
    }

    #[test]
    #[should_panic(expected = "índices fora do intervalo válido")]
    fn range_sum_em_arvore_vazia_causa_panic() {
        let ft = FenwickTree::new(0);
        ft.range_sum(0, 0);
    }

    #[test]
    #[should_panic(expected = "índice fora do intervalo válido")]
    fn prefix_sum_com_i_fora_do_array_causa_panic() {
        let ft = FenwickTree::from_slice(&[1, 2, 3]);
        ft.prefix_sum(3);
    }

    #[test]
    #[should_panic(expected = "índice fora do intervalo válido")]
    fn add_com_i_fora_do_array_causa_panic() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3]);
        ft.add(3, 10);
    }

    #[test]
    #[should_panic(expected = "índice fora do intervalo válido")]
    fn set_com_i_fora_do_array_causa_panic() {
        let mut ft = FenwickTree::from_slice(&[1, 2, 3]);
        ft.set(3, 10);
    }

    // ── overflow ────────────────────────────────────────────────────────────

    #[test]
    fn somas_intermediarias_maiores_que_i64_nao_causam_overflow() {
        // o bloco tree[2] guarda i64::MAX + 1, mas a soma total cabe em i64
        let ft = FenwickTree::from_slice(&[i64::MAX, 1, -1]);
        assert_eq!(ft.range_sum(0, 2), i64::MAX);
        assert_eq!(ft.range_sum(0, 0), i64::MAX);
    }

    #[test]
    fn intervalo_que_cabe_em_i64_com_prefixos_que_nao_cabem() {
        // prefix(1) = 2·i64::MAX, mas range_sum(2, 2) = 5
        let ft = FenwickTree::from_slice(&[i64::MAX, i64::MAX, 5]);
        assert_eq!(ft.range_sum(2, 2), 5);
    }

    #[test]
    fn set_com_valor_atual_maior_que_i64() {
        // depois de dois adds, o elemento vale 2·i64::MAX; set deve substituí-lo
        let mut ft = FenwickTree::new(2);
        ft.add(0, i64::MAX);
        ft.add(0, i64::MAX);
        ft.set(0, 1);
        assert_eq!(ft.range_sum(0, 1), 1);
    }

    #[test]
    #[should_panic(expected = "a soma do intervalo excede os limites de i64")]
    fn soma_maior_que_i64_causa_panic() {
        let ft = FenwickTree::from_slice(&[i64::MAX, 1]);
        ft.range_sum(0, 1);
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
            let mut ft = FenwickTree::from_slice(&referencia);
            for _ in 0..2_000 {
                match rng.next() % 4 {
                    0 => {
                        let i = rng.intervalo(n);
                        let delta = rng.valor();
                        ft.add(i, delta);
                        referencia[i] += delta;
                    }
                    1 => {
                        let i = rng.intervalo(n);
                        let val = rng.valor();
                        ft.set(i, val);
                        referencia[i] = val;
                    }
                    2 => {
                        let i = rng.intervalo(n);
                        let esperado: i64 = referencia[..=i].iter().sum();
                        assert_eq!(ft.prefix_sum(i), esperado, "n={n}, prefix_sum({i})");
                    }
                    _ => {
                        let a = rng.intervalo(n);
                        let b = rng.intervalo(n);
                        let (l, r) = (a.min(b), a.max(b));
                        let esperado: i64 = referencia[l..=r].iter().sum();
                        assert_eq!(ft.range_sum(l, r), esperado, "n={n}, range_sum({l},{r})");
                    }
                }
            }
        }
    }

    #[test]
    fn array_grande() {
        let n = 1_000_000;
        let arr: Vec<i64> = (0..n as i64).collect();
        let mut ft = FenwickTree::from_slice(&arr);
        // soma de 0 até n-1 = n(n-1)/2
        assert_eq!(ft.range_sum(0, n - 1), (n as i64) * (n as i64 - 1) / 2);
        assert_eq!(ft.range_sum(n / 2, n / 2), (n / 2) as i64);
        ft.set(n - 1, 0);
        assert_eq!(
            ft.range_sum(0, n - 1),
            (n as i64) * (n as i64 - 1) / 2 - (n as i64 - 1)
        );
    }

    #[test]
    fn muitas_operacoes_em_array_grande() {
        // 200.000 sets e 400.000 consultas em n ≈ 1 milhão.
        // Com O(log n) por operação: ~10^7 passos (menos de 1 s).
        // Com O(n) por operação: ~10^11 passos (horas) — o teste não terminaria.
        let n = 1 << 20;
        let mut rng = Xorshift(0x9E37_79B9_7F4A_7C15);
        let mut ft = FenwickTree::from_slice(&vec![1; n]);
        // referência mantida em O(1) por passo, para não virar uma soma O(n)
        let mut total: i64 = n as i64;
        for _ in 0..200_000 {
            let i = rng.intervalo(n);
            let antigo = ft.range_sum(i, i);
            ft.set(i, 2);
            total += 2 - antigo;
            let a = rng.intervalo(n);
            let b = rng.intervalo(n);
            ft.range_sum(a.min(b), a.max(b));
        }
        assert_eq!(ft.prefix_sum(n - 1), total);
    }
}
