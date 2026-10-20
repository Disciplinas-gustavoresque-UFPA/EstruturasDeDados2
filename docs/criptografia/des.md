# DES: implementação de blocos em Rust 2024

Closes #19

Referência de acompanhamento: #6.

## O que o algoritmo faz e como funciona

O DES transforma blocos de 64 bits usando uma rede de Feistel de 16 rodadas. A permutação inicial divide o bloco em duas metades. Em cada rodada, a metade direita é expandida para 48 bits, combinada com uma subchave por XOR, substituída por oito S-boxes e permutada; o resultado é combinado com a metade esquerda. A troca final e a permutação inversa produzem o bloco cifrado. A descriptografia usa a mesma rede com as subchaves em ordem inversa.

A expansão aplica PC-1, rotações das duas metades de 28 bits e PC-2. Os bits de paridade são descartados por PC-1. Chaves fracas e semifracas são aceitas e testadas: esta API representa a primitiva, sem uma política de seleção de chaves.

## Interface e exemplo

- `Des::new(key: &[u8]) -> Result<Des, DesKeyError>` aceita 8 bytes (56 bits efetivos).
- `encrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- `decrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- Comprimentos inválidos retornam erro tipado com o comprimento recebido, sem truncar nem completar a chave.
- A instância guarda a chave expandida; chamadas não alteram seu estado.

O pacote atual declara seus módulos em `main.rs`. O exemplo abaixo é para uso dentro desse pacote; os testes executáveis demonstram a mesma API.

```rust
use crate::encryption::Des;

let cipher = Des::new(&0x133457799bbcdff1u64.to_be_bytes()).unwrap();
let plain = 0x0123456789abcdefu64.to_be_bytes();
let encrypted = cipher.encrypt_block(&plain);
assert_eq!(encrypted, 0x85e813540f0ab405u64.to_be_bytes());
assert_eq!(cipher.decrypt_block(&encrypted), plain);
```

## Arquivos existentes no projeto e sua utilidade

| Arquivo | Utilidade |
| --- | --- |
| `.gitignore` | Ignora arquivos gerados e temporários. |
| `Cargo.lock` | Fixa as versões das dependências usadas nos testes reproduzíveis. |
| `Cargo.toml` | Define o pacote, a edição Rust 2024 e as dependências de desenvolvimento. |
| `LICENSE` | Licença Apache 2.0 do projeto. |
| `README.md` | Apresentação geral e preparação do ambiente; a seção de estado inicial é histórica. |
| `docs/criptografia/des.md` | Funcionamento, interface, exemplos, complexidade e validação da cifra. |
| `src/encryption/mod.rs` | Exporta as APIs de criptografia. |
| `src/encryption/symmetric/caesar.rs` | Cifra de César e seus testes existentes. |
| `src/encryption/symmetric/des.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/des_tables.rs` | Constantes normativas e referência da sua origem. |
| `src/encryption/symmetric/des_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
| `src/encryption/symmetric/mod.rs` | Organiza as implementações de criptografia simétrica. |
| `src/encryption/symmetric/xor.rs` | Cifra XOR e seus testes existentes. |
| `src/main.rs` | Entrada do pacote binário e declaração dos módulos públicos. |
| `src/searching/find.rs` | Busca linear e seus testes. |
| `src/searching/max.rs` | Busca do maior elemento e seus testes. |
| `src/searching/min.rs` | Busca do menor elemento e seus testes. |
| `src/searching/mod.rs` | Exporta os algoritmos de busca. |
| `src/sorting/bubble_sort.rs` | Bubble Sort e seus testes. |
| `src/sorting/insertion_sort.rs` | Insertion Sort e seus testes. |
| `src/sorting/merge_sort.rs` | Merge Sort e seus testes. |
| `src/sorting/mod.rs` | Exporta os algoritmos de ordenação. |
| `src/sorting/selection_sort.rs` | Selection Sort e seus testes. |

## Análise de complexidade

A expansão executa 16 rotações e permutações de tamanho fixo. Cada bloco executa 16 rodadas de tamanho fixo. Portanto, construção, criptografia e descriptografia têm tempo **O(1)** para os parâmetros do DES. Para **b blocos**, reutilizando a instância, o tempo é **O(b)**.

As 16 subchaves ocupam 128 bytes (armazenadas em `u64`). O espaço auxiliar por chamada é **O(1)** e a saída tem oito bytes. Ao acumular b saídas, o espaço da saída é **O(b)**; a instância continua ocupando espaço constante.

## Testes e validação

As dependências de RustCrypto e `cipher` são exclusivamente de desenvolvimento. A implementação executada pela API foi escrita no projeto e não delega operações a essas bibliotecas.

Os testes verificam vetores publicados em ambas as direções, comparação independente da criptografia e da descriptografia, padrões binários, variação de bits de bloco e chave, todos os comprimentos aceitos, comprimentos inválidos, reutilização e independência das instâncias. A reversibilidade é uma verificação complementar. Amostras geradas usam sequência determinística, sem depender de aleatoriedade externa. DES e 3DES também verificam paridade e chaves fracas ou degeneradas.

Validação executada com Rust 1.99.1:

```text
cargo test --all-targets                 # 91 passaram; 0 falhas
cargo test --release --all-targets       # 91 passaram; 0 falhas
cargo clippy --all-targets               # nenhum aviso novo
```

A formatação dos arquivos Rust alterados foi verificada com `rustfmt --edition 2024 --check`. Os quatro avisos de Clippy preexistentes (`needless_return` em César e nas três buscas) foram preservados. Antes das mudanças, os 83 testes existentes passaram.

## Limitações

Esta é uma implementação didática da primitiva de bloco. Não inclui modos de operação, padding, autenticação, geração de chaves nem apagamento garantido das chaves na memória. Não oferece garantia de resistência a canais laterais. DES e 3DES são algoritmos históricos obsoletos para novas aplicações; os blocos de 64 bits também limitam o uso de Blowfish. Os vetores demonstram conformidade funcional, não certificação criptográfica.

## Referências

- FIPS 46-3: https://csrc.nist.gov/pubs/fips/46-3/final
- FIPS 81 (exemplo ECB): https://csrc.nist.gov/pubs/fips/81/final
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
