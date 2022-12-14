#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg"
)]

mod errors;
pub use crate::errors::{ExampleErrorKind, ExampleError};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, ClientSocket, LanguageServer, LspService, Server};

#[derive(Debug)]
pub struct ValkyrieLanguageClient {
    proxy: Client,
}

impl ValkyrieLanguageClient {
    pub fn launch() -> (LspService<ValkyrieLanguageClient>, ClientSocket) {
        LspService::new(|client| ValkyrieLanguageClient { proxy: client })
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ValkyrieLanguageClient {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.proxy
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let debug = format!("```json\n{:#?}\n```", params);
        Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value:debug}), range: None }))
    }
}
