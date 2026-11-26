//! Deque genérico com armazenamento circular e crescimento por duplicação.

/// Sequência que admite inserções e remoções nas duas extremidades.
///
/// As remoções e consultas custam O(1). Inserções custam O(1) amortizado e
/// O(n) quando o armazenamento precisa crescer. Os elementos são movidos por
/// valor, inclusive durante o crescimento.
pub struct CircularDeque<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> CircularDeque<T> {
    /// Cria um deque vazio, com capacidade zero, em O(1).
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            head: 0,
            len: 0,
        }
    }

    /// Cria um deque vazio com espaço para `capacity` elementos.
    /// Aceita zero. Inicializar as posições livres custa O(capacity).
    ///
    /// # Panics
    ///
    /// Entra em pânico se o tamanho solicitado exceder os limites de `Vec`
    /// ou se o alocador informar falha na reserva.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(capacity)
            .expect("não foi possível reservar o armazenamento do deque");
        slots.resize_with(capacity, || None);
        Self {
            slots,
            head: 0,
            len: 0,
        }
    }

    /// Insere um valor no início.
    ///
    /// # Panics
    ///
    /// Entra em pânico se a nova capacidade for irrepresentável ou se a
    /// reserva de memória falhar.
    pub fn push_front(&mut self, value: T) {
        self.ensure_space();
        self.head = if self.head == 0 {
            self.capacity() - 1
        } else {
            self.head - 1
        };
        self.slots[self.head] = Some(value);
        self.len += 1;
    }

    /// Insere um valor no final.
    ///
    /// # Panics
    ///
    /// Entra em pânico se a nova capacidade for irrepresentável ou se a
    /// reserva de memória falhar.
    pub fn push_back(&mut self, value: T) {
        self.ensure_space();
        let index = self.physical_index(self.len);
        self.slots[index] = Some(value);
        self.len += 1;
    }

    /// Remove e devolve o primeiro valor, ou `None` se o deque estiver vazio.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let index = self.head;
        self.head = self.physical_index(1);
        self.len -= 1;
        self.slots[index].take()
    }

    /// Remove e devolve o último valor, ou `None` se o deque estiver vazio.
    pub fn pop_back(&mut self) -> Option<T> {
        let offset = self.len.checked_sub(1)?;
        let index = self.physical_index(offset);
        self.len = offset;
        self.slots[index].take()
    }

    /// Empresta o primeiro valor, ou devolve `None` se o deque estiver vazio.
    pub fn front(&self) -> Option<&T> {
        self.slots.get(self.head)?.as_ref()
    }

    /// Empresta o último valor, ou devolve `None` se o deque estiver vazio.
    pub fn back(&self) -> Option<&T> {
        let offset = self.len.checked_sub(1)?;
        self.slots[self.physical_index(offset)].as_ref()
    }

    /// Devolve o número de elementos armazenados.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica se o deque está vazio.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Devolve o número de posições utilizáveis antes de um novo crescimento.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Descarta os elementos em ordem, do início ao final, e mantém a
    /// capacidade para reutilização. Custa O(n), além do custo de destruir os valores.
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    // Equivale a (head + offset) % capacidade, sem somar índices que
    // possam transbordar. Requer capacidade positiva e offset <= capacidade.
    fn physical_index(&self, offset: usize) -> usize {
        let until_end = self.capacity() - self.head;
        if offset < until_end {
            self.head + offset
        } else {
            offset - until_end
        }
    }

    fn ensure_space(&mut self) {
        if self.len < self.capacity() {
            return;
        }
        let capacity = self
            .capacity()
            .checked_mul(2)
            .expect("overflow ao duplicar a capacidade do deque")
            .max(1);
        let mut grown = Self::with_capacity(capacity);
        for (offset, slot) in grown.slots[..self.len].iter_mut().enumerate() {
            let index = self.physical_index(offset);
            *slot = self.slots[index].take();
        }
        grown.len = self.len;
        *self = grown;
    }
}

impl<T> Default for CircularDeque<T> {
    /// Equivale a [`CircularDeque::new`].
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CircularDeque;
    use std::cell::Cell;

    // Percorre as posições físicas independentemente de physical_index.
    fn assert_state(deque: &CircularDeque<i32>, expected: &[i32]) {
        assert_eq!(deque.len(), expected.len());
        assert_eq!(deque.is_empty(), expected.is_empty());
        assert_eq!(deque.front(), expected.first());
        assert_eq!(deque.back(), expected.last());
        let capacity = deque.capacity();
        assert!(deque.len() <= capacity);
        if capacity == 0 {
            assert_eq!(deque.head, 0);
            return;
        }
        assert!(deque.head < capacity);
        let mut index = deque.head;
        for offset in 0..capacity {
            assert_eq!(deque.slots[index].as_ref(), expected.get(offset));
            index = (index + 1) % capacity;
        }
    }

    #[test]
    fn empty_zero_and_one_slot_can_be_reused() {
        for (initial_capacity, mut deque) in [
            (0, CircularDeque::new()),
            (0, CircularDeque::with_capacity(0)),
            (1, CircularDeque::with_capacity(1)),
        ] {
            assert_eq!(deque.capacity(), initial_capacity);
            assert_state(&deque, &[]);
            assert_eq!(deque.pop_back(), None);
            assert_eq!(deque.pop_front(), None);
            for value in 0..8 {
                deque.push_front(value);
                assert_state(&deque, &[value]);
                assert_eq!(deque.pop_back(), Some(value));
                deque.push_back(value);
                assert_eq!(deque.pop_front(), Some(value));
                assert_eq!(deque.capacity(), 1);
                assert_state(&deque, &[]);
            }
        }
    }

    #[test]
    fn wrapped_growth_at_either_end_keeps_order() {
        for grow_at_front in [false, true] {
            let mut deque = CircularDeque::with_capacity(4);
            for value in 1..=4 {
                deque.push_back(value);
            }
            assert_eq!(deque.pop_front(), Some(1));
            assert_eq!(deque.pop_front(), Some(2));
            deque.push_back(5);
            deque.push_back(6);
            assert_eq!(deque.slots, [Some(5), Some(6), Some(3), Some(4)]);
            assert_state(&deque, &[3, 4, 5, 6]);
            if grow_at_front {
                deque.push_front(2);
                assert_state(&deque, &[2, 3, 4, 5, 6]);
            } else {
                deque.push_back(7);
                assert_state(&deque, &[3, 4, 5, 6, 7]);
            }
            assert_eq!(deque.capacity(), 8);
        }
    }

    #[test]
    fn successive_growth_and_front_wrap_match_a_linear_sequence() {
        let mut deque = CircularDeque::with_capacity(3);
        let mut model = Vec::new();
        for value in 0..100 {
            let old_capacity = deque.capacity();
            let was_full = deque.len() == old_capacity;
            deque.push_front(value);
            model.insert(0, value);
            assert_eq!(
                deque.capacity(),
                if was_full {
                    old_capacity * 2
                } else {
                    old_capacity
                }
            );
            assert_state(&deque, &model);
        }
        while !model.is_empty() {
            assert_eq!(deque.pop_back(), model.pop());
            assert_state(&deque, &model);
        }
    }

    #[test]
    fn mixed_operations_match_a_linear_model_after_every_step() {
        for initial_capacity in [0, 1, 3, 8] {
            let mut state = 0xA17E_2026_u32;
            let mut deque = CircularDeque::with_capacity(initial_capacity);
            let mut model = Vec::new();
            for step in 0..2048 {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let value = (state % 7) as i32;
                let old_capacity = deque.capacity();
                if step % 97 == 96 {
                    deque.clear();
                    model.clear();
                    assert_eq!(deque.capacity(), old_capacity);
                } else {
                    match state >> 30 {
                        0 => {
                            deque.push_front(value);
                            model.insert(0, value);
                        }
                        1 => {
                            deque.push_back(value);
                            model.push(value);
                        }
                        2 => {
                            let expected = if model.is_empty() {
                                None
                            } else {
                                Some(model.remove(0))
                            };
                            assert_eq!(deque.pop_front(), expected);
                        }
                        _ => assert_eq!(deque.pop_back(), model.pop()),
                    }
                }
                assert_state(&deque, &model);
            }
        }
    }

    #[test]
    fn moves_non_clone_values_and_drops_each_exactly_once() {
        struct Token<'a>(&'a Cell<usize>);
        impl Drop for Token<'_> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let counts = [const { Cell::new(0) }; 6];
        let mut deque = CircularDeque::with_capacity(2);
        deque.push_back(Token(&counts[0]));
        deque.push_back(Token(&counts[1]));
        let removed = deque.pop_front().unwrap();
        deque.push_back(Token(&counts[2]));
        deque.push_front(Token(&counts[3]));
        assert!(counts.iter().all(|count| count.get() == 0));
        let capacity = deque.capacity();
        deque.clear();
        deque.clear();
        assert_eq!(deque.capacity(), capacity);
        assert_eq!(counts.each_ref().map(Cell::get), [0, 1, 1, 1, 0, 0]);
        deque.push_back(Token(&counts[4]));
        deque.push_front(Token(&counts[5]));
        drop(deque.pop_back());
        assert_eq!(counts[4].get(), 1);
        drop(deque);
        drop(removed);
        assert!(counts.iter().all(|count| count.get() == 1));
    }

    #[test]
    #[should_panic(expected = "não foi possível reservar")]
    fn impossible_capacity_fails_before_initialization() {
        CircularDeque::<u64>::with_capacity(usize::MAX);
    }
}
