use eframe::egui;
use crate::app::RustOpsApp;

pub fn processar_mensagens_ia(app: &mut RustOpsApp, ctx: &egui::Context) {
    let mut precisa_salvar = false;
    
    // Percorre todas as sessões existentes
    for sessao in app.db.sessoes.iter_mut() {
        // Se essa sessão tiver um receptor ativo...
        if let Some(rx) = &mut sessao.receptor {
            // Tenta pegar o texto vindo da IA
            if let Ok(pedaco_texto) = rx.try_recv() {
                if pedaco_texto == "[FIM]" {
                    sessao.is_loading = false;
                    sessao.receptor = None;
                    precisa_salvar = true;
                } else {
                    // Adiciona o texto específicamente nesta sessão
                    if let Some(ultima_msg) = sessao.mensagens.last_mut() {
                        if ultima_msg.role == "assistant" {
                            ultima_msg.content.push_str(&pedaco_texto);
                        }
                    }
                }
                ctx.request_repaint();
            }
        }
    }
    if precisa_salvar {
        app.db.salvar();
    }
}