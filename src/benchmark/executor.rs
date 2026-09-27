use super::configuracao::{Algoritmo, TipoEntrada};
use super::medicao::{Medicao, medir};
use crate::algorithms::{
    heap::heap_sort, insertion::insertion_sort, merge::merge_sort, quick::quick_sort,
};
use crate::geradores::entrada::{
    entrada_aleatoria, entrada_com_duplicatas, entrada_invertida, entrada_ordenada,
    entrada_quase_ordenada,
};

/// Executa uma medição para uma combinação específica de algoritmo,
/// tipo de entrada e tamanho.
pub fn executar(algoritmo: Algoritmo, tipo_entrada: TipoEntrada, tamanho: usize) -> Medicao {
    let entrada = gerar_entrada(tipo_entrada, tamanho);
    medir(selecionar_algoritmo(algoritmo), &entrada)
}

fn selecionar_algoritmo(algoritmo: Algoritmo) -> fn(&mut [i32]) {
    match algoritmo {
        Algoritmo::Insercao => insertion_sort,
        Algoritmo::Merge => merge_sort,
        Algoritmo::Quick => quick_sort,
        Algoritmo::Heap => heap_sort,
    }
}

fn gerar_entrada(tipo_entrada: TipoEntrada, tamanho: usize) -> Vec<i32> {
    match tipo_entrada {
        TipoEntrada::Aleatoria => entrada_aleatoria(tamanho),
        TipoEntrada::Ordenada => entrada_ordenada(tamanho),
        TipoEntrada::Invertida => entrada_invertida(tamanho),
        TipoEntrada::QuaseOrdenada => entrada_quase_ordenada(tamanho),
        TipoEntrada::ComDuplicatas => entrada_com_duplicatas(tamanho),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn executa_todas_as_combinacoes_de_algoritmos() {
        let algoritmos = [
            Algoritmo::Insercao,
            Algoritmo::Merge,
            Algoritmo::Quick,
            Algoritmo::Heap,
        ];

        for algoritmo in algoritmos {
            let medicao = executar(algoritmo, TipoEntrada::Aleatoria, 20);

            assert!(medicao.duracao >= std::time::Duration::ZERO);
        }
    }

    #[test]
    fn executa_todos_os_tipos_de_entrada() {
        let entradas = [
            TipoEntrada::Aleatoria,
            TipoEntrada::Ordenada,
            TipoEntrada::Invertida,
            TipoEntrada::QuaseOrdenada,
            TipoEntrada::ComDuplicatas,
        ];

        for tipo_entrada in entradas {
            let medicao = executar(Algoritmo::Merge, tipo_entrada, 20);

            assert!(medicao.duracao >= std::time::Duration::ZERO);
        }
    }

    #[test]
    fn aceita_entrada_vazia() {
        let medicao = executar(Algoritmo::Heap, TipoEntrada::Aleatoria, 0);

        assert!(medicao.duracao >= std::time::Duration::ZERO);
    }
}
