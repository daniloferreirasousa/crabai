use std::env;

fn main() {
    // Embutir o icone somente se estiver compilando para windows
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "windows" {
        let mut res = winres::WindowsResource::new();

        // Aponta para o arquivo .ico na raiz do projeto
        res.set_icon("icone.ico");

        // Pega o ícone e injeta no execultave
        if let Err(e) = res.compile() {
            println!("Aviso: Falha ao compilar os recursos do windows: {}", e);
        }
    }
}