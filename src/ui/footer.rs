use eframe::egui;
use crate::app::CrabAIApp;
use crate::storage::ChatMessage;
use std::sync::mpsc::channel;
use std::thread;
use crate::ollama;

pub fn desenhar_rodape(app: &mut CrabAIApp, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("rodape").show(ctx, |ui| {
        ui.add_space(10.0);

        // horizontal_top mantém o botão alinhado ao topo enquando o input cresce
        ui.horizontal_top(|ui| {

            let largura_disponivel = ui.available_width() - 80.0;
            let altura_maxima_input = 100.0;

            // 1. Área de rolagem para o texto (limita o crescimento vertical)
            egui::ScrollArea::vertical()
                .max_height(altura_maxima_input)
                .id_salt("scroll_input_usuario")
                .show(ui, |ui| {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut app.user_input)
                            .hint_text("Insira um comando para o CrabAI")
                            .desired_width(largura_disponivel)
                            .desired_rows(2)
                            .lock_focus(true)
                    );

                    // 2. Lógica de detecção do teclado dentro da closure
                    if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) {
                        app.requisitou_envio = true;
                    }
                });

                // Botão com tamanho fixo
                let button = ui.add_sized([70.0, 35.0], egui::Button::new("Enviar"));

                // 3. Processamento do Envio (seja por clique ou por teclado)
                let sessao_ativa = app.db.get_sessao_ativa_mut();

               if (button.clicked() || app.requisitou_envio) && !app.user_input.trim().is_empty() && !sessao_ativa.is_loading {
                   let (historico, tx) = {
                        sessao_ativa.is_loading = true;

                    // Adiciona mensagem do usuário
                    sessao_ativa.mensagens.push(ChatMessage {
                        role: "user".to_string(),
                        content: app.user_input.trim().to_string(),
                    });

                    let historico = sessao_ativa.mensagens.clone();

                    // Prepara o balão da resposta da IA
                    sessao_ativa.mensagens.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "".to_string(),
                    });

                    // Seta o receptor
                    let (tx, rx) = channel();
                    sessao_ativa.receptor = Some(rx);

                    (historico, tx)
                   };

                   // Finaliza as ações globais após o bloco
                   app.requisitou_envio = false;
                   app.db.salvar();
                   app.user_input.clear();

                   thread::spawn(move || {
                        ollama::send_to_ollama_chat(historico, tx);
                   });
                }
        });
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(4.0);

        ui.horizontal(|ui| {

            let cor_laranja = egui::Color32::from_rgb(255, 87, 34);

            let botao_apoio = egui::Button::new(egui::RichText::new("☕ Apoie o Projeto").color(cor_laranja))
                .fill(egui::Color32::TRANSPARENT)
                .stroke(egui::Stroke::new(1.0, cor_laranja));

            if ui.add(botao_apoio).clicked() {
                app.mostrar_janela_apoio = true;
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);


            // 1. Status de Hardware (Esquerda)
            ui.spacing_mut().item_spacing.x = 15.0; // Espaço entre os itens


            // CPU com cor dinâmica
            let cpu_cor = if app.cpu_usage > 70.0 {
                egui::Color32::from_rgb(255,100,100) // Vermelho se estiver alto
            } else {
                egui::Color32::DARK_GRAY
            };

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("CPU:")
                    .small()
                    .color(cpu_cor));
                ui.label(
                    egui::RichText::new(format!("{:.1}%", app.cpu_usage))
                    .small()
                    .color(cpu_cor)
                );
            });

            // RAM
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("RAM:")
                    .small()
                    .color(egui::Color32::DARK_GRAY));
                ui.label(
                    egui::RichText::new(format!("{:.1} GB", app.ram_usage))
                    .small()
                    .color(egui::Color32::DARK_GRAY)
                );
            });

            // 2. Assinatura e Versão (Direita)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .small()
                        .color(egui::Color32::from_rgb(80,80,80))
                );

                ui.label(
                    egui::RichText::new(format!("| {} |", env!("CARGO_PKG_AUTHORS")))
                    .small()
                    .color(egui::Color32::from_rgb(80,80,80))
                );
            });
        });
        ui.add_space(6.0);
    });
}