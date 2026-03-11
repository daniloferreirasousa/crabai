// Resposável APENAS pelo carregamento da janela de apoio
use eframe::egui;
use crate::app::RustOpsApp;

pub fn desenhar_janela_apoio(app: &mut RustOpsApp, ctx: &egui::Context) {
        // Se for false, nem desenha nada
        if !app.mostrar_janela_apoio {
            return;
        }

        let mut aberta = app.mostrar_janela_apoio;

        egui::Window::new("☕ Apoie o Projeto")
            .open(&mut aberta) // Adiciona um "X" para fechar a janela
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Abre bem no meio da tela
            .show(ctx, |ui|{
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Gostou do RustOps? Ele é gratuito e open-source!");
                    ui.label("Considere me pagar um café para ajudar a manter o projeto.");
                    ui.add_space(20.0);

                    let chave_pix = "00020126580014BR.GOV.BCB.PIX013693cc5dfd-0c3a-4e80-b087-4ac00a96b62e5204000053039865802BR5925DANILO DE ANDRADE FERREIR6007RESENDE62070503***63048F81";

                    if ui.button("Copiar Chave PIX").clicked() {
                        ui.ctx().copy_text(chave_pix.to_string());
                    }
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Danilo Ferreira Sousa").small().color(egui::Color32::GRAY));
                    ui.add_space(10.0);
                });
            });
            app.mostrar_janela_apoio = aberta;
    }