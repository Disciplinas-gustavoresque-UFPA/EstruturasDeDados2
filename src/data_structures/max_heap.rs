/// Juan Carlo Teles Martins - 13/11/2026.
/// Lista de Prioridades implementada usando a estrutura de um Max-Heap.
pub struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    // Funções para cálculo de índices da árvore binária em um vetor (0-indexed)
    fn parent(i: usize) -> usize {
        if i == 0 { 0 } else { (i - 1) / 2 }
    }

    fn left(i: usize) -> usize { 2 * i + 1 }
    fn right(i: usize) -> usize { 2 * i + 2 }

    /// Insere um novo elemento no final e corrige a prioridade "subindo" a árvore
    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    /// Retorna uma referência ao elemento de maior prioridade (raiz)
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// Remove e retorna o elemento de maior prioridade, corrigindo a árvore "descendo"
    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() { return None; }
        if self.data.len() == 1 { return self.data.pop(); }
        
        // O último elemento substitui a raiz, e a raiz antiga é retornada
        let max = self.data.swap_remove(0);
        self.heapify_down(0);
        
        Some(max)
    }

    // Move o elemento para cima até que seja menor ou igual ao seu pai
    fn heapify_up(&mut self, mut i: usize) {
        while i > 0 && self.data[i] > self.data[Self::parent(i)] {
            self.data.swap(i, Self::parent(i));
            i = Self::parent(i);
        }
    }

    // Move o elemento para baixo trocando-o pelo seu maior filho
    fn heapify_down(&mut self, mut i: usize) {
        let len = self.data.len();
        loop {
            let mut largest = i;
            let l = Self::left(i);
            let r = Self::right(i);

            if l < len && self.data[l] > self.data[largest] { largest = l; }
            if r < len && self.data[r] > self.data[largest] { largest = r; }
            
            if largest == i { break; } // Elemento já está na posição correta
            
            self.data.swap(i, largest);
            i = largest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_insercao_e_remocao_max_heap() {
        let mut heap = MaxHeap::new();
        
        // Inserindo elementos desordenados
        heap.insert(10);
        heap.insert(30);
        heap.insert(20);
        heap.insert(5);
        heap.insert(40);

        // O maior elemento deve ser o 40
        assert_eq!(heap.peek(), Some(&40));

        // Extraindo e garantindo que saem na ordem decrescente (do maior para o menor)
        assert_eq!(heap.extract_max(), Some(40));
        assert_eq!(heap.extract_max(), Some(30));
        assert_eq!(heap.extract_max(), Some(20));
        assert_eq!(heap.extract_max(), Some(10));
        assert_eq!(heap.extract_max(), Some(5));
        
        // O heap deve estar vazio agora
        assert_eq!(heap.extract_max(), None);
    }
}