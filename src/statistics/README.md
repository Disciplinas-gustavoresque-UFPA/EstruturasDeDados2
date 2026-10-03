# Estatística descritiva

Este módulo calcula as **medidas-resumo** de uma lista de números: média, moda, variância, desvio padrão, coeficiente de variação, amplitude, quantis, quartis, intervalo interquartílico, resumo de cinco números, covariância e correlação.

Cada medida tem a sua própria função, num arquivo próprio desta pasta. A função `summary` calcula de uma vez todas as medidas de **uma** lista. Covariância e correlação ficam de fora dela, porque precisam de **duas** listas.

O código foi escrito para ser **lido e estudado**. Cada função tem uma explicação com a fórmula, um exemplo feito passo a passo, os casos em que não há resposta e a análise de complexidade.

**Tudo foi implementado à mão.** O módulo não usa nenhum pacote externo (o `Cargo.toml` não tem dependências), e as fórmulas e os laços estão escritos por extenso. Para ordenar, ele usa o `merge_sort` da própria biblioteca (`src/sorting/merge_sort.rs`). Da linguagem, usa só o básico (listas `Vec`, texto e impressão) e as operações do processador (soma, subtração, multiplicação, divisão, comparações e raiz quadrada). Nenhum algoritmo vem pronto.

---

## As medidas

| Medida | Função | Arquivo | O que responde |
|---|---|---|---|
| Média | `mean(dados)` | `mean.rs` | o "ponto de equilíbrio" dos dados |
| Moda | `mode(dados)` | `mode.rs` | o(s) valor(es) que mais se repete(m) |
| Variância | `variance(dados)` | `variance.rs` | o quanto os dados se espalham em volta da média (amostral, divide por n − 1) |
| Desvio padrão | `std_dev(dados)` | `std_dev.rs` | a raiz da variância, na mesma unidade dos dados |
| Coeficiente de variação | `coefficient_of_variation(dados)` | `coefficient_of_variation.rs` | o desvio padrão em proporção da média |
| Amplitude | `range(dados)` | `range.rs` | máximo − mínimo |
| Quantil | `quantile(dados, p)` | `quantile.rs` | o valor abaixo do qual fica a fração `p` dos dados |
| Mediana | `median(dados)` | `median.rs` | o valor do meio |
| Quartis | `quartiles(dados)` | `quartiles.rs` | Q1, Q2 (mediana) e Q3 |
| Intervalo interquartílico | `iqr(dados)` | `iqr.rs` | Q3 − Q1: a largura dos 50% centrais |
| Resumo de cinco números | `five_number_summary(dados)` | `five_number.rs` | mínimo, Q1, mediana, Q3 e máximo |
| Covariância | `covariance(x, y)` | `covariance.rs` | se duas listas pareadas variam juntas |
| Correlação de Pearson | `correlation(x, y)` | `correlation.rs` | a força da relação linear, de −1 a 1 |
| Resumo completo | `summary(dados)` | `summary.rs` | todas as medidas de uma lista, prontas para imprimir |

O arquivo `helpers.rs` guarda as regras usadas por todos: conferir se a lista é válida e fazer uma cópia ordenada. A cópia é ordenada pelo `merge_sort` da biblioteca, com os valores "embrulhados" num tipo que tem ordem total (o `f64` sozinho não tem, por causa do `NaN`).

---

## O que significa `None`

Todas as funções devolvem um `Option`:

- `Some(valor)` quando existe resposta;
- `None` quando **não existe resposta**: lista vazia, lista com `NaN` ("não é um número") ou infinito, ou um caso que a própria medida não aceita (por exemplo, a variância de um único número).

**Na moda, `None` e a lista vazia são coisas diferentes.** `None` quer dizer "não há dados". `Some(vec![])` quer dizer "há dados, mas nenhuma moda" (por exemplo, quando todos os valores aparecem o mesmo número de vezes).

### Quando a resposta é `None`

| Medida | Responde `None` quando... |
|---|---|
| Todas | a lista está vazia ou tem `NaN` ou infinito |
| Variância, desvio padrão | há menos de 2 valores |
| Coeficiente de variação | há menos de 2 valores, ou a média é zero |
| Quantil | `p` está fora do intervalo de 0 a 1 |
| Covariância | as listas têm tamanhos diferentes, ou há menos de 2 pares |
| Correlação | os casos da covariância, ou uma das listas é constante (todos os valores iguais) |

---

## Exemplo de uso

```rust
use crate::statistics::{mean, median, summary};

let dados = [2.0, 4.0, 4.0, 5.0, 7.0, 9.0];

println!("{:?}", mean(&dados));   // Some(5.166666666666667)
println!("{:?}", median(&dados)); // Some(4.5)

if let Some(resumo) = summary(&dados) {
    println!("{resumo}");
}
```

Saída do resumo:

```text
Resumo estatístico (n = 6)
Média ................ 5.1667
Moda ................. [4.0000]
Variância ............ 6.1667
Desvio padrão ........ 2.4833
Coef. de variação .... 0.4806
Mínimo ............... 2.0000
Q1 ................... 4.0000
Mediana .............. 4.5000
Q3 ................... 6.5000
Máximo ............... 9.0000
IQR .................. 2.5000
Amplitude ............ 7.0000
```

**Tem números inteiros?** As funções recebem números decimais (`f64`). Converta antes:

```rust
let inteiros = [2, 4, 4, 5, 7, 9];
let mut dados: Vec<f64> = Vec::new();
for &v in &inteiros {
    dados.push(v as f64);
}
```

---

## Como rodar

Todos os comandos são digitados no **terminal**, dentro da pasta do projeto (a pasta que tem o arquivo `Cargo.toml`).

**Rodar os testes do módulo:**

```text
cargo test statistics
```

No fim, deve aparecer uma linha como esta (o número de testes pode mudar):

```text
test result: ok. 104 passed; 0 failed; 0 ignored; 0 measured; ...
```

**Ver o módulo funcionando**, com o resumo impresso na tela:

```text
cargo test print_summary_demo -- --nocapture
```

**Ver a documentação** como página web, com a explicação de cada função:

```text
cargo doc --open
```

---

## Como ler este código: o Rust que aparece aqui

Se você está começando no Rust, esta tabela explica cada recurso da linguagem usado no módulo.

| Recurso | O que significa | Exemplo no módulo |
|---|---|---|
| `fn` | declara uma função | `fn mean(data: &[f64]) -> Option<f64>` |
| `&[f64]` | uma lista de números decimais **emprestada**: a função só lê a lista, não pode alterá-la | `data: &[f64]` |
| `-> Option<f64>` | o tipo da resposta que a função devolve | `fn variance(...) -> Option<f64>` |
| `Option`, `Some(valor)`, `None` | uma resposta que pode existir (`Some`) ou não existir (`None`) | `Some(sum / data.len() as f64)` |
| `let` e `let mut` | cria uma variável; com `mut`, ela pode mudar de valor depois | `let mut sum = 0.0;` |
| `for &value in data { ... }` | repete o bloco para cada valor da lista; o `&` pega o próprio número (uma cópia), e não uma referência a ele | `mean.rs` |
| `while condição { ... }` | repete o bloco enquanto a condição for verdadeira | `mode.rs` |
| `sum += value` | o mesmo que `sum = sum + value` | `mean.rs` |
| `?` | "se a resposta for `None`, pare aqui e devolva `None` também" | `let average = mean(data)?;` |
| `match` | escolhe um caminho conforme o valor: `Some(v) => ...` ou `None => ...` | `summary.rs` |
| `as f64`, `as usize` | converte um número de um tipo para outro (inteiro ↔ decimal) | `data.len() as f64` |
| `data.len()` | a quantidade de elementos da lista | `variance.rs` |
| `data[0]` e `&data[1..]` | o primeiro elemento; a lista do segundo elemento em diante | `range.rs` |
| `Vec<f64>`, `Vec::new()`, `.push(x)` | uma lista que pode crescer; criar uma lista vazia; acrescentar `x` no fim | `mode.rs` |
| `pub`, `pub(super)` ou nada | quem pode usar: qualquer parte do projeto; só a pasta `statistics`; só o próprio arquivo | `helpers.rs` |
| `mod` e `pub use` | em `mod.rs`: `mod` declara um arquivo do módulo, e `pub use` deixa a função acessível como `statistics::mean` | `mod.rs` |
| `struct` | uma "ficha": um pacote de valores com nomes | `Quartiles { q1, q2, q3 }` |
| `#[derive(...)]` | pede ao Rust para gerar sozinho habilidades simples: `Debug` (mostrar o conteúdo), `Clone` e `Copy` (copiar), `PartialEq` (comparar com `==`) | `quartiles.rs` |
| `impl Trait for Tipo` | "ensina" uma habilidade a um tipo. Um *trait* é um contrato, como "sei me comparar" (`Ord`) ou "sei me imprimir" (`Display`) | `helpers.rs`, `summary.rs` |
| `self` e `self.0` | o próprio valor; o primeiro campo de um tipo como `Sortable(f64)` | `helpers.rs` |
| `Ordering::Less`, `Greater`, `Equal` | a resposta de uma comparação: menor, maior ou igual | `helpers.rs` |
| `f64::NAN`, `f64::INFINITY` | os valores especiais "não é um número" e infinito | testes |
| `#[cfg(test)] mod tests` e `#[test]` | o bloco de testes, que só é compilado ao rodar `cargo test`; cada `#[test]` marca um teste | fim de cada arquivo |
| `assert_eq!(a, b)` e `assert!(c)` | o teste falha se `a` for diferente de `b`, ou se `c` for falso | testes |
| `37.0_f64` | o sufixo `_f64` diz que o número é um `f64` (necessário para usar `.sqrt()` nele) | testes |
| `//` e `///` | `//` é um comentário comum; `///` é documentação, que aparece na página do `cargo doc` | todos os arquivos |

---

## Como o quantil é calculado

Existem vários métodos para calcular quantis, e eles dão resultados um pouco diferentes. Este módulo usa a **interpolação linear entre vizinhos**:

```text
1. Ordene a lista. As posições são contadas a partir de 0.
2. Calcule a posição procurada: h = (n − 1) × p
3. Separe h em parte inteira (i) e parte decimal (f).
4. Se i é a última posição, o quantil é o valor da posição i.
5. Senão: quantil = valor[i] + f × (valor[i + 1] − valor[i])
```

Exemplo, Q3 (p = 0,75) de `[2, 4, 4, 5, 7, 9]`: h = 5 × 0,75 = 3,75, então i = 3 e f = 0,75. O resultado é 5 + 0,75 × (7 − 5) = **6,5**.

A mediana (p = 0,5), os quartis, o intervalo interquartílico e o resumo de cinco números usam todos esse mesmo método.

---

## Complexidade dos algoritmos

### O que é complexidade

Complexidade responde a uma pergunta: **"se a lista ficar maior, quanto trabalho a mais o computador vai ter?"**

Ela não se mede em segundos, porque os segundos dependem do computador. Ela se mede em **número de operações**, em função de **n**, o tamanho da lista. O teste mais simples é: **dobre o n e veja o que acontece com o trabalho.**

São duas medidas:

- **Tempo:** quantas operações a função faz.
- **Espaço:** quanta memória **extra** a função precisa, além da própria lista que recebeu.

### Grupo 1: tempo O(n) e espaço O(1). As medidas que só "passam" pela lista

**Quais são:** média, variância, desvio padrão, coeficiente de variação, amplitude, covariância e correlação.

**Por que O(n) de tempo.** Essas funções percorrem a lista do começo ao fim um **número fixo** de vezes. A variância, por exemplo: confere os dados (uma passada), calcula a média (outra passada) e soma os desvios ao quadrado (mais uma). Se a lista dobra de tamanho, cada passada dobra, e o trabalho total também dobra. O trabalho cresce **na mesma proporção** de n, e é isso que O(n) quer dizer.

**O número de passadas não muda a classe.** 2 passadas (2n), 3 passadas (3n) ou as várias passadas da correlação continuam O(n). A notação O ignora as constantes que multiplicam n e olha só **como** o trabalho cresce.

**Por que O(1) de espaço.** Essas funções guardam só alguns números: uma soma, uma média, o menor e o maior valor. Com 1.000 ou com 1.000.000 de valores, são as **mesmas poucas variáveis**.

**Não dá para fazer melhor.** Para calcular a média, é preciso olhar cada número **pelo menos uma vez**. Se um único número ficar de fora, a média pode sair errada. Então qualquer algoritmo precisa de pelo menos n operações, e estas funções já estão no melhor possível: são **Θ(n)**.

### Grupo 2: tempo O(n log n) e espaço O(n). As medidas que precisam da lista em ordem

**Quais são:** moda, quantil, mediana, quartis, intervalo interquartílico, resumo de cinco números e resumo completo.

**Por que O(n log n) de tempo.** O passo caro é **ordenar a cópia da lista**. A ordenação é o `merge_sort` da própria biblioteca: ele divide a lista em duas metades, ordena cada metade (chamando a si mesmo) e junta as duas metades ordenadas numa passada. O custo segue a recorrência

```text
T(n) = 2·T(n/2) + n
```

que, pelo Teorema Mestre (caso 2: a = 2, b = 2, f(n) = n = n^(log₂ 2)), dá **Θ(n log n)**. Depois de ordenar, o resto é barato:

- o quantil só calcula uma posição e olha dois vizinhos, um trabalho **constante**, O(1);
- a moda percorre a lista ordenada uma vez, contando os blocos de valores iguais: O(n).

Somando: O(n log n) + O(n) = **O(n log n)**. A parcela que cresce mais rápido é a que manda.

**O que n log n significa na prática:**

| n | n (linear) | n · log₂ n | n² (quadrático) |
|---|---|---|---|
| 1.000 | 1.000 | ≈ 9.966 | 1.000.000 |
| 2.000 | 2.000 | ≈ 21.932 (**×2,2**) | 4.000.000 (**×4**) |
| 1.000.000 | 1.000.000 | ≈ 19.931.569 | 1.000.000.000.000 |
| 2.000.000 | 2.000.000 | ≈ 41.863.137 (**×2,1**) | 4.000.000.000.000 (**×4**) |

Ao dobrar n, o trabalho de n log n fica **um pouco mais que o dobro**. É pior que o linear, mas muito melhor que o quadrático, que **quadruplica**.

**Por que O(n) de espaço.** A cópia ordenada tem o mesmo tamanho da lista. A cópia existe para **não alterar os dados de quem chamou a função**. O preço é uma memória proporcional a n.

### Por que o resumo completo ordena uma vez só

Se o `summary` simplesmente chamasse as funções públicas, a lista seria ordenada **4 vezes**: uma em `mode`, uma em `quartiles`, uma em `iqr` e uma em `five_number_summary`. Na notação O, 4 · n log n continua O(n log n), a mesma classe. **Na prática, porém, a parte da ordenação, que é a mais cara, ficaria cerca de 4 vezes mais lenta.** Por isso o `summary` ordena **uma vez** e reaproveita a lista ordenada com as funções internas `quantile_sorted` (O(1) por consulta) e `mode_sorted` (O(n)).

**Lição:** a notação O diz **como** o custo cresce. Mas as constantes continuam importando na prática.

### Existe algo mais rápido?

- **Mediana e quartis:** o algoritmo **Quickselect** acha a mediana em O(n) **em média**, sem ordenar a lista inteira. Aqui preferimos ordenar: é mais simples de explicar e de conferir, e o resumo completo precisa da lista ordenada de qualquer forma.
- **Moda:** com uma tabela hash, daria para contar as frequências em O(n) em média. Mas, no Rust, um `f64` não pode ser usado diretamente como chave de tabela hash (por causa do `NaN` e do 0,0 / −0,0).

### Tabela-resumo

| Medida | Tempo | Espaço extra | De onde vem o custo |
|---|---|---|---|
| Média | O(n) | O(1) | conferência dos dados + 1 passada somando |
| Variância e desvio padrão | O(n) | O(1) | conferência + média + passada dos desvios |
| Coeficiente de variação | O(n) | O(1) | média + desvio padrão |
| Amplitude | O(n) | O(1) | conferência + 1 passada guardando o mínimo e o máximo |
| Covariância | O(n) | O(1) | conferência + duas médias + passada dos produtos dos desvios |
| Correlação | O(n) | O(1) | covariância + dois desvios padrão + conferência de lista constante |
| Moda | O(n log n) | O(n) | ordenação da cópia + 1 passada contando blocos |
| Quantil e mediana | O(n log n) | O(n) | ordenação da cópia + O(1) para achar a posição |
| Quartis, IQR e resumo de 5 números | O(n log n) | O(n) | **uma** ordenação + 3 ou 5 consultas de O(1) |
| Resumo completo | O(n log n) | O(n) | **uma** ordenação + algumas passadas de O(n) |
