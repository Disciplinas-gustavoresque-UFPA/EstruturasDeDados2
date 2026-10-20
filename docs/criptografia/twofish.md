# Twofish: implementação de blocos em Rust 2024

Closes #17

Referência de acompanhamento: #6.

## O que o algoritmo faz e como funciona

O Twofish transforma quatro palavras little-endian de 32 bits por whitening de entrada, 16 rodadas de Feistel e whitening de saída. As funções de rodada combinam permutações q0/q1 dependentes da chave com uma matriz MDS em GF(2⁸), seguida de uma transformação pseudo-Hadamard, adições módulo 2³² e rotações de um bit. A descriptografia desfaz as trocas, rotações e combinações na ordem inversa.

A expansão separa as palavras pares e ímpares da chave para gerar 40 subchaves. Uma matriz Reed–Solomon gera as palavras S, armazenadas em ordem inversa, que parametrizam a função h. A matriz MDS usa o polinômio 0x169 e a matriz RS usa 0x14d. A implementação calcula h diretamente, sem tabelas grandes dependentes da chave.

## Interface e exemplo

- `Twofish::new(key: &[u8]) -> Result<Twofish, TwofishKeyError>` aceita 16, 24 ou 32 bytes.
- `encrypt_block(&self, block: &[u8; 16]) -> [u8; 16]`.
- `decrypt_block(&self, block: &[u8; 16]) -> [u8; 16]`.
- Comprimentos inválidos retornam erro tipado com o comprimento recebido, sem truncar nem completar a chave.
- A instância guarda a chave expandida; chamadas não alteram seu estado.

O pacote atual declara seus módulos em `main.rs`. O exemplo abaixo é para uso dentro desse pacote; os testes executáveis demonstram a mesma API.

```rust
use crate::encryption::Twofish;

let cipher = Twofish::new(&[0; 32]).unwrap();
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
| `docs/criptografia/twofish.md` | Funcionamento, interface, exemplos, complexidade e validação da cifra. |
| `src/encryption/mod.rs` | Exporta as APIs de criptografia. |
| `src/encryption/symmetric/caesar.rs` | Cifra de César e seus testes existentes. |
| `src/encryption/symmetric/mod.rs` | Organiza as implementações de criptografia simétrica. |
| `src/encryption/symmetric/twofish.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/twofish_tables.rs` | Constantes normativas e referência da sua origem. |
| `src/encryption/symmetric/twofish_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
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

Se **k** é o número de grupos de oito bytes da chave (2, 3 ou 4), a expansão calcula 40 funções h e k produtos RS: **O(k)**. Cada bloco executa 16 rodadas com duas funções h: **O(16 · k)**. Como k e o número de rodadas são fixos e limitados, construção e operação por bloco são **O(1)**. Para **b blocos**, o tempo é **O(b)**.

As 40 subchaves e quatro palavras S reservam 176 bytes, além do contador de palavras da chave. O estado e os temporários ocupam espaço auxiliar **O(1)**. A saída tem 16 bytes; acumular b saídas exige **O(b)**.

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

- Especificação, apêndice de vetores e KAT dos autores: https://www.schneier.com/academic/twofish/
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
