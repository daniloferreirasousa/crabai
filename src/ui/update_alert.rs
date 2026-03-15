use eframe::egui;
use crate::app::CrabAIApp;

pub fn desenhar_alerta_atualizacao(app: &mut CrabAIApp, ctx: &egui::Context) {
    // 1. Tenta ler a mensage da thread do Github
    if let Some(rx) = &app.receptor_update {
        if let Ok(nova_versao) = rx.try_recv() {
            app.versao_disponivel = Some(nova_versao);
            app.receptor_update = None; // Limpa o canal
        }
    }

    // 2. Se tem versão nova, desenha uma barra superior de destaque
    if let Some(versao) = &app.versao_disponivel {
        egui::TopBottomPanel::top("painel_atualizacao").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("🚀 Nova versão do CrabAI disponível ({})!", versao))
                        .color(egui::Color32::YELLOW)
                        .strong()
                );

                // Empurra o botão para a direita
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    
                    if ui.button("Baixar Atualização").clicked() {
                        let url_release = "https://github.com/daniloferreirasousa/crabai/releases/latest";
                        let _ = webbrowser::open(url_release);
                    }
                });
            });
            ui.add_space(5.0);
        });
    }
}