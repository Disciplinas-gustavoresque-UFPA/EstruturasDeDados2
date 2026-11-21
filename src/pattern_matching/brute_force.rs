//! Força bruta: tenta o padrão em todas as posições do texto.

/// Procura **todas** as ocorrências de `pattern` em `text` (inclusive as que
/// se sobrepõem).
///
/// # Parâmetros
///
/// - `text` e `n`: o texto e o seu tamanho em bytes;
/// - `pattern` e `m`: o padrão e o seu tamanho em bytes;
/// - `out` e `cap`: o vetor onde as posições são guardadas e a sua capacidade.
///
/// # Resposta
///
/// - `-1` se o padrão é vazio (`m == 0`);
/// - senão, o número total de ocorrências. As posições (em bytes) das
///   primeiras `cap` ocorrências ficam em `out[0..]`. Se o padrão for maior
///   que o texto, a resposta é 0.
///
/// # Como funciona
///
/// A janela de tamanho m anda pelo texto de 0 até n − m. Em cada posição, o
/// padrão é comparado byte a byte. Depois de cada tentativa, a janela anda
/// uma posição para a direita.
///
/// # Exemplo
///
/// ```text
/// texto:  a a a a      padrão: a a
/// posição 0: casa   posição 1: casa   posição 2: casa   -> 3 ocorrências: 0, 1, 2
/// ```
///
/// # Complexidade
///
/// - Tempo: O(n·m) no pior caso. O melhor caso é O(n).
/// - Espaço extra: O(1).
pub fn brute_force(
    text: &[u8],
    n: usize,
    pattern: &[u8],
    m: usize,
    out: &mut [usize],
    cap: usize,
) -> isize {
    if m == 0 {
        return -1;
    }
    if m > n {
        return 0;
    }

    let mut count: usize = 0;
    let mut i = 0;
    while i + m <= n {
        let mut j = 0;
        while j < m && text[i + j] == pattern[j] {
            j += 1;
        }
        if j == m {
            if count < cap {
                out[count] = i;
            }
            count += 1;
        }
        i += 1;
    }
    count as isize
}
