//! Etapa 3 do PCA: **matriz de covariância**.

use super::center::center;
use super::helpers::{all_finite, largest_absolute};
use super::linear_algebra::dot;

/// Calcula a **matriz de covariância amostral** de uma tabela de dados.
///
/// # O que é
///
/// Para p variáveis, é uma tabela p × p. Na diagonal ficam as variâncias de
/// cada variável; fora dela, a covariância de cada par de variáveis, que mede
/// o quanto as duas andam juntas. A matriz é **simétrica**: a casa (i, j) é
/// igual à casa (j, i), porque a covariância de i com j é a mesma de j com i.
///
/// # Fórmula
///
/// Com a tabela centralizada (cada valor menos a média da sua coluna) e n
/// linhas, a casa (i, j) é o produto escalar da coluna i com a coluna j,
/// dividido por n − 1 (o divisor da versão **amostral**):
///
/// ```text
/// C[i][j] = (xc₁ᵢ·xc₁ⱼ + xc₂ᵢ·xc₂ⱼ + ... + xcₙᵢ·xcₙⱼ) ÷ (n − 1)
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados: [1, 2], [2, 1], [3, 4], [4, 3]        (n = 4, n − 1 = 3)
/// centralizados: [−1,5; −0,5], [−0,5; −1,5], [0,5; 1,5], [1,5; 0,5]
///
/// C[1][1] = (2,25 + 0,25 + 0,25 + 2,25) ÷ 3 = 5 ÷ 3 = 1,6667
/// C[2][2] = (0,25 + 2,25 + 2,25 + 0,25) ÷ 3 = 5 ÷ 3 = 1,6667
/// C[1][2] = (0,75 + 0,75 + 0,75 + 0,75) ÷ 3 = 3 ÷ 3 = 1
///
///     [1,6667  1     ]
/// C = [1       1,6667]
/// ```
///
/// # Números enormes: quando alguma conta estoura
///
/// Para a coluna `[1e154, −1e154, 0]`, a soma dos quadrados (2 × 10³⁰⁸) passa
/// do maior `f64`, mas a variância (2 × 10³⁰⁸ ÷ 2 = 10³⁰⁸) cabe. Quando alguma
/// conta estoura, a função divide a tabela inteira pelo seu maior valor
/// absoluto s (os valores ficam entre −1 e 1), refaz as contas e multiplica o
/// resultado por s duas vezes. Se a covariância verdadeira não couber no
/// `f64`, a resposta é infinito, e nunca `NaN`.
///
/// Essa forma só é usada quando a primeira estoura, porque as divisões
/// arredondam um pouco.
///
/// # Quando a resposta é `None`
///
/// - Menos de 2 linhas: o divisor n − 1 seria 0.
/// - Tabela vazia, linhas de tamanhos diferentes, `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n·p²). A matriz tem p(p + 1)/2 casas diferentes (a outra metade
///   é cópia, pela simetria), e cada uma é um produto escalar de n valores.
/// - Espaço: O(n·p + p²): a tabela centralizada, as suas colunas e a matriz.
pub fn covariance_matrix(data: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    // Passo 1: pelo menos 2 linhas, porque o divisor é n − 1.
    if data.len() < 2 {
        return None;
    }

    // Passo 2: centralizar (esta função também confere a tabela).
    let (_, centered) = center(data)?;

    // Passo 3 (caminho normal): calcular as casas da matriz. Se a
    // centralização ou alguma soma estourou, `products_of_columns` responde
    // `None` ou a matriz tem infinito/NaN; o `filter` descarta esse caso
    // (transforma o resultado em `None`), e a conta segue para o passo 4.
    let normal =
        products_of_columns(&centered).filter(|matrix| matrix.iter().all(|row| all_finite(row)));
    if let Some(matrix) = normal {
        return Some(matrix);
    }

    // Passo 4 (alguma conta estourou): dividir a tabela pelo seu maior valor
    // absoluto s, refazer as contas e multiplicar o resultado por s duas
    // vezes. Se algo estourou, a tabela tem valores diferentes de zero, e s
    // nunca é 0.
    let scale = data
        .iter()
        .map(|row| largest_absolute(row))
        .fold(0.0, f64::max);
    let scaled: Vec<Vec<f64>> = data
        .iter()
        .map(|row| row.iter().map(|&x| x / scale).collect())
        .collect();
    let (_, scaled_centered) = center(&scaled)?;
    let small = products_of_columns(&scaled_centered)?;
    Some(
        small
            .iter()
            .map(|row| row.iter().map(|&value| value * scale * scale).collect())
            .collect(),
    )
}

/// As casas da matriz: `C[i][j] = (coluna i · coluna j) ÷ (n − 1)`.
///
/// Responde `None` se algum produto escalar não puder ser feito (por exemplo,
/// se a tabela centralizada tiver infinito).
fn products_of_columns(centered: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let divisor = (centered.len() - 1) as f64;
    let columns = columns(centered);
    let p = columns.len();

    let mut matrix = vec![vec![0.0; p]; p];
    for (i, column_i) in columns.iter().enumerate() {
        // Pela simetria, basta calcular as casas com j ≥ i (`skip(i)` pula as
        // colunas antes da i) e copiar cada uma para a casa (j, i).
        for (j, column_j) in columns.iter().enumerate().skip(i) {
            let value = dot(column_i, column_j)? / divisor;
            matrix[i][j] = value;
            matrix[j][i] = value;
        }
    }
    Some(matrix)
}

/// As colunas da tabela, cada uma como uma lista: a tabela "deitada"
/// (transposta). Cada coluna é uma variável, com um valor por observação.
fn columns(table: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let width = table[0].len();
    (0..width)
        .map(|j| table.iter().map(|row| row[j]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::covariance_matrix;
    use crate::pca::helpers::assert_close;

    #[test]
    fn test_two_variables() {
        // Centralizados: [−1,5; −0,5], [−0,5; −1,5], [0,5; 1,5], [1,5; 0,5]
        // n − 1 = 3
        // C₁₁ = (2,25 + 0,25 + 0,25 + 2,25) ÷ 3 = 5 ÷ 3
        // C₂₂ = (0,25 + 2,25 + 2,25 + 0,25) ÷ 3 = 5 ÷ 3
        // C₁₂ = (0,75 + 0,75 + 0,75 + 0,75) ÷ 3 = 3 ÷ 3 = 1
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 1.0],
            vec![3.0, 4.0],
            vec![4.0, 3.0],
        ];
        let c = covariance_matrix(&data).unwrap();
        assert_close(c[0][0], 5.0 / 3.0);
        assert_close(c[1][1], 5.0 / 3.0);
        assert_close(c[0][1], 1.0);
        assert_close(c[1][0], 1.0);
    }

    #[test]
    fn test_three_variables() {
        // Médias: [5, 10, 2]. Centralizados:
        //   [2 − 5; 10 − 10; 1 − 2] = [−3;   0; −1]
        //   [4 − 5; 20 − 10; 1 − 2] = [−1;  10; −1]
        //   [9 − 5;  0 − 10; 4 − 2] = [ 4; −10;  2]
        // n − 1 = 2
        // C₁₁ = (9 + 1 + 16) ÷ 2 = 13        C₁₂ = (0 − 10 − 40) ÷ 2 = −25
        // C₂₂ = (0 + 100 + 100) ÷ 2 = 100    C₁₃ = (3 + 1 + 8) ÷ 2 = 6
        // C₃₃ = (1 + 1 + 4) ÷ 2 = 3          C₂₃ = (0 − 10 − 20) ÷ 2 = −15
        let data = vec![
            vec![2.0, 10.0, 1.0],
            vec![4.0, 20.0, 1.0],
            vec![9.0, 0.0, 4.0],
        ];
        let expected = [[13.0, -25.0, 6.0], [-25.0, 100.0, -15.0], [6.0, -15.0, 3.0]];
        let c = covariance_matrix(&data).unwrap();
        assert_eq!(c.len(), 3);
        for (row, expected_row) in c.iter().zip(&expected) {
            assert_eq!(row.len(), 3);
            for (&value, &target) in row.iter().zip(expected_row) {
                assert_close(value, target);
            }
        }
    }

    #[test]
    fn test_constant_column() {
        // A coluna 1 não varia: variância 0 e covariância 0 com a outra.
        // Coluna 2: média 3, centralizada [−2; −1; 3], variância (4 + 1 + 9) ÷ 2 = 7
        let data = vec![vec![3.0, 1.0], vec![3.0, 2.0], vec![3.0, 6.0]];
        assert_eq!(
            covariance_matrix(&data),
            Some(vec![vec![0.0, 0.0], vec![0.0, 7.0]])
        );
    }

    #[test]
    fn test_needs_two_rows() {
        // Com uma única linha, n − 1 = 0: não existe covariância amostral.
        assert_eq!(covariance_matrix(&[vec![1.0, 2.0]]), None);
    }

    #[test]
    fn test_invalid_table() {
        assert_eq!(covariance_matrix(&[]), None);
        assert_eq!(covariance_matrix(&[vec![1.0, 2.0], vec![3.0]]), None);
        assert_eq!(covariance_matrix(&[vec![1.0], vec![f64::NAN]]), None);
    }

    #[test]
    fn test_sum_overflows_but_result_fits() {
        // (1e154)² + (−1e154)² = 2e308 passa do maior f64 (≈ 1,8e308),
        // mas a variância, 2e308 ÷ 2 = 1e308, cabe.
        let data = vec![vec![1e154], vec![-1e154], vec![0.0]];
        let c = covariance_matrix(&data).unwrap();
        assert_close(c[0][0], 1e308);
    }

    #[test]
    fn test_too_big_is_infinity_not_nan() {
        // A variância verdadeira (4/3 · MAX²) não cabe no f64: a resposta é
        // infinito, e não NaN.
        let data = vec![vec![-f64::MAX], vec![f64::MAX], vec![f64::MAX]];
        assert_eq!(covariance_matrix(&data), Some(vec![vec![f64::INFINITY]]));
    }
}
