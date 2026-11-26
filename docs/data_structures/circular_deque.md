# Deque circular genérico

Um deque (*double-ended queue*) guarda uma sequência na qual as duas pontas são acessíveis. Em uma fila convencional, quem entra por último espera atrás de quem já estava lá (FIFO). Em uma pilha, a última entrada é a primeira saída (LIFO). Com um deque, o chamador escolhe a disciplina. Por exemplo, inserir no final e remover no início produz uma fila. Inserir e remover no final produz uma pilha.

## Representação e invariantes

`CircularDeque<T>` usa um `Vec<Option<T>>`. Cada posição existe desde a reserva, quer contenha `Some(valor)`, quer esteja livre com `None`. Chamamos de **capacidade** o comprimento desse vetor, ou seja, o número de posições utilizáveis. O alocador pode ter reservado mais bytes. O campo `head` marca o começo e `len` conta os elementos. A posição do último é calculada quando necessária.

- O tamanho satisfaz `0 <= len <= capacidade`. Em particular, um anel cheio pode usar todas as posições.
- Com capacidade zero, `head` e `len` valem zero. Com capacidade positiva, `head` é um índice válido, inclusive quando o deque está vazio.
- A partir de `head`, as primeiras `len` posições do percurso circular contêm valores na ordem lógica. Todas as demais contêm `None`. Esvaziar o deque permite reutilizar as posições a partir do índice em que o percurso terminou.

Para capacidade positiva, o elemento lógico de deslocamento `i` fica em `(head + i) % capacidade`. O código evita uma soma que poderia transbordar. Calcula quantas posições faltam até o fim, `restantes = capacidade - head`. Se `i < restantes`, o índice é `head + i`. Caso contrário, é `i - restantes`. Os deslocamentos usados chegam no máximo à capacidade, de modo que basta uma volta.

## Operações nas pontas

| Operação | Efeito |
| --- | --- |
| `new()`, `default()` | Começam vazios com capacidade zero. |
| `with_capacity(c)` | Inicializa `c` posições livres. Aceita zero. |
| `push_front(x)` | Garante uma posição livre, recua `head` (de zero para a última posição se necessário), grava `x` e incrementa `len`. |
| `push_back(x)` | Garante espaço e grava na posição de deslocamento `len`, logo após o último elemento. |
| `pop_front()` | Retira o valor de `head`, avança o início circularmente e diminui o tamanho. |
| `pop_back()` | Retira o valor de deslocamento `len - 1` e diminui o tamanho. |
| `front()`, `back()` | Emprestam referências às extremidades. |
| `len()`, `is_empty()`, `capacity()` | Consultam tamanho, condição de vazio e capacidade utilizável. |
| `clear()` | Remove e descarta os elementos a partir do início e mantém a capacidade para o próximo uso. |

Remoções em um deque vazio retornam `None`. Quando há um valor, `Option::take` deixa a posição livre e transfere a propriedade para quem chamou. As consultas retornam `Option<&T>`. O deque pode armazenar valores que têm destrutor e tipos sem `Copy` ou `Clone`.

## Exemplo de volta e crescimento

Comece com quatro posições e insira A, B, C e D pelo final. O ponto indica uma posição livre. Os índices físicos crescem da esquerda para a direita.

| Após a operação | Posições físicas a partir de 0 | head | Ordem de remoção pelo início |
| --- | --- | ---: | --- |
| Inserir A, B, C, D | A B C D | 0 | A B C D |
| Remover A e B pelo início | · · C D | 2 | C D |
| Inserir E e F pelo final | E F C D | 2 | C D E F |
| Inserir G pelo final | C D E F G · · · | 0 | C D E F G |
| Inserir X pelo início | C D E F G · · X | 7 | X C D E F G |

E e F reutilizam as posições 0 e 1. Quando chega G, o anel está cheio. A implementação prepara oito posições livres e visita os índices antigos 2, 3, 0 e 1. Move C, D, E e F, nessa ordem, para as posições novas 0 a 3. G entra na posição 4. Na operação seguinte, a inserção pelo início recua o índice de 0 para 7.

## Custos e limites

Sejam `n` o número de elementos e `c` a capacidade. A tabela considera custo constante para mover um valor de um tipo fixo. Some o custo dos destrutores de `T` quando houver descarte.

| Operação | Tempo | Motivo |
| --- | --- | --- |
| `new()`, `default()` | O(1) | Adiam a alocação até a primeira inserção. |
| `with_capacity(c)` | O(c) para c positivo, O(1) para zero | Inicializa cada posição com `None`. |
| Inserir em qualquer ponta | O(1) amortizado, O(n) no pior caso para n positivo | Uma inserção em anel cheio inicializa o novo vetor e move os elementos existentes. |
| Remover, consultar pontas, tamanho, vazio ou capacidade | O(1) | Acessa um número constante de posições ou campos. |
| `clear()` | O(n) para n positivo, O(1) quando vazio | Visita somente os elementos presentes. |
| Crescer | O(n) para n positivo, O(1) de zero para um | O anel cresce somente quando cheio. |

A capacidade passa de zero para um e depois dobra sempre que falta espaço. Uma inserção individual pode pagar toda a mudança. Em uma sequência, os tamanhos realocados formam uma progressão geométrica cuja soma é proporcional ao número de inserções, dando custo amortizado constante por inserção. Uma reserva inicial explícita tem seu próprio custo.

O armazenamento é O(c), além dos metadados de tamanho fixo. Durante o crescimento, os dois vetores coexistem e o pico continua O(c) para c positivo. A estrutura conserva o espaço depois de remoções e de `clear()`. Quando sai de escopo, o vetor libera o armazenamento e descarta os valores que ainda possui.

Capacidades são do tipo `usize`. A duplicação usa `checked_mul`. O construtor usa `try_reserve_exact` antes de inicializar as posições. Overflow, tamanho incompatível com `Vec` ou falha de reserva comunicada pelo alocador causam pânico. Durante o crescimento, a reserva acontece antes de mover o primeiro elemento. A API usa operações infalíveis na assinatura e dispensa `unsafe`, dependências externas e clonagem de valores.

## Uso no projeto

O projeto declara os módulos em `src/main.rs`. Este exemplo pode ser usado no corpo de uma função desse crate binário.

```rust
use crate::data_structures::circular_deque::CircularDeque;

let mut tarefas = CircularDeque::with_capacity(2);
tarefas.push_back(String::from("revisar"));
tarefas.push_front(String::from("compilar"));
tarefas.push_back(String::from("entregar"));

assert_eq!(tarefas.front().map(String::as_str), Some("compilar"));
assert_eq!(tarefas.pop_back().as_deref(), Some("entregar"));
assert_eq!(tarefas.pop_front().as_deref(), Some("compilar"));
assert_eq!(tarefas.back().map(String::as_str), Some("revisar"));
```

## Referências

- Pat Morin. *Open Data Structures*, seção 2.4, “ArrayDeque”. Representação em vetor circular e custo de redimensionamento. <https://opendatastructures.org/ods-java/2_4_ArrayDeque_Fast_Deque_O.html>
- Rust Project. Documentação de `Vec::try_reserve_exact`, incluindo limites e falhas de reserva. <https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact>
- Rust Project. Documentação de `Option::take` e transferência do valor armazenado. <https://doc.rust-lang.org/std/option/enum.Option.html#method.take>
