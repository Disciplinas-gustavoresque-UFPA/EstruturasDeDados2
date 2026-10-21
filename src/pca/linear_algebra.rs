//! Etapa 1 do PCA: **operações básicas de álgebra linear**.
//!
//! Aqui, um **vetor** é uma lista de números (`&[f64]`), e uma **matriz** é
//! uma lista de linhas (`&[Vec<f64>]`), em que cada linha é um vetor.
//!
//! Todas as funções seguem as mesmas regras:
//!
//! - respondem `None` quando os tamanhos não combinam (por exemplo, somar um
//!   vetor de 2 valores com um de 3) ou quando aparece `NaN` ou infinito;
//! - quando a resposta verdadeira passa do maior `f64` (cerca de 1,8 × 10³⁰⁸),
//!   ela vira infinito, e nunca `NaN`.

use super::helpers::{all_finite, largest_absolute};

/// Soma dois vetores, **posição por posição**.
///
/// # Fórmula
///
/// ```text
/// (a + b)ᵢ = aᵢ + bᵢ
/// ```
///
/// # Exemplo
///
/// ```text
/// [1, 2, 3] + [4, 5, 6] = [1 + 4, 2 + 5, 3 + 6] = [5, 7, 9]
/// ```
///
/// # Quando a resposta é `None`
///
/// - Vetores de tamanhos diferentes.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n), uma passada pelos n valores.
/// - Espaço: O(n), o vetor da resposta.
pub fn add(a: &[f64], b: &[f64]) -> Option<Vec<f64>> {
    if a.len() != b.len() || !all_finite(a) || !all_finite(b) {
        return None;
    }

    // `zip` anda pelos dois vetores ao mesmo tempo, formando os pares
    // (a[0], b[0]), (a[1], b[1]), ...; `map` soma cada par; `collect` junta
    // as somas num vetor novo.
    Some(a.iter().zip(b).map(|(&x, &y)| x + y).collect())
}

/// Subtrai o vetor `b` do vetor `a`, **posição por posição**.
///
/// # Fórmula
///
/// ```text
/// (a − b)ᵢ = aᵢ − bᵢ
/// ```
///
/// # Exemplo
///
/// ```text
/// [5, 7, 9] − [4, 5, 6] = [5 − 4, 7 − 5, 9 − 6] = [1, 2, 3]
/// ```
///
/// # Quando a resposta é `None`
///
/// - Vetores de tamanhos diferentes.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n).
/// - Espaço: O(n), o vetor da resposta.
pub fn subtract(a: &[f64], b: &[f64]) -> Option<Vec<f64>> {
    if a.len() != b.len() || !all_finite(a) || !all_finite(b) {
        return None;
    }
    Some(a.iter().zip(b).map(|(&x, &y)| x - y).collect())
}

/// Calcula o **produto escalar** de dois vetores.
///
/// # O que é
///
/// Multiplica os valores das mesmas posições e soma tudo. O resultado mede o
/// quanto os dois vetores "apontam para o mesmo lado": é positivo quando
/// apontam para lados parecidos, negativo quando apontam para lados opostos e
/// zero quando são perpendiculares.
///
/// # Fórmula
///
/// ```text
/// a · b = a₁b₁ + a₂b₂ + ... + aₙbₙ
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// [1, 2, 3] · [4, 5, 6] = 1·4 + 2·5 + 3·6 = 4 + 10 + 18 = 32
/// [1, 1]    · [1, −1]   = 1·1 + 1·(−1)    = 0   (perpendiculares)
/// ```
///
/// # Números enormes: quando a soma estoura
///
/// Para `a = [1e200, 1e200]` e `b = [1e109, −0,9e109]`, os produtos (1e309 e
/// −9e308) passam do maior `f64` e viram +infinito e −infinito; somados,
/// dariam `NaN`. Mas o resultado, 1e309 − 0,9e309 = 1e308, cabe no `f64`.
/// Nesse caso, a função usa uma segunda forma da mesma conta: divide cada
/// vetor pelo seu maior valor absoluto (os valores ficam entre −1 e 1), faz o
/// produto escalar e multiplica o resultado de volta pelos dois divisores.
///
/// Essa forma só é usada quando a primeira estoura, porque as divisões
/// arredondam um pouco.
///
/// # Quando a resposta é `None`
///
/// - Vetores de tamanhos diferentes.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n). Uma passada para conferir os dados e outra para somar (mais
///   três, só quando a soma estoura).
/// - Espaço: O(1). Guarda só a soma.
pub fn dot(a: &[f64], b: &[f64]) -> Option<f64> {
    // Passo 1: conferir os tamanhos e os valores.
    if a.len() != b.len() || !all_finite(a) || !all_finite(b) {
        return None;
    }

    // Passo 2 (caminho normal): somar os produtos das mesmas posições.
    let sum: f64 = a.iter().zip(b).map(|(&x, &y)| x * y).sum();
    if sum.is_finite() {
        return Some(sum);
    }

    // Passo 3 (a soma estourou): dividir cada vetor pelo seu maior valor
    // absoluto, somar os produtos e multiplicar de volta pelos divisores.
    // Se a soma estourou, os dois vetores têm valores diferentes de zero, e
    // os divisores nunca são 0.
    let scale_a = largest_absolute(a);
    let scale_b = largest_absolute(b);
    let scaled: f64 = a
        .iter()
        .zip(b)
        .map(|(&x, &y)| (x / scale_a) * (y / scale_b))
        .sum();
    Some(scaled * scale_a * scale_b)
}

/// Multiplica uma **matriz por um vetor**.
///
/// # O que é
///
/// Cada valor da resposta é o produto escalar de uma linha da matriz com o
/// vetor. Uma matriz de m linhas, multiplicada por um vetor, dá um vetor de m
/// valores.
///
/// # Exemplo, passo a passo
///
/// ```text
/// [1 2]   [2]   [1·2 + 2·1]   [ 4]
/// [3 4] · [1] = [3·2 + 4·1] = [10]
/// [5 6]         [5·2 + 6·1]   [16]
/// ```
///
/// # Quando a resposta é `None`
///
/// - Alguma linha da matriz tem um tamanho diferente do vetor.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(m·n), para m linhas de n valores: um produto escalar por linha.
/// - Espaço: O(m), o vetor da resposta.
pub fn matrix_vector(matrix: &[Vec<f64>], v: &[f64]) -> Option<Vec<f64>> {
    // Cada linha vira um número: o produto escalar dela com o vetor. Se alguma
    // linha der `None`, o `collect` devolve `None` para a matriz toda.
    matrix.iter().map(|row| dot(row, v)).collect()
}

/// Calcula o **produto externo** de dois vetores: o resultado é uma matriz.
///
/// # O que é
///
/// O valor da linha i, coluna j, é `aᵢ · bⱼ`. No PCA, ele aparece na
/// **deflação**: depois de achar um autovalor λ e o seu autovetor v, a parte
/// λ · (v vᵀ) é tirada da matriz de covariância, e v vᵀ é o produto externo
/// de v com ele mesmo.
///
/// # Exemplo, passo a passo
///
/// ```text
/// [1]               [1·3  1·4  1·5]   [3  4   5]
/// [2] · [3, 4, 5] = [2·3  2·4  2·5] = [6  8  10]
/// ```
///
/// # Quando a resposta é `None`
///
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(m·n), para vetores de m e n valores: uma multiplicação por casa.
/// - Espaço: O(m·n), a matriz da resposta.
pub fn outer(a: &[f64], b: &[f64]) -> Option<Vec<Vec<f64>>> {
    if !all_finite(a) || !all_finite(b) {
        return None;
    }

    // Para cada valor x de `a`, uma linha: x multiplicado por cada valor de `b`.
    Some(
        a.iter()
            .map(|&x| b.iter().map(|&y| x * y).collect())
            .collect(),
    )
}

/// Calcula a **norma** (o comprimento) de um vetor.
///
/// # Fórmula
///
/// É o teorema de Pitágoras, em quantas dimensões o vetor tiver:
///
/// ```text
/// ‖v‖ = √(v₁² + v₂² + ... + vₙ²)
/// ```
///
/// # Exemplo
///
/// ```text
/// ‖[3, 4]‖ = √(3² + 4²) = √(9 + 16) = √25 = 5
/// ```
///
/// # Números enormes ou minúsculos
///
/// Elevar ao quadrado dobra o expoente: (1e200)² = 1e400 passa do maior `f64`
/// e vira infinito, e (1e-200)² = 1e-400 é pequeno demais e vira 0. Nos dois
/// casos, a norma (√2 · 1e200 ou √2 · 1e-200) cabe no `f64`. Quando a soma
/// dos quadrados estoura ou fica abaixo do menor `f64` "normal"
/// (`f64::MIN_POSITIVE`, cerca de 2,2 × 10⁻³⁰⁸, abaixo do qual as contas
/// perdem precisão), a função divide o vetor pelo seu maior valor absoluto
/// (os valores ficam entre −1 e 1), calcula a norma e multiplica de volta.
///
/// # Quando a resposta é `None`
///
/// - Algum valor `NaN` ou infinito.
///
/// O vetor vazio e o vetor nulo têm norma 0.
///
/// # Complexidade
///
/// - Tempo: O(n). Uma passada para conferir e outra para somar os quadrados
///   (mais duas nos casos extremos).
/// - Espaço: O(1).
pub fn norm(v: &[f64]) -> Option<f64> {
    // Passo 1: conferir os valores.
    if !all_finite(v) {
        return None;
    }

    // Passo 2 (caminho normal): a raiz da soma dos quadrados.
    let sum_of_squares: f64 = v.iter().map(|&x| x * x).sum();
    if sum_of_squares.is_finite() && sum_of_squares >= f64::MIN_POSITIVE {
        return Some(sum_of_squares.sqrt());
    }

    // Passo 3: o vetor nulo (ou vazio) tem norma 0.
    let scale = largest_absolute(v);
    if scale == 0.0 {
        return Some(0.0);
    }

    // Passo 4 (números enormes ou minúsculos): dividir pelo maior valor
    // absoluto antes de elevar ao quadrado e multiplicar a raiz de volta.
    let scaled_sum: f64 = v
        .iter()
        .map(|&x| {
            let y = x / scale;
            y * y
        })
        .sum();
    Some(scale * scaled_sum.sqrt())
}

/// **Normaliza** um vetor: devolve o vetor de comprimento 1 que aponta para o
/// mesmo lado.
///
/// # Fórmula
///
/// ```text
/// v ÷ ‖v‖
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// ‖[3, 4]‖ = 5
/// [3, 4] ÷ 5 = [0,6; 0,8]
/// conferindo: √(0,6² + 0,8²) = √(0,36 + 0,64) = √1 = 1
/// ```
///
/// # Como a conta é feita
///
/// Primeiro, o vetor é dividido pelo seu maior valor absoluto (os valores
/// ficam entre −1 e 1); só depois, pela sua norma. A primeira divisão não muda
/// a direção, e assim a conta funciona até para `[f64::MAX, f64::MAX]`, cuja
/// norma não cabe no `f64`.
///
/// # Quando a resposta é `None`
///
/// - Vetor nulo ou vazio: ele não aponta para lado nenhum.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n), algumas passadas pelos n valores.
/// - Espaço: O(n), o vetor da resposta.
pub fn normalize(v: &[f64]) -> Option<Vec<f64>> {
    // Passo 1: conferir os valores.
    if !all_finite(v) {
        return None;
    }

    // Passo 2: o vetor nulo (ou vazio) não tem direção.
    let scale = largest_absolute(v);
    if scale == 0.0 {
        return None;
    }

    // Passo 3: dividir pelo maior valor absoluto. A norma de `scaled` fica
    // entre 1 e √n: nunca estoura nem vira 0.
    let scaled: Vec<f64> = v.iter().map(|&x| x / scale).collect();
    let length = norm(&scaled)?;

    // Passo 4: dividir pela norma.
    Some(scaled.iter().map(|&x| x / length).collect())
}

#[cfg(test)]
mod tests {
    use super::{add, dot, matrix_vector, norm, normalize, outer, subtract};
    use crate::pca::helpers::{assert_close, assert_vector_close};

    #[test]
    fn test_add() {
        // [1 + 4, 2 + 5, 3 + 6] = [5, 7, 9]
        assert_eq!(
            add(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]),
            Some(vec![5.0, 7.0, 9.0])
        );
        assert_eq!(add(&[], &[]), Some(vec![]));
    }

    #[test]
    fn test_subtract() {
        // [5 − 4, 7 − 5, 9 − 6] = [1, 2, 3]
        assert_eq!(
            subtract(&[5.0, 7.0, 9.0], &[4.0, 5.0, 6.0]),
            Some(vec![1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn test_add_and_subtract_reject_bad_input() {
        // Tamanhos diferentes: não existe a soma posição por posição.
        assert_eq!(add(&[1.0, 2.0], &[1.0]), None);
        assert_eq!(subtract(&[1.0], &[1.0, 2.0]), None);
        // NaN ou infinito: sem resposta.
        assert_eq!(add(&[1.0, f64::NAN], &[1.0, 2.0]), None);
        assert_eq!(subtract(&[1.0, 2.0], &[f64::INFINITY, 2.0]), None);
    }

    #[test]
    fn test_dot() {
        // 1·4 + 2·5 + 3·6 = 4 + 10 + 18 = 32
        assert_eq!(dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), Some(32.0));
        // Vetores perpendiculares: 1·1 + 1·(−1) = 0
        assert_eq!(dot(&[1.0, 1.0], &[1.0, -1.0]), Some(0.0));
        // Vetores vazios: a soma de nenhum produto é 0.
        assert_eq!(dot(&[], &[]), Some(0.0));
    }

    #[test]
    fn test_dot_rejects_bad_input() {
        assert_eq!(dot(&[1.0, 2.0], &[1.0]), None);
        assert_eq!(dot(&[1.0, f64::NAN], &[1.0, 2.0]), None);
    }

    #[test]
    fn test_dot_when_the_products_overflow() {
        // 1e200 · 1e109 = 1e309 e 1e200 · (−0,9e109) = −9e308 passam do maior
        // f64 (≈ 1,8e308): viram +infinito e −infinito, e a soma daria NaN.
        // Mas o resultado, 1e309 − 0,9e309 = 1e308, cabe no f64.
        let result = dot(&[1e200, 1e200], &[1e109, -0.9e109]).unwrap();
        assert_close(result, 1e308);
    }

    #[test]
    fn test_dot_too_big_is_infinity_not_nan() {
        // 1e200 · 1e200 = 1e400 não cabe no f64: a resposta é infinito.
        assert_eq!(dot(&[1e200], &[1e200]), Some(f64::INFINITY));
    }

    #[test]
    fn test_matrix_vector() {
        // [1 2]   [2]   [1·2 + 2·1]   [ 4]
        // [3 4] · [1] = [3·2 + 4·1] = [10]
        // [5 6]         [5·2 + 6·1]   [16]
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        assert_eq!(
            matrix_vector(&matrix, &[2.0, 1.0]),
            Some(vec![4.0, 10.0, 16.0])
        );

        // A matriz identidade não muda o vetor.
        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        assert_eq!(
            matrix_vector(&identity, &[-3.0, 8.0]),
            Some(vec![-3.0, 8.0])
        );

        // Matriz sem linhas: o resultado é um vetor vazio.
        assert_eq!(matrix_vector(&[], &[1.0, 2.0]), Some(vec![]));
    }

    #[test]
    fn test_matrix_vector_rejects_bad_input() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        // O vetor tem 3 valores, mas cada linha tem 2.
        assert_eq!(matrix_vector(&matrix, &[1.0, 2.0, 3.0]), None);
        // Linhas de tamanhos diferentes.
        assert_eq!(
            matrix_vector(&[vec![1.0, 2.0], vec![3.0]], &[1.0, 2.0]),
            None
        );
        // NaN na matriz.
        assert_eq!(matrix_vector(&[vec![1.0, f64::NAN]], &[1.0, 2.0]), None);
    }

    #[test]
    fn test_outer() {
        // [1]               [1·3 1·4 1·5]   [3 4  5]
        // [2] · [3 4 5]  =  [2·3 2·4 2·5] = [6 8 10]
        assert_eq!(
            outer(&[1.0, 2.0], &[3.0, 4.0, 5.0]),
            Some(vec![vec![3.0, 4.0, 5.0], vec![6.0, 8.0, 10.0]])
        );
        assert_eq!(outer(&[], &[1.0]), Some(vec![]));
        assert_eq!(outer(&[1.0, f64::NAN], &[1.0]), None);
    }

    #[test]
    fn test_norm() {
        // √(3² + 4²) = √25 = 5
        assert_eq!(norm(&[3.0, 4.0]), Some(5.0));
        assert_eq!(norm(&[0.0, 0.0]), Some(0.0));
        assert_eq!(norm(&[]), Some(0.0));
        assert_eq!(norm(&[1.0, f64::NAN]), None);
    }

    #[test]
    fn test_norm_of_huge_values() {
        // (1e200)² = 1e400 estoura, mas a norma, √2 · 1e200, cabe no f64.
        assert_close(norm(&[1e200, 1e200]).unwrap(), 2f64.sqrt() * 1e200);
    }

    #[test]
    fn test_norm_of_tiny_values() {
        // (1e-200)² = 1e-400 é pequeno demais e vira 0, mas a norma,
        // √2 · 1e-200, cabe no f64 (e não pode ser 0: o vetor não é nulo).
        assert_close(norm(&[1e-200, 1e-200]).unwrap(), 2f64.sqrt() * 1e-200);
    }

    #[test]
    fn test_normalize() {
        // [3, 4] tem norma 5: [3/5, 4/5] = [0,6; 0,8]
        assert_vector_close(&normalize(&[3.0, 4.0]).unwrap(), &[0.6, 0.8]);
        assert_vector_close(&normalize(&[0.0, -5.0]).unwrap(), &[0.0, -1.0]);
    }

    #[test]
    fn test_normalize_zero_vector() {
        // O vetor nulo não tem direção: não dá para normalizar.
        assert_eq!(normalize(&[0.0, 0.0]), None);
        assert_eq!(normalize(&[]), None);
        assert_eq!(normalize(&[f64::NAN, 1.0]), None);
    }

    #[test]
    fn test_normalize_extreme_values() {
        // A norma de [MAX, MAX] não cabe no f64, mas a direção cabe: [√2/2, √2/2].
        let half_sqrt2 = 2f64.sqrt() / 2.0;
        assert_vector_close(
            &normalize(&[f64::MAX, f64::MAX]).unwrap(),
            &[half_sqrt2, half_sqrt2],
        );
        // Valores minúsculos também têm direção.
        assert_vector_close(&normalize(&[1e-200, 0.0]).unwrap(), &[1.0, 0.0]);
    }
}
