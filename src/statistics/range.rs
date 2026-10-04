use super::helpers::is_valid;

/// Calcula a **amplitude**: a distância entre o maior e o menor valor.
///
/// # O que é
///
/// É a medida de espalhamento mais simples: mostra o tamanho da faixa que os
/// dados ocupam, do menor ao maior.
///
/// # Fórmula
///
/// ```text
/// amplitude = máximo − mínimo
/// ```
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados     = [2, 4, 4, 5, 7, 9]
/// mínimo    = 2
/// máximo    = 9
/// amplitude = 9 − 2 = 7
/// ```
///
/// Por que não usar as funções `max` e `min` do módulo `searching`? Elas só
/// aceitam tipos com "ordem total" (`Ord`), que o `f64` não tem. Daria para
/// usá-las com o mesmo "embrulho" que a ordenação usa (veja `helpers.rs`), mas
/// isso exigiria copiar a lista inteira (espaço O(n)). O laço abaixo faz o
/// mesmo trabalho guardando só dois números (espaço O(1)).
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n). Uma passada para conferir os dados e outra guardando o
///   menor e o maior valor.
/// - Espaço: O(1). Guarda só dois números: o menor e o maior.
pub fn range(data: &[f64]) -> Option<f64> {
    // Passo 1: conferir os dados.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: começar com o primeiro valor como menor e como maior.
    let mut smallest = data[0];
    let mut largest = data[0];

    // Passo 3: percorrer o resto da lista, atualizando o menor e o maior.
    for &value in &data[1..] {
        if value < smallest {
            smallest = value;
        }
        if value > largest {
            largest = value;
        }
    }

    // Passo 4: a amplitude é a distância entre os dois.
    Some(largest - smallest)
}

#[cfg(test)]
mod tests {
    use super::range;
    use crate::statistics::helpers::assert_close;

    #[test]
    fn test_main_example() {
        // 9 − 2 = 7
        assert_close(range(&[2.0, 4.0, 4.0, 5.0, 7.0, 9.0]), 7.0);
    }

    #[test]
    fn test_unsorted_input() {
        // A ordem não importa: 9 − 2 = 7
        assert_close(range(&[9.0, 4.0, 2.0, 7.0, 4.0, 5.0]), 7.0);
    }

    #[test]
    fn test_single_value() {
        // Um número só: 5 − 5 = 0
        assert_close(range(&[5.0]), 0.0);
    }

    #[test]
    fn test_negative_values() {
        // 2 − (−3) = 5
        assert_close(range(&[-3.0, -1.0, 0.0, 2.0]), 5.0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(range(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(range(&[1.0, f64::NAN, 3.0]), None);
    }
}
