// Renderiza APENAS a tela de de carregamento incial
use eframe::egui;
use crate::app::RustOpsApp;


pub fn desenhar_tela_carregamento(app: &mut RustOpsApp, ctx: &egui::Context) -> bool {
    // Se já carregou tudo, avisa o update() para desenhar o resto do app
    if app.is_initialized {
        return false;
    }

    // Verifica se há novas atualizações da thread de setup
    if let Some(rx) = &app.startup_receiver {
        // Usamos 'while let' para ler todas as mensagens pendentes rapidamente
        while let Ok(msg) = rx.try_recv() {
            if msg == "CONCLUIDO" {
                app.is_initialized = true;
                app.startup_receiver = None; // Limpa o canal da memória
                return false;
            } else if msg.starts_with("ERRO_FATAL") {
                app.startup_status_text = msg.replace("ERRO_FATAL: ","Erro: ");
            } else {
                app.startup_status_text = msg;
            }
        }
    }

    // Desenha a tela de loading bonitona
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading(egui::RichText::new("🚀 RustOps").size(40.0).strong());
            ui.add_space(40.0);
            
            // Se for um erro,  não mostra o spinner
            if !app.startup_status_text.starts_with("Erro:") {
                ui.spinner();
                ui.add_space(20.0)
            }

            // O texto dinâmico que vem do utils.rs
            ui.label(egui::RichText::new(&app.startup_status_text).size(16.0));

            if app.startup_status_text.starts_with("Erro:") {
                ui.add_space(30.0);
                ui.label(egui::RichText::new("Por favor, verificque sua internet e reinicie o aplicativo.").color(egui::Color32::LIGHT_RED));
            } else {
                ui.add_space(30.0);
                ui.label(egui::RichText::new("A primeira execução pode levar vários minutos.\nPor favor, não feche o aplicativo.").color(egui::Color32::LIGHT_YELLOW));
            }
        });
    });

    // Força a tela a atualizar em 60fps enquanto carrega
    ctx.request_repaint();
    true 
}
