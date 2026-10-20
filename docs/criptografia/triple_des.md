# 3DES: implementação de blocos em Rust 2024

Closes #20

Referência de acompanhamento: #6.

Este PR depende da entrega de DES (#19): sua branch parte da branch de DES para reutilizar a mesma implementação. O destino continua sendo `main`; enquanto DES não for integrado, seu código também aparece no diff. Nenhum merge foi realizado.

## O que o algoritmo faz e como funciona

O 3DES aplica três operações DES ao mesmo bloco de 64 bits, na composição EDE: `C = E(K3, D(K2, E(K1, P)))`. A descriptografia inverte a ordem e cada operação: `P = D(K1, E(K2, D(K3, C)))`. As três chaves são expandidas uma única vez e reutilizadas.

Com 16 bytes, os oito primeiros formam K1 e K3 e os oito seguintes formam K2. Com 24 bytes, cada grupo de oito bytes forma uma chave. Os bits de paridade são ignorados pelo DES. Chaves iguais, fracas e degeneradas são aceitas pela primitiva; quando K1 = K2 = K3, o resultado coincide com DES.

## Interface e exemplo

- `TripleDes::new(key: &[u8]) -> Result<TripleDes, TripleDesKeyError>` aceita 16 ou 24 bytes.
- `encrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- `decrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- Comprimentos inválidos retornam erro tipado com o comprimento recebido, sem truncar nem completar a chave.
- A instância guarda a chave expandida; chamadas não alteram seu estado.

O pacote atual declara seus módulos em `main.rs`. O exemplo abaixo é para uso dentro desse pacote; os testes executáveis demonstram a mesma API.

```rust
use crate::encryption::TripleDes;

let cipher = TripleDes::new(b"12345678abcdefgh87654321").unwrap();
let plain = *b"mensagem";
let encrypted = cipher.encrypt_block(&plain);
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
| `docs/criptografia/triple_des.md` | Funcionamento, interface, exemplos, complexidade e validação da cifra. |
| `src/encryption/mod.rs` | Exporta as APIs de criptografia. |
| `src/encryption/symmetric/caesar.rs` | Cifra de César e seus testes existentes. |
| `src/encryption/symmetric/des.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/des_tables.rs` | Constantes normativas e referência da sua origem. |
| `src/encryption/symmetric/des_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
| `src/encryption/symmetric/mod.rs` | Organiza as implementações de criptografia simétrica. |
| `src/encryption/symmetric/triple_des.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/triple_des_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
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

A expansão constrói três instâncias DES, cada uma com 16 subchaves: **O(1)**. Criptografar ou descriptografar um bloco executa três passagens DES, totalizando 48 rodadas: **O(1)** para os parâmetros fixos, com custo aproximadamente triplo do DES. Para **b blocos**, o tempo é **O(b)**.

As subchaves ocupam 384 bytes. O espaço auxiliar é **O(1)**, incluindo os blocos intermediários; a saída possui oito bytes. Acumular b saídas ocupa **O(b)**.

## Testes e validação

As dependências de RustCrypto e `cipher` são exclusivamente de desenvolvimento. A implementação executada pela API foi escrita no projeto e não delega operações a essas bibliotecas.

Os testes verificam vetores publicados em ambas as direções, comparação independente da criptografia e da descriptografia, padrões binários, variação de bits de bloco e chave, todos os comprimentos aceitos, comprimentos inválidos, reutilização e independência das instâncias. A reversibilidade é uma verificação complementar. Amostras geradas usam sequência determinística, sem depender de aleatoriedade externa. DES e 3DES também verificam paridade e chaves fracas ou degeneradas.

Validação executada com Rust 1.99.1:

```text
cargo test --all-targets                 # 99 passaram; 0 falhas
cargo test --release --all-targets       # 99 passaram; 0 falhas
cargo clippy --all-targets               # nenhum aviso novo
```

A formatação dos arquivos Rust alterados foi verificada com `rustfmt --edition 2024 --check`. Os quatro avisos de Clippy preexistentes (`needless_return` em César e nas três buscas) foram preservados. Antes das mudanças, os 83 testes existentes passaram.

## Limitações

Esta é uma implementação didática da primitiva de bloco. Não inclui modos de operação, padding, autenticação, geração de chaves nem apagamento garantido das chaves na memória. Não oferece garantia de resistência a canais laterais. DES e 3DES são algoritmos históricos obsoletos para novas aplicações; os blocos de 64 bits também limitam o uso de Blowfish. Os vetores demonstram conformidade funcional, não certificação criptográfica.

## Referências

- FIPS 46-3: https://csrc.nist.gov/pubs/fips/46-3/final
- SP 800-67: https://csrc.nist.gov/pubs/sp/800/67/r2/final
- Vetores CAVP: https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Block-Ciphers
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
