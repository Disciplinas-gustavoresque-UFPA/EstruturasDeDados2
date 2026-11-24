// Implementação de Tabela Hash com Endereçamento Aberto (Hash Duplo / Double Hashing)
// Capacidades sempre primas, passo d(k) coprimo de m, estados Vazio/Ocupado/Liberado e rehashing
// Autor: Juan Carlo Teles Martins

use std::fmt::{self, Display};

/// Trait própria para gerar dois hashes independentes a partir de fatias de bytes (&[u8]).
pub trait DoubleHashBytes {
    fn primary_hash(&self) -> u64;
    fn secondary_hash(&self) -> u64;
}

/// Primeira função de dispersão: FNV-1a sobre bytes.
fn fnv1a_bytes(bytes: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Segunda função de dispersão: DJB2 modificado sobre bytes.
fn djb2_bytes(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for &byte in bytes {
        hash = hash.wrapping_shl(5).wrapping_add(hash) ^ u64::from(byte);
    }
    hash
}

impl DoubleHashBytes for &str {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(self.as_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(self.as_bytes())
    }
}

impl DoubleHashBytes for String {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(self.as_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(self.as_bytes())
    }
}

impl DoubleHashBytes for i32 {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(&self.to_le_bytes())
    }
}

impl DoubleHashBytes for u32 {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(&self.to_le_bytes())
    }
}

impl DoubleHashBytes for i64 {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(&self.to_le_bytes())
    }
}

impl DoubleHashBytes for usize {
    fn primary_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        djb2_bytes(&self.to_le_bytes())
    }
}

impl DoubleHashBytes for char {
    fn primary_hash(&self) -> u64 {
        let mut buf = [0u8; 4];
        let encoded = self.encode_utf8(&mut buf);
        fnv1a_bytes(encoded.as_bytes())
    }
    fn secondary_hash(&self) -> u64 {
        let mut buf = [0u8; 4];
        let encoded = self.encode_utf8(&mut buf);
        djb2_bytes(encoded.as_bytes())
    }
}

// Cada compartimento da tabela assume um dos três estados exigidos pelo endereçamento aberto.
enum Slot<K, V> {
    Vazio,
    Liberado,
    Ocupado(K, V),
}

// Tabela Hash com Endereçamento Aberto e resolução de colisões via Hash Duplo.
pub struct DoubleHashTable<K, V> {
    slots: Vec<Slot<K, V>>,
    len: usize,
    tombstones: usize,
}

impl<K: DoubleHashBytes + Eq, V> Default for DoubleHashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: DoubleHashBytes + Eq, V> DoubleHashTable<K, V> {
    const MAX_LOAD_FACTOR: f64 = 0.70;

    /// Cria uma tabela com capacidade inicial prima (11).
    pub fn new() -> Self {
        Self::with_prime_capacity(11)
    }

    // Cria uma tabela garantindo que o tamanho `m` seja sempre um número primo >= 5.
    pub fn with_prime_capacity(min_capacity: usize) -> Self {
        let m = next_prime(min_capacity.max(5));
        let mut slots = Vec::with_capacity(m);
        for _ in 0..m {
            slots.push(Slot::Vazio);
        }
        Self {
            slots,
            len: 0,
            tombstones: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    // Retorna o fator de carga atual: alpha = n / m.
    pub fn load_factor(&self) -> f64 {
        self.len as f64 / self.slots.len() as f64
    }

    // Endereço base: h(k) = h1(k) mod m.
    fn primary_index(&self, key: &K) -> usize {
        (key.primary_hash() as usize) % self.slots.len()
    }

    // Passo de sondagem: d(k) = 1 + (h2(k) mod (m - 1)), garantindo 1 <= d(k) < m.
    fn step_size(&self, key: &K) -> usize {
        let m = self.slots.len();
        1 + ((key.secondary_hash() as usize) % (m - 1))
    }

    // Sequência de sondagem: h_j(k) = (h(k) + j * d(k)) mod m.
    fn probe_index(&self, h1: usize, step: usize, j: usize) -> usize {
        let m = self.slots.len();
        (h1 + j.wrapping_mul(step)) % m
    }

    // 1. Inserção: insere novo par (chave, valor) ou atualiza o valor se a chave já existir.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let active_slots = self.len + self.tombstones + 1;
        if (active_slots as f64 / self.slots.len() as f64) > Self::MAX_LOAD_FACTOR {
            self.rehash();
        }

        let m = self.slots.len();
        let h1 = self.primary_index(&key);
        let step = self.step_size(&key);
        let mut first_liberado: Option<usize> = None;

        for j in 0..m {
            let idx = self.probe_index(h1, step, j);
            match &mut self.slots[idx] {
                Slot::Vazio => {
                    let target = first_liberado.unwrap_or(idx);
                    if first_liberado.is_some() {
                        self.tombstones -= 1;
                    }
                    self.slots[target] = Slot::Ocupado(key, value);
                    self.len += 1;
                    return None;
                }
                Slot::Liberado => {
                    if first_liberado.is_none() {
                        first_liberado = Some(idx);
                    }
                }
                Slot::Ocupado(existing_key, existing_val) if existing_key == &key => {
                    return Some(std::mem::replace(existing_val, value));
                }
                Slot::Ocupado(..) => {}
            }
        }

        if let Some(target) = first_liberado {
            self.tombstones -= 1;
            self.slots[target] = Slot::Ocupado(key, value);
            self.len += 1;
            return None;
        }

        unreachable!("O rehashing impede que a tabela fique completamente cheia");
    }

    // 2. Busca: percorre h_j(k) ignorando células Liberado e parando em Vazio ou na chave alvo.
    pub fn get(&self, key: &K) -> Option<&V> {
        let m = self.slots.len();
        let h1 = self.primary_index(key);
        let step = self.step_size(key);

        for j in 0..m {
            let idx = self.probe_index(h1, step, j);
            match &self.slots[idx] {
                Slot::Vazio => return None,
                Slot::Ocupado(k, v) if k == key => return Some(v),
                Slot::Liberado | Slot::Ocupado(..) => {}
            }
        }
        None
    }

    // 3. Remoção: marca o compartimento como Liberado para preservar a cadeia de busca.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let m = self.slots.len();
        let h1 = self.primary_index(key);
        let step = self.step_size(key);

        for j in 0..m {
            let idx = self.probe_index(h1, step, j);
            match &self.slots[idx] {
                Slot::Vazio => return None,
                Slot::Ocupado(k, _) if k == key => {
                    let old_slot = std::mem::replace(&mut self.slots[idx], Slot::Liberado);
                    if let Slot::Ocupado(_, val) = old_slot {
                        self.len -= 1;
                        self.tombstones += 1;
                        return Some(val);
                    }
                    unreachable!();
                }
                Slot::Liberado | Slot::Ocupado(..) => {}
            }
        }
        None
    }

    // Redimensiona a tabela para o próximo primo > 2m, descartando células Liberado.
    fn rehash(&mut self) {
        let new_capacity = next_prime(self.slots.len() * 2 + 1);
        let mut new_slots = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_slots.push(Slot::Vazio);
        }

        let old_slots = std::mem::replace(&mut self.slots, new_slots);
        self.len = 0;
        self.tombstones = 0;

        for slot in old_slots {
            if let Slot::Ocupado(k, v) = slot {
                self.insert(k, v);
            }
        }
    }
}

impl<K: Display, V: Display> DoubleHashTable<K, V> {
    // 4. Visualizar toda a tabela: exibe cada índice [0..m-1] e seu estado atual.
    pub fn visualizar(&self) -> String {
        let mut output = String::new();
        for (i, slot) in self.slots.iter().enumerate() {
            match slot {
                Slot::Vazio => output.push_str(&format!("[{}] -> Vazio\n", i)),
                Slot::Liberado => output.push_str(&format!("[{}] -> Liberado\n", i)),
                Slot::Ocupado(k, v) => output.push_str(&format!("[{}] -> ({}: {})\n", i, k, v)),
            }
        }
        output
    }
}

impl<K: Display, V: Display> Display for DoubleHashTable<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.visualizar())
    }
}

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn next_prime(mut candidate: usize) -> usize {
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_insercao_busca_e_atualizacao() {
        let mut tabela = DoubleHashTable::new();
        assert!(tabela.is_empty());
        assert_eq!(tabela.capacity(), 11);

        assert_eq!(tabela.insert("belem", 1), None);
        assert_eq!(tabela.insert("ufpa", 2), None);
        assert_eq!(tabela.get(&"belem"), Some(&1));
        assert_eq!(tabela.get(&"inexistente"), None);

        // Atualizar chave existente substitui o valor sem duplicar entrada
        assert_eq!(tabela.insert("belem", 100), Some(1));
        assert_eq!(tabela.get(&"belem"), Some(&100));
        assert_eq!(tabela.len(), 2);
    }

    #[test]
    fn testa_colisoes_e_remocao_com_slot_liberado() {
        let mut tabela = DoubleHashTable::with_prime_capacity(13);
        let base = 1;
        let colisoes: Vec<i32> = (2..500)
            .filter(|k| tabela.primary_index(k) == tabela.primary_index(&base))
            .take(2)
            .collect();

        let k2 = colisoes[0];
        let k3 = colisoes[1];

        tabela.insert(base, "primeiro");
        tabela.insert(k2, "segundo");
        tabela.insert(k3, "terceiro");

        // Remove o primeiro da sequência de sondagem; k2 e k3 devem continuar acessíveis
        assert_eq!(tabela.remove(&base), Some("primeiro"));
        assert_eq!(tabela.get(&base), None);
        assert_eq!(tabela.get(&k2), Some(&"segundo"));
        assert_eq!(tabela.get(&k3), Some(&"terceiro"));
        assert_eq!(tabela.len(), 2);

        // Nova inserção deve reaproveitar o slot Liberado
        assert_eq!(tabela.insert(k3, "terceiro_atualizado"), Some("terceiro"));
        assert_eq!(tabela.get(&k3), Some(&"terceiro_atualizado"));
    }

    #[test]
    fn testa_rehashing_automatico_para_proximo_primo() {
        let mut tabela = DoubleHashTable::with_prime_capacity(7);
        assert_eq!(tabela.capacity(), 7);

        for i in 0..12 {
            tabela.insert(i, i * 5);
        }

        assert!(tabela.capacity() > 7);
        assert!(is_prime(tabela.capacity()));
        assert_eq!(tabela.len(), 12);

        for i in 0..12 {
            assert_eq!(tabela.get(&i), Some(&(i * 5)));
        }
    }

    #[test]
    fn testa_visualizacao_da_tabela() {
        let mut tabela = DoubleHashTable::with_prime_capacity(5);
        tabela.insert("A", 10);
        tabela.insert("B", 20);
        tabela.remove(&"A");

        let vis = tabela.visualizar();
        println!("\n{}", vis);

        assert!(vis.contains("Liberado"));
        assert!(vis.contains("(B: 20)"));
        assert!(vis.contains("Vazio"));
    }
}
