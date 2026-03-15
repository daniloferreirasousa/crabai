use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use egui_commonmark::{CommonMarkCache};
use sysinfo::System;
use log::{info, warn, error};

use crate::storage::{AppDatabase};
use crate::utils;
use crate::ui;
use crate::errors::CrabAIError;

pub struct CrabAIApp {
    pub user_input: String,
    pub db: AppDatabase,
    
    // Controle de edição do título
    pub editando_titulo: bool,
    pub novo_titulo_temp: String,

    // Variáveis para a tela de carregamento
    pub is_initialized: bool,
    pub startup_receiver: Option<Receiver<String>>,
    pub startup_status_text: String,

    // Variaveis para o atualizador:
    pub receptor_update: Option<Receiver<String>>,
    pub versao_disponivel: Option<String>,

    // Contrle da janela de apoio
    pub mostrar_janela_apoio: bool,
    pub markdown_cache: CommonMarkCache,
    
    // Contrrole de envio para o Input Multiline
    pub requisitou_envio: bool,

    // Monitor de Hardware
    pub sys: System,
    pub cpu_usage: f32,
    pub ram_usage: f32, // em GB

    // Para exibir a mensagem do CrabAIError
    pub erro_fatal: Option<String>,
}

// =========================================================
// INICIALIZAÇÃO E THREAD DE CARREGAMENTO
// =========================================================
impl CrabAIApp {
    pub fn new() -> Self {
        let (tx, rx) = channel::<String>();
        let (tx_update, rx_update) = channel::<String>();

        thread::spawn(move || {
            // Closure imediata para facilitar o tratamento de erros com '?'
            let setup_result = (|| -> Result<(), CrabAIError> {
                
                let _ = tx.send("Verificando motod de IA...".to_string());
                if !utils::is_ollama_installed() {
                    let _ = tx.send("Instalando Ollama...".to_string());
                    utils::instalar_ollama()?;
                }

                let _ = tx.send("Iniciando serviço...".to_string());
                if !utils::ollama_is_running() {
                    utils::start_ollama_serve();
                    if !utils::wait_for_ollama_ready(60) {
                        return Err(CrabAIError::Generic("O motor de IA não respondeu a tempo.".to_string()).log_error());
                    }
                }

                // Nova chamada que utiliza o Result e o tx para status.
                utils::setup_custom_model(&tx)?;
                Ok(())
            })();

            if let Err(e) = setup_result {
                error!("Falha crítica durante a inicialização do motor: {}", e);
                let _ = tx.send(format!("ERRO_FATAL: {}", e));
            } else {
                info!("Motor de IA inicializado com sucesso!");
                let _ = tx.send("CONCLUIDO".to_string());
            }
        });

        // Thread rodando em segundo plano (Verificador do GitHub)
        let versao_atual = env!("CARGO_PKG_VERSION").to_string();
        thread::spawn(move ||{
            let url = "https://api.github.com/repos/daniloferreirasousa/rustops-gui/releases/latest";

            let client = reqwest::blocking::Client::builder()
                .user_agent("CrabAI-App")
                .build()
                .unwrap();

            if let Ok(resposta) = client.get(url).send() {
                if let Ok(json) = resposta.json::<serde_json::Value>() {
                    if let Some(tag) = json["tag_name"].as_str() {
                        let tag_limpa = tag.trim_start_matches('v');
                        if tag_limpa != versao_atual {
                            let _ = tx_update.send(tag.to_string());
                        }
                    }
                } else {
                    warn!("Não foi possíve conectar à API do GitHub para buscar atualizações.");
                }
            } else {
                warn!("Não foi possível conectar à API do GitHub para buscar atualizações.");
            }
        });

        let mut sys = System::new_all();
        sys.refresh_all(); // Faz uma leitura inicial

        Self {
            user_input: String::new(),
            db: AppDatabase::carregar(),
            editando_titulo: false,
            novo_titulo_temp: String::new(),
            is_initialized: false,
            startup_receiver: Some(rx),
            startup_status_text: "Iniciando CrabAI...".to_string(),
            receptor_update: Some(rx_update),
            versao_disponivel: None,
            mostrar_janela_apoio: false,
            markdown_cache: CommonMarkCache::default(),
            requisitou_envio: false,
            sys,
            cpu_usage: 0.0,
            ram_usage: 0.0,
            erro_fatal: None,
        }
    }
}


// =========================================================
// O LOOP PRINCIPAL DA INTERFACE (Módulo eframe)
// =========================================================
impl eframe::App for CrabAIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // 0.0. APLICANDO O TEMA CrabAI
        let mut style = (*ctx.style()).clone();

        let laranja_crab = egui::Color32::from_rgb(255,87,34);

        style.visuals.selection.bg_fill = laranja_crab;
        style.visuals.hyperlink_color = laranja_crab;

        ctx.set_style(style);

        // 0. DADOS DO PC
        let (cpu, ram) = crate::system_stats::obter_dados_hardware(&mut self.sys);
        self.cpu_usage = cpu;
        self.ram_usage = ram;

        // 1. TELAS DE BLOQUEIO (Loading e Termos)
        if ui::splash::desenhar_tela_carregamento(self, ctx) { return; }
        if ui::terms::termos_de_uso(self, ctx) { return; }

        // 1.5 ALERTAS
        ui::update_alert::desenhar_alerta_atualizacao(self, ctx);

        // 2. PROCESSAMENTO EM SEGUNDO PLANO
        ui::messages::processar_mensagens_ia(self, ctx);

        // 3. DESENHO DOS PAINÉIS
        ui::sidebar::desenhar_painel_lateral(self, ctx);
        ui::footer::desenhar_rodape(self, ctx);
        ui::chat::desenhar_painel_central(self, ctx);

        // 4. JANELAS FLUTUANTES (Modais)
        ui::donations::desenhar_janela_apoio(self, ctx);
        ui::modals::exibir_erros_criticos(self, ctx);
    }
}