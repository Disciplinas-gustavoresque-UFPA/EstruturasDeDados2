//! Etapa 5 do PCA: **projeção dos dados centralizados** sobre os componentes
//! principais.

use super::linear_algebra::dot;

/// Escreve cada observação nos novos eixos: os **escores**.
///
/// # O que é
///
/// Cada componente principal é uma direção (um vetor de norma 1). O escore de
/// uma observação num componente é o produto escalar da linha centralizada
/// com esse componente: a "sombra" da observação sobre aquele eixo. Com k
/// componentes, cada observação passa a ser descrita por k números.
///
/// # Fórmula
///
/// ```text
/// escore[i][c] = (linha centralizada i) · (componente c)
/// ```
///
/// # Exemplo, passo a passo
///
/// Com os componentes (1, 1)/√2 e (1, −1)/√2, a linha centralizada
/// [−1,5; −0,5] vira:
///
/// ```text
/// no 1º componente: (−1,5 · 1 + (−0,5) · 1) ÷ √2 = −2 ÷ √2 = −√2  = −1,4142
/// no 2º componente: (−1,5 · 1 + (−0,5) · (−1)) ÷ √2 = −1 ÷ √2    = −0,7071
/// ```
///
/// Quem chama garante que cada linha e cada componente têm o mesmo tamanho; a
/// resposta é `None` se isso falhar.
///
/// # Complexidade
///
/// - Tempo: O(n·p·k), para n linhas de p valores e k componentes: um produto
///   escalar de p valores para cada par (linha, componente).
/// - Espaço: O(n·k), a tabela de escores.
pub(super) fn project(centered: &[Vec<f64>], components: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    // Para cada linha, um escore por componente. Se algum produto escalar der
    // `None`, os `collect` devolvem `None` para a tabela toda.
    centered
        .iter()
        .map(|row| components.iter().map(|c| dot(row, c)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::project;
    use crate::pca::helpers::assert_vector_close;

    #[test]
    fn test_project_onto_the_components() {
        // Componentes: (1, 1)/√2 e (1, −1)/√2. O escore de uma linha (a, b)
        // é (a + b)/√2 no primeiro e (a − b)/√2 no segundo:
        //   [−1,5; −0,5] → (−2)/√2 = −√2   e   (−1)/√2 = −1/√2
        //   [−0,5; −1,5] → (−2)/√2 = −√2   e   ( 1)/√2 =  1/√2
        //   [ 0,5;  1,5] → ( 2)/√2 =  √2   e   (−1)/√2 = −1/√2
        //   [ 1,5;  0,5] → ( 2)/√2 =  √2   e   ( 1)/√2 =  1/√2
        let s = 2f64.sqrt();
        let centered = vec![
            vec![-1.5, -0.5],
            vec![-0.5, -1.5],
            vec![0.5, 1.5],
            vec![1.5, 0.5],
        ];
        let components = vec![vec![1.0 / s, 1.0 / s], vec![1.0 / s, -1.0 / s]];
        let scores = project(&centered, &components).unwrap();
        assert_eq!(scores.len(), 4);
        assert_vector_close(&scores[0], &[-s, -1.0 / s]);
        assert_vector_close(&scores[1], &[-s, 1.0 / s]);
        assert_vector_close(&scores[2], &[s, -1.0 / s]);
        assert_vector_close(&scores[3], &[s, 1.0 / s]);
    }

    #[test]
    fn test_project_onto_one_axis() {
        // Projetar sobre o eixo (0, 1) é pegar a segunda coluna.
        let centered = vec![vec![1.0, -2.0], vec![-1.0, 2.0]];
        assert_eq!(
            project(&centered, &[vec![0.0, 1.0]]),
            Some(vec![vec![-2.0], vec![2.0]])
        );
    }

    #[test]
    fn test_sizes_must_match() {
        // Linhas com 2 valores e componente com 3: não dá para projetar.
        assert_eq!(project(&[vec![1.0, 2.0]], &[vec![1.0, 0.0, 0.0]]), None);
    }
}
