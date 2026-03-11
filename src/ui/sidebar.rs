// Renderiza APENAS o painel lateral (lista de chats)
use eframe::egui;
use crate::app::RustOpsApp;

pub fn desenhar_painel_lateral(app: &mut RustOpsApp, ctx: &egui::Context) {
    egui::SidePanel::left("menu_lateral")
        .resizable(true)
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Conversas");
            ui.add_space(10.0);

            if ui.button("➕ Novo Chat").clicked() {
                app.db.criar_nova_sessao();
            }
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut id_clicado = None;
                for sessao in &app.db.sessoes {
                    let is_active = sessao.id == app.db.sessao_ativa_id;
                    if ui.selectable_label(is_active, &sessao.titulo).clicked() {
                        id_clicado = Some(sessao.id);
                    }
                }
                if let Some(id) = id_clicado {
                    app.db.sessao_ativa_id = id;
                    app.db.salvar();
                }
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                if ui.button("☕ Apoie o Projeto").clicked() {
                    app.mostrar_janela_apoio = true;
                }
                ui.add_space(10.0);
                ui.separator();
            });
        });
}
