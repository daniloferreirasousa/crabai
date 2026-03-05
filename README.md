# RustOps GUI - Assistente de IA Local

**RustOps GUI** é uma interface gráfica nativa, rápida e independente desenvolvida em Rust para interagir com modelos de linguagem locais via Ollama. O foco do projeto é fornecer uma experiência "plug and play", gerenciando as suas próprias dependências de infraestrutura de forma invisível para o usuário final, operando 100% offline e com alta performance.

### ✨ Funcionalidades Atuais

* **Automação de Infraestrutura (Zero-Touch Setup):**
    * Detecta automaticamente se o Ollama está instalado.
    * Baixa e instala o Ollama silenciosamente caso não seja encontrado.
    * Inicia o serviço em background automaticamente (`ollama serve`).
* **Gestão Dinâmica de Modelos:**
    * Gera e compila um modelo customizado (`rustops`) a partir de um Modelfile embutido no próprio código, focado em engenharia Rust.
* **Interface Gráfica Nativa Assíncrona:**
    * Construída com `eframe`/`egui` para renderização incrivelmente rápida e leve.
    * **Tela de Carregamento (Splash Screen):** O aplicativo abre instantaneamente e exibe o progresso de inicialização do motor de IA em segundo plano, sem travar a tela.
    * Utiliza multithreading e Canais MPSC (Message Passing) para comunicação fluida.
* **Gerenciamento de Conversas e Persistência:**
    * **Múltiplos Chats:** Painel lateral intuitivo para criar, alternar, renomear e excluir diferentes tópicos de conversa.
    * **Persistência Local:** Todo o histórico de mensagens é salvo automaticamente no disco local, permitindo fechar e abrir o aplicativo sem perder dados.
    * **Memória de Contexto:** A IA compreende o fluxo da conversa enviando o histórico completo da sessão ativa.
* **Streaming de Respostas em Tempo Real:**
    * Efeito "máquina de escrever": as palavras aparecem na tela assim que são geradas, eliminando tempos de espera.
* **Distribuição Profissional:**
    * Ocultação do terminal em ambiente de produção.
    * Suporte a geração de instaladores `.deb` (Linux) e executáveis standalone `.exe` (Windows).

### 🗂️ Arquitetura do Projeto

O projeto segue os rígidos princípios de *Separation of Concerns* (Separação de Responsabilidades), com o estado da interface refatorado em submódulos de desenho:

```text
rustops_gui/
├── Cargo.toml          # Gerenciador de dependências e configuração de build/deb
└── src/
    ├── main.rs         # Ponto de entrada enxuto: Carrega o ícone e inicia a janela gráfica.
    ├── app.rs          # Interface (egui): Splash screen, painéis modulares e loop de eventos.
    ├── storage.rs      # Banco de Dados: Gerencia sessões, histórico e persistência no disco.
    ├── ollama.rs       # Rede: Cliente HTTP, streaming assíncrono e parsing de JSON.
    └── utils.rs        # Infraestrutura: Comandos de SO, detecção de ambiente e setup de modelos.
```

### 🚀 Como Executar e Distribuir

**Pré-requisitos de Desenvolvimento:**
* Ter o Rust e o Cargo instalados.
* Conexão com a internet na primeira execução (para baixar o Ollama e os modelos).

**Para Desenvolver (Modo Debug):**
```bash
cargo run
```

**Para Distribuir no Linux (Gerar Instalador .deb):**
*Requer a instalação prévia da ferramenta: `cargo install cargo-deb`*
```bash
cargo deb
```
O arquivo de instalação estará disponível em `target/debian/`. Basta dar dois cliques para instalar no Ubuntu/Mint/Debian.

**Para Distribuir no Windows (Executável Final):**
```bash
cargo build --release
```
O executável otimizado estará em `target/release/rustops-gui.exe`. O terminal de fundo é ocultado automaticamente graças à flag `windows_subsystem`.

### 🗺️ Roadmap (Próximos Passos)

- [x] Streaming de texto em tempo real via Canais MPSC.
- [x] Memória de contexto (Histórico de sessão).
- [x] Persistência de Dados: Salvar conversas no disco local para recuperar históricos.
- [x] Múltiplos Chats: Painel lateral para criar, gerenciar, renomear e excluir conversas.
- [x] Indicadores Visuais de Carga: Tela de carregamento assíncrona que não trava a interface.
- [ ] **Renderização de Markdown:** Implementar um parser rico na interface para exibir formatações (negrito, itálico) e blocos de código com *syntax highlighting* nativo.
- [ ] **Integração de Ícone no .exe (Windows):** Utilizar `build.rs` e `winres` para embutir o ícone no binário do Windows Explorer.