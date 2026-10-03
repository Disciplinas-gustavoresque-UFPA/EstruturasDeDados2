//! Funções de apoio usadas pelas medidas do módulo.
//!
//! Este arquivo não calcula nenhuma estatística. Ele guarda as regras que,
//! sem ele, se repetiriam em quase todos os outros arquivos:
//!
//! - conferir se uma lista pode ser usada nos cálculos;
//! - fazer uma cópia ordenada da lista, usando o `merge_sort` da própria
//!   biblioteca (`src/sorting/merge_sort.rs`);
//! - (só nos testes) comparar dois números decimais com tolerância.

use std::cmp::Ordering;

use crate::sorting::merge_sort;

/// Confere se a lista pode ser usada nos cálculos.
///
/// Uma lista é **válida** quando:
///
/// - não está vazia; e
/// - todos os valores são números finitos: nenhum `NaN` ("não é um
///   número") e nenhum infinito.
///
/// # Complexidade
///
/// - Tempo: O(n). Olha cada valor uma vez.
/// - Espaço: O(1). Não guarda nada além da resposta.
pub(super) fn is_valid(data: &[f64]) -> bool {
    // Passo 1: lista vazia não é válida.
    if data.is_empty() {
        return false;
    }

    // Passo 2: olhar cada valor; se algum for NaN ou infinito, a lista não é
    // válida.
    for &value in data {
        if !value.is_finite() {
            return false;
        }
    }

    // Passo 3: passou por todos os valores sem problema.
    true
}

/// Um número decimal "embrulhado", para poder ser ordenado pelo `merge_sort`
/// da biblioteca.
///
/// # Por que o embrulho é necessário
///
/// O `merge_sort` deste repositório (em `src/sorting/merge_sort.rs`) ordena
/// qualquer tipo que possa ser copiado (`Copy`) e que tenha uma **ordem
/// total** (`Ord`): para quaisquer dois valores a e b, ou a < b, ou a > b, ou
/// a = b. O `f64` pode ser copiado, mas não tem a ordem total, por causa do
/// `NaN`, que não é menor, nem maior, nem igual a nada (nem a ele mesmo).
///
/// Aqui só ordenamos listas **já conferidas** por `is_valid` (sem `NaN` e sem
/// infinito). Nelas, a ordem é total. O embrulho é a forma de **prometer**
/// isso ao Rust: ele implementa a comparação "à mão", com `<` e `>`.
// Como ler as próximas linhas:
// - `struct Sortable(f64)` cria um tipo novo que guarda um único `f64`.
//   O número guardado é acessado com `.0` (o "campo número 0").
// - `#[derive(Clone, Copy, PartialEq)]` pede ao Rust para gerar sozinho três
//   habilidades simples: copiar o embrulho (Clone e Copy, que o `merge_sort`
//   exige), como se copia um número, e comparar dois embrulhos com `==`
//   (PartialEq).
#[derive(Clone, Copy, PartialEq)]
struct Sortable(f64);

// Cada `impl ... for Sortable` abaixo "ensina" uma habilidade ao embrulho.
// No Rust, essas habilidades se chamam *traits*: são contratos do tipo
// "eu sei me comparar". O `merge_sort` exige que o tipo cumpra o contrato
// `Ord` (ordem total), que por sua vez exige `Eq` e `PartialOrd`.

/// A igualdade do embrulho é total, porque ele só guarda números conferidos.
// `Eq` não tem nada para escrever: é só a promessa de que todo valor é igual
// a si mesmo, o que vale porque aqui nunca há NaN.
impl Eq for Sortable {}

/// Comparação parcial: aqui ela nunca falha, então devolve sempre a total.
impl PartialOrd for Sortable {
    // `partial_cmp` responde `Some(resultado)` ou `None` (quando não dá para
    // comparar). Como sempre dá, ela só repassa a resposta de `cmp`, abaixo.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A comparação total, escrita à mão: menor, maior ou igual.
impl Ord for Sortable {
    // `self` é este embrulho e `other` é o outro. A resposta é um `Ordering`:
    // `Less` (menor), `Greater` (maior) ou `Equal` (igual).
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

/// Devolve uma **cópia** da lista em ordem crescente.
///
/// A lista original não é alterada: quem chamou a função continua com os
/// seus dados na ordem em que estavam.
///
/// A ordenação é feita pelo `merge_sort` da própria biblioteca
/// (`src/sorting/merge_sort.rs`), com os valores "embrulhados" em
/// `Sortable` (veja acima por quê).
///
/// Quem chama deve passar uma lista já conferida por `is_valid`.
///
/// # Complexidade
///
/// - Tempo: O(n log n). Embrulhar e desembrulhar são uma passada cada, O(n);
///   o `merge_sort` custa O(n log n), pela recorrência
///   T(n) = 2T(n/2) + n. A parcela maior é a que manda.
/// - Espaço: O(n), o tamanho das cópias.
pub(super) fn sorted_copy(data: &[f64]) -> Vec<f64> {
    // Passo 1: copiar a lista, embrulhando cada valor.
    let mut wrapped: Vec<Sortable> = Vec::new();
    for &value in data {
        wrapped.push(Sortable(value));
    }

    // Passo 2: ordenar a cópia com o merge_sort da biblioteca.
    merge_sort(&mut wrapped);

    // Passo 3: desembrulhar, já em ordem crescente.
    let mut sorted: Vec<f64> = Vec::new();
    for item in &wrapped {
        sorted.push(item.0);
    }
    sorted
}

/// (Só nos testes.) Confere se `obtained` está perto de `expected`.
///
/// O computador erra na 16ª casa decimal: por exemplo, `0.1 + 0.2` dá
/// `0.30000000000000004`. Por isso os testes não exigem igualdade exata:
/// aceitam uma diferença menor que `0.000000001`.
///
/// Se a função testada responder `None` onde se esperava um número, o teste
/// falha com uma mensagem clara.
#[cfg(test)]
pub(super) fn assert_close(obtained: Option<f64>, expected: f64) {
    const TOLERANCE: f64 = 1e-9;
    match obtained {
        Some(value) => assert!(
            (value - expected).abs() < TOLERANCE,
            "esperado {expected}, obtido {value}: a diferença passou de {TOLERANCE}"
        ),
        None => panic!("esperado {expected}, mas a função respondeu None"),
    }
}

#[cfg(test)]
mod tests {
    use super::{is_valid, sorted_copy};

    #[test]
    fn test_is_valid() {
        // Uma lista comum é válida.
        assert!(is_valid(&[2.0, 4.0, 4.0]));
        // Lista vazia, com NaN ou com infinito não é válida.
        assert!(!is_valid(&[]));
        assert!(!is_valid(&[1.0, f64::NAN]));
        assert!(!is_valid(&[1.0, f64::INFINITY]));
        assert!(!is_valid(&[f64::NEG_INFINITY, 1.0]));
    }

    #[test]
    fn test_sorted_copy_keeps_original() {
        // A cópia sai ordenada...
        let data = [9.0, 2.0, 5.0];
        assert_eq!(sorted_copy(&data), vec![2.0, 5.0, 9.0]);
        // ...e a lista original continua na ordem de antes.
        assert_eq!(data, [9.0, 2.0, 5.0]);
    }

    #[test]
    fn test_sorted_copy_with_decimals_negatives_and_repeats() {
        // O merge_sort da biblioteca, com o embrulho, ordena decimais,
        // negativos e valores repetidos.
        let data = [3.5, -2.25, 0.0, 3.5, 10.0, -7.75, 0.1];
        assert_eq!(
            sorted_copy(&data),
            vec![-7.75, -2.25, 0.0, 0.1, 3.5, 3.5, 10.0]
        );
    }

    #[test]
    fn test_sorted_copy_single_value() {
        assert_eq!(sorted_copy(&[5.0]), vec![5.0]);
    }
}
