use std::fmt;

use super::coefficient_of_variation::coefficient_of_variation;
use super::helpers::{is_valid, sorted_copy};
use super::mean::mean;
use super::mode::mode_sorted;
use super::quantile::quantile_sorted;
use super::std_dev::std_dev;
use super::variance::variance;

/// Todas as medidas de **uma** lista, calculadas de uma vez.
///
/// Covariância e correlação não entram aqui, porque precisam de duas listas.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    /// Quantidade de valores.
    pub n: usize,
    /// Média aritmética.
    pub mean: f64,
    /// Moda: pode ter um, vários ou nenhum valor.
    pub mode: Vec<f64>,
    /// Variância amostral: `None` quando há menos de 2 valores.
    pub variance: Option<f64>,
    /// Desvio padrão: `None` quando há menos de 2 valores.
    pub std_dev: Option<f64>,
    /// Coeficiente de variação: `None` com menos de 2 valores ou média zero.
    pub coefficient_of_variation: Option<f64>,
    /// O menor valor.
    pub min: f64,
    /// Primeiro quartil.
    pub q1: f64,
    /// Mediana.
    pub median: f64,
    /// Terceiro quartil.
    pub q3: f64,
    /// O maior valor.
    pub max: f64,
    /// Intervalo interquartílico: Q3 − Q1.
    pub iqr: f64,
    /// Amplitude: máximo − mínimo.
    pub range: f64,
}

/// Calcula o **resumo completo** de uma lista: todas as medidas de uma vez.
///
/// # Como funciona
///
/// A lista é ordenada **uma única vez**, e a mesma lista ordenada é usada para
/// a moda, os quartis, o mínimo e o máximo. Se cada medida ordenasse a lista
/// por conta própria, seriam várias ordenações, a parte mais cara do cálculo.
///
/// # Exemplo de saída
///
/// Para `[2, 4, 4, 5, 7, 9]`, imprimir o resumo (`println!("{resumo}")`) mostra:
///
/// ```text
/// Resumo estatístico (n = 6)
/// Média ................ 5.1667
/// Moda ................. [4.0000]
/// Variância ............ 6.1667
/// Desvio padrão ........ 2.4833
/// Coef. de variação .... 0.4806
/// Mínimo ............... 2.0000
/// Q1 ................... 4.0000
/// Mediana .............. 4.5000
/// Q3 ................... 6.5000
/// Máximo ............... 9.0000
/// IQR .................. 2.5000
/// Amplitude ............ 7.0000
/// ```
///
/// Quando uma medida não pode ser calculada, aparece `indefinido`, com o
/// motivo entre parênteses.
///
/// # Quando a resposta é `None`
///
/// - Lista vazia.
/// - Lista com `NaN` ou infinito.
///
/// # Complexidade
///
/// - Tempo: O(n log n). Uma ordenação, mais algumas passadas de O(n).
/// - Espaço: O(n), o tamanho da cópia ordenada.
pub fn summary(data: &[f64]) -> Option<Summary> {
    // Passo 1: conferir os dados.
    if !is_valid(data) {
        return None;
    }

    // Passo 2: ordenar uma cópia, UMA única vez.
    let sorted = sorted_copy(data);

    // Passo 3: as medidas de posição, todas lidas da mesma lista ordenada.
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let q1 = quantile_sorted(&sorted, 0.25);
    let q3 = quantile_sorted(&sorted, 0.75);

    // Passo 4: montar a ficha. Média, variância, desvio padrão e CV não
    // precisam da lista ordenada: usam as funções de cada medida.
    Some(Summary {
        n: data.len(),
        mean: mean(data)?,
        mode: mode_sorted(&sorted),
        variance: variance(data),
        std_dev: std_dev(data),
        coefficient_of_variation: coefficient_of_variation(data),
        min,
        q1,
        median: quantile_sorted(&sorted, 0.5),
        q3,
        max,
        iqr: q3 - q1,
        range: max - min,
    })
}

/// Ensina o Rust a **imprimir** a ficha, no formato mostrado em `summary`.
// Como ler: `impl fmt::Display for Summary` cumpre o contrato (trait)
// `Display`, que é o que permite escrever `println!("{}", resumo)`.
// A função `fmt` recebe `f`, o "papel" onde o texto é escrito.
// `writeln!(f, ...)` escreve uma linha nesse papel, e o `?` no fim de cada
// linha interrompe tudo se a escrita falhar.
impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Motivos para "indefinido".
        let few_values = "precisa de pelo menos 2 valores";
        let cv_reason = if self.n < 2 {
            few_values
        } else {
            "média igual a zero"
        };

        writeln!(f, "Resumo estatístico (n = {})", self.n)?;
        writeln!(f, "{}", line("Média", &number(self.mean)))?;
        writeln!(f, "{}", line("Moda", &mode_text(&self.mode)))?;
        writeln!(
            f,
            "{}",
            line("Variância", &optional(self.variance, few_values))
        )?;
        writeln!(
            f,
            "{}",
            line("Desvio padrão", &optional(self.std_dev, few_values))
        )?;
        writeln!(
            f,
            "{}",
            line(
                "Coef. de variação",
                &optional(self.coefficient_of_variation, cv_reason)
            )
        )?;
        writeln!(f, "{}", line("Mínimo", &number(self.min)))?;
        writeln!(f, "{}", line("Q1", &number(self.q1)))?;
        writeln!(f, "{}", line("Mediana", &number(self.median)))?;
        writeln!(f, "{}", line("Q3", &number(self.q3)))?;
        writeln!(f, "{}", line("Máximo", &number(self.max)))?;
        writeln!(f, "{}", line("IQR", &number(self.iqr)))?;
        write!(f, "{}", line("Amplitude", &number(self.range)))
    }
}

/// Monta uma linha do resumo: o nome, pontinhos até a coluna do valor, e o
/// valor. Exemplo: `Média ................ 5.1667`
fn line(label: &str, value: &str) -> String {
    // Passo 1: contar as letras do nome (acentos contam como uma letra só).
    let mut letters = 0;
    for _ in label.chars() {
        letters += 1;
    }

    // Passo 2: completar com pontinhos até a coluna 21.
    let mut dots = String::new();
    for _ in letters..21 {
        dots.push('.');
    }

    format!("{label} {dots} {value}")
}

/// Escreve um número com 4 casas decimais. Exemplo: 5.166666… → `5.1667`
fn number(value: f64) -> String {
    format!("{value:.4}")
}

/// Escreve um valor que pode não existir: o número, ou `indefinido (motivo)`.
fn optional(value: Option<f64>, reason: &str) -> String {
    match value {
        Some(v) => number(v),
        None => format!("indefinido ({reason})"),
    }
}

/// Escreve a moda: `nenhuma`, ou a lista de valores. Exemplo: `[1.0000, 2.0000]`
fn mode_text(mode: &[f64]) -> String {
    if mode.is_empty() {
        return "nenhuma".to_string();
    }

    // Monta "[a, b, c]", colocando ", " entre um valor e o seguinte.
    let mut text = String::from("[");
    let mut first = true;
    for &value in mode {
        if !first {
            text.push_str(", ");
        }
        text.push_str(&number(value));
        first = false;
    }
    text.push(']');
    text
}

#[cfg(test)]
mod tests {
    use super::summary;
    use crate::statistics::helpers::assert_close;
    use crate::statistics::{
        coefficient_of_variation, five_number_summary, iqr, mean, mode, range, std_dev, variance,
    };

    const X: [f64; 6] = [2.0, 4.0, 4.0, 5.0, 7.0, 9.0];

    #[test]
    fn test_main_example() {
        let s = summary(&X).unwrap();
        assert_eq!(s.n, 6);
        assert_close(Some(s.mean), 31.0 / 6.0);
        assert_eq!(s.mode, vec![4.0]);
        assert_close(s.variance, 37.0 / 6.0);
        assert_close(s.std_dev, (37.0_f64 / 6.0).sqrt());
        assert_close(
            s.coefficient_of_variation,
            (37.0_f64 / 6.0).sqrt() / (31.0 / 6.0),
        );
        assert_close(Some(s.min), 2.0);
        assert_close(Some(s.q1), 4.0);
        assert_close(Some(s.median), 4.5);
        assert_close(Some(s.q3), 6.5);
        assert_close(Some(s.max), 9.0);
        assert_close(Some(s.iqr), 2.5);
        assert_close(Some(s.range), 7.0);
    }

    #[test]
    fn test_agrees_with_individual_functions() {
        // O resumo precisa dar exatamente o mesmo que cada função sozinha,
        // mesmo com dados fora de ordem, repetidos e negativos.
        let data = [3.5, -2.0, 7.25, 3.5, 0.0, 10.0, -2.0, 3.5];
        let s = summary(&data).unwrap();
        let five = five_number_summary(&data).unwrap();
        assert_eq!(Some(s.mean), mean(&data));
        assert_eq!(Some(s.mode.clone()), mode(&data));
        assert_eq!(s.variance, variance(&data));
        assert_eq!(s.std_dev, std_dev(&data));
        assert_eq!(s.coefficient_of_variation, coefficient_of_variation(&data));
        assert_eq!(
            (s.min, s.q1, s.median, s.q3, s.max),
            (five.min, five.q1, five.median, five.q3, five.max)
        );
        assert_eq!(Some(s.iqr), iqr(&data));
        assert_eq!(Some(s.range), range(&data));
    }

    #[test]
    fn test_single_value() {
        // Um valor só: variância, desvio padrão e CV não existem.
        let s = summary(&[5.0]).unwrap();
        assert_close(Some(s.mean), 5.0);
        assert_eq!(s.mode, vec![5.0]);
        assert_eq!(s.variance, None);
        assert_eq!(s.std_dev, None);
        assert_eq!(s.coefficient_of_variation, None);
        assert_close(Some(s.iqr), 0.0);
        assert_close(Some(s.range), 0.0);
    }

    #[test]
    fn test_printed_text() {
        // O texto impresso precisa ser exatamente este.
        let expected = "\
Resumo estatístico (n = 6)
Média ................ 5.1667
Moda ................. [4.0000]
Variância ............ 6.1667
Desvio padrão ........ 2.4833
Coef. de variação .... 0.4806
Mínimo ............... 2.0000
Q1 ................... 4.0000
Mediana .............. 4.5000
Q3 ................... 6.5000
Máximo ............... 9.0000
IQR .................. 2.5000
Amplitude ............ 7.0000";
        assert_eq!(summary(&X).unwrap().to_string(), expected);
    }

    #[test]
    fn test_printed_undefined() {
        // Com um valor só, aparece "indefinido" com o motivo.
        let text = summary(&[5.0]).unwrap().to_string();
        assert!(
            text.contains("Variância ............ indefinido (precisa de pelo menos 2 valores)")
        );
        // Com média zero, o motivo do CV é outro.
        let text = summary(&[-1.0, 1.0]).unwrap().to_string();
        assert!(text.contains("Coef. de variação .... indefinido (média igual a zero)"));
        // Sem moda, aparece "nenhuma".
        let text = summary(&[1.0, 2.0, 3.0]).unwrap().to_string();
        assert!(text.contains("Moda ................. nenhuma"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(summary(&[]), None);
    }

    #[test]
    fn test_with_nan() {
        assert_eq!(summary(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn print_summary_demo() {
        // Demonstração: para ver o resumo na tela, rode
        //   cargo test print_summary_demo -- --nocapture
        let resumo = summary(&X).unwrap();
        println!("\nDados: {X:?}\n\n{resumo}\n");
    }
}
