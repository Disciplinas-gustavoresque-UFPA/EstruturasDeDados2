// Implementação de Tabela Hash com Encadeamento Externo (Separate Chaining)
// Listas encadeadas manuais via Box, método da divisão com tamanho primo e rehashing dinâmico
// Autor: Juan Carlo Teles Martins

use std::fmt::{self, Display};

/// Trait própria para gerar hash a partir de fatias de bytes (&[u8]) sem depender do DefaultHasher.
pub trait HashBytes {
    fn compute_hash(&self) -> u64;
}

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

impl HashBytes for &str {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(self.as_bytes())
    }
}

impl HashBytes for String {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(self.as_bytes())
    }
}

impl HashBytes for i32 {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
}

impl HashBytes for u32 {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
}

impl HashBytes for i64 {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
}

impl HashBytes for usize {
    fn compute_hash(&self) -> u64 {
        fnv1a_bytes(&self.to_le_bytes())
    }
}

impl HashBytes for char {
    fn compute_hash(&self) -> u64 {
        let mut buf = [0u8; 4];
        let encoded = self.encode_utf8(&mut buf);
        fnv1a_bytes(encoded.as_bytes())
    }
}

/// Nó individual da lista encadeada alocada manualmente no heap.
struct Node<K, V> {
    key: K,
    value: V,
    next: Option<Box<Node<K, V>>>,
}

/// Tabela Hash com tratamento de colisões por Encadeamento Externo.
pub struct ChainedHashTable<K, V> {
    buckets: Vec<Option<Box<Node<K, V>>>>,
    len: usize,
}

impl<K: HashBytes + Eq, V> Default for ChainedHashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: HashBytes + Eq, V> ChainedHashTable<K, V> {
    const MAX_LOAD_FACTOR: f64 = 0.75;

    /// Cria uma tabela com capacidade inicial prima (7), conforme recomendação do método da divisão.
    pub fn new() -> Self {
        Self::with_prime_capacity(7)
    }

    /// Cria uma tabela garantindo que o tamanho `m` seja sempre um número primo.
    pub fn with_prime_capacity(min_capacity: usize) -> Self {
        let m = next_prime(min_capacity.max(3));
        let mut buckets = Vec::with_capacity(m);
        for _ in 0..m {
            buckets.push(None);
        }
        Self { buckets, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    /// Retorna o fator de carga atual: alpha = n / m.
    pub fn load_factor(&self) -> f64 {
        self.len as f64 / self.buckets.len() as f64
    }

    /// Função de dispersão pelo método da divisão: h(k) = hash(k) mod m.
    fn bucket_index(&self, key: &K) -> usize {
        (key.compute_hash() as usize) % self.buckets.len()
    }

    /// 1. Inserção: insere novo par (chave, valor) no início da lista ou atualiza se a chave já existir.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let idx = self.bucket_index(&key);

        let mut current = self.buckets[idx].as_mut();
        while let Some(node) = current {
            if node.key == key {
                return Some(std::mem::replace(&mut node.value, value));
            }
            current = node.next.as_mut();
        }

        if (self.len + 1) as f64 / self.buckets.len() as f64 > Self::MAX_LOAD_FACTOR {
            self.rehash();
        }

        let new_idx = self.bucket_index(&key);
        let head = self.buckets[new_idx].take();
        self.buckets[new_idx] = Some(Box::new(Node {
            key,
            value,
            next: head,
        }));
        self.len += 1;
        None
    }

    /// 2. Busca: percorre a lista encadeada do endereço h(k) e retorna referência ao valor.
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.bucket_index(key);
        let mut current = self.buckets[idx].as_deref();

        while let Some(node) = current {
            if &node.key == key {
                return Some(&node.value);
            }
            current = node.next.as_deref();
        }
        None
    }

    /// 3. Remoção: desconecta o nó da lista encadeada e retorna o valor removido.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.bucket_index(key);
        let mut link = &mut self.buckets[idx];

        while link.is_some() {
            if link.as_ref().unwrap().key == *key {
                let mut removed_node = link.take().unwrap();
                *link = removed_node.next.take();
                self.len -= 1;
                return Some(removed_node.value);
            }
            link = &mut link.as_mut().unwrap().next;
        }
        None
    }

    /// Redimensiona a tabela para o próximo primo > 2m e redistribui todos os nós existentes.
    fn rehash(&mut self) {
        let new_capacity = next_prime(self.buckets.len() * 2 + 1);
        let mut new_buckets = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_buckets.push(None);
        }

        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

        for mut head in old_buckets {
            while let Some(mut node) = head {
                head = node.next.take();
                let new_idx = (node.key.compute_hash() as usize) % self.buckets.len();
                node.next = self.buckets[new_idx].take();
                self.buckets[new_idx] = Some(node);
            }
        }
    }
}

impl<K: Display, V: Display> ChainedHashTable<K, V> {
    /// 4. Visualizar toda a tabela: exibe cada endereço [0..m-1] e sua lista encadeada até nil.
    pub fn visualizar(&self) -> String {
        let mut output = String::new();
        for (i, bucket) in self.buckets.iter().enumerate() {
            output.push_str(&format!("[{}] -> ", i));
            let mut current = bucket.as_deref();
            while let Some(node) = current {
                output.push_str(&format!("({}: {}) -> ", node.key, node.value));
                current = node.next.as_deref();
            }
            output.push_str("nil\n");
        }
        output
    }
}

impl<K: Display, V: Display> Display for ChainedHashTable<K, V> {
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
        let mut tabela = ChainedHashTable::new();
        assert!(tabela.is_empty());
        assert_eq!(tabela.capacity(), 7);

        assert_eq!(tabela.insert("algoritmos", 10), None);
        assert_eq!(tabela.insert("grafos", 20), None);
        assert_eq!(tabela.get(&"algoritmos"), Some(&10));
        assert_eq!(tabela.get(&"inexistente"), None);

        // Atualizar chave repetida deve retornar o valor antigo sem duplicar nó
        assert_eq!(tabela.insert("algoritmos", 99), Some(10));
        assert_eq!(tabela.get(&"algoritmos"), Some(&99));
        assert_eq!(tabela.len(), 2);
    }

    #[test]
    fn testa_colisoes_e_remocao_na_cadeia() {
        // Capacidade prima 7 com 5 chaves garantindo colisões no mesmo bucket
        let mut tabela = ChainedHashTable::with_prime_capacity(7);
        let chaves = [5, 12, 19, 26, 33];
        for &c in &chaves {
            tabela.insert(c, c * 10);
        }

        for &c in &chaves {
            assert_eq!(tabela.get(&c), Some(&(c * 10)));
        }

        // Remove elemento intermediário da cadeia e verifica integridade dos ponteiros
        assert_eq!(tabela.remove(&19), Some(190));
        assert_eq!(tabela.get(&19), None);
        assert_eq!(tabela.get(&5), Some(&50));
        assert_eq!(tabela.get(&33), Some(&330));
        assert_eq!(tabela.len(), 4);
    }

    #[test]
    fn testa_rehashing_automatico_para_proximo_primo() {
        let mut tabela = ChainedHashTable::with_prime_capacity(5);
        assert_eq!(tabela.capacity(), 5);

        for i in 0..10 {
            tabela.insert(i, format!("valor_{}", i));
        }

        // Capacidade deve ter crescido automaticamente para números primos maiores
        assert!(tabela.capacity() > 5);
        assert!(is_prime(tabela.capacity()));
        assert_eq!(tabela.len(), 10);

        for i in 0..10 {
            assert_eq!(tabela.get(&i), Some(&format!("valor_{}", i)));
        }
    }

    #[test]
    fn testa_visualizacao_da_tabela() {
        let mut tabela = ChainedHashTable::with_prime_capacity(3);
        tabela.insert("UFPA", 1);
        tabela.insert("ICEN", 2);

        let vis = tabela.visualizar();
        println!("\n{}", vis);

        assert!(vis.contains("[0] -> "));
        assert!(vis.contains("[1] -> "));
        assert!(vis.contains("[2] -> "));
        assert!(vis.contains("nil"));
    }
}
