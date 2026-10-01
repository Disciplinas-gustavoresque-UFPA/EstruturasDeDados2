

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Cada posição (slot) da tabela pode estar vazia ou ocupada por um par chave/valor.
enum Slot<K, V> {
    Vazio,
    Ocupado(K, V),
}

/// Tabela hash que resolve colisões procurando a próxima posição livre no próprio vetor.
pub struct TabelaHash<K, V> {
    slots: Vec<Slot<K, V>>,
    tamanho: usize, // quantos pares estão guardados
}

impl<K: Hash + Eq, V> TabelaHash<K, V> {
  
    pub fn new() -> Self {
        Self::com_capacidade(8)
    }

    pub fn com_capacidade(capacidade: usize) -> Self {
        let capacidade = capacidade.max(1);
        let slots = (0..capacidade).map(|_| Slot::Vazio).collect();
        TabelaHash { slots, tamanho: 0 }
    }

   
    pub fn len(&self) -> usize {
        self.tamanho
    }


    pub fn is_empty(&self) -> bool {
        self.tamanho == 0
    }

  
    pub fn capacidade(&self) -> usize {
        self.slots.len()
    }

    /// Calcula a posição inicial da chave: hash(chave) mod capacidade.
    fn indice_inicial(&self, chave: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        chave.hash(&mut hasher);
        (hasher.finish() as usize) % self.slots.len()
    }

   
    pub fn insert(&mut self, chave: K, valor: V) -> Option<V> {
        let cap = self.slots.len();
        let inicio = self.indice_inicial(&chave);

        for i in 0..cap {
            let idx = (inicio + i) % cap; // sondagem linear
            match &mut self.slots[idx] {
                Slot::Vazio => {
                    self.slots[idx] = Slot::Ocupado(chave, valor);
                    self.tamanho += 1;
                    return None;
                }
                Slot::Ocupado(k, v) if *k == chave => {
                    return Some(std::mem::replace(v, valor));
                }
                Slot::Ocupado(..) => {} // colisão: tenta a próxima posição
            }
        }
      
        panic!("tabela cheia");
    }

   
    pub fn get(&self, chave: &K) -> Option<&V> {
        let cap = self.slots.len();
        let inicio = self.indice_inicial(chave);

        for i in 0..cap {
            let idx = (inicio + i) % cap;
            match &self.slots[idx] {
                Slot::Vazio => return None, // achou buraco: a chave não existe
                Slot::Ocupado(k, v) if k == chave => return Some(v),
                Slot::Ocupado(..) => {}
            }
        }
        None
    }

   
    pub fn contains_key(&self, chave: &K) -> bool {
        self.get(chave).is_some()
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn tabela_nova_esta_vazia() {
        let t: TabelaHash<&str, i32> = TabelaHash::new();
        assert!(t.is_empty());
        assert_eq!(t.get(&"a"), None);
    }

    #[test]
    fn insere_e_busca() {
        let mut t = TabelaHash::new();
        t.insert("maçã", 3);
        t.insert("banana", 5);
        assert_eq!(t.get(&"maçã"), Some(&3));
        assert_eq!(t.get(&"banana"), Some(&5));
        assert_eq!(t.get(&"uva"), None);
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn inserir_chave_repetida_substitui_valor() {
        let mut t = TabelaHash::new();
        assert_eq!(t.insert(1, "um"), None);
        assert_eq!(t.insert(1, "UM"), Some("um"));
        assert_eq!(t.get(&1), Some(&"UM"));
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn colisoes_sao_resolvidas() {
        // Capacidade 8 com 8 chaves: com certeza haverá colisões.
        let mut t = TabelaHash::com_capacidade(8);
        for i in 0..8 {
            t.insert(i, i * 10);
        }
        for i in 0..8 {
            assert_eq!(t.get(&i), Some(&(i * 10)));
        }
    }
}