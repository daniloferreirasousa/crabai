#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/* * Projeto: CrabAI GUI
 * Autor: Danilo Ferreira Sousa
 * Descrição: Interface gráfica independente para rodar e interagir com modelos locais.
 */

mod app;
mod ollama;
mod storage;
mod utils;
pub mod ui;
mod system_stats;
mod errors;

use app::CrabAIApp;
use eframe::egui;
use simplelog::*;
use std::fs::File;

// Função para carregar os pixels da imagem durante a compilação
fn carregar_icone() -> egui::IconData {
    let image_bytes = include_bytes!("../icone.png");
    let image = image::load_from_memory(image_bytes)
        .expect("Falha ao carregar o ícone. Verifique se icone.png está na raiz do projeto.")
        .into_rgba8();
    
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    
    egui::IconData {
        rgba,
        width,
        height,
    }
}


fn iniciar_logger() {
    let log_file = File::create("crabai.log").expect("Falha ao criar arquivo de log.");


    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto
        ),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            log_file
        ),
    ]).unwrap();
}

fn main() -> eframe::Result<()> {
    iniciar_logger();
    
    println!("=== INICIANDO INTERFACE GRÁFICA ===");

    let mut options = eframe::NativeOptions::default();

    options.viewport = egui::ViewportBuilder::default()
        .with_icon(carregar_icone());

    eframe::run_native(
        "CrabAI - Ferramenta de IA Local",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            utils::configurar_fontes(&cc.egui_ctx);

            Ok(Box::new(CrabAIApp::new()))
        }),
    )
}