/// Gera uma entrada pseudoaleatória determinística com `tamanho` elementos.
///
/// O gerador usa um estado inicial fixo para que a mesma entrada seja
/// reproduzida em diferentes execuções.
pub fn entrada_aleatoria(tamanho: usize) -> Vec<i32> {
    let mut estado = 0x1234_5678_u64;
    let mut dados = Vec::with_capacity(tamanho);

    for _ in 0..tamanho {
        estado = estado
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);

        dados.push((estado >> 32) as i32);
    }

    dados
}

/// Gera uma entrada já ordenada em ordem crescente.
pub fn entrada_ordenada(tamanho: usize) -> Vec<i32> {
    (0..tamanho as i32).collect()
}

/// Gera uma entrada ordenada em ordem decrescente.
pub fn entrada_invertida(tamanho: usize) -> Vec<i32> {
    (0..tamanho as i32).rev().collect()
}

/// Gera uma entrada quase ordenada.
///
/// A entrada começa ordenada e alguns pares de elementos são trocados.
pub fn entrada_quase_ordenada(tamanho: usize) -> Vec<i32> {
    let mut dados = entrada_ordenada(tamanho);

    if tamanho >= 2 {
        for indice in (0..tamanho - 1).step_by(10) {
            dados.swap(indice, indice + 1);
        }
    }

    dados
}

/// Gera uma entrada contendo muitos valores repetidos.
///
/// Os valores ficam limitados a um pequeno intervalo, aumentando
/// significativamente a quantidade de duplicatas.
pub fn entrada_com_duplicatas(tamanho: usize) -> Vec<i32> {
    (0..tamanho).map(|indice| (indice % 10) as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gera_tamanho_solicitado() {
        assert_eq!(entrada_aleatoria(100).len(), 100);
        assert_eq!(entrada_ordenada(100).len(), 100);
        assert_eq!(entrada_invertida(100).len(), 100);
        assert_eq!(entrada_quase_ordenada(100).len(), 100);
        assert_eq!(entrada_com_duplicatas(100).len(), 100);
    }

    #[test]
    fn entrada_aleatoria_e_deterministica() {
        assert_eq!(entrada_aleatoria(100), entrada_aleatoria(100));
    }

    #[test]
    fn gera_entrada_ordenada() {
        assert_eq!(entrada_ordenada(5), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn gera_entrada_invertida() {
        assert_eq!(entrada_invertida(5), vec![4, 3, 2, 1, 0]);
    }

    #[test]
    fn gera_entrada_quase_ordenada() {
        assert_eq!(entrada_quase_ordenada(5), vec![1, 0, 2, 3, 4]);
    }

    #[test]
    fn gera_entrada_com_duplicatas() {
        assert_eq!(
            entrada_com_duplicatas(15),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn lida_com_entrada_vazia() {
        assert!(entrada_aleatoria(0).is_empty());
        assert!(entrada_ordenada(0).is_empty());
        assert!(entrada_invertida(0).is_empty());
        assert!(entrada_quase_ordenada(0).is_empty());
        assert!(entrada_com_duplicatas(0).is_empty());
    }
}
