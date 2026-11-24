//! Etapa 2 do PCA: **centralização dos dados**.
//!
//! Centralizar é subtrair de cada valor a média da sua coluna. Depois disso,
//! cada coluna passa a ter média 0, e a nuvem de pontos fica "centrada" na
//! origem. O PCA procura as direções em que os dados mais variam **em torno
//! da média**, e por isso este passo vem antes dos outros.

use super::helpers::is_valid_matrix;
use super::linear_algebra::subtract;

/// Calcula a **média de cada coluna** de uma tabela de dados.
///
/// # A tabela
///
/// Cada linha é uma observação (por exemplo, uma pessoa) e cada coluna é uma
/// variável (por exemplo, a altura e o peso):
///
/// ```text
///             variável 1  variável 2
/// observação 1 [   2,         10   ]
/// observação 2 [   4,         20   ]
/// observação 3 [   9,          0   ]
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// coluna 1: (2 + 4 + 9) ÷ 3 = 15 ÷ 3 = 5
/// coluna 2: (10 + 20 + 0) ÷ 3 = 30 ÷ 3 = 10
/// médias = [5, 10]
/// ```
///
/// # Números enormes: quando a soma estoura
///
/// Para a coluna `[1e308, 1e308]`, a soma (2 × 10³⁰⁸) passa do maior `f64` e
/// vira infinito, embora a média (1e308) caiba. Nesse caso, cada valor é
/// dividido por n antes da soma. Como esses n arredondamentos podem empurrar
/// a conta um pouco para fora do intervalo entre o menor e o maior valor da
/// coluna, o resultado é trazido de volta para dentro dele.
///
/// # Quando a resposta é `None`
///
/// - Tabela vazia, ou com a primeira linha vazia.
/// - Linhas de tamanhos diferentes.
/// - Algum valor `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n·p), para n linhas e p colunas: uma passada por coluna (mais
///   três, só nas colunas cuja soma estoura).
/// - Espaço: O(p), as médias.
pub fn column_means(data: &[Vec<f64>]) -> Option<Vec<f64>> {
    // Passo 1: conferir a tabela.
    if !is_valid_matrix(data) {
        return None;
    }

    // Passo 2: a média de cada coluna j = 0, 1, ..., p − 1.
    let width = data[0].len();
    Some((0..width).map(|j| column_mean(data, j)).collect())
}

/// A média da coluna `j` de uma tabela já conferida.
fn column_mean(data: &[Vec<f64>], j: usize) -> f64 {
    let n = data.len() as f64;

    // Caminho normal: somar os valores da coluna e dividir por n.
    let sum: f64 = data.iter().map(|row| row[j]).sum();
    if sum.is_finite() {
        return sum / n;
    }

    // A soma estourou: dividir cada valor por n antes de somar e manter o
    // resultado entre o menor e o maior valor da coluna.
    let average: f64 = data.iter().map(|row| row[j] / n).sum();
    let smallest = data.iter().map(|row| row[j]).fold(f64::INFINITY, f64::min);
    let largest = data
        .iter()
        .map(|row| row[j])
        .fold(f64::NEG_INFINITY, f64::max);
    average.clamp(smallest, largest)
}

/// **Centraliza** a tabela: subtrai de cada valor a média da sua coluna.
///
/// Devolve um par: as médias das colunas e a tabela centralizada.
///
/// # Exemplo, passo a passo
///
/// ```text
/// médias: (1 + 2 + 3 + 4) ÷ 4 = 2,5   e   (2 + 1 + 4 + 3) ÷ 4 = 2,5
///
/// dados      cada valor menos a média      centralizados
/// [1  2]     [1 − 2,5;  2 − 2,5]       =   [−1,5; −0,5]
/// [2  1]     [2 − 2,5;  1 − 2,5]       =   [−0,5; −1,5]
/// [3  4]     [3 − 2,5;  4 − 2,5]       =   [ 0,5;  1,5]
/// [4  3]     [4 − 2,5;  3 − 2,5]       =   [ 1,5;  0,5]
/// ```
///
/// Cada coluna centralizada soma 0: −1,5 − 0,5 + 0,5 + 1,5 = 0.
///
/// # Quando a resposta é `None`
///
/// Nos mesmos casos de `column_means`.
///
/// Se a distância de um valor até a média passar do maior `f64` (o que só
/// acontece numa coluna que vai de perto de −1,8 × 10³⁰⁸ até perto de
/// 1,8 × 10³⁰⁸), ela vira infinito. A função `pca` não passa por isso, porque
/// normaliza a tabela antes de centralizar.
///
/// # Complexidade
///
/// - Tempo: O(n·p): as médias e uma subtração por valor.
/// - Espaço: O(n·p), a tabela centralizada.
pub fn center(data: &[Vec<f64>]) -> Option<(Vec<f64>, Vec<Vec<f64>>)> {
    // Passo 1: as médias das colunas (esta função também confere a tabela).
    let means = column_means(data)?;

    // Passo 2: cada linha menos o vetor das médias, com o `subtract` da
    // etapa 1. Se alguma subtração der `None`, o `collect` dá `None`.
    let centered: Option<Vec<Vec<f64>>> = data.iter().map(|row| subtract(row, &means)).collect();

    Some((means, centered?))
}

#[cfg(test)]
mod tests {
    use super::{center, column_means};

    #[test]
    fn test_column_means() {
        // Coluna 1: (2 + 4 + 9) ÷ 3 = 15 ÷ 3 = 5
        // Coluna 2: (10 + 20 + 0) ÷ 3 = 30 ÷ 3 = 10
        let data = vec![vec![2.0, 10.0], vec![4.0, 20.0], vec![9.0, 0.0]];
        assert_eq!(column_means(&data), Some(vec![5.0, 10.0]));
    }

    #[test]
    fn test_column_means_single_row() {
        // Uma única linha: a média de cada coluna é o próprio valor.
        assert_eq!(
            column_means(&[vec![1.0, -2.0, 3.5]]),
            Some(vec![1.0, -2.0, 3.5])
        );
    }

    #[test]
    fn test_column_means_huge_values() {
        // 1e308 + 1e308 passa do maior f64 (≈ 1,8e308), mas a média, 1e308,
        // cabe. A resposta não pode ser infinito.
        let data = vec![vec![1e308, 1.0], vec![1e308, 3.0]];
        assert_eq!(column_means(&data), Some(vec![1e308, 2.0]));
    }

    #[test]
    fn test_column_means_invalid_table() {
        assert_eq!(column_means(&[]), None);
        assert_eq!(column_means(&[vec![]]), None);
        assert_eq!(column_means(&[vec![1.0, 2.0], vec![3.0]]), None);
        assert_eq!(column_means(&[vec![1.0, f64::NAN]]), None);
    }

    #[test]
    fn test_center() {
        // Médias: (1 + 2 + 3 + 4) ÷ 4 = 2,5 e (2 + 1 + 4 + 3) ÷ 4 = 2,5.
        // Cada valor menos a média da sua coluna:
        //   [1 − 2,5; 2 − 2,5] = [−1,5; −0,5]
        //   [2 − 2,5; 1 − 2,5] = [−0,5; −1,5]
        //   [3 − 2,5; 4 − 2,5] = [ 0,5;  1,5]
        //   [4 − 2,5; 3 − 2,5] = [ 1,5;  0,5]
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 1.0],
            vec![3.0, 4.0],
            vec![4.0, 3.0],
        ];
        let (means, centered) = center(&data).unwrap();
        assert_eq!(means, vec![2.5, 2.5]);
        assert_eq!(
            centered,
            vec![
                vec![-1.5, -0.5],
                vec![-0.5, -1.5],
                vec![0.5, 1.5],
                vec![1.5, 0.5],
            ]
        );
    }

    #[test]
    fn test_center_invalid_table() {
        assert_eq!(center(&[]), None);
        assert_eq!(center(&[vec![1.0], vec![2.0, 3.0]]), None);
        assert_eq!(center(&[vec![f64::INFINITY]]), None);
    }
}
