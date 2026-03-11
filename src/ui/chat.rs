use eframe::egui;
use crate::app::RustOpsApp;
use egui_commonmark::{CommonMarkViewer};


pub fn desenhar_painel_central(app: &mut RustOpsApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Cabeçalho Dinâmico
        ui.horizontal(|ui| {
            if app.editando_titulo {
                ui.add(egui::TextEdit::singleline(&mut app.novo_titulo_temp));
                if ui.button("✅ Salvar").clicked() {
                    app.db.renomear_sessao_ativa(app.novo_titulo_temp.clone());
                    app.editando_titulo = false;
                }
            } else {
                let titulo_chat = app.db.get_sessao_ativa_mut().titulo.clone();
                ui.heading(format!("RustOps - {}", titulo_chat));
                
                if ui.button("✏️").clicked() {
                    app.editando_titulo = true;
                    app.novo_titulo_temp = titulo_chat;
                }
                if app.db.sessoes.len() > 1 {
                    if ui.button("🗑️").clicked() {
                        app.db.deletar_sessao_ativa();
                    }
                }
            }
        });
        ui.separator();

        // Área de Mensagens
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                
                // Referência mutável da sessão ativa
                let sessao_ativa = app.db.get_sessao_ativa_mut();
                let id_sessao = sessao_ativa.id;

                // Iter().enumarate para ter um indice (0,1,2,...) para cada mensagem
                for (indice, msg) in sessao_ativa.mensagens.iter().enumerate() {
                    if msg.role == "system" { continue; }

                    if msg.role == "user" {
                        // Mensagem do usuário (texto normal azulzinho)
                        ui.label(
                            egui::RichText::new(format!("Você: {}", msg.content))
                                .color(egui::Color32::LIGHT_BLUE)
                        );
                    } else {
                        // Mensagem da IA
                        ui.label(
                            egui::RichText::new("RustOps:")
                                .color(egui::Color32::LIGHT_GREEN)
                        );

                        // Criar um id único juntando o ID do chat + posição da mensagem
                        let id_mensagem = format!("chat_{}_msg_{}", id_sessao, indice);

                        ui.push_id(&id_mensagem, |ui| {
                            CommonMarkViewer::new()
                                .show(ui, &mut app.markdown_cache, &msg.content);
                        });
                    }

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                }

                if app.db.get_sessao_ativa_mut().is_loading {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(egui::RichText::new("RustOps está digitando...").color(egui::Color32::DARK_GRAY));
                    });
                }
            });
    });
}