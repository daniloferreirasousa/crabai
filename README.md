# RustOps GUI - Ferramenta Educacional de Red Team com IA Local

**RustOps GUI** é uma interface gráfica nativa, rápida e independente desenvolvida em Rust para interagir com modelos de linguagem locais via Ollama. O foco do projeto é fornecer uma experiência "plug and play", gerenciando as suas próprias dependências de infraestrutura de forma invisível para o usuário final, operando 100% offline e com alta performance. Esta ferramenta foi desenhada estritamente para fins educacionais e de pesquisa em Segurança da Informação.

### 🛡️ Por que escolher o RustOps?

* **Privacidade Absoluta (100% Offline):** Seus dados, seus prompts e seus relatórios nunca saem da sua máquina. Não há telemetria, não há assinaturas mensais e nenhum dado é enviado para a nuvem.
* **Ambiente Livre de Filtros Corporativos:** IAs comerciais frequentemente bloqueiam prompts legítimos de cibersegurança e análise de código. O RustOps utiliza um modelo local focado em engenharia, garantindo que suas pesquisas teóricas de *Red Teaming* não sejam interrompidas.
* **Zero Configuração:** Esqueça tutoriais complexos envolvendo Python ou Docker. O aplicativo é independente e gerencia sua própria infraestrutura em background com apenas um clique.

### 🚀 Novidades na v0.1.5
* **Modularização da UI:** Descentralização da lógica de interface para módulos específicos, melhorando a manutenibilidade.
* **Monitor de Hardware:** Widget no rodapé exibindo consumo de CPU e RAM em tempo real via `sysinfo`.
* **Isolamento de Estado:** Gerenciamento de estado por sessão, com canais de comunicação dedicados e isolados.
* **Desacoplamento de Lógica:** Banco de dados refatorado para o padrão `impl AppDatabase` no `storage.rs`.
* **Input Multiline:** Campo de entrada otimizado para múltiplas linhas com foco ajustado.
* **Donation 2.0:** Botão de apoio ao projeto reposicionado estrategicamente no rodapé.

### ✨ Funcionalidades Atuais

* **Monitoramento:** Exibição de recursos de hardware em tempo real.
* **Gerenciamento de Sessões:** Criação, alternância e persistência local de conversas.
* **Streaming de IA:** Efeito "máquina de escrever" e renderização de Markdown.
* **Automação:** Instalação e gestão do serviço Ollama transparente (Zero-Touch).
* **Interface:** Construída com `eframe`/`egui` para renderização nativa leve.

### 🗂️ Arquitetura do Projeto

O projeto segue os rígidos princípios de *Separation of Concerns*:

```Text
rustops_gui/
├── Cargo.toml          # Gerenciamento de dependências
└── src/
    ├── main.rs         # Entry point: Gerencia a janela e o loop do egui.
    ├── app.rs          # Estado central e lógica principal da aplicação.
    ├── storage.rs      # Persistência de dados e gestão de sessões.
    ├── ollama.rs       # Cliente de comunicação com a API da IA.
    ├── utils.rs        # Utilitários de sistema e automação.
    ├── system_stats.rs # Lógica de monitoramento de hardware.
    └── ui/             # Módulos de interface (componentes e widgets):
        ├── mod.rs      # Exportação dos módulos de UI.
        ├── chat.rs     # Renderização da área de chat.
        ├── sidebar.rs  # Painel de gerenciamento de conversas.
        ├── footer.rs   # Rodapé (status, doações e controles).
        ├── splash.rs   # Tela de carregamento inicial.
        ├── terms.rs    # Exibição dos termos de uso.
        ├── settings.rs # Configurações da aplicação.
        ├── messages.rs # Lógica de renderização de mensagens.
        ├── donations.rs# Lógica da tela de apoio ao projeto.
        └── update_alert.rs # Notificações de atualização.
```
### 🚀 Como Executar e Distribuir

#### Para Desenvolver (Modo Debug):

```Bash
cargo run
```
#### Para Distribuir no Linux (Gerar Instalador .deb):
```Bash
cargo deb
```
Para Distribuir no Windows (Cross-Compile):
```Bash
cargo build --target x86_64-pc-windows-gnu --release
```
### 🗺️ Roadmap (Evolução do Projeto)
- [x] Streaming de texto em tempo real (Canais MPSC).

- [x] Memória de contexto (Histórico de sessão).

- [x] Persistência de Dados (Salvar conversas localmente).

- [x] Múltiplos Chats: Gerenciamento no painel lateral.

- [x] Automação Multiplataforma (Zero-Touch Setup).

- [x] Verificador de Atualizações via GitHub API.

- [x] Renderização de Markdown com syntax highlighting.

- [x] Migração para Dolphin 3.0 Llama 3.1 (8B) para maior precisão lógica.

- [x] **Modularização da UI:** Descentralização do `app.rs` em módulos específicos.

- [x] **Monitor de Hardware:** Widget no rodapé exibindo consumo de CPU/RAM em tempo real (via `sysinfo`).

- [x] **Isolamento de Estado:** Mover `is_processing` para o struct de cada sessão e canal de comunicação por sessão.

- [x] **Desacoplamento de Lógica:** Mover funções de banco de dados para dentro do `impl AppDatabase` no `storage.rs`.

- [x] **Input Multiline:** Trocar o `TextEdit::singleline` pelo `TextEdit::multiline` no rodapé (se necessário ajustar responsividade).

- [x] **Donation 2.0 (Apoie o Projeto):** Migrar o botão de doações para um local estratégico e visível (colocado no footer ao lado dos status).

### 📄 Licença
Este projeto está licenciado sob a GNU General Public License v3.0 (GPLv3). Software livre e de código aberto. Consulte o arquivo [LICENSE] para detalhes.

### 🖥️ Referência Técnica para Requisitos

| Componente | Mínimo | Recomendado |
| :--- | :--- | :--- |
| **RAM** | 8GB | 16GB+ |
| **GPU** | Integrada | Dedicada (NVIDIA 4GB+ VRAM) |
| **Espaço** | 10GB | 20GB (SSD) |

## ☕ Apoie o Projeto
Se curtiu o projeto, ele é gratuito e open-source! Considere me pagar um café apontando a câmera do seu celular para o QR Code abaixo ou usando a chave PIX:

<img src="assets/pix.png" alt="QR Code PIX" width="200">

**Chave PIX (Copia e Cola)**: 
00020126580014BR.GOV.BCB.PIX013693cc5dfd-0c3a-4e80-b087-4ac00a96b62e5204000053039865802BR5925DANILO DE ANDRADE FERREIR6007RESENDE62070503***63048F81

---

Desenvolvido por Danilo Ferreira Sousa.