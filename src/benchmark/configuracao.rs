/// Algoritmos de ordenação avaliados pelo benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algoritmo {
    Insercao,
    Merge,
    Quick,
    Heap,
}

/// Tipos de entrada utilizados nos experimentos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoEntrada {
    Aleatoria,
    Ordenada,
    Invertida,
    QuaseOrdenada,
    ComDuplicatas,
}

impl Algoritmo {
    /// Retorna o nome do algoritmo para apresentação dos resultados.
    pub fn nome(self) -> &'static str {
        match self {
            Self::Insercao => "insercao",
            Self::Merge => "merge",
            Self::Quick => "quick",
            Self::Heap => "heap",
        }
    }
}

impl TipoEntrada {
    /// Retorna o nome do tipo de entrada para apresentação dos resultados.
    pub fn nome(self) -> &'static str {
        match self {
            Self::Aleatoria => "aleatoria",
            Self::Ordenada => "ordenada",
            Self::Invertida => "invertida",
            Self::QuaseOrdenada => "quase_ordenada",
            Self::ComDuplicatas => "com_duplicatas",
        }
    }
}

/// Configuração de uma execução experimental.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfiguracaoExperimento {
    pub algoritmo: Algoritmo,
    pub tipo_entrada: TipoEntrada,
    pub tamanho: usize,
    pub repeticoes: usize,
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn retorna_nome_dos_algoritmos() {
        assert_eq!(Algoritmo::Insercao.nome(), "insercao");
        assert_eq!(Algoritmo::Merge.nome(), "merge");
        assert_eq!(Algoritmo::Quick.nome(), "quick");
        assert_eq!(Algoritmo::Heap.nome(), "heap");
    }

    #[test]
    fn retorna_nome_dos_tipos_de_entrada() {
        assert_eq!(TipoEntrada::Aleatoria.nome(), "aleatoria");
        assert_eq!(TipoEntrada::Ordenada.nome(), "ordenada");
        assert_eq!(TipoEntrada::Invertida.nome(), "invertida");
        assert_eq!(TipoEntrada::QuaseOrdenada.nome(), "quase_ordenada");
        assert_eq!(TipoEntrada::ComDuplicatas.nome(), "com_duplicatas");
    }

    #[test]
    fn cria_configuracao_com_dez_repeticoes() {
        let configuracao = ConfiguracaoExperimento {
            algoritmo: Algoritmo::Quick,
            tipo_entrada: TipoEntrada::Aleatoria,
            tamanho: 1_000,
            repeticoes: 10,
        };
        assert_eq!(configuracao.repeticoes, 10);
        assert_eq!(configuracao.tamanho, 1_000);
    }
}
