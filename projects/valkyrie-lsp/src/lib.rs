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

use std::future::Future;
use std::pin::Pin;
pub use crate::errors::{ExampleErrorKind, ExampleError};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, ClientSocket, LanguageServer, LspService};
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse, GotoImplementationParams, GotoImplementationResponse, GotoTypeDefinitionParams, GotoTypeDefinitionResponse};

#[derive(Debug)]
pub struct ValkyrieLanguageServer {
    proxy: Client,
}

impl ValkyrieLanguageServer {
    pub fn launch() -> (LspService<ValkyrieLanguageServer>, ClientSocket) {
        LspService::new(|client| ValkyrieLanguageServer { proxy: client })
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ValkyrieLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: None,
                text_document_sync: None,
                selection_range_provider: None,
                hover_provider: Some(HoverProviderCapability::Options(HoverOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    }
                })),
                completion_provider: None,
                signature_help_provider: None,
                declaration_provider: Some(DeclarationCapability::RegistrationOptions(DeclarationRegistrationOptions {
                    declaration_options: DeclarationOptions { work_done_progress_options: Default::default() },
                    text_document_registration_options: Default::default(),
                    static_registration_options: Default::default(),
                })),
                definition_provider: Some(OneOf::Right(DefinitionOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                type_definition_provider: Some(TypeDefinitionProviderCapability::Options(StaticTextDocumentRegistrationOptions {
                    id: None,
                    document_selector: Some(vec![]),
                })),
                implementation_provider: Some(ImplementationProviderCapability::Options(StaticTextDocumentRegistrationOptions {
                    id: None,
                    document_selector: Some(vec![]),
                })),
                references_provider: Some(OneOf::Right(ReferencesOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                document_highlight_provider: None,
                document_symbol_provider: None,
                workspace_symbol_provider: None,
                code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
                    code_action_kinds: None,
                    resolve_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                code_lens_provider: None,
                document_formatting_provider: None,
                document_range_formatting_provider: None,
                document_on_type_formatting_provider: None,
                rename_provider: None,
                document_link_provider: None,
                color_provider: None,
                folding_range_provider: None,
                execute_command_provider: None,
                workspace: None,
                call_hierarchy_provider: None,
                semantic_tokens_provider: None,
                moniker_provider: None,
                linked_editing_range_provider: None,
                inline_value_provider: None,
                inlay_hint_provider: None,
                diagnostic_provider: None,
                experimental: None,
            },
            server_info: Some(ServerInfo {
                name: "Valkyrie Language Server".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            offset_encoding: Some("utf-8".to_string()),
        })
    }

    async fn goto_type_definition(&self, params: GotoTypeDefinitionParams) -> Result<Option<GotoTypeDefinitionResponse>> {
        // each type has only one declaration position
        // But in the case of repeated definitions by mistake, there will be multiple declaration locations
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // a function or method may have multiple definition locations
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }
    async fn goto_declaration(&self, params: GotoDeclarationParams) -> Result<Option<GotoDeclarationResponse>> {
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }
    async fn goto_implementation(&self, params: GotoImplementationParams) -> Result<Option<GotoImplementationResponse>> {
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
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
        Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: debug }), range: None }))
    }
}
