use tower_lsp::{ Server};
use valkyrie_lsp::ValkyrieLanguageClient;


#[tokio::main]
pub async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = ValkyrieLanguageClient::launch();
    Server::new(stdin, stdout, socket).serve(service).await;
}