use eframe::egui;
use crate::RustOpsApp;

pub fn exibir_erros_criticos(app: &mut RustOpsApp, ctx: &egui::Context) {
    if let Some(msg) = &app.erro_fatal {
        // Criar uma área de "escurecimento" atrás do modal (diming)
        egui::Area::new(egui::Id::new("dimmer"))
            .interactable(true)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let screen_rect = ctx.content_rect();
                ui.painter().rect_filled(screen_rect, 0.0, egui::Color32::from_black_alpha(150));
            });

        egui::Window::new("Erro de Inicialização")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .fixed_size(egui::vec2(400.0, 200.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("⚠️").size(40.0));
                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label(egui::RichText::new(msg.to_string()).color(egui::Color32::LIGHT_RED));
                    });

                    ui.add_space(10.0);
                    ui.label("O RustOps não pode continar sem resolver esse problema.");
                    ui.add_space(10.0);

                    if ui.button(egui::RichText::new("  OK  ").strong()).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.add_space(10.0);
                });
            });
    }
}