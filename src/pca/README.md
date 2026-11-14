# Análise de Componentes Principais (PCA)

Este módulo faz a **análise de componentes principais** (PCA, do inglês *Principal Component Analysis*) de uma tabela de dados: acha as direções em que os dados mais variam, quanto da variação total cada uma explica e como cada observação fica descrita nesses novos eixos.

O código foi escrito para ser **lido e estudado**. Cada função tem uma explicação com a fórmula, um exemplo feito passo a passo, os casos em que não há resposta e a análise de complexidade.

**Os algoritmos foram implementados à mão.** O módulo não usa nenhum pacote externo (o `Cargo.toml` não tem dependências). A álgebra linear (produto escalar, norma, produto externo...), a matriz de covariância e o cálculo dos autovalores e autovetores estão todos escritos aqui. Da linguagem, o módulo usa as listas (`Vec`), texto e impressão, a raiz quadrada (`sqrt`) e os **iteradores** (`iter`, `map`, `sum`, `zip`, `fold`...) para percorrer as listas.

O módulo implementa o que foi proposto na issue #7, uma etapa por arquivo:

| Etapa | Arquivo |
|---|---|
| 1. Operações básicas de álgebra linear | `linear_algebra.rs` |
| 2. Centralização dos dados | `center.rs` |
| 3. Matriz de covariância | `covariance_matrix.rs` |
| 4. Iteração de potências com deflação (os k maiores autovalores e autovetores) | `power_iteration.rs` |
| 5. Projeção dos dados centralizados sobre os componentes | `projection.rs` |
| A análise completa, que junta as etapas | `principal_components.rs` |

O arquivo `helpers.rs` guarda as regras usadas por todos: conferir se a tabela é válida e achar o maior valor absoluto de uma lista.

---

## O que o PCA faz

Imagine cada observação como um ponto e cada variável como um eixo: a tabela vira uma **nuvem de pontos**. O PCA gira os eixos:

- o **primeiro componente principal** aponta para a direção em que a nuvem mais se espalha (a direção de maior variância);
- o **segundo** aponta para a direção de maior variância que sobra, perpendicular ao primeiro;
- e assim por diante, até um eixo por variável.

Muitas vezes, poucos componentes explicam quase toda a variação. Aí cada observação pode ser resumida por poucos números, os **escores**, em vez de todas as variáveis.

Na linguagem da estatística: os componentes são os **autovetores** da matriz de covariância, e a variância dos dados ao longo de cada componente é o **autovalor** correspondente. Um autovetor v e o seu autovalor λ cumprem

```text
C · v = λ · v
```

ou seja, multiplicar v pela matriz de covariância C só "estica" v, sem mudar a sua direção. A soma de todos os autovalores é a **variância total** (a soma das variâncias de todas as variáveis), e a proporção explicada por um componente é o seu autovalor dividido por esse total.

---

## As funções

| Função | Arquivo | O que responde |
|---|---|---|
| `pca(dados, k)` | `principal_components.rs` | a análise completa, com os `k` componentes mais importantes |
| `column_means(dados)` | `center.rs` | a média de cada variável (coluna) |
| `center(dados)` | `center.rs` | as médias e a tabela centralizada (cada valor menos a média da sua coluna) |
| `covariance_matrix(dados)` | `covariance_matrix.rs` | a matriz de covariância amostral (divide por n − 1) |
| `linear_algebra::add(a, b)` e `subtract(a, b)` | `linear_algebra.rs` | soma e subtração de vetores, posição por posição |
| `linear_algebra::dot(a, b)` | `linear_algebra.rs` | o produto escalar |
| `linear_algebra::matrix_vector(m, v)` | `linear_algebra.rs` | o produto de uma matriz por um vetor |
| `linear_algebra::outer(a, b)` | `linear_algebra.rs` | o produto externo (o resultado é uma matriz) |
| `linear_algebra::norm(v)` e `normalize(v)` | `linear_algebra.rs` | o comprimento do vetor, e o vetor de comprimento 1 na mesma direção |

A iteração de potências (`power_iteration.rs`) e a projeção (`projection.rs`) são internas: a função `pca` as usa, e os testes de cada arquivo as conferem.

### A tabela de dados

A tabela é uma lista de linhas (`&[Vec<f64>]`): **cada linha é uma observação** e **cada coluna é uma variável**.

```rust
//              variável 1  variável 2
let dados = vec![
    vec![1.0, 2.0], // observação 1
    vec![2.0, 1.0], // observação 2
    vec![3.0, 4.0], // observação 3
    vec![4.0, 3.0], // observação 4
];
```

### O resultado: `Pca`

| Campo | O que é | Tamanho |
|---|---|---|
| `means` | a média de cada variável | p |
| `eigenvalues` | os k maiores autovalores, em ordem decrescente: a variância ao longo de cada componente | k |
| `explained_variance_ratio` | a proporção da variância total explicada por cada componente (entre 0 e 1) | k |
| `components` | os k componentes (autovetores), um por linha, com p pesos cada | k × p |
| `scores` | os escores: cada observação nos novos eixos | n × k |

Os componentes têm norma 1 e são perpendiculares entre si. Como v e −v são o mesmo autovetor, o sinal é escolhido assim: **o peso de maior valor absoluto fica positivo** (a variável que mais pesa no componente entra com sinal +); se dois pesos empatam, vale o primeiro deles.

---

## O que significa `None`

Todas as funções devolvem um `Option`:

- `Some(valor)` quando existe resposta;
- `None` quando **não existe resposta**.

| Função | Responde `None` quando... |
|---|---|
| Todas as que recebem uma tabela | a tabela está vazia, as linhas têm tamanhos diferentes, ou há `NaN` ("não é um número") ou infinito |
| `covariance_matrix` | há menos de 2 observações (o divisor n − 1 seria 0) |
| `pca` | há menos de 2 observações; `k` é 0 ou maior que o número de variáveis; ou todas as observações são iguais (nenhuma variável varia, e não existe direção principal) |
| `add`, `subtract`, `dot` | os vetores têm tamanhos diferentes, ou há `NaN` ou infinito |
| `matrix_vector` | alguma linha da matriz tem um tamanho diferente do vetor, ou há `NaN` ou infinito |
| `outer`, `norm` | há `NaN` ou infinito |
| `normalize` | o vetor é nulo ou vazio (não aponta para lado nenhum), ou há `NaN` ou infinito |

Quando a resposta verdadeira não cabe no `f64` (passa de cerca de 1,8 × 10³⁰⁸), ela vira **infinito**, e nunca `NaN`.

---

## Exemplo de uso

```rust
use crate::pca::pca;

let dados = vec![
    vec![1.0, 2.0],
    vec![2.0, 1.0],
    vec![3.0, 4.0],
    vec![4.0, 3.0],
];

// Os 2 componentes (k = 2).
if let Some(resultado) = pca(&dados, 2) {
    println!("{resultado}");
}
```

Saída:

```text
Análise de componentes principais (n = 4 observações, p = 2 variáveis)

Componente     Autovalor   Variância explicada   Acumulada
CP1               2.6667                80.00%      80.00%
CP2               0.6667                20.00%     100.00%

Pesos das variáveis em cada componente (autovetores):
                     CP1         CP2
Variável 1        0.7071      0.7071
Variável 2        0.7071     -0.7071

Escores (as observações nos novos eixos):
                     CP1         CP2
Obs. 1           -1.4142     -0.7071
Obs. 2           -1.4142      0.7071
Obs. 3            1.4142     -0.7071
Obs. 4            1.4142      0.7071
```

**Como ler:** o primeiro componente explica 80% da variação. Ele tem pesos iguais (0,7071) nas duas variáveis: é a direção (1, 1), em que as duas crescem juntas. As observações 3 e 4 ficam do lado positivo desse eixo, e as observações 1 e 2, do lado negativo. A tabela de escores mostra no máximo 10 observações; os campos do `Pca` guardam todas.

**Variáveis em escalas muito diferentes?** O PCA da matriz de covariância dá mais peso às variáveis de maior variância. Se uma variável está em metros e outra em milímetros, por exemplo, costuma-se **padronizar** cada coluna antes (dividir pelo seu desvio padrão), e o PCA passa a olhar só para as correlações.

---

## O exemplo feito à mão, passo a passo

Com os dados do exemplo acima (n = 4 observações, p = 2 variáveis).

**1. Médias.**

```text
variável 1: (1 + 2 + 3 + 4) ÷ 4 = 10 ÷ 4 = 2,5
variável 2: (2 + 1 + 4 + 3) ÷ 4 = 10 ÷ 4 = 2,5
```

**2. Centralização:** cada valor menos a média da sua coluna.

```text
[1 − 2,5;  2 − 2,5] = [−1,5; −0,5]
[2 − 2,5;  1 − 2,5] = [−0,5; −1,5]
[3 − 2,5;  4 − 2,5] = [ 0,5;  1,5]
[4 − 2,5;  3 − 2,5] = [ 1,5;  0,5]
```

**3. Matriz de covariância** (divisor n − 1 = 3):

```text
C₁₁ = (2,25 + 0,25 + 0,25 + 2,25) ÷ 3 = 5 ÷ 3
C₂₂ = (0,25 + 2,25 + 2,25 + 0,25) ÷ 3 = 5 ÷ 3
C₁₂ = (0,75 + 0,75 + 0,75 + 0,75) ÷ 3 = 3 ÷ 3 = 1

C = [5/3   1 ]
    [ 1   5/3]
```

**4. Autovalores e autovetores.** Para uma matriz 2 × 2 dá para fazer à mão. Os autovalores são os λ que zeram o determinante de C − λ·I:

```text
(5/3 − λ)² − 1² = 0
5/3 − λ = ±1
λ = 5/3 + 1 = 8/3 = 2,6667   ou   λ = 5/3 − 1 = 2/3 = 0,6667
```

Para λ = 8/3, a primeira linha de (C − λ·I)·v = 0 dá (5/3 − 8/3)·v₁ + v₂ = 0, ou seja, −v₁ + v₂ = 0: v = (1, 1), que com norma 1 é (0,7071; 0,7071). Para λ = 2/3: v₁ + v₂ = 0, então v = (1, −1), ou (0,7071; −0,7071).

**5. Proporções da variância.** A variância total é 5/3 + 5/3 = 10/3 (é também 8/3 + 2/3):

```text
(8/3) ÷ (10/3) = 0,8 = 80%
(2/3) ÷ (10/3) = 0,2 = 20%
```

**6. Escores:** o produto escalar de cada linha centralizada com cada componente. Para a primeira observação, (−1,5; −0,5):

```text
no CP1: (−1,5 · 0,7071) + (−0,5 · 0,7071) = −2 · 0,7071 = −1,4142
no CP2: (−1,5 · 0,7071) + (−0,5 · (−0,7071)) = −1 · 0,7071 = −0,7071
```

### Como o código acha os autovalores: a iteração de potências

Com mais de 2 ou 3 variáveis, o determinante vira um polinômio de grau alto, e resolvê-lo deixa de ser prático. O código usa a **iteração de potências**: multiplicar um vetor pela matriz C repetidas vezes, voltando-o para o comprimento 1 a cada vez, faz o vetor girar até a direção do maior autovalor. Começando em v = (1, 0):

```text
repetição   C · v               v novo (norma 1)    λ = v · (C·v)
    1       (1,6667; 1,0000)    (0,8575; 0,5145)    1,6667
    2       (1,9437; 1,7150)    (0,7498; 0,6616)    2,5490
    3       (1,9114; 1,8525)    (0,7181; 0,6960)    2,6589
    4       (1,8928; 1,8780)    (0,7099; 0,7043)    2,6662
   ...
   10       (1,8856; 1,8856)    (0,7071; 0,7071)    2,6667 = 8/3
```

**Por que funciona.** Escrevendo o vetor inicial na base dos autovetores, v = c₁v₁ + c₂v₂, multiplicar por C repetidas t vezes dá c₁λ₁ᵗv₁ + c₂λ₂ᵗv₂. Como λ₁ = 8/3 é maior que λ₂ = 2/3, o primeiro termo cresce mais depressa: a cada repetição, a parte de v₂ encolhe na proporção λ₂ ÷ λ₁ = 1/4.

**A deflação.** Achado o primeiro par (λ₁, v₁), a parte dele é tirada da matriz: C ← C − λ₁ · (v₁ v₁ᵀ), em que v₁ v₁ᵀ é o produto externo de v₁ com ele mesmo.

```text
v₁ v₁ᵀ = [0,5  0,5]        C − (8/3)·(v₁ v₁ᵀ) = [ 1/3  −1/3]
         [0,5  0,5]                             [−1/3   1/3]
```

Na matriz que sobra, a direção mais forte é a do segundo autovetor, (1, −1), com autovalor 2/3, e a mesma iteração o encontra. Para k componentes, são k rodadas de iteração e deflação.

---

## Cuidados com os números

O `f64` guarda números de cerca de 10⁻³⁰⁸ até 1,8 × 10³⁰⁸, com umas 16 casas decimais. O módulo toma estes cuidados:

- **Normalização.** Antes das contas, `pca` divide a tabela inteira pelo seu maior valor absoluto s: os valores ficam entre −1 e 1, e os quadrados e produtos não estouram nem somem, mesmo com dados perto de 1e300 ou de 1e-300. Dividir tudo pelo mesmo número não muda as direções. No fim, os escores são multiplicados por s, e os autovalores por s duas vezes.
- **Somas que estouram.** O produto escalar, a norma, as médias e a matriz de covariância calculam primeiro do jeito direto. Só quando a conta estoura (ou, na norma, fica pequena demais) é que refazem a conta com os valores divididos pelo maior valor absoluto, porque essas divisões arredondam um pouco. Exemplo: `dot(&[1e200, 1e200], &[1e109, -0.9e109])` tem produtos que viram +infinito e −infinito, mas a resposta, 1e308, cabe e sai certa.
- **Resultados que não cabem.** Se um autovalor verdadeiro passa de 1,8 × 10³⁰⁸ (dados perto de 1e200, por exemplo), ele vira infinito, mas os componentes, as proporções e os escores continuam certos.
- **Dados constantes.** Se todas as observações são iguais, `pca` responde `None`. Sem esse cuidado, valores como 0,1 (que o computador não guarda de forma exata) criariam uma variância minúscula e falsa.
- **Variáveis perfeitamente correlacionadas** (por exemplo, y = 2x). Algumas direções não têm variação nenhuma, e os autovalores delas são 0. Depois de tirar as direções com variação, o que sobra da matriz é só arredondamento; aí os autovalores restantes saem 0 exato, e os autovetores são direções perpendiculares às já encontradas. Um autovalor menor que 10⁻¹² vezes a variância total também é tratado como arredondamento e sai 0.
- **O vetor inicial.** Se o vetor inicial da iteração for perpendicular ao autovetor procurado, a iteração não o encontra. Em [[1, −1], [−1, 1]], por exemplo, (1, 1) é perpendicular ao autovetor (1, −1). Por isso o vetor inicial vem de um gerador de números pseudoaleatórios, escrito em `power_iteration.rs`, com semente fixa (o resultado é sempre o mesmo). Cada componente começa de um vetor novo: com um autovalor repetido, como em diag(2, 2, 0), o mesmo vetor inicial não teria mais nada a oferecer depois do primeiro autovetor.

### Precisão: quantas casas saem certas

A iteração para quando ‖C·v − λ·v‖ fica menor que 10⁻¹² vezes a variância total, ou depois de 10.000 repetições. Na conferência com dados conhecidos, os autovalores saíram com erro da ordem de 10⁻¹⁶ vezes a variância total.

Os autovetores dependem também da distância até o autovalor vizinho: o erro de um autovetor é mais ou menos o erro aceito dividido por essa distância. Por isso, **os componentes de variância muito pequena em relação à total** (por exemplo, numa tabela com uma variável na casa das centenas e outra na casa das unidades) saem com menos casas certas: os pesos dos últimos componentes podem ficar certos só até a 5ª ou 6ª casa decimal. Padronizar as variáveis antes ajuda. Quando dois autovalores são quase iguais, os dois autovetores ficam mal definidos (qualquer mistura deles serve quase igualmente), mas os autovalores continuam certos.

---

## Como rodar

Todos os comandos são digitados no **terminal**, dentro da pasta do projeto (a pasta que tem o arquivo `Cargo.toml`).

**Rodar os testes do módulo:**

```text
cargo test pca
```

No fim, deve aparecer uma linha como esta (o número de testes pode mudar):

```text
test result: ok. 67 passed; 0 failed; 0 ignored; 0 measured; ...
```

**Ver o PCA funcionando**, com o resultado impresso na tela:

```text
cargo test print_pca_demo -- --nocapture
```

**Ver a documentação** como página web, com a explicação de cada função:

```text
cargo doc --open
```

---

## Como ler este código: o Rust que aparece aqui

Se você está começando no Rust, esta tabela explica os recursos da linguagem usados no módulo.

| Recurso | O que significa | Exemplo no módulo |
|---|---|---|
| `fn` | declara uma função | `fn dot(a: &[f64], b: &[f64]) -> Option<f64>` |
| `&[f64]` | uma lista de números decimais **emprestada**: a função só lê a lista, não pode alterá-la | `linear_algebra.rs` |
| `&[Vec<f64>]` e `Vec<Vec<f64>>` | uma tabela: uma lista de linhas, cada linha uma lista de números | `pca(data: &[Vec<f64>], k: usize)` |
| `Option`, `Some(valor)`, `None` | uma resposta que pode existir (`Some`) ou não existir (`None`) | todas as funções |
| `?` | "se a resposta for `None`, pare aqui e devolva `None` também" | `let means = column_means(data)?;` |
| `let` e `let mut` | cria uma variável; com `mut`, ela pode mudar de valor depois | `let mut remaining = matrix.to_vec();` |
| `const` | um valor fixo, com nome | `const TOLERANCE: f64 = 1e-12;` |
| `(a, b)` | um par de valores (uma tupla). `let (_, centered) = ...` guarda o segundo e descarta o primeiro (`_`) | `center.rs`, `covariance_matrix.rs` |
| `data.iter()` | um **iterador**: anda pela lista, entregando um valor de cada vez | todos os arquivos |
| `\|x\| ...` | uma função curta e sem nome (*closure*): recebe `x` e devolve o que vem depois. `\|&x\|` pega o próprio número, e não uma referência a ele | `.map(\|&x\| x / scale)` |
| `.map(f)` | transforma cada valor com a função `f` | todos os arquivos |
| `.zip(b)` | anda por duas listas ao mesmo tempo, formando os pares (a[0], b[0]), (a[1], b[1]), ... | `linear_algebra.rs` |
| `.sum()` | soma todos os valores que o iterador entrega | `linear_algebra.rs` |
| `.fold(início, f)` | começa com `início` e vai acumulando: a cada valor, `f` devolve o acumulado novo | `helpers.rs` |
| `.all(f)` e `.find(f)` | "todos passam no teste `f`?"; "qual é o primeiro que passa?" | `helpers.rs`, `principal_components.rs` |
| `.enumerate()` | entrega cada valor junto com a sua posição: (0, a), (1, b), ... | `covariance_matrix.rs` |
| `.skip(i)` e `.take(n)` | pula os `i` primeiros valores; pega só os `n` primeiros | `covariance_matrix.rs`, `principal_components.rs` |
| `.collect()` | junta os valores numa lista nova. Se os valores são `Option`, junta num `Option`: basta um `None` para o resultado todo ser `None` | `linear_algebra.rs` |
| `.concat()` | junta as linhas de uma tabela numa lista só | `power_iteration.rs` |
| `vec![0.0; p]` | uma lista com `p` zeros | `power_iteration.rs` |
| `for x in lista { ... }` | repete o bloco para cada valor | `power_iteration.rs` |
| `if let Some(m) = ... { ... }` | "se a resposta for `Some`, chame o valor de `m` e rode o bloco" | `covariance_matrix.rs` |
| `as f64`, `as usize` | converte um número de um tipo para outro (inteiro ↔ decimal) | `data.len() as f64` |
| `.abs()`, `.sqrt()`, `.max(b)`, `.clamp(a, b)` | valor absoluto, raiz quadrada, o maior dos dois, trazer de volta para o intervalo de `a` a `b` | vários arquivos |
| `f64::MAX`, `f64::MIN_POSITIVE`, `f64::INFINITY`, `f64::NAN` | o maior `f64`; o menor `f64` positivo "normal"; infinito; "não é um número" | `linear_algebra.rs`, testes |
| `u64`, `wrapping_mul`, `wrapping_add`, `>>` | inteiro sem sinal de 64 bits; multiplicação e soma que "dão a volta" ao passar do maior valor; deslocar os bits para a direita | `power_iteration.rs` |
| `&mut u64` e `*state` | um valor **emprestado para alteração**: a função muda o valor de quem a chamou; `*state` é o próprio valor | `power_iteration.rs` |
| `struct` e `#[derive(...)]` | uma "ficha" com campos nomeados; o `derive` gera sozinho habilidades simples: `Debug` (mostrar), `Clone` (copiar), `PartialEq` (comparar com `==`) | `Pca`, em `principal_components.rs` |
| `impl fmt::Display for Pca` | ensina o `Pca` a se imprimir com `println!("{resultado}")` | `principal_components.rs` |
| `format!("{:>12}", x)` e `{:.4}` | texto alinhado à direita em 12 colunas; número com 4 casas decimais | `principal_components.rs` |
| `pub`, `pub(super)` ou nada | quem pode usar: qualquer parte do projeto; só a pasta `pca`; só o próprio arquivo | todos os arquivos |
| `mod`, `pub mod` e `pub use` | em `mod.rs`: declaram os arquivos do módulo e deixam as funções acessíveis como `pca::pca` | `mod.rs` |
| `#[cfg(test)] mod tests` e `#[test]` | o bloco de testes, que só é compilado ao rodar `cargo test`; cada `#[test]` marca um teste | fim de cada arquivo |
| `assert_eq!(a, b)` e `assert!(c)` | o teste falha se `a` for diferente de `b`, ou se `c` for falso | testes |
| `//` e `///` | `//` é um comentário comum; `///` é documentação, que aparece na página do `cargo doc` | todos os arquivos |

---

## Complexidade dos algoritmos

### O que é complexidade

A complexidade descreve **como o custo cresce** quando os dados crescem. Aqui são quatro tamanhos:

- **n**: o número de observações (linhas);
- **p**: o número de variáveis (colunas);
- **k**: o número de componentes pedidos (k ≤ p);
- **t**: o número de repetições da iteração de potências para cada componente.

### Etapa por etapa

| Etapa | Tempo | Espaço extra | De onde vem o custo |
|---|---|---|---|
| 1. Soma, subtração, produto escalar, norma, normalização | O(m) | O(1) ou O(m) | uma ou poucas passadas por vetores de m valores |
| 1. Produto matriz-vetor (matriz a × b) | O(a·b) | O(a) | um produto escalar por linha |
| 1. Produto externo (vetores de a e b valores) | O(a·b) | O(a·b) | uma multiplicação por casa da matriz |
| 2. Médias e centralização | O(n·p) | O(n·p) | uma passada por valor da tabela |
| 3. Matriz de covariância | O(n·p²) | O(n·p + p²) | p(p + 1)/2 casas diferentes (a outra metade é cópia, pela simetria), cada uma um produto escalar de n valores |
| 4. Iteração de potências com deflação | O(k·t·p²) | O(p²) | t multiplicações matriz-vetor (p² cada) por componente, mais uma deflação de O(p²) por componente |
| 5. Projeção | O(n·p·k) | O(n·k) | um produto escalar de p valores por par (observação, componente) |
| **`pca` completo** | **O(n·p² + k·t·p²)** | **O(n·p + p²)** | a matriz de covariância e a iteração de potências; a projeção, O(n·p·k), nunca passa de O(n·p²), porque k ≤ p |

### Quantas repetições: o t

A cada repetição, o erro do autovetor encolhe na proporção λ₂ ÷ λ₁ (o segundo maior autovalor dividido pelo maior). Para o erro cair de 1 até 10⁻¹², a quantidade de repetições é mais ou menos

```text
t ≈ log(10¹²) ÷ log(λ₁ ÷ λ₂)
```

- No exemplo, λ₂ ÷ λ₁ = (2/3) ÷ (8/3) = 1/4: o erro cai 4 vezes a cada repetição, e cerca de **20 repetições** bastam.
- Se λ₂ ÷ λ₁ = 0,99 (autovalores muito parecidos), são necessárias cerca de **2.750**.

Por isso t não é uma constante do problema: depende dos dados. O limite de 10.000 repetições por componente garante que a função sempre termina. Ele só é atingido quando dois autovalores são quase iguais, e aí os autovalores ainda saem certos (veja "Precisão", acima).

**Na prática** (num computador comum, com `cargo run --release`), para tabelas de dados aleatórios:

| Tabela (n × p) | Componentes (k) | Tempo |
|---|---|---|
| 500 × 200 | 5 | menos de 1 segundo |
| 2000 × 50 | 50 (todos) | cerca de 1 segundo |
| 500 × 200 | 200 (todos) | cerca de 1 minuto |

O último caso é lento porque, com 200 variáveis aleatórias, os 200 autovalores ficam muito próximos uns dos outros (λ₂ ÷ λ₁ perto de 1), e cada componente precisa de milhares de repetições. No uso mais comum do PCA, com poucos componentes, isso não acontece.

### Por que a iteração de potências, e não "resolver tudo de uma vez"?

Há algoritmos que acham **todos** os p autovalores de uma vez (como o método de Jacobi ou o algoritmo QR), com custo da ordem de p³. A iteração de potências acha só os k que interessam, com custo O(k·t·p²). Quando poucos componentes bastam (k bem menor que p), ela faz menos trabalho, e cada passo dela é simples de entender: multiplicar, normalizar, repetir.

### Existe algo mais rápido ou mais preciso?

- **Método de Lanczos:** uma versão mais esperta da iteração de potências, que aproveita todos os vetores da sequência (e não só o último) e precisa de bem menos repetições.
- **Decomposição em valores singulares (SVD) da tabela centralizada:** acha os mesmos componentes sem montar a matriz de covariância. Montar C "eleva ao quadrado" as diferenças de escala entre os componentes, e é daí que vem a perda de casas decimais nos componentes de variância pequena. A SVD evita isso, mas é um algoritmo bem mais longo de escrever à mão.

### Tabela-resumo

| Função | Tempo | Espaço extra |
|---|---|---|
| `add`, `subtract`, `dot`, `norm`, `normalize` | O(m) | O(1) a O(m) |
| `matrix_vector` | O(a·b) | O(a) |
| `outer` | O(a·b) | O(a·b) |
| `column_means` | O(n·p) | O(p) |
| `center` | O(n·p) | O(n·p) |
| `covariance_matrix` | O(n·p²) | O(n·p + p²) |
| `pca` | O(n·p² + k·t·p²) | O(n·p + p²) |
