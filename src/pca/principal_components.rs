//! A **análise de componentes principais** completa: junta as cinco etapas
//! (álgebra linear, centralização, matriz de covariância, iteração de
//! potências com deflação e projeção) numa única função, `pca`.

use std::fmt;

use super::center::{center, column_means};
use super::covariance_matrix::covariance_matrix;
use super::helpers::{is_valid_matrix, largest_absolute};
use super::power_iteration::top_eigenpairs;
use super::projection::project;

/// O resultado de uma análise de componentes principais.
///
/// Para uma tabela de n observações (linhas) e p variáveis (colunas),
/// analisada com k componentes:
#[derive(Debug, Clone, PartialEq)]
pub struct Pca {
    /// A média de cada variável: p números. É o ponto em torno do qual os
    /// dados foram centralizados.
    pub means: Vec<f64>,
    /// Os k maiores autovalores da matriz de covariância, em ordem
    /// decrescente. Cada um é a variância dos dados ao longo de um componente.
    pub eigenvalues: Vec<f64>,
    /// A fração da variância total que cada componente explica: autovalor ÷
    /// soma das variâncias de todas as p variáveis. Fica entre 0 e 1.
    pub explained_variance_ratio: Vec<f64>,
    /// Os k componentes principais (autovetores), um por linha, cada um com p
    /// pesos. Têm norma 1, são perpendiculares entre si, e o peso de maior
    /// valor absoluto de cada um é positivo.
    pub components: Vec<Vec<f64>>,
    /// Os escores: cada observação escrita nos novos eixos. Uma linha por
    /// observação (n linhas), com um número por componente (k colunas).
    pub scores: Vec<Vec<f64>>,
}

/// Faz a **análise de componentes principais** (PCA) de uma tabela de dados,
/// guardando os `k` componentes mais importantes.
///
/// # O que é
///
/// Imagine os dados como uma nuvem de pontos, uma observação por ponto e uma
/// variável por eixo. O PCA gira os eixos para que o primeiro aponte para a
/// direção em que a nuvem mais se espalha (a de maior variância); o segundo,
/// para a direção de maior variância que sobra, perpendicular ao primeiro; e
/// assim por diante. Esses novos eixos são os **componentes principais**.
/// Muitas vezes poucos componentes explicam quase toda a variação, e eles
/// resumem os dados com menos números.
///
/// # Como a conta é feita
///
/// 1. Conferir a tabela e o `k`.
/// 2. Calcular as médias das variáveis (etapa 2).
/// 3. **Normalizar**: dividir a tabela inteira pelo seu maior valor absoluto
///    s. Os valores ficam entre −1 e 1, e as contas seguintes não estouram
///    nem somem, mesmo com dados perto de 1e300 ou de 1e-300. Dividir tudo
///    pelo mesmo número não muda as direções.
/// 4. Centralizar (etapa 2) e calcular a matriz de covariância (etapa 3).
/// 5. Achar os k maiores autovalores e autovetores pela iteração de potências
///    com deflação (etapa 4).
/// 6. Escolher o sinal de cada componente (se v é autovetor, −v também é): o
///    peso de maior valor absoluto fica positivo, ou seja, a variável que
///    mais pesa no componente entra com sinal +. Se dois pesos empatam, vale
///    o primeiro deles.
/// 7. Projetar os dados centralizados sobre os componentes (etapa 5).
/// 8. Voltar à escala original: escores × s e autovalores × s × s. As
///    proporções não mudam com a escala.
///
/// # Exemplo, passo a passo
///
/// ```text
/// dados: [1, 2], [2, 1], [3, 4], [4, 3]          (n = 4, p = 2)
///
/// médias:        [2,5; 2,5]
/// covariância:   [[5/3, 1], [1, 5/3]]
/// autovalores:   5/3 + 1 = 8/3 = 2,6667   e   5/3 − 1 = 2/3 = 0,6667
/// variância total: 5/3 + 5/3 = 10/3
/// proporções:    (8/3) ÷ (10/3) = 0,8   e   (2/3) ÷ (10/3) = 0,2
/// componentes:   (1, 1)/√2 = (0,7071; 0,7071)
///                (1, −1)/√2 = (0,7071; −0,7071)
/// escores da 1ª observação, centralizada em (−1,5; −0,5):
///                (−1,5 − 0,5)/√2 = −1,4142   e   (−1,5 + 0,5)/√2 = −0,7071
/// ```
///
/// O primeiro componente explica 80% da variação: os pontos se espalham
/// sobretudo ao longo da direção (1, 1), em que as duas variáveis crescem
/// juntas.
///
/// # Quando a resposta é `None`
///
/// - Tabela vazia, linhas de tamanhos diferentes, `NaN` ou infinito.
/// - Menos de 2 observações: não existe variância amostral (n − 1 = 0).
/// - `k` igual a 0 ou maior que o número de variáveis.
/// - Todas as observações iguais: nenhuma variável varia, e não existe
///   direção principal.
///
/// Se um autovalor verdadeiro não couber no `f64` (dados perto de 1e200, por
/// exemplo), ele vira infinito, mas os componentes, as proporções e os
/// escores continuam certos.
///
/// # Complexidade
///
/// Para n observações, p variáveis, k componentes e t repetições da iteração
/// de potências por componente:
///
/// - Tempo: O(n·p² + k·t·p²). A matriz de covariância custa O(n·p²), os
///   autovetores O(k·t·p²) e a projeção O(n·p·k), que nunca passa de O(n·p²)
///   porque k ≤ p. As médias, a normalização e a centralização são O(n·p).
/// - Espaço: O(n·p + p²): a tabela normalizada e centralizada, a matriz de
///   covariância e os escores.
pub fn pca(data: &[Vec<f64>], k: usize) -> Option<Pca> {
    // Passo 1: conferir a tabela (pelo menos 2 observações) e o k.
    if !is_valid_matrix(data) || data.len() < 2 {
        return None;
    }
    let p = data[0].len();
    if k == 0 || k > p {
        return None;
    }

    // Se todas as linhas são iguais, nenhuma variável varia. (Sem este teste,
    // o arredondamento de valores como 0,1 criaria uma variância falsa.)
    if data.iter().all(|row| *row == data[0]) {
        return None;
    }

    // Passo 2: as médias das variáveis, na escala original.
    let means = column_means(data)?;

    // Passo 3: normalizar: dividir a tabela pelo seu maior valor absoluto s.
    // Como as linhas não são todas iguais, a tabela tem algum valor diferente
    // de zero, e s nunca é 0.
    let scale = data
        .iter()
        .map(|row| largest_absolute(row))
        .fold(0.0, f64::max);
    let scaled: Vec<Vec<f64>> = data
        .iter()
        .map(|row| row.iter().map(|&x| x / scale).collect())
        .collect();

    // Passo 4: centralizar e calcular a matriz de covariância da tabela
    // normalizada. A variância total é a soma da diagonal da matriz. Se ela
    // for 0 (variação pequena demais para o f64), não há o que analisar.
    let (_, centered) = center(&scaled)?;
    let covariance = covariance_matrix(&scaled)?;
    let total_variance: f64 = covariance.iter().enumerate().map(|(i, row)| row[i]).sum();
    if total_variance == 0.0 {
        return None;
    }

    // Passo 5: os k maiores autovalores e autovetores.
    let (values, vectors) = top_eigenpairs(&covariance, k)?;

    // Passo 6: o sinal de cada componente.
    let components: Vec<Vec<f64>> = vectors
        .iter()
        .map(|v| with_positive_largest_weight(v))
        .collect();

    // Passo 7: os escores, de volta à escala original (× s).
    let scores: Vec<Vec<f64>> = project(&centered, &components)?
        .iter()
        .map(|row| row.iter().map(|&x| x * scale).collect())
        .collect();

    // Passo 8: os autovalores na escala original (× s duas vezes) e as
    // proporções da variância total.
    let eigenvalues: Vec<f64> = values.iter().map(|&value| value * scale * scale).collect();
    let explained_variance_ratio: Vec<f64> =
        values.iter().map(|&value| value / total_variance).collect();

    Some(Pca {
        means,
        eigenvalues,
        explained_variance_ratio,
        components,
        scores,
    })
}

/// Escolhe o sinal do componente: se o peso de maior valor absoluto for
/// negativo, troca o sinal de todos os pesos.
///
/// Dois pesos que diferem menos de 10⁻⁹ em valor absoluto contam como
/// empate, e vale o primeiro deles. Assim, em (0,7071; −0,7071), o
/// arredondamento não decide o sinal: o primeiro peso fica positivo.
///
/// Por que não "o primeiro peso diferente de zero"? Porque pesos que
/// deveriam ser 0 saem com um ruído de arredondamento (por exemplo, 10⁻¹⁰),
/// e esse ruído decidiria o sinal do componente inteiro.
fn with_positive_largest_weight(v: &[f64]) -> Vec<f64> {
    let largest = largest_absolute(v);
    let dominant = v
        .iter()
        .copied()
        .find(|w| w.abs() >= largest - 1e-9)
        .unwrap_or(0.0);
    if dominant < 0.0 {
        v.iter().map(|&w| -w).collect()
    } else {
        v.to_vec()
    }
}

/// Quantas observações aparecem na tabela de escores impressa.
const SHOWN_OBSERVATIONS: usize = 10;

// Como ler: `impl fmt::Display for Pca` cumpre o contrato (trait) `Display`,
// que é o que permite escrever `println!("{}", resultado)`. A função `fmt`
// recebe `f`, o "papel" onde o texto é escrito; `writeln!(f, ...)` escreve uma
// linha nele, e o `?` no fim interrompe tudo se a escrita falhar.
impl fmt::Display for Pca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.scores.len();
        let p = self.means.len();
        let variables = if p == 1 { "variável" } else { "variáveis" };
        writeln!(
            f,
            "Análise de componentes principais (n = {n} observações, p = {p} {variables})"
        )?;
        writeln!(f)?;

        // Tabela 1: autovalores e variância explicada.
        writeln!(
            f,
            "{:<12}{:>12}{:>22}{:>12}",
            "Componente", "Autovalor", "Variância explicada", "Acumulada"
        )?;
        let mut cumulative = 0.0;
        for (c, (&value, &ratio)) in self
            .eigenvalues
            .iter()
            .zip(&self.explained_variance_ratio)
            .enumerate()
        {
            cumulative += ratio;
            writeln!(
                f,
                "{:<12}{:>12}{:>22}{:>12}",
                format!("CP{}", c + 1),
                number(value),
                percent(ratio),
                percent(cumulative)
            )?;
        }
        writeln!(f)?;

        // Tabela 2: os pesos (cada linha é uma variável, cada coluna um
        // componente).
        writeln!(f, "Pesos das variáveis em cada componente (autovetores):")?;
        writeln!(f, "{}", header(self.components.len()))?;
        for i in 0..p {
            let weights: String = self
                .components
                .iter()
                .map(|component| format!("{:>12}", weight(component[i])))
                .collect();
            writeln!(f, "{:<12}{weights}", format!("Variável {}", i + 1))?;
        }
        writeln!(f)?;

        // Tabela 3: os escores das primeiras observações. Valores menores que
        // 10⁻¹² vezes o maior escore são só arredondamento e aparecem como 0.
        let largest = self
            .scores
            .iter()
            .map(|row| largest_absolute(row))
            .fold(0.0, f64::max);
        writeln!(f, "Escores (as observações nos novos eixos):")?;
        write!(f, "{}", header(self.components.len()))?;
        for (i, row) in self.scores.iter().take(SHOWN_OBSERVATIONS).enumerate() {
            let values: String = row
                .iter()
                .map(|&x| {
                    let x = if x.abs() <= 1e-12 * largest { 0.0 } else { x };
                    format!("{:>12}", number(x))
                })
                .collect();
            write!(f, "\n{:<12}{values}", format!("Obs. {}", i + 1))?;
        }
        if n > SHOWN_OBSERVATIONS {
            write!(f, "\n... (mais {} observações)", n - SHOWN_OBSERVATIONS)?;
        }
        Ok(())
    }
}

/// O cabeçalho das tabelas de pesos e de escores: CP1, CP2, ...
fn header(k: usize) -> String {
    let names: String = (1..=k)
        .map(|c| format!("{:>12}", format!("CP{c}")))
        .collect();
    format!("{:<12}{names}", "")
}

/// Um número com 4 casas decimais, ou em notação científica (por exemplo,
/// `2.6667e300`) quando ele é grande ou pequeno demais para isso.
fn number(value: f64) -> String {
    let size = value.abs();
    if value == 0.0 {
        "0.0000".to_string()
    } else if (1e-4..1e6).contains(&size) {
        format!("{value:.4}")
    } else {
        format!("{value:.4e}")
    }
}

/// Um peso (entre −1 e 1) com 4 casas decimais. Um peso que arredonda para
/// zero aparece como `0.0000`, e nunca como `-0.0000`.
fn weight(value: f64) -> String {
    if value.abs() < 0.00005 {
        "0.0000".to_string()
    } else {
        format!("{value:.4}")
    }
}

/// Uma proporção (entre 0 e 1) como porcentagem, com 2 casas decimais.
fn percent(ratio: f64) -> String {
    format!("{:.2}%", ratio * 100.0)
}

#[cfg(test)]
mod tests {
    use super::pca;
    use crate::pca::center;
    use crate::pca::helpers::{assert_close, assert_vector_close};
    use crate::pca::linear_algebra::{dot, norm};

    /// A tabela do exemplo: 4 observações (linhas) de 2 variáveis (colunas).
    fn example() -> Vec<Vec<f64>> {
        vec![
            vec![1.0, 2.0],
            vec![2.0, 1.0],
            vec![3.0, 4.0],
            vec![4.0, 3.0],
        ]
    }

    /// A tabela do exemplo com cada valor multiplicado por `factor`.
    fn example_times(factor: f64) -> Vec<Vec<f64>> {
        example()
            .iter()
            .map(|row| row.iter().map(|&x| x * factor).collect())
            .collect()
    }

    /// A coluna `c` de uma tabela.
    fn column(table: &[Vec<f64>], c: usize) -> Vec<f64> {
        table.iter().map(|row| row[c]).collect()
    }

    #[test]
    fn test_main_example() {
        // Médias [2,5; 2,5]. Covariância [[5/3, 1], [1, 5/3]], com autovalores
        // 5/3 + 1 = 8/3 e 5/3 − 1 = 2/3. Variância total: 5/3 + 5/3 = 10/3.
        // Proporções: (8/3) ÷ (10/3) = 0,8 e (2/3) ÷ (10/3) = 0,2.
        // Componentes: (1, 1)/√2 e (1, −1)/√2. No segundo, os dois pesos
        // empatam em valor absoluto, e o primeiro fica positivo.
        // Escores: (a + b)/√2 e (a − b)/√2 para cada linha centralizada (a, b).
        let s = 2f64.sqrt();
        let result = pca(&example(), 2).unwrap();
        assert_eq!(result.means, vec![2.5, 2.5]);
        assert_vector_close(&result.eigenvalues, &[8.0 / 3.0, 2.0 / 3.0]);
        assert_vector_close(&result.explained_variance_ratio, &[0.8, 0.2]);
        assert_vector_close(&result.components[0], &[1.0 / s, 1.0 / s]);
        assert_vector_close(&result.components[1], &[1.0 / s, -1.0 / s]);
        assert_vector_close(&column(&result.scores, 0), &[-s, -s, s, s]);
        assert_vector_close(
            &column(&result.scores, 1),
            &[-1.0 / s, 1.0 / s, -1.0 / s, 1.0 / s],
        );
    }

    #[test]
    fn test_only_the_first_component() {
        let result = pca(&example(), 1).unwrap();
        assert_vector_close(&result.eigenvalues, &[8.0 / 3.0]);
        assert_vector_close(&result.explained_variance_ratio, &[0.8]);
        assert_eq!(result.components.len(), 1);
        assert_eq!(result.scores.len(), 4);
        assert!(result.scores.iter().all(|row| row.len() == 1));
    }

    #[test]
    fn test_single_variable() {
        // Uma variável só: o único componente é o próprio eixo, [1]; o
        // autovalor é a variância, ((−3)² + (−1)² + 4²) ÷ 2 = 26 ÷ 2 = 13; e os
        // escores são os dados centralizados: [2 − 5; 4 − 5; 9 − 5].
        let result = pca(&[vec![2.0], vec![4.0], vec![9.0]], 1).unwrap();
        assert_vector_close(&result.eigenvalues, &[13.0]);
        assert_vector_close(&result.explained_variance_ratio, &[1.0]);
        assert_vector_close(&result.components[0], &[1.0]);
        assert_vector_close(&column(&result.scores, 0), &[-3.0, -1.0, 4.0]);
    }

    #[test]
    fn test_perfectly_correlated_variables() {
        // y = 2x. var(x) = 5/3, var(y) = 4 · 5/3 e cov = 2 · 5/3, então a
        // covariância é (5/3) · [[1, 2], [2, 4]], com autovalores (5/3) · 5 = 25/3
        // e 0, e componentes (1, 2)/√5 e (2, −1)/√5. Toda a variação está no
        // primeiro componente. Escores nele: (xc + 2·yc)/√5, por exemplo
        // (−1,5 + 2 · (−3))/√5 = −7,5/√5; no segundo, (2·xc − yc)/√5 = 0.
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 4.0],
            vec![3.0, 6.0],
            vec![4.0, 8.0],
        ];
        let r5 = 5f64.sqrt();
        let result = pca(&data, 2).unwrap();
        assert_vector_close(&result.eigenvalues, &[25.0 / 3.0, 0.0]);
        assert_vector_close(&result.explained_variance_ratio, &[1.0, 0.0]);
        assert_vector_close(&result.components[0], &[1.0 / r5, 2.0 / r5]);
        assert_vector_close(&result.components[1], &[2.0 / r5, -1.0 / r5]);
        assert_vector_close(
            &column(&result.scores, 0),
            &[-7.5 / r5, -2.5 / r5, 2.5 / r5, 7.5 / r5],
        );
        assert_vector_close(&column(&result.scores, 1), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_perfectly_opposite_variables() {
        // y = −x, com var(x) = ((−1)² + 0² + 1²) ÷ 2 = 1: covariância
        // [[1, −1], [−1, 1]], autovalores 2 e 0, componentes (1, −1)/√2 e
        // (1, 1)/√2. (Um vetor inicial (1, 1) cairia perpendicular ao primeiro.)
        let data = vec![vec![1.0, -1.0], vec![2.0, -2.0], vec![3.0, -3.0]];
        let s = 2f64.sqrt();
        let result = pca(&data, 2).unwrap();
        assert_vector_close(&result.eigenvalues, &[2.0, 0.0]);
        assert_vector_close(&result.components[0], &[1.0 / s, -1.0 / s]);
        assert_vector_close(&result.components[1], &[1.0 / s, 1.0 / s]);
    }

    #[test]
    fn test_one_constant_variable() {
        // A variável 1 não varia. Toda a variação está na variável 2, com
        // variância ((−2)² + (−1)² + 3²) ÷ 2 = 14 ÷ 2 = 7.
        let data = vec![vec![3.0, 1.0], vec![3.0, 2.0], vec![3.0, 6.0]];
        let result = pca(&data, 2).unwrap();
        assert_vector_close(&result.eigenvalues, &[7.0, 0.0]);
        assert_vector_close(&result.explained_variance_ratio, &[1.0, 0.0]);
        assert_vector_close(&result.components[0], &[0.0, 1.0]);
        assert_vector_close(&result.components[1], &[1.0, 0.0]);
    }

    #[test]
    fn test_many_variables() {
        // 64 observações de 30 variáveis. A coluna j (j = 1, ..., 30) é j vezes
        // um padrão de +1 e −1 (a coluna j de uma matriz de Hadamard): o sinal
        // da linha i é + quando i e j têm um número par de bits 1 em comum.
        // Esses padrões têm média 0, são perpendiculares entre si e têm 64
        // quadrados iguais a 1. Então a covariância é diagonal, com
        // j² · 64 ÷ 63 na variável j, e os componentes são os eixos, da
        // variável 30 (a de maior variância) até a variável 1.
        //
        // As tolerâncias são relativas à variância total: os autovetores dos
        // menores autovalores saem com menos casas certas (veja o README).
        let data: Vec<Vec<f64>> = (0..64u32)
            .map(|i| {
                (1..=30u32)
                    .map(|j| {
                        let sign = if (i & j).count_ones() % 2 == 0 {
                            1.0
                        } else {
                            -1.0
                        };
                        j as f64 * sign
                    })
                    .collect()
            })
            .collect();
        let result = pca(&data, 30).unwrap();

        let sum_of_squares: f64 = (1..=30).map(|j| (j * j) as f64).sum(); // 9455
        let total_variance = sum_of_squares * 64.0 / 63.0;
        for (c, j) in (1..=30usize).rev().enumerate() {
            let jj = (j * j) as f64;
            assert!((result.eigenvalues[c] - jj * 64.0 / 63.0).abs() <= 1e-9 * total_variance);
            assert!((result.explained_variance_ratio[c] - jj / sum_of_squares).abs() <= 1e-9);
            for (i, &weight) in result.components[c].iter().enumerate() {
                let expected = if i == j - 1 { 1.0 } else { 0.0 };
                assert!(
                    (weight - expected).abs() <= 1e-6,
                    "componente {c}, peso {i}"
                );
            }
        }
    }

    #[test]
    fn test_more_variables_than_observations() {
        // 3 observações de 5 variáveis: as linhas são os eixos 1, 2 e 3.
        // Médias (1/3, 1/3, 1/3, 0, 0). Centralizadas: (2/3, −1/3, −1/3, 0, 0)
        // e as suas trocas de posição. Covariância (n − 1 = 2): no bloco das 3
        // primeiras variáveis, (4/9 + 1/9 + 1/9) ÷ 2 = 1/3 na diagonal e
        // (−2/9 − 2/9 + 1/9) ÷ 2 = −1/6 fora dela; zero no resto. Autovalores
        // do bloco: 1/3 + 1/6 = 1/2 (duas vezes) e 1/3 − 2/6 = 0; e 0 nas
        // variáveis 4 e 5. Com 3 observações, no máximo 2 direções variam.
        let data = vec![
            vec![1.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0, 0.0],
        ];
        let result = pca(&data, 5).unwrap();
        assert_vector_close(&result.eigenvalues, &[0.5, 0.5, 0.0, 0.0, 0.0]);
        assert_vector_close(&result.explained_variance_ratio, &[0.5, 0.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_properties() {
        // Para uma tabela qualquer, com todos os componentes (k = p):
        //   1. componentes de norma 1 e perpendiculares entre si;
        //   2. o peso de maior valor absoluto de cada componente é positivo;
        //   3. as proporções somam 1;
        //   4. cada coluna de escores tem média 0 e variância igual ao autovalor;
        //   5. nada se perde: escores × componentes devolvem os dados
        //      centralizados.
        let data: Vec<Vec<f64>> = (0..12)
            .map(|i| {
                (0..4)
                    .map(|j| ((i * 5 + j * 7 + i * j) % 13) as f64 - 6.0 + j as f64 * 0.5)
                    .collect()
            })
            .collect();
        let result = pca(&data, 4).unwrap();

        for (c, u) in result.components.iter().enumerate() {
            assert_close(norm(u).unwrap(), 1.0);
            for w in &result.components[c + 1..] {
                assert_close(dot(u, w).unwrap(), 0.0);
            }
            let largest = u.iter().fold(0.0, |m: f64, w| m.max(w.abs()));
            let dominant = u.iter().find(|w| w.abs() >= largest - 1e-9).unwrap();
            assert!(*dominant > 0.0);
        }

        assert_close(result.explained_variance_ratio.iter().sum(), 1.0);

        for (c, &eigenvalue) in result.eigenvalues.iter().enumerate() {
            let scores = column(&result.scores, c);
            let mean = scores.iter().sum::<f64>() / 12.0;
            let variance = scores.iter().map(|s| s * s).sum::<f64>() / 11.0;
            assert_close(mean, 0.0);
            assert_close(variance, eigenvalue);
        }

        let (_, centered) = center(&data).unwrap();
        for (row, scores) in centered.iter().zip(&result.scores) {
            for (j, &value) in row.iter().enumerate() {
                let rebuilt: f64 = scores
                    .iter()
                    .zip(&result.components)
                    .map(|(&s, u)| s * u[j])
                    .sum();
                assert_close(rebuilt, value);
            }
        }
    }

    #[test]
    fn test_invalid_input() {
        // Tabela vazia.
        assert_eq!(pca(&[], 1), None);
        // Uma única observação: não existe variância amostral (n − 1 = 0).
        assert_eq!(pca(&[vec![1.0, 2.0]], 1), None);
        // k = 0, ou k maior que o número de variáveis.
        assert_eq!(pca(&example(), 0), None);
        assert_eq!(pca(&example(), 3), None);
        // Linhas de tamanhos diferentes, NaN, infinito.
        assert_eq!(pca(&[vec![1.0, 2.0], vec![3.0]], 1), None);
        assert_eq!(pca(&[vec![1.0, f64::NAN], vec![3.0, 4.0]], 1), None);
        assert_eq!(pca(&[vec![1.0, 2.0], vec![f64::INFINITY, 4.0]], 1), None);
    }

    #[test]
    fn test_no_variable_varies() {
        // Todas as linhas iguais: nenhuma variável varia, e não existe direção
        // principal. Como 0,1 não é guardado de forma exata, sem esse cuidado o
        // arredondamento criaria uma "variância" minúscula e falsa.
        let data = vec![vec![0.1, 5.0], vec![0.1, 5.0], vec![0.1, 5.0]];
        assert_eq!(pca(&data, 1), None);
    }

    #[test]
    fn test_variation_too_small_for_f64() {
        // As linhas diferem, mas só por 1e-320 (perto do menor f64). A
        // variância verdadeira, cerca de 5e-641, é pequena demais para o f64 e
        // vira 0: não há variação para analisar, e a resposta é None (sem esse
        // cuidado, as proporções seriam 0 ÷ 0 = NaN).
        let data = vec![vec![1.0, 0.0], vec![1.0, 1e-320]];
        assert_eq!(pca(&data, 1), None);
    }

    #[test]
    fn test_shifting_the_data_changes_only_the_means() {
        // Somar 1000 a todos os valores muda as médias (2,5 + 1000), mas não a
        // variação: autovalores, componentes e escores continuam os do exemplo.
        let s = 2f64.sqrt();
        let shifted: Vec<Vec<f64>> = example()
            .iter()
            .map(|row| row.iter().map(|&x| x + 1000.0).collect())
            .collect();
        let result = pca(&shifted, 2).unwrap();
        assert_vector_close(&result.means, &[1002.5, 1002.5]);
        assert_vector_close(&result.eigenvalues, &[8.0 / 3.0, 2.0 / 3.0]);
        assert_vector_close(&result.components[0], &[1.0 / s, 1.0 / s]);
        assert_vector_close(&column(&result.scores, 0), &[-s, -s, s, s]);
    }

    #[test]
    fn test_huge_values() {
        // O exemplo vezes 1e200. Os autovalores verdadeiros, (8/3)·1e400 e
        // (2/3)·1e400, não cabem no f64 e viram infinito. Mas as médias, as
        // direções, as proporções e os escores cabem e saem certos.
        let s = 2f64.sqrt();
        let result = pca(&example_times(1e200), 2).unwrap();
        assert_eq!(result.eigenvalues, vec![f64::INFINITY, f64::INFINITY]);
        assert_vector_close(&result.means, &[2.5e200, 2.5e200]);
        assert_vector_close(&result.explained_variance_ratio, &[0.8, 0.2]);
        assert_vector_close(&result.components[0], &[1.0 / s, 1.0 / s]);
        assert_vector_close(&result.scores[0], &[-s * 1e200, -1e200 / s]);
    }

    #[test]
    fn test_large_values_that_fit() {
        // O exemplo vezes 1e150: os quadrados (1e300) cabem, e os autovalores,
        // (8/3)·1e300 e (2/3)·1e300, também.
        let result = pca(&example_times(1e150), 2).unwrap();
        assert_vector_close(&result.eigenvalues, &[8.0 / 3.0 * 1e300, 2.0 / 3.0 * 1e300]);
    }

    #[test]
    fn test_tiny_values() {
        // O exemplo vezes 1e-200. Os autovalores verdadeiros, (8/3)·1e-400 e
        // (2/3)·1e-400, são pequenos demais para o f64 e viram 0. Mas as
        // direções, as proporções e os escores saem certos.
        let s = 2f64.sqrt();
        let result = pca(&example_times(1e-200), 2).unwrap();
        assert_eq!(result.eigenvalues, vec![0.0, 0.0]);
        assert_vector_close(&result.explained_variance_ratio, &[0.8, 0.2]);
        assert_vector_close(&result.components[1], &[1.0 / s, -1.0 / s]);
        assert_vector_close(&result.scores[3], &[s * 1e-200, 1e-200 / s]);
    }

    #[test]
    fn test_display() {
        let text = pca(&example(), 2).unwrap().to_string();
        let expected = "\
Análise de componentes principais (n = 4 observações, p = 2 variáveis)

Componente     Autovalor   Variância explicada   Acumulada
CP1               2.6667                80.00%      80.00%
CP2               0.6667                20.00%     100.00%

Pesos das variáveis em cada componente (autovetores):
                     CP1         CP2
Variável 1        0.7071      0.7071
Variável 2        0.7071     -0.7071

Escores (as observações nos novos eixos):
                     CP1         CP2
Obs. 1           -1.4142     -0.7071
Obs. 2           -1.4142      0.7071
Obs. 3            1.4142     -0.7071
Obs. 4            1.4142      0.7071";
        assert_eq!(text, expected);
    }

    #[test]
    fn print_pca_demo() {
        // Demonstração: para ver o resultado na tela, rode
        //   cargo test print_pca_demo -- --nocapture
        let resultado = pca(&example(), 2).unwrap();
        println!("\nDados: {:?}\n\n{resultado}\n", example());
    }
}
