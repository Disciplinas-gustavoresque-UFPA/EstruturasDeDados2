//! Regras usadas por todo o módulo `pca`.
//!
//! Este arquivo guarda o que os outros arquivos têm em comum:
//!
//! - conferir se uma tabela de dados (uma matriz) pode ser usada nos cálculos;
//! - conferir se uma lista de números não tem `NaN` nem infinito;
//! - achar o maior valor absoluto de uma lista;
//! - (só nos testes) comparar números decimais com tolerância.

/// Confere se a tabela de dados pode ser usada nos cálculos.
///
/// A tabela é uma lista de linhas, e cada linha é uma lista de números. Ela é
/// válida quando:
///
/// - tem pelo menos uma linha, e a primeira linha tem pelo menos um valor;
/// - todas as linhas têm o mesmo tamanho (a tabela é "retangular");
/// - nenhum valor é `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n·p), para uma tabela de n linhas e p colunas. Olha cada valor
///   uma vez.
/// - Espaço: O(1).
pub(super) fn is_valid_matrix(data: &[Vec<f64>]) -> bool {
    // Passo 1: precisa de pelo menos uma linha, com pelo menos um valor.
    if data.is_empty() || data[0].is_empty() {
        return false;
    }

    // Passo 2: todas (`all`) as linhas com o tamanho da primeira e só com
    // valores finitos. O `all` para na primeira linha que falhar.
    let width = data[0].len();
    data.iter().all(|row| row.len() == width && all_finite(row))
}

/// Confere se nenhum valor da lista é `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n). Olha cada valor uma vez.
/// - Espaço: O(1).
pub(super) fn all_finite(values: &[f64]) -> bool {
    values.iter().all(|value| value.is_finite())
}

/// O maior valor absoluto (o maior |x|) da lista. Para uma lista vazia, 0.
///
/// É usado para **normalizar**: dividir todos os valores por ele deixa cada
/// um entre −1 e 1, e as contas seguintes não estouram o limite do `f64`.
///
/// # Complexidade
///
/// - Tempo: O(n).
/// - Espaço: O(1).
pub(super) fn largest_absolute(values: &[f64]) -> f64 {
    values
        .iter()
        .fold(0.0, |largest, &value| largest.max(value.abs()))
}

/// (Só nos testes.) Confere se `obtained` está perto de `expected`.
///
/// O computador erra na 16ª casa decimal: por exemplo, `0.1 + 0.2` dá
/// `0.30000000000000004`. Por isso os testes não exigem igualdade exata:
///
/// - se o valor esperado é 0, a diferença tem que ser menor que `0.000000001`;
/// - se não, a diferença tem que ser menor que `0.000000001` vezes o valor
///   esperado (sem o sinal). Assim, a mesma regra serve para 0,7 e para 1e200.
#[cfg(test)]
pub(super) fn assert_close(obtained: f64, expected: f64) {
    const TOLERANCE: f64 = 1e-9;

    // Infinito só é "perto" de infinito do mesmo sinal.
    if expected.is_infinite() {
        assert_eq!(obtained, expected);
        return;
    }

    let allowed = if expected == 0.0 {
        TOLERANCE
    } else {
        TOLERANCE * expected.abs()
    };
    assert!(
        (obtained - expected).abs() <= allowed,
        "esperado {expected}, obtido {obtained}"
    );
}

/// (Só nos testes.) Confere, posição por posição, se duas listas estão perto.
#[cfg(test)]
pub(super) fn assert_vector_close(obtained: &[f64], expected: &[f64]) {
    assert_eq!(
        obtained.len(),
        expected.len(),
        "tamanhos diferentes: obtido {obtained:?}, esperado {expected:?}"
    );
    for (&value, &target) in obtained.iter().zip(expected) {
        assert_close(value, target);
    }
}

#[cfg(test)]
mod tests {
    use super::{is_valid_matrix, largest_absolute};

    #[test]
    fn test_valid_matrix() {
        assert!(is_valid_matrix(&[vec![1.0, 2.0], vec![3.0, 4.0]]));
        // Uma única linha também é uma tabela.
        assert!(is_valid_matrix(&[vec![-1.5, 0.0, 7.25]]));
    }

    #[test]
    fn test_empty_table_or_empty_rows() {
        assert!(!is_valid_matrix(&[]));
        assert!(!is_valid_matrix(&[vec![]]));
        assert!(!is_valid_matrix(&[vec![], vec![]]));
    }

    #[test]
    fn test_rows_of_different_sizes() {
        // A segunda linha tem um valor a menos: a tabela não é retangular.
        assert!(!is_valid_matrix(&[vec![1.0, 2.0], vec![3.0]]));
        assert!(!is_valid_matrix(&[vec![1.0], vec![2.0, 3.0]]));
    }

    #[test]
    fn test_nan_or_infinity() {
        assert!(!is_valid_matrix(&[vec![1.0, 2.0], vec![f64::NAN, 4.0]]));
        assert!(!is_valid_matrix(&[vec![1.0, f64::INFINITY]]));
        assert!(!is_valid_matrix(&[vec![f64::NEG_INFINITY, 2.0]]));
    }

    #[test]
    fn test_largest_absolute() {
        // |−7| = 7 é maior que |3| = 3 e |0,5| = 0,5.
        assert_eq!(largest_absolute(&[3.0, -7.0, 0.5]), 7.0);
        assert_eq!(largest_absolute(&[0.0, -0.0]), 0.0);
        // Lista vazia: não há nenhum valor, e a resposta é 0.
        assert_eq!(largest_absolute(&[]), 0.0);
    }
}
