# gtk-rust

Projeto simples de interface gráfica em Rust usando GTK4.

A aplicação abre uma janela com um botão. Ao clicar nele, o texto muda de `Sample Label` para `Hello`.

## Requisitos

- Rust e Cargo com suporte à edição 2024.
- Bibliotecas de desenvolvimento do GTK4 instaladas no sistema.
- Ambiente gráfico para exibir a janela.

## Como executar

Na pasta do projeto, execute:

```sh
cargo run
```

## Como compilar

```sh
cargo build --release
```

O executável será gerado em `target/release/gtk-rust`.
