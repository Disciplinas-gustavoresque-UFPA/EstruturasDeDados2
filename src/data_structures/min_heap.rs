/// Juan Carlo Teles Martins

pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        if self.data.len() == 1 {
            return self.data.pop();
        }
        let min = self.data.swap_remove(0);
        self.heapify_down(0);
        Some(min)
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            // No Min-Heap, o filho sobe se for menor que o pai
            if self.data[index] < self.data[parent] {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest != index {
                self.data.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_insercao_e_remocao_min_heap() {
        let mut heap = MinHeap::new();
        
        heap.insert(30);
        heap.insert(10);
        heap.insert(40);
        heap.insert(5);
        heap.insert(20);

        // O menor elemento deve ser o 5
        assert_eq!(heap.peek(), Some(&5));

        // Extraindo e garantindo que saem na ordem crescente (do menor para o maior)
        assert_eq!(heap.extract_min(), Some(5));
        assert_eq!(heap.extract_min(), Some(10));
        assert_eq!(heap.extract_min(), Some(20));
        assert_eq!(heap.extract_min(), Some(30));
        assert_eq!(heap.extract_min(), Some(40));
        
        // O heap deve estar vazio agora
        assert_eq!(heap.extract_min(), None);
    }

    #[test]
    fn testa_min_heap_visual() {
        let mut heap = MinHeap::new();
        let valores = [30, 10, 40, 5, 20];
        
        println!("\n== TESTE VISUAL DO MIN-HEAP ==");
        for &v in &valores {
            println!("-> Inserindo o valor: {}", v);
            heap.insert(v);
            // Como o teste está no mesmo arquivo, ele tem acesso ao campo privado `data`
            println!("   Estado do vetor (Árvore): {:?}", heap.data);
        }
        
        println!("\n-> Iniciando extração (extract_min)...");
        let extraido = heap.extract_min();
        println!("   Valor removido da raiz: {:?}", extraido);
        println!("   Estado do vetor após reordenar (heapify_down): {:?}", heap.data);
        println!("___________________________________\n");
    }
}