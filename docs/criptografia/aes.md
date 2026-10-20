# AES-128/192/256: implementação de blocos em Rust 2024

Closes #18

Referência de acompanhamento: #6.

## O que o algoritmo faz e como funciona

O AES organiza os 16 bytes do bloco em um estado 4×4, preenchido por colunas, e executa 10, 12 ou 14 rodadas para chaves de 128, 192 ou 256 bits. Após AddRoundKey inicial, cada rodada aplica SubBytes, ShiftRows, MixColumns e AddRoundKey; a rodada final omite MixColumns. A descriptografia aplica as operações inversas em ordem inversa.

A expansão usa RotWord, SubWord e Rcon; AES-256 inclui SubWord adicional. As S-boxes são calculadas em tempo de compilação pelo inverso multiplicativo em GF(2⁸) e pela transformação afim da FIPS 197. MixColumns e sua inversa usam multiplicação nesse corpo com o polinômio 0x11b.

## Interface e exemplo

- `Aes::new(key: &[u8]) -> Result<Aes, AesKeyError>` aceita 16, 24 ou 32 bytes.
- `encrypt_block(&self, block: &[u8; 16]) -> [u8; 16]`.
- `decrypt_block(&self, block: &[u8; 16]) -> [u8; 16]`.
- Comprimentos inválidos retornam erro tipado com o comprimento recebido, sem truncar nem completar a chave.
- A instância guarda a chave expandida; chamadas não alteram seu estado.

O pacote atual declara seus módulos em `main.rs`. O exemplo abaixo é para uso dentro desse pacote; os testes executáveis demonstram a mesma API.

```rust
use crate::encryption::Aes;

let cipher = Aes::new(&[0; 16]).unwrap();
let plain = [0; 16];
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
| `docs/criptografia/aes.md` | Funcionamento, interface, exemplos, complexidade e validação da cifra. |
| `src/encryption/mod.rs` | Exporta as APIs de criptografia. |
| `src/encryption/symmetric/aes.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/aes_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
| `src/encryption/symmetric/caesar.rs` | Cifra de César e seus testes existentes. |
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

Para **r** rodadas (10, 12 ou 14), a expansão gera 4(r + 1) palavras e tem tempo **O(r)**. Cada bloco percorre r rodadas sobre 16 bytes, também **O(r)**. Como r pertence a um conjunto fixo, ambos são **O(1)** para AES. Para **b blocos**, o tempo é **O(b · r)**, simplificado para **O(b)**.

O armazenamento reserva 240 bytes para até 15 chaves de rodada, além do contador de rodadas. O estado e os temporários ocupam espaço auxiliar **O(1)** e a saída tem 16 bytes; acumular b saídas exige **O(b)**.

## Testes e validação

As dependências de RustCrypto e `cipher` são exclusivamente de desenvolvimento. A implementação executada pela API foi escrita no projeto e não delega operações a essas bibliotecas.

Os testes verificam vetores publicados em ambas as direções, comparação independente da criptografia e da descriptografia, padrões binários, variação de bits de bloco e chave, todos os comprimentos aceitos, comprimentos inválidos, reutilização e independência das instâncias. A reversibilidade é uma verificação complementar. Amostras geradas usam sequência determinística, sem depender de aleatoriedade externa. DES e 3DES também verificam paridade e chaves fracas ou degeneradas.

Validação executada com Rust 1.99.1:

```text
cargo test --all-targets                 # 92 passaram; 0 falhas
cargo test --release --all-targets       # 92 passaram; 0 falhas
cargo clippy --all-targets               # nenhum aviso novo
```

A formatação dos arquivos Rust alterados foi verificada com `rustfmt --edition 2024 --check`. Os quatro avisos de Clippy preexistentes (`needless_return` em César e nas três buscas) foram preservados. Antes das mudanças, os 83 testes existentes passaram.

## Limitações

Esta é uma implementação didática da primitiva de bloco. Não inclui modos de operação, padding, autenticação, geração de chaves nem apagamento garantido das chaves na memória. Não oferece garantia de resistência a canais laterais. DES e 3DES são algoritmos históricos obsoletos para novas aplicações; os blocos de 64 bits também limitam o uso de Blowfish. Os vetores demonstram conformidade funcional, não certificação criptográfica.

## Referências

- FIPS 197: https://csrc.nist.gov/pubs/fips/197/final
- Vetores CAVP: https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Block-Ciphers
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
