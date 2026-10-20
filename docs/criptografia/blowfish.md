# Blowfish: implementação de blocos em Rust 2024

Closes #16

Referência de acompanhamento: #6.

## O que o algoritmo faz e como funciona

O Blowfish cifra blocos de 64 bits em uma rede de Feistel de 16 rodadas. A função de rodada divide uma palavra de 32 bits em quatro bytes e usa esses índices nas quatro S-boxes, combinando as consultas com adições módulo 2³² e XOR. A descriptografia percorre o vetor P em ordem inversa.

A expansão começa com 18 palavras P e quatro S-boxes de 256 palavras, inicializadas com os dígitos hexadecimais da parte fracionária de π. Os bytes da chave são combinados ciclicamente com P. Em seguida, 521 criptografias sucessivas de um bloco inicialmente zerado substituem todas as palavras de P e das S-boxes. A expansão é feita uma vez por instância.

## Interface e exemplo

- `Blowfish::new(key: &[u8]) -> Result<Blowfish, BlowfishKeyError>` aceita qualquer comprimento inteiro de 4 a 56 bytes.
- `encrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- `decrypt_block(&self, block: &[u8; 8]) -> [u8; 8]`.
- Comprimentos inválidos retornam erro tipado com o comprimento recebido, sem truncar nem completar a chave.
- A instância guarda a chave expandida; chamadas não alteram seu estado.

O pacote atual declara seus módulos em `main.rs`. O exemplo abaixo é para uso dentro desse pacote; os testes executáveis demonstram a mesma API.

```rust
use crate::encryption::Blowfish;

let cipher = Blowfish::new(b"chave de exemplo").unwrap();
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
| `docs/criptografia/blowfish.md` | Funcionamento, interface, exemplos, complexidade e validação da cifra. |
| `src/encryption/mod.rs` | Exporta as APIs de criptografia. |
| `src/encryption/symmetric/blowfish.rs` | Implementação da primitiva, expansão da chave, erro tipado e testes básicos. |
| `src/encryption/symmetric/blowfish_constants.rs` | Constantes normativas e referência da sua origem. |
| `src/encryption/symmetric/blowfish_tests.rs` | Vetores publicados e testes diferenciais contra RustCrypto. |
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

A expansão copia 1.042 palavras, combina a chave com as 18 palavras P e realiza 521 criptografias de 16 rodadas: **O(1)** para os parâmetros fixos do Blowfish, embora seja uma constante significativamente maior que o custo por bloco. Cada criptografia ou descriptografia executa 16 rodadas: **O(1)**. Para **b blocos** com a mesma instância, o tempo é **O(b)**.

P e as quatro S-boxes da instância ocupam 4.168 bytes. As constantes iniciais também ocupam espaço estático fixo. O espaço auxiliar por bloco é **O(1)** e a saída tem oito bytes; acumular b saídas exige **O(b)**.

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

- Especificação e vetores do autor: https://www.schneier.com/academic/blowfish/
- Vetores publicados: https://www.schneier.com/wp-content/uploads/2015/12/vectors-2.txt
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
