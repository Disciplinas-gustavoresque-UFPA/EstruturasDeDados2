use std::hint::black_box;
use std::time::{Duration, Instant};

/// Resultado de uma medição de tempo de execução.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Medicao {
    /// Tempo decorrido durante a execução do algoritmo.
    pub duracao: Duration,
}

/// Executa um algoritmo de ordenação e mede seu tempo de execução.
///
/// A cópia da entrada não faz parte da medição. O resultado é validado
/// após a execução para garantir que o algoritmo realmente ordenou os dados.
pub fn medir(algoritmo: fn(&mut [i32]), entrada: &[i32]) -> Medicao {
    let mut dados = entrada.to_vec();

    let inicio = Instant::now();
    algoritmo(black_box(&mut dados));
    let duracao = inicio.elapsed();

    assert!(
        dados.windows(2).all(|janela| janela[0] <= janela[1]),
        "o algoritmo não ordenou corretamente a entrada"
    );

    Medicao { duracao }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::algorithms::insertion::insertion_sort;

    #[test]
    fn mede_algoritmo_de_ordenacao() {
        let entrada = [5, 2, 4, 1, 3];

        let medicao = medir(insertion_sort, &entrada);

        assert!(medicao.duracao >= Duration::ZERO);
    }

    #[test]
    fn preserva_a_entrada_original() {
        let entrada = [5, 2, 4, 1, 3];

        let _ = medir(insertion_sort, &entrada);

        assert_eq!(entrada, [5, 2, 4, 1, 3]);
    }

    #[test]
    #[should_panic(expected = "o algoritmo não ordenou corretamente")]
    fn detecta_algoritmo_incorreto() {
        fn algoritmo_incorreto(_dados: &mut [i32]) {}

        medir(algoritmo_incorreto, &[3, 2, 1]);
    }
}
