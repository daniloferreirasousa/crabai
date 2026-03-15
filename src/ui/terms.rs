use eframe::egui;
use crate::app::CrabAIApp;

pub fn termos_de_uso(app: &mut CrabAIApp, ctx: &egui::Context) -> bool {
    // 1. Verifica no banco de dados JSON se já aceitou
    if app.db.aceitou_termos {
        return false; // Já ceitou não precisa desenhar esta tela
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 4.0);

            // O Cartão Premiun
            egui::Frame::window(&ctx.style())
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .corner_radius(12.0)
                .inner_margin(35.0)
                .shadow(egui::epaint::Shadow {
                    offset: [0, 8],
                    blur: 20,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(150),
                })
                .show(ui, |ui| {
                    ui.heading(
                        egui::RichText::new("⚠ AVISO LEGAL E TERMOS DE USO")
                            .color(egui::Color32::from_rgb(255, 87,34))
                            .size(24.0)
                    );

                    ui.add_space(20.0);

                    ui.label(egui::RichText::new("O CrabAI é uma ferramenta local desenvolvida estritamente para fins educacionais\ne de pesquisas em Segurança da Informação.").size(15.0));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("O desenvolvedor não se responsabiliza por nenhum dano, uso indevido\nou atividade ilegal realizada com auxílio desta ferramenta.").size(15.0));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Ao utilizar o CrabAI, você concorda com que todas as ações tomadas\nsão de sua e única responsabilidade.").size(15.0));

                    ui.add_space(30.0);

                    let botao_aceitar = egui::Button::new(
                        egui::RichText::new("Li, compreendo e aceito os termos")
                            .color(egui::Color32::WHITE)
                            .size(18.0)
                    ).fill(egui::Color32::from_rgb(255, 87, 34));

                    if ui.add_sized([400.0, 45.0], botao_aceitar).clicked() {
                        app.db.aceitou_termos = true;
                        app.db.salvar();
                    }
                });
        });
    });
    true
}