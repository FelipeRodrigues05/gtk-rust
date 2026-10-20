# gtk-rust

Projeto simples de interface gráfica em Rust usando GTK4, com gravação e transcrição de áudio usando Whisper.

## Gravação de áudio

Ao iniciar, a aplicação grava áudio por 60 segundos usando o dispositivo de entrada padrão do sistema. A gravação acontece antes de abrir a janela, sem precisar clicar em um botão.

O áudio é salvo em `recording.wav`, no diretório de trabalho atual, sobrescrevendo o arquivo caso ele já exista. O arquivo usa o formato WAV com amostras inteiras de 16 bits, na frequência de amostragem e no número de canais da configuração padrão do dispositivo. A captura usa `cpal` e a escrita do arquivo usa `hound`.

É necessário ter um microfone ou outro dispositivo de entrada disponível. Erros retornados pela gravação ou transcrição são exibidos no terminal; a janela ainda é aberta. Erros do fluxo de áudio também são exibidos no terminal. A captura aceita os formatos de entrada `i16`, `u16` e `f32`, convertendo as amostras para inteiros de 16 bits.

## Transcrição de áudio

Após finalizar `recording.wav`, a aplicação chama a função assíncrona `transcribe(filename: &str)` usando um runtime Tokio. A função usa `simple_transcribe_rs` com o modelo Whisper `tiny` e retorna o texto como `Result<String, Box<dyn std::error::Error>>`. O texto é exibido no terminal, sem ser salvo em um arquivo ou exibido na interface gráfica. A janela abre após a gravação e a transcrição.

Na primeira execução, `simple_transcribe_rs` tenta baixar o modelo do Hugging Face para `models/ggml-tiny.bin`, relativo ao diretório de trabalho atual. É necessário acesso à internet e permissão de escrita nesse diretório. Nas próximas execuções, o arquivo local é reutilizado.

Antes de iniciar a transcrição, a função verifica se o caminho do modelo corresponde a um arquivo não vazio. Caso contrário, retorna um erro indicando o caminho e orientando a verificar a conexão e a permissão de escrita em `models/`. Erros da transcrição também são propagados para `main`, que os exibe no terminal e continua abrindo a janela.

## Interface gráfica

A aplicação abre uma janela com um botão. Ao clicar nele, o texto muda de `Sample Label` para `Hello` e a aplicação usa `std::fs::write` para gravar `Hello World` no arquivo `foo.txt`, no diretório de trabalho atual.

O arquivo é criado se não existir e seu conteúdo é sobrescrito a cada clique. Se a gravação falhar, uma mensagem de erro é exibida no terminal.

## Requisitos

- Rust e Cargo com suporte à edição 2024.
- Bibliotecas de desenvolvimento do GTK4 instaladas no sistema.
- CMake e compiladores C/C++ para compilar o Whisper. No Arch Linux: `sudo pacman -S --needed cmake base-devel`.
- Dispositivo de entrada de áudio configurado como padrão e acesso permitido ao microfone.
- Ambiente gráfico para exibir a janela.

O projeto usa os bindings incluídos em `whisper-rs-sys`, configurados em `.cargo/config.toml`, dispensando a geração com `libclang`.

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
