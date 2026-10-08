# Manual de Contribuição e Governança

Este documento estabelece os padrões de engenharia de software para o repositório da disciplina de Projeto e Análise de Algoritmos (UFPA). Para garantir a integridade da base de código e a precisão das análises de benchmark, todas as submissões devem rigorosamente seguir as diretrizes abaixo.

## 1. Padrão de Nomenclatura de Branches
A branch `main` é protegida. O desenvolvimento deve ocorrer em branches isoladas utilizando o seguinte padrão de nomenclatura:
* **`feat/`**: Para a implementação de novos algoritmos ou estruturas de dados (ex: `feat/quick-sort-aleatorio`).
* **`fix/`**: Para correção de bugs, falhas de lógica ou *memory leaks* em implementações existentes (ex: `fix/avl-tree-rotations`).
* **`test/`**: Para inclusão de testes unitários ou scripts de benchmark do *criterion*.
* **`docs/`**: Para inclusão ou alteração exclusiva de documentação.

## 2. Qualidade e Formatação de Código (Rust)
Todo o código submetido será avaliado automaticamente pelo pipeline de CI/CD do repositório. Antes de abrir um Pull Request (PR), você deve obrigatoriamente executar os seguintes comandos locais:
* `cargo fmt`: O código deve estar formatado no padrão oficial do ecossistema Rust.
* `cargo clippy`: O linter não deve apontar *warnings* (avisos) ou vulnerabilidades de segurança de memória.
* `cargo test`: Todos os testes unitários do seu algoritmo devem compilar e passar sem falhas.

## 3. Diretrizes para Pull Requests
O processo de integração de código (*merge*) na `main` é auditado e gerenciado pelos alunos da pós-graduação. Para que seu PR seja avaliado e aprovado:
1. **Cobertura de Testes:** A estrutura ou algoritmo deve possuir testes unitários demonstrando o seu funcionamento no melhor caso e no pior caso.
2. **Resolução de Conflitos:** O PR deve estar atualizado (*rebased*) com a branch `main` e completamente livre de conflitos.
3. **Revisão de Código (Code Review):** O PR será bloqueado até receber a aprovação formal de um revisor líder (pós-graduação), que auditará a complexidade assintótica de tempo/espaço implementada e as boas práticas da linguagem.