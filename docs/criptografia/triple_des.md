# 3DES: implementação de blocos em Rust 2024

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

## Arquivos do módulo

| Arquivo | Utilidade |
| --- | --- |
| `src/encryption/symmetric/triple_des/mod.rs` | API, erro tipado e composição EDE com duas ou três chaves. |
| `src/encryption/symmetric/triple_des/tests.rs` | Oito testes específicos de 3DES. |
| `src/encryption/symmetric/des/` | Primitiva DES reutilizada pela composição. |
| `docs/criptografia/triple_des.md` | Funcionamento, interface e complexidade do 3DES. |

## Análise de complexidade

A expansão constrói três instâncias DES, cada uma com 16 subchaves: **O(1)**. Criptografar ou descriptografar um bloco executa três passagens DES, totalizando 48 rodadas: **O(1)** para os parâmetros fixos, com custo aproximadamente triplo do DES. Para **b blocos**, o tempo é **O(b)**.

As subchaves ocupam 384 bytes. O espaço auxiliar é **O(1)**, incluindo os blocos intermediários; a saída possui oito bytes. Acumular b saídas ocupa **O(b)**.

## Testes e validação

A suíte específica de 3DES contém oito testes: exemplo de uso, comprimentos inválidos, vetores do apêndice B da SP 800-67, padrões binários e variação de bits, equivalência entre duas e três chaves, redução a DES com chaves iguais, paridade e amostras determinísticas com instâncias independentes.

Para executar somente essa suíte:

```text
cargo test encryption::symmetric::triple_des
cargo test --release encryption::symmetric::triple_des
```

Os testes diferenciais usam `des` exclusivamente como dependência de desenvolvimento, com imports por `reference_des::cipher`. O total de testes da branch inclui outras suítes e não representa a quantidade de testes de 3DES.

## Limitações

Esta é uma implementação didática da primitiva de bloco. Não inclui modos de operação, padding, autenticação, geração de chaves nem apagamento garantido das chaves na memória. Não oferece garantia de resistência a canais laterais. 3DES está obsoleto para novas aplicações. Os vetores demonstram conformidade funcional, não certificação criptográfica.

## Referências

- FIPS 46-3: https://csrc.nist.gov/pubs/fips/46-3/final
- SP 800-67: https://csrc.nist.gov/pubs/sp/800/67/r2/final
- Vetores CAVP: https://csrc.nist.gov/Projects/cryptographic-algorithm-validation-program/Block-Ciphers
- Implementações independentes usadas apenas nos testes: https://github.com/RustCrypto/block-ciphers
