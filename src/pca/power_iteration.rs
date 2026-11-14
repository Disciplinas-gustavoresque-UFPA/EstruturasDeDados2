//! Etapa 4 do PCA: **iteração de potências com deflação**.
//!
//! Acha os maiores autovalores de uma matriz de covariância, e os autovetores
//! correspondentes, um de cada vez.
//!
//! # A ideia, em palavras
//!
//! Multiplicar um vetor pela matriz C "puxa" o vetor para a direção em que os
//! dados mais variam. Repetindo a multiplicação muitas vezes, e voltando o
//! vetor para o comprimento 1 a cada vez (para ele não crescer sem parar), o
//! vetor acaba apontando para essa direção: o **primeiro autovetor**. O
//! quanto a matriz estica o vetor nessa direção é o **primeiro autovalor**.
//!
//! Depois, a **deflação** tira da matriz a parte já encontrada:
//! C ← C − λ · (v vᵀ). Na matriz que sobra, a direção mais forte é a segunda,
//! e a mesma iteração acha o segundo autovetor. E assim por diante, até k.
//!
//! # Por que funciona
//!
//! Escrevendo o vetor inicial na base dos autovetores, v = c₁v₁ + c₂v₂ + ...,
//! multiplicar por C repetidas t vezes dá c₁λ₁ᵗv₁ + c₂λ₂ᵗv₂ + .... Como λ₁ é o
//! maior, λ₁ᵗ cresce mais depressa que os outros, e o primeiro termo domina. A
//! cada repetição, a parte de v₂ encolhe na proporção λ₂ ÷ λ₁.
//!
//! # Exemplo, passo a passo
//!
//! Para C = [[5/3, 1], [1, 5/3]], cujos autovalores são 8/3 e 2/3, começando
//! em v = (1, 0). O λ de cada linha é v · (C·v), com o v do começo da linha:
//!
//! ```text
//! repetição   C · v               v novo (norma 1)    λ = v · (C·v)
//!     1       (1,6667; 1,0000)    (0,8575; 0,5145)    1,6667
//!     2       (1,9437; 1,7150)    (0,7498; 0,6616)    2,5490
//!     3       (1,9114; 1,8525)    (0,7181; 0,6960)    2,6589
//!     4       (1,8928; 1,8780)    (0,7099; 0,7043)    2,6662
//!    ...
//!    10       (1,8856; 1,8856)    (0,7071; 0,7071)    2,6667 = 8/3
//! ```
//!
//! O vetor chegou a (1, 1)/√2. A deflação tira (8/3) · (v vᵀ) da matriz:
//!
//! ```text
//! v vᵀ = [[0,5; 0,5], [0,5; 0,5]]
//! C − (8/3)·(v vᵀ) = [[5/3 − 4/3; 1 − 4/3], [1 − 4/3; 5/3 − 4/3]]
//!                  = [[1/3; −1/3], [−1/3; 1/3]]
//! ```
//!
//! Nessa matriz, a direção mais forte é (1, −1), com autovalor 2/3: o segundo
//! par.
//!
//! # O vetor inicial
//!
//! Se o vetor inicial for perpendicular ao autovetor procurado (c₁ = 0), a
//! iteração não o encontra. Na matriz [[1, −1], [−1, 1]], por exemplo, o vetor
//! (1, 1) é perpendicular ao autovetor (1, −1), e C · (1, 1) = (0, 0). Por
//! isso o vetor inicial é "embaralhado": os seus valores vêm de um gerador de
//! números pseudoaleatórios (um gerador congruencial linear, escrito no fim
//! deste arquivo) com uma semente fixa. Os números parecem sorteados, mas são
//! sempre os mesmos, e o PCA dá sempre o mesmo resultado para os mesmos dados.
//!
//! Cada autovetor começa de um vetor inicial novo, tirado da continuação do
//! gerador. Isso importa quando o maior autovalor se repete, como em
//! diag(2, 2, 0): o primeiro autovetor sai igual à "sombra" do vetor inicial
//! no plano do autovalor 2, e, tirada essa sombra, o mesmo vetor não teria
//! mais nada nesse plano para oferecer ao segundo.
//!
//! Pelo mesmo motivo, os vetores iniciais de autovetores diferentes não podem
//! ser "parentes". Se um fosse o outro mais uma constante, por exemplo, os
//! dois teriam a mesma sombra em qualquer direção perpendicular a
//! (1, 1, ..., 1), e as direções de dados centralizados costumam ser
//! assim. Sequências com padrão (como a da razão áurea, em que os valores
//! i + p e i diferem por uma constante) caem nessa armadilha; um gerador
//! pseudoaleatório, não.
//!
//! # Quando parar
//!
//! A iteração para quando C·v ≈ λ·v, isto é, quando ‖C·v − λ·v‖ fica menor
//! que 10⁻¹² vezes a soma da diagonal de C (a variância total), ou depois de
//! 10.000 repetições. Esse limite só é atingido quando dois autovalores são
//! quase iguais: aí os autovalores ainda saem certos, mas os dois autovetores
//! ficam mal definidos, porque qualquer mistura deles serve quase igualmente.
//!
//! # Autovalores iguais a zero
//!
//! Quando as variáveis são perfeitamente correlacionadas (por exemplo,
//! y = 2x), algumas direções não têm variação nenhuma: os autovalores delas
//! são 0. Depois de tirar todas as direções com variação, o que sobra da
//! matriz é praticamente zero. Os autovalores restantes são, então, 0, e
//! qualquer vetor perpendicular aos já encontrados é um autovetor. A função
//! escolhe um deles a partir dos eixos (1, 0, ..., 0), (0, 1, ..., 0), ....
//!
//! Pelo mesmo motivo, um autovalor menor que 10⁻¹² vezes a variância total é
//! tratado como arredondamento e devolvido como 0.

use super::linear_algebra::{dot, matrix_vector, norm, normalize, outer, subtract};

/// Erro aceito: a iteração para quando ‖C·v − λ·v‖ ≤ 10⁻¹² × (soma da
/// diagonal de C). A mesma tolerância decide se o que sobrou da matriz,
/// depois das deflações, já é praticamente zero.
const TOLERANCE: f64 = 1e-12;

/// Limite de repetições da iteração, para cada autovetor.
const MAX_ITERATIONS: usize = 10_000;

/// A semente do gerador dos vetores iniciais: sempre a mesma, para que o
/// resultado seja sempre o mesmo.
const SEED: u64 = 2026;

/// Acha os `k` maiores autovalores de uma matriz simétrica sem autovalores
/// negativos (como toda matriz de covariância) e os autovetores
/// correspondentes.
///
/// Devolve um par: os autovalores, em ordem decrescente, e os autovetores, na
/// mesma ordem, cada um com norma 1 e perpendicular aos outros.
///
/// Quem chama garante que a matriz é quadrada, simétrica e sem `NaN` nem
/// infinito, e que 1 ≤ k ≤ p. A resposta só seria `None` se essa garantia
/// falhasse.
///
/// # Complexidade
///
/// - Tempo: O(k · t · p²), para uma matriz p × p e t repetições por
///   autovetor. Cada repetição tem uma multiplicação matriz-vetor (p²
///   multiplicações), e cada deflação custa O(p²). O número de repetições t
///   depende da distância entre os autovalores: o erro encolhe na proporção
///   λ₂ ÷ λ₁ a cada repetição, então t fica perto de
///   log(10¹²) ÷ log(λ₁ ÷ λ₂). Por isso t não tem um limite fixo, a não ser
///   as 10.000 repetições.
/// - Espaço: O(p²), a cópia da matriz que a deflação vai "esvaziando".
pub(super) fn top_eigenpairs(matrix: &[Vec<f64>], k: usize) -> Option<(Vec<f64>, Vec<Vec<f64>>)> {
    let p = matrix.len();

    // O "tamanho" da matriz: a soma da sua diagonal (o traço). Numa matriz de
    // covariância, é a variância total dos dados.
    let trace: f64 = matrix.iter().enumerate().map(|(i, row)| row[i]).sum();

    let mut remaining = matrix.to_vec();
    let mut values: Vec<f64> = Vec::new();
    let mut vectors: Vec<Vec<f64>> = Vec::new();

    // O estado do gerador dos vetores iniciais (veja `next_random`).
    let mut state = SEED;

    for _ in 0..k {
        // Passo 1: se o que sobrou da matriz é praticamente zero, o próximo
        // autovalor é 0, e o autovetor é qualquer direção perpendicular às já
        // encontradas. (`concat` junta as linhas numa lista só, e a norma
        // dessa lista mede o tamanho da matriz inteira.)
        let leftover = norm(&remaining.concat())?;
        let (value, vector) = if leftover <= TOLERANCE * trace {
            (0.0, perpendicular_unit_vector(&vectors, p)?)
        } else {
            // Passo 2: a iteração de potências na matriz que sobrou. Um
            // autovalor menor que a tolerância é só arredondamento: vira 0.
            let (value, vector) = power_iteration(&remaining, &vectors, trace, &mut state)?;
            let value = if value <= TOLERANCE * trace {
                0.0
            } else {
                value
            };
            (value, vector)
        };

        // Passo 3: a deflação, C ← C − λ · (v vᵀ).
        remaining = deflate(&remaining, value, &vector)?;
        values.push(value);
        vectors.push(vector);
    }

    Some((values, vectors))
}

/// A iteração de potências: acha o maior autovalor da matriz e o seu
/// autovetor, procurando só nas direções perpendiculares às já encontradas.
fn power_iteration(
    matrix: &[Vec<f64>],
    found: &[Vec<f64>],
    trace: f64,
    state: &mut u64,
) -> Option<(f64, Vec<f64>)> {
    let p = matrix.len();

    // Passo 1: um vetor inicial novo, tirado do gerador (veja "O vetor
    // inicial", no começo do arquivo), sem as direções já encontradas e com
    // norma 1.
    //
    // Se quase nada sobrar dele (menos de um milionésimo do seu comprimento),
    // o que sobrou é só arredondamento e pode apontar para qualquer lado,
    // inclusive para uma direção já encontrada. Nesse caso, a iteração começa
    // de uma direção perpendicular às encontradas, escolhida entre os eixos.
    let start: Vec<f64> = (0..p).map(|_| next_random(state)).collect();
    let rest = remove_directions(&start, found)?;
    let mut v = if norm(&rest)? > 1e-6 * norm(&start)? {
        normalize(&rest)?
    } else {
        perpendicular_unit_vector(found, p)?
    };

    let mut value = 0.0;
    for _ in 0..MAX_ITERATIONS {
        // Passo 2: multiplicar pela matriz e tirar de novo as direções já
        // encontradas, que os arredondamentos podem trazer de volta.
        let cv = remove_directions(&matrix_vector(matrix, &v)?, found)?;

        // Passo 3: o autovalor estimado é v · (C·v), o "quociente de
        // Rayleigh". Como v tem norma 1, é o quanto C estica v.
        value = dot(&v, &cv)?;

        // Passo 4: parar quando C·v ≈ λ·v.
        let stretched: Vec<f64> = v.iter().map(|&x| value * x).collect();
        if norm(&subtract(&cv, &stretched)?)? <= TOLERANCE * trace {
            break;
        }

        // Passo 5: o próximo v é C·v com norma 1.
        v = normalize(&cv)?;
    }

    // Uma matriz de covariância não tem autovalores negativos: um valor
    // negativo minúsculo é só arredondamento.
    Some((value.max(0.0), v))
}

/// Tira de `v` a sua parte em cada direção de `found` (vetores de norma 1,
/// perpendiculares entre si). A parte de v na direção u é (u · v) · u: a
/// "sombra" de v sobre u.
fn remove_directions(v: &[f64], found: &[Vec<f64>]) -> Option<Vec<f64>> {
    let mut result = v.to_vec();
    for u in found {
        let amount = dot(u, &result)?;
        let shadow: Vec<f64> = u.iter().map(|&x| amount * x).collect();
        result = subtract(&result, &shadow)?;
    }
    Some(result)
}

/// Um vetor de norma 1, perpendicular a todos os vetores de `found`.
///
/// Tenta os eixos (1, 0, ..., 0), (0, 1, ..., 0), ...: tira de cada um as
/// direções já encontradas e fica com o que sobrar de maior comprimento. Como
/// há menos de p direções encontradas, algum eixo sempre sobra.
fn perpendicular_unit_vector(found: &[Vec<f64>], p: usize) -> Option<Vec<f64>> {
    let mut best = vec![0.0; p];
    let mut best_length = 0.0;
    for i in 0..p {
        let mut axis = vec![0.0; p];
        axis[i] = 1.0;
        let candidate = remove_directions(&axis, found)?;
        let length = norm(&candidate)?;
        if length > best_length {
            best_length = length;
            best = candidate;
        }
    }
    normalize(&best)
}

/// A deflação: C − λ · (v vᵀ), com o produto externo v vᵀ da etapa 1. Tira da
/// matriz a parte que corresponde ao autovalor λ e ao autovetor v.
fn deflate(matrix: &[Vec<f64>], value: f64, v: &[f64]) -> Option<Vec<Vec<f64>>> {
    let piece = outer(v, v)?;
    matrix
        .iter()
        .zip(&piece)
        .map(|(row, piece_row)| {
            let scaled: Vec<f64> = piece_row.iter().map(|&x| value * x).collect();
            subtract(row, &scaled)
        })
        .collect()
}

/// O próximo número do gerador congruencial linear: um número entre −0,5 e
/// 0,5 que parece sorteado.
///
/// O estado avança com estado ← a · estado + c. Quando a conta passa do maior
/// `u64` (2⁶⁴ − 1), ela "dá a volta", como os ponteiros de um relógio: é isso
/// que `wrapping_mul` e `wrapping_add` fazem. Os 53 bits mais altos do estado
/// (`>> 11` descarta os 11 mais baixos, que são os menos embaralhados) viram
/// um número entre 0 e 1, porque um `f64` guarda exatamente 53 bits.
///
/// O `&mut u64` é o estado "emprestado para alteração": a função muda o valor
/// guardado por quem a chamou, e a próxima chamada continua de onde esta
/// parou.
fn next_random(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    (*state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
}

#[cfg(test)]
mod tests {
    use super::top_eigenpairs;
    use crate::pca::helpers::{assert_close, assert_vector_close};
    use crate::pca::linear_algebra::{dot, matrix_vector, norm, normalize};

    /// Confere se `v` tem norma 1 e aponta na direção de `expected`. O sinal
    /// não importa: se v é autovetor, −v também é.
    fn assert_same_direction(v: &[f64], expected: &[f64]) {
        let unit = normalize(expected).unwrap();
        let sign = if dot(v, &unit).unwrap() < 0.0 {
            -1.0
        } else {
            1.0
        };
        let aligned: Vec<f64> = v.iter().map(|&x| sign * x).collect();
        assert_vector_close(&aligned, &unit);
    }

    /// Confere se os vetores têm norma 1 e são perpendiculares entre si.
    fn assert_orthonormal(vectors: &[Vec<f64>]) {
        for (i, u) in vectors.iter().enumerate() {
            assert_close(norm(u).unwrap(), 1.0);
            for w in &vectors[i + 1..] {
                assert_close(dot(u, w).unwrap(), 0.0);
            }
        }
    }

    #[test]
    fn test_two_by_two() {
        // Para [[a, b], [b, a]], os autovalores são a + b e a − b, com os
        // autovetores (1, 1) e (1, −1). Aqui: 5/3 + 1 = 8/3 e 5/3 − 1 = 2/3.
        let matrix = vec![vec![5.0 / 3.0, 1.0], vec![1.0, 5.0 / 3.0]];
        let (values, vectors) = top_eigenpairs(&matrix, 2).unwrap();
        assert_vector_close(&values, &[8.0 / 3.0, 2.0 / 3.0]);
        assert_same_direction(&vectors[0], &[1.0, 1.0]);
        assert_same_direction(&vectors[1], &[1.0, -1.0]);
    }

    #[test]
    fn test_diagonal_matrix_in_decreasing_order() {
        // Numa matriz diagonal, os autovalores são a própria diagonal, e os
        // autovetores são os eixos. Em ordem decrescente: 3, 2, 1.
        let matrix = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 3).unwrap();
        assert_vector_close(&values, &[3.0, 2.0, 1.0]);
        assert_same_direction(&vectors[0], &[0.0, 1.0, 0.0]);
        assert_same_direction(&vectors[1], &[0.0, 0.0, 1.0]);
        assert_same_direction(&vectors[2], &[1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_three_by_three() {
        // [[2, −1, 0], [−1, 2, −1], [0, −1, 2]] tem autovalores 2 + √2, 2 e
        // 2 − √2, com autovetores (1, −√2, 1), (1, 0, −1) e (1, √2, 1).
        // Conferindo o primeiro, linha a linha:
        //   2·1 − 1·(−√2) + 0·1  = 2 + √2         = (2 + √2) · 1
        //  −1·1 + 2·(−√2) − 1·1  = −2 − 2√2       = (2 + √2) · (−√2)
        //   0·1 − 1·(−√2) + 2·1  = √2 + 2         = (2 + √2) · 1
        let s = 2f64.sqrt();
        let matrix = vec![
            vec![2.0, -1.0, 0.0],
            vec![-1.0, 2.0, -1.0],
            vec![0.0, -1.0, 2.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 3).unwrap();
        assert_vector_close(&values, &[2.0 + s, 2.0, 2.0 - s]);
        assert_same_direction(&vectors[0], &[1.0, -s, 1.0]);
        assert_same_direction(&vectors[1], &[1.0, 0.0, -1.0]);
        assert_same_direction(&vectors[2], &[1.0, s, 1.0]);
    }

    #[test]
    fn test_only_the_k_largest() {
        // Pedindo k = 1, sai só o maior autovalor (2 + √2) e o seu autovetor.
        let matrix = vec![
            vec![2.0, -1.0, 0.0],
            vec![-1.0, 2.0, -1.0],
            vec![0.0, -1.0, 2.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 1).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(vectors.len(), 1);
        assert_close(values[0], 2.0 + 2f64.sqrt());
    }

    #[test]
    fn test_zero_eigenvalues() {
        // u uᵀ com u = (1, 2, 2): só uma direção tem variação. Autovalores:
        // u · u = 1 + 4 + 4 = 9, e depois 0 e 0. Os autovetores de autovalor 0
        // são quaisquer dois vetores perpendiculares a u (e entre si).
        let matrix = vec![
            vec![1.0, 2.0, 2.0],
            vec![2.0, 4.0, 4.0],
            vec![2.0, 4.0, 4.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 3).unwrap();
        assert_vector_close(&values, &[9.0, 0.0, 0.0]);
        assert_same_direction(&vectors[0], &[1.0, 2.0, 2.0]);
        assert_orthonormal(&vectors);
    }

    #[test]
    fn test_start_vector_trap() {
        // [[1, −1], [−1, 1]]: o maior autovalor (2) tem o autovetor (1, −1).
        // Um vetor inicial como (1, 1) seria perpendicular a ele, e C · (1, 1)
        // = (0, 0): a iteração não sairia do lugar.
        let matrix = vec![vec![1.0, -1.0], vec![-1.0, 1.0]];
        let (values, vectors) = top_eigenpairs(&matrix, 2).unwrap();
        assert_vector_close(&values, &[2.0, 0.0]);
        assert_same_direction(&vectors[0], &[1.0, -1.0]);
        assert_same_direction(&vectors[1], &[1.0, 1.0]);
    }

    #[test]
    fn test_repeated_eigenvalue() {
        // 2 · identidade: todo vetor é autovetor, com autovalor 2. A resposta
        // tem que ter três vetores de norma 1, perpendiculares entre si.
        let matrix = vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 3).unwrap();
        assert_vector_close(&values, &[2.0, 2.0, 2.0]);
        assert_orthonormal(&vectors);
    }

    #[test]
    fn test_repeated_largest_eigenvalue() {
        // diag(2, 2, 0): o maior autovalor, 2, aparece duas vezes, e o terceiro
        // é 0. Se todos os autovetores partissem do mesmo vetor inicial, o
        // segundo 2 se perderia: o primeiro autovetor seria a "sombra" do
        // vetor inicial no plano dos dois eixos, e, tirada essa sombra, o
        // vetor inicial não teria mais nada nesse plano.
        let matrix = vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ];
        let (values, vectors) = top_eigenpairs(&matrix, 3).unwrap();
        assert_vector_close(&values, &[2.0, 2.0, 0.0]);
        assert_same_direction(&vectors[2], &[0.0, 0.0, 1.0]);
        assert_orthonormal(&vectors);
    }

    #[test]
    fn test_nearly_equal_eigenvalues() {
        // Autovalores quase iguais (1 + 1e-10 e 1) fazem a iteração andar
        // devagar, mas os autovalores saem certos e os vetores, perpendiculares.
        let matrix = vec![vec![1.0 + 1e-10, 0.0], vec![0.0, 1.0]];
        let (values, vectors) = top_eigenpairs(&matrix, 2).unwrap();
        assert_vector_close(&values, &[1.0 + 1e-10, 1.0]);
        assert_orthonormal(&vectors);
    }

    #[test]
    fn test_zero_matrix() {
        // A matriz nula tem todos os autovalores iguais a 0.
        let matrix = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let (values, vectors) = top_eigenpairs(&matrix, 2).unwrap();
        assert_eq!(values, vec![0.0, 0.0]);
        assert_orthonormal(&vectors);
    }

    #[test]
    fn test_properties_on_a_larger_matrix() {
        // B · Bᵀ, para uma matriz B qualquer, é simétrica e não tem
        // autovalores negativos, como uma matriz de covariância. Para ela, a
        // resposta tem que cumprir as propriedades de autovalores e autovetores:
        //   1. C · v = λ · v para cada par;
        //   2. vetores de norma 1 e perpendiculares entre si;
        //   3. autovalores em ordem decrescente;
        //   4. a soma dos autovalores é a soma da diagonal (o traço).
        let b: Vec<Vec<f64>> = (0..6)
            .map(|i| {
                (0..4)
                    .map(|j| ((i * 7 + j * 3) % 11) as f64 - 5.0)
                    .collect()
            })
            .collect();
        let matrix: Vec<Vec<f64>> = b
            .iter()
            .map(|row_i| b.iter().map(|row_j| dot(row_i, row_j).unwrap()).collect())
            .collect();
        let trace: f64 = (0..6).map(|i| matrix[i][i]).sum();

        let (values, vectors) = top_eigenpairs(&matrix, 6).unwrap();
        for (&value, v) in values.iter().zip(&vectors) {
            let cv = matrix_vector(&matrix, v).unwrap();
            for (&a, &b) in cv.iter().zip(v) {
                assert!((a - value * b).abs() <= 1e-9 * trace);
            }
        }
        assert_orthonormal(&vectors);
        for pair in values.windows(2) {
            assert!(pair[0] >= pair[1] - 1e-9 * trace);
        }
        assert_close(values.iter().sum::<f64>(), trace);
    }

    #[test]
    fn test_rounding_noise_becomes_zero() {
        // B · Bᵀ, com B de 6 linhas e 4 colunas, tem no máximo 4 autovalores
        // diferentes de zero: os outros 2 são exatamente 0. Depois das 4
        // deflações, o que sobra da matriz é só arredondamento, e os
        // autovalores que faltam têm que sair 0, e não um número minúsculo.
        let b: Vec<Vec<f64>> = (0..6)
            .map(|i| {
                (0..4)
                    .map(|j| ((i * 7 + j * 3) % 11) as f64 - 5.0)
                    .collect()
            })
            .collect();
        let matrix: Vec<Vec<f64>> = b
            .iter()
            .map(|row_i| b.iter().map(|row_j| dot(row_i, row_j).unwrap()).collect())
            .collect();
        let (values, _) = top_eigenpairs(&matrix, 6).unwrap();
        assert_eq!(values[4], 0.0);
        assert_eq!(values[5], 0.0);
    }
}
