# RustOps GUI - Ferramenta Educacional de Red Team com IA Local

**RustOps GUI** é uma interface gráfica nativa, rápida e independente desenvolvida em Rust para interagir com modelos de linguagem locais via Ollama. O foco do projeto é fornecer uma experiência "plug and play", gerenciando as suas próprias dependências de infraestrutura de forma invisível para o usuário final, operando 100% offline e com alta performance. Esta ferramenta foi desenhada estritamente para fins educacionais e de pesquisa em Segurança da Informação.

### 🛡️ Por que escolher o RustOps?

* **Privacidade Absoluta (100% Offline):** Seus dados, seus prompts e seus relatórios nunca saem da sua máquina. Não há telemetria, não há assinaturas mensais e nenhum dado é enviado para a nuvem.
* **Ambiente Livre de Filtros Corporativos:** IAs comerciais frequentemente bloqueiam prompts legítimos de cibersegurança e análise de código. O RustOps utiliza um modelo local focado em engenharia, garantindo que suas pesquisas teóricas de *Red Teaming* não sejam interrompidas.
* **Zero Configuração:** Esqueça tutoriais complexos envolvendo Python ou Docker. O aplicativo é independente e gerencia sua própria infraestrutura em background com apenas um clique.

### ✨ Funcionalidades Atuais

* **Automação Zero-Touch:** Instalação e gestão do serviço Ollama totalmente transparentes (Idempotente), com fallback de rede e verificação de requisitos (espaço em disco, permissões).
* **Tratamento de Erros Resiliente:** Modais visuais amigáveis que guiam o usuário em caso de falhas críticas de ambiente.
* **Monitoramento Integrado:** Exibição de recursos de hardware (CPU/RAM) em tempo real no rodapé.
* **Gerenciamento de Sessões:** Criação, alternância e persistência local de múltiplas conversas com banco de dados desacoplado.
* **Streaming e Markdown:** Efeito "máquina de escrever" em tempo real, suporte total a Markdown com *syntax highlighting* e renderização nativa de Emojis.
* **Atualizador Automático:** Verificação inteligente de novas versões via GitHub API, notificando o usuário diretamente na interface.

### 🗂️ Arquitetura do Projeto

O projeto segue rígidos princípios de *Separation of Concerns* e modularização:

```text
rustops_gui/
├── Cargo.toml          # Gerenciamento de dependências
├── assets/             # Recursos estáticos (Fontes, Imagens)
└── src/
    ├── main.rs         # Entry point: Gerencia a janela, loop do egui e fontes.
    ├── app.rs          # Estado central e lógica principal da aplicação.
    ├── storage.rs      # Persistência de dados e gestão de sessões.
    ├── errors.rs       # Sistema centralizado de tratamento de erros customizados.
    ├── utils.rs        # Utilitários de sistema, hardware checks e automação.
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
        ├── modals.rs   # Janelas flutuantes e modais de erro crítico.
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
### ✅ Concluído (Core & Estabilidade)

- [x] Automação Multiplataforma (Zero-Touch Setup) e Resiliência de Instalação.

- [x] Persistência de Dados (Histórico e Sessões Múltiplas).

- [x] Streaming de texto em tempo real (MPSC) e Renderização Markdown.

- [x] Tratamento de Erros Centralizado (RustOpsError) com Modais de UI.

- [x] Atualizador Automático via GitHub API.

- [x] Suporte a Emojis (NotoEmoji fallback).

- [x] Monitor de Hardware Integrado (CPU/RAM).

- [x] Migração para Dolphin 3.0 / Llama 3.1 (8B).

### 🔜 Próximos Passos (Backlog)

- [ ] **Persistência de Logs**: Gravar erros críticos em arquivo de log local para auditoria e suporte.

- [ ] **Seletor de Modelos**: Interface para escolha de diferentes modelos do Ollama (ex: llama3, mistral).

- [ ] **Internacionalização (i18n)**: Sistema de tradução dinâmica para suporte a múltiplos idiomas.

- [ ] **Exportação de Relatórios**: Gerar arquivos .md formatados a partir do histórico de conversas.

- [ ] **Busca Global**: Sistema de indexação simples para busca de texto em todas as sessões gravadas.

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