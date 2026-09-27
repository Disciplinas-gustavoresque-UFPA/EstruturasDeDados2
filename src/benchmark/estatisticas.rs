use super::medicao::Medicao;
use std::time::Duration;

/// Estatísticas calculadas a partir de um conjunto de medições.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Estatisticas {
    pub minimo: Duration,
    pub maximo: Duration,
    pub media: Duration,
    pub mediana: Duration,
}

/// Calcula estatísticas básicas das medições.
pub fn calcular(medicoes: &[Medicao]) -> Option<Estatisticas> {
    if medicoes.is_empty() {
        return None;
    }

    let mut tempos: Vec<Duration> = medicoes.iter().map(|medicao| medicao.duracao).collect();

    tempos.sort();

    let minimo = tempos[0];
    let maximo = tempos[tempos.len() - 1];

    let soma: Duration = tempos.iter().sum();
    let media = soma / tempos.len() as u32;

    let mediana = if tempos.len() % 2 == 0 {
        let meio = tempos.len() / 2;
        (tempos[meio - 1] + tempos[meio]) / 2
    } else {
        tempos[tempos.len() / 2]
    };

    Some(Estatisticas {
        minimo,
        maximo,
        media,
        mediana,
    })
}

#[cfg(test)]
mod testes {
    use super::*;

    fn medicao(nanos: u64) -> Medicao {
        Medicao {
            duracao: Duration::from_nanos(nanos),
        }
    }

    #[test]
    fn calcula_estatisticas_com_quantidade_impar() {
        let medicoes = [
            medicao(10),
            medicao(30),
            medicao(20),
            medicao(40),
            medicao(50),
        ];

        let estatisticas = calcular(&medicoes).unwrap();

        assert_eq!(estatisticas.minimo, Duration::from_nanos(10));
        assert_eq!(estatisticas.maximo, Duration::from_nanos(50));
        assert_eq!(estatisticas.media, Duration::from_nanos(30));
        assert_eq!(estatisticas.mediana, Duration::from_nanos(30));
    }

    #[test]
    fn calcula_mediana_com_quantidade_par() {
        let medicoes = [medicao(10), medicao(20), medicao(30), medicao(40)];

        let estatisticas = calcular(&medicoes).unwrap();

        assert_eq!(estatisticas.mediana, Duration::from_nanos(25));
    }

    #[test]
    fn retorna_none_para_medicoes_vazias() {
        assert_eq!(calcular(&[]), None);
    }
}
