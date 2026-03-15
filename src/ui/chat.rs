use eframe::egui;
use crate::app::CrabAIApp;
use egui_commonmark::{CommonMarkViewer};
use rfd::FileDialog;
use std::fs;


pub fn desenhar_painel_central(app: &mut CrabAIApp, ctx: &egui::Context) {
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
                ui.heading(format!("CrabAI - {}", titulo_chat));
                
                if ui.button("✏️").clicked() {
                    app.editando_titulo = true;
                    app.novo_titulo_temp = titulo_chat;
                }
                if app.db.sessoes.len() > 1 {
                    if ui.button("🗑️").clicked() {
                        app.db.deletar_sessao_ativa();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn_exportar = egui::Button::new(
                        egui::RichText::new("📥 Exportar .md")
                            .color(egui::Color32::WHITE)
                    ).fill(egui::Color32::from_rgb(60,60,60));

                    let sessao_ativa = app.db.get_sessao_ativa_mut();

                    let mensagens_visiveis = sessao_ativa.mensagens.iter().filter(|m| m.role != "system").count();

                    if mensagens_visiveis > 0 {
                        if ui.add(btn_exportar).on_hover_text("Exportar esta conversa para um relatório Markdown").clicked() {
                            let sessao_ativa = app.db.get_sessao_ativa_mut();
                            exportar_para_markdown(sessao_ativa);
                        }
                    }

                });
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

                // Conta quantas mensagens reais o usuário e a IA trocaram (ignora o system prompt)
                let mensagens_visiveis = sessao_ativa.mensagens.iter().filter(|m| m.role != "system").count();

                // TELA DE BOAS VINDAS (Empty State)
                if mensagens_visiveis == 0 && !sessao_ativa.is_loading {
                    ui.vertical_centered(|ui| {
                        
                        // Empurra o conteúdo um pouco para o meio da tela;
                        ui.add_space(ui.available_height() / 3.0);

                        // Título principal com a cor da logo
                        ui.heading(
                            egui::RichText::new("🦀 CrabAI")
                                .size(45.0)
                                .color(egui::Color32::from_rgb(255, 87, 34))
                        );

                       
                        
                        ui.add_space(10.0);

                        // Subtítulo elegante
                        ui.label(
                            egui::RichText::new("Seu mentor local de Rust e Segurança da Informação.")
                                .size(18.0)
                                .color(egui::Color32::LIGHT_GRAY)
                        );

                        ui.add_space(30.0);

                        // Sugestões de uso
                        ui.label(egui::RichText::new("Sugetões para começar:").strong());
                        ui.label("👉 'Me explique Ownership em Rust como se eu tivesse 10 anos'");
                        ui.label("👉 'Escreva um script básico de port scanner em Rust'");
                        ui.label("👉 'Como criar um Trait genérico?'");


                    });
                } else {
                    // LISTAGEM DE MENSAGENS (Chat normal)
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
                                egui::RichText::new("🦀 CrabAI:")
                                    .color(egui::Color32::from_rgb(255, 87, 34))
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
                }

                // ANIMAÇÃO DE CARREGAMENTO
                if app.db.get_sessao_ativa_mut().is_loading {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(egui::RichText::new("CrabAI está digitando...").color(egui::Color32::DARK_GRAY));
                    });
                }
            });
    });
}

pub fn exportar_para_markdown(sessao: &crate::storage::ChatSession) {
    // 1. Prepara o conteúdo do Markdown
    let mut conteudo = format!("# Relatório - **{}**\n\n", sessao.titulo);

    for msg in &sessao.mensagens {
        if msg.role == "system" { continue; }
    
        let prefixo = if msg.role == "user" { "### 👤 Usuário" } else { "### 🦀 CrabAI" };
        conteudo.push_str(&format!("{}\n{}\n\n---\n", prefixo, msg.content));
    }

    // 2. Abre a caixa de diálogo nativa
    let arquivo = FileDialog::new()
        .set_file_name(&format!("{}.md", sessao.titulo))
        .add_filter("Markdown", &["md"])
        .save_file();

    // 3. Se  o usuário escolher um caminho, salva o arquivo
    if let Some(caminho) = arquivo {
        if let Ok(_) = fs::write(caminho, conteudo) {
            println!("Conversa exportada com sucesso!");
        }
    }
}