use eframe::egui;
use crate::app::RustOpsApp;

pub fn termos_de_uso(app: &mut RustOpsApp, ctx: &egui::Context) -> bool {
    if !app.aceitou_termos {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);

                ui.heading("⚠️ AVISO LEGAL E TERMOS DE USO");
                ui.add_space(20.0);

                ui.label("O RustOps é uma ferramenta local desenvolvida estritamente para fins educacionais");
                ui.label("e de pesquisa em Segurança da Informação (Red Teaming).");
                ui.add_space(10.0);

                ui.label("O desenvolvedor não se responsabilida por nenhum dano, uso indevido");
                ui.label("ou atividade ilegal realizada com o auxilio desta ferramenta.");
                ui.add_space(10.0);

                ui.label("Ao utilizar o RustOps, você concorda que todas as ações tomadas");
                ui.label("são de sua única e exclusiva responsabilidade.");
                ui.add_space(30.0);

                // Botão que destrava o aplicativo
                if ui.button("🚨 Eu li, compreendo e aceito os termos").clicked() {
                    app.aceitou_termos = true;
                }
            });
        });
        return true;
    }
    false
}