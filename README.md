# Estruturas de Dados II

Biblioteca de estruturas de dados e algoritmos que será desenvolvida de forma **colaborativa em Rust** por alunos da **Universidade Federal do Pará (UFPA)**, integrando duas disciplinas:

| Nível | Disciplina |
| --- | --- |
| Graduação | Estruturas de Dados |
| Mestrado | Projeto e Análise de Algoritmos |

O projeto reúne as contribuições das duas turmas na construção de uma biblioteca compartilhada, aproximando a implementação de estruturas de dados do projeto e da análise de algoritmos. Seu desenvolvimento será parte das atividades das disciplinas, com foco no aprendizado, na colaboração e na produção de código documentado e reutilizável.

## Objetivos

- Desenvolver uma biblioteca em Rust com estruturas de dados e algoritmos.
- Analisar a eficiência das soluções, considerando tempo de execução e uso de memória.
- Relacionar a análise teórica ao comportamento das implementações.
- Promover a colaboração entre alunos de graduação e mestrado.
- Registrar exemplos, testes e documentação que apoiem o estudo e o uso da biblioteca.

## Estado do projeto

O repositório está em fase de organização inicial e ainda não contém implementações nem um arquivo `Cargo.toml`. A estrutura da biblioteca e as orientações de contribuição serão adicionadas ao longo do desenvolvimento.

## Preparação do ambiente: instalação do Rust e do Cargo

**Antes de começar as atividades, instale o Rust e confirme que o Cargo está funcionando.** Esta seção apresenta o passo a passo para preparar o ambiente.

### Entenda as ferramentas

| Ferramenta | Função |
| --- | --- |
| **Rust** | Linguagem utilizada no desenvolvimento da biblioteca. |
| **`rustc`** | Compilador que transforma o código Rust em código executável ou bibliotecas. |
| **Cargo** | Ferramenta para gerenciar dependências, compilar projetos, executar testes e gerar documentação. |
| **`rustup`** | Instalador e gerenciador das versões e ferramentas do Rust. |

> **O Cargo é instalado junto com o Rust ao usar o `rustup`. Não é necessário instalar o Cargo separadamente.** Utilize a instalação padrão com o canal estável (`stable`), conforme a [documentação oficial de instalação do Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### 1. Instalação no Windows

1. Acesse a [página oficial de instalação do Rust](https://rust-lang.org/tools/install/) e baixe o `rustup-init.exe` correspondente à arquitetura do computador, normalmente **x64**.
2. Execute o instalador. Caso ele solicite os pré-requisitos do Visual Studio, siga a opção de instalação oferecida.
3. Se precisar configurar esses componentes manualmente no Visual Studio Installer, selecione **Desenvolvimento para desktop com C++** (*Desktop development with C++*), incluindo as ferramentas **MSVC** e o **Windows SDK**. Se já estiverem instalados, não é necessário reinstalá-los.
4. Continue a instalação do Rust e aceite as opções padrão, com o canal `stable` e a cadeia de ferramentas MSVC.
5. Ao terminar, feche e abra novamente o PowerShell ou o Prompt de Comando antes de verificar a instalação.

As ferramentas C++ fornecem componentes de ligação e bibliotecas do sistema necessários para compilar Rust no Windows. Consulte os [pré-requisitos oficiais para Windows/MSVC](https://rust-lang.github.io/rustup/installation/windows-msvc.html) e o [guia da Microsoft para configurar Rust no Windows](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).

### 2. Instalação no Linux

Em **Ubuntu e Debian**, prepare as ferramentas de compilação e o `curl`:

```bash
sudo apt update
sudo apt install build-essential curl
```

Em outras distribuições, instale os pacotes equivalentes usando o gerenciador do sistema. O ambiente precisa de um compilador C, como GCC ou Clang, e de um linker, responsável por combinar os arquivos compilados.

Em seguida, instale o Rust pelo `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Escolha a instalação padrão. Ao concluir, abra um novo terminal. Em Bash ou Zsh, também é possível carregar o ambiente na sessão atual:

```bash
source "$HOME/.cargo/env"
```

Se você utiliza **WSL**, siga estas instruções dentro do terminal Linux da distribuição. Consulte o [guia oficial de instalação do Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

### 3. Instalação no macOS

Instale as ferramentas de linha de comando do Xcode, caso ainda não estejam disponíveis:

```bash
xcode-select --install
```

Depois, instale o Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Aceite a instalação padrão e abra um novo terminal ao finalizar. Em Bash ou Zsh, o comando `source "$HOME/.cargo/env"` também carrega o ambiente na sessão atual. Esse procedimento segue o [guia oficial para Linux e macOS](https://doc.rust-lang.org/book/ch01-01-installation.html).

### 4. Confirme a instalação

Em qualquer um dos sistemas, execute:

```text
rustc --version
cargo --version
rustup --version
```

Cada comando deve exibir a versão da ferramenta correspondente. Os números podem variar conforme a data de instalação.

Se aparecer uma mensagem como **comando não encontrado** ou **comando não reconhecido**, reabra o terminal e, se estiver usando um terminal integrado, reinicie o editor. Se o problema persistir, confira se a pasta das ferramentas está no `PATH`:

| Sistema | Pasta padrão |
| --- | --- |
| Windows | `%USERPROFILE%\.cargo\bin` |
| Linux e macOS | `~/.cargo/bin` |

Veja como o instalador configura esses caminhos na [documentação do rustup](https://rust-lang.github.io/rustup/installation/).

### 5. Teste a compilação com o Cargo

Para confirmar que o ambiente consegue compilar e executar um programa, abra um terminal em uma **pasta de estudos fora deste repositório** e crie um pequeno projeto de teste:

```text
cargo new verificacao_rust --bin
cd verificacao_rust
cargo run
```

O Cargo criará um programa de exemplo, fará a compilação e o executará. Ao final, deverá aparecer:

```text
Hello, world!
```

Esse exemplo serve apenas para verificar o ambiente. Se houver erro de linker, como `link.exe` ausente no Windows, revise as ferramentas de compilação da etapa do seu sistema. Saiba mais nos [primeiros passos com o Cargo](https://doc.rust-lang.org/cargo/getting-started/first-steps.html).

### 6. Atualize as ferramentas quando necessário

Para atualizar as versões instaladas do Rust e do Cargo pelo `rustup`, execute:

```text
rustup update
```

O Cargo acompanha a atualização do Rust. Consulte a seção sobre gerenciamento de versões na [página oficial de instalação](https://rust-lang.org/tools/install/).

## Estrutura atual

```text
EstruturasDeDados2/
├── .gitignore    # Regras para ignorar arquivos gerados e temporários
├── LICENSE       # Licença Apache 2.0
└── README.md     # Apresentação e orientações do projeto
```

## Como obter o projeto

Com o Git instalado, execute no terminal:

```bash
git clone https://github.com/Disciplinas-gustavoresque-UFPA/EstruturasDeDados2.git
cd EstruturasDeDados2
```

**Neste estágio, ainda não é possível compilar esta biblioteca**, pois o arquivo `Cargo.toml` e os códigos-fonte ainda serão adicionados. O teste com `verificacao_rust` descrito acima pode ser realizado independentemente disso.

## Materiais para começar

- [The Rust Programming Language](https://doc.rust-lang.org/book/): livro oficial para aprender a linguagem.
- [The Cargo Book](https://doc.rust-lang.org/cargo/): documentação do gerenciador de projetos e dependências.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): exemplos práticos da linguagem.

## Licença

Este projeto está licenciado sob a **Apache License 2.0**. Consulte o arquivo [LICENSE](LICENSE) para mais detalhes.
