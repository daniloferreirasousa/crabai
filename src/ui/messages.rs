use eframe::egui;
use crate::app::RustOpsApp;

pub fn processar_mensagens_ia(app: &mut RustOpsApp, ctx: &egui::Context) {
    if let Some(rx) = &app.receptor_de_texto {
        if let Ok(pedaco_texto) = rx.try_recv() {
            if pedaco_texto == "[FIM]" {
                app.is_processing = false;
                app.receptor_de_texto = None;
                app.db.salvar();
            } else {
                let sessao_atual = app.db.get_sessao_ativa_mut();
                if let Some(ultima_msg) = sessao_atual.mensagens.last_mut() {
                    if ultima_msg.role == "assistant" {
                        ultima_msg.content.push_str(&pedaco_texto);
                    }
                }
            }
            ctx.request_repaint();
        }
    }
}