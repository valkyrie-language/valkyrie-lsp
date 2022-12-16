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

use tower_lsp::jsonrpc::{Error, Result};
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
                    declaration_options: DeclarationOptions {
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: Some(true),
                        }
                    },
                    text_document_registration_options: TextDocumentRegistrationOptions {
                        document_selector: None,
                    },
                    static_registration_options: StaticRegistrationOptions {
                        id: None,
                    },
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
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: None,
                    file_operations: None,
                }),
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

    async fn initialized(&self, _: InitializedParams) {
        self.proxy
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {

    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {

    }
    async fn will_save(&self, params: WillSaveTextDocumentParams) {

    }

    async fn will_save_wait_until(&self, params: WillSaveTextDocumentParams) -> Result<Option<Vec<TextEdit>>> {
        Err(Error::method_not_found())
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {

    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {

    }
    async fn goto_declaration(&self, params: GotoDeclarationParams) -> Result<Option<GotoDeclarationResponse>> {
        let here1 = Location {
            uri: params.text_document_position_params.text_document.uri.clone(),
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: params.text_document_position_params.position,
            },
        };
        let here2 = Location {
            uri: params.text_document_position_params.text_document.uri.clone(),
            range: Range {
                start: params.text_document_position_params.position,
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
        };
        Ok(Some(GotoDefinitionResponse::Array(vec![here1, here2])))
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // a function or method may have multiple definition locations
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }
    async fn goto_type_definition(&self, params: GotoTypeDefinitionParams) -> Result<Option<GotoTypeDefinitionResponse>> {
        // each type has only one declaration position
        // But in the case of repeated definitions by mistake, there will be multiple declaration locations
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }
    async fn goto_implementation(&self, params: GotoImplementationParams) -> Result<Option<GotoImplementationResponse>> {
        Ok(Some(GotoDefinitionResponse::Array(vec![])))
    }
    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(Some(vec![]))
    }
    async fn prepare_call_hierarchy(&self, params: CallHierarchyPrepareParams) -> Result<Option<Vec<CallHierarchyItem>>> {
        Err(Error::method_not_found())
    }
    async fn incoming_calls(&self, params: CallHierarchyIncomingCallsParams) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        Err(Error::method_not_found())
    }
    async fn outgoing_calls(&self, params: CallHierarchyOutgoingCallsParams) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        Err(Error::method_not_found())
    }
    async fn prepare_type_hierarchy(&self, params: TypeHierarchyPrepareParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        Err(Error::method_not_found())
    }
    async fn supertypes(&self, params: TypeHierarchySupertypesParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let item = TypeHierarchyItem {
            name: "supertypes".to_string(),
            kind: SymbolKind::CLASS,
            tags: None,
            detail: Some("supertypes details".to_string()),
            uri: params.item.uri,
            range: Default::default(),
            selection_range: Default::default(),
            data: None,
        };
        Ok(Some(vec![item]))
    }
    async fn subtypes(&self, params: TypeHierarchySubtypesParams) -> Result<Option<Vec<TypeHierarchyItem>>> {
        let item = TypeHierarchyItem {
            name: "subtypes".to_string(),
            kind: SymbolKind::CLASS,
            tags: None,
            detail: Some("subtypes details".to_string()),
            uri: params.item.uri,
            range: Default::default(),
            selection_range: Default::default(),
            data: None,
        };
        Ok(Some(vec![item]))
    }
    async fn document_highlight(&self, params: DocumentHighlightParams) -> Result<Option<Vec<DocumentHighlight>>> {
        Err(Error::method_not_found())
    }
    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        Err(Error::method_not_found())
    }
    async fn document_link_resolve(&self, params: DocumentLink) -> Result<DocumentLink> {
        Err(Error::method_not_found())
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let debug = format!("```json\n{:#?}\n```", params);
        Ok(Some(Hover { contents: HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: debug }), range: None }))
    }
    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        Err(Error::method_not_found())
    }
    async fn code_lens_resolve(&self, params: CodeLens) -> Result<CodeLens> {
        Err(Error::method_not_found())
    }
    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        Err(Error::method_not_found())
    }
    async fn selection_range(&self, params: SelectionRangeParams) -> Result<Option<Vec<SelectionRange>>> {
        Err(Error::method_not_found())
    }
    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        Err(Error::method_not_found())
    }
    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        Err(Error::method_not_found())
    }
    async fn semantic_tokens_full_delta(&self, params: SemanticTokensDeltaParams) -> Result<Option<SemanticTokensFullDeltaResult>> {
        Err(Error::method_not_found())
    }
    async fn semantic_tokens_range(&self, params: SemanticTokensRangeParams) -> Result<Option<SemanticTokensRangeResult>> {
        Err(Error::method_not_found())
    }

    async fn inline_value(&self, params: InlineValueParams) -> Result<Option<Vec<InlineValue>>> {
        Err(Error::method_not_found())
    }
    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        Err(Error::method_not_found())
    }
    async fn inlay_hint_resolve(&self, params: InlayHint) -> Result<InlayHint> {
        Err(Error::method_not_found())
    }
    async fn moniker(&self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        Err(Error::method_not_found())
    }
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Err(Error::method_not_found())
    }
    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        Err(Error::method_not_found())
    }
    async fn diagnostic(&self, params: DocumentDiagnosticParams) -> Result<DocumentDiagnosticReportResult> {
        Err(Error::method_not_found())
    }
    async fn workspace_diagnostic(&self, params: WorkspaceDiagnosticParams) -> Result<WorkspaceDiagnosticReportResult> {
        Err(Error::method_not_found())
    }
    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        Err(Error::method_not_found())
    }
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let command = CodeActionOrCommand::Command(Command {
            title: "command place holder".to_string(),
            command: "".to_string(),
            arguments: None,
        });
        let command2 = Command {
            title: "command in action".to_string(),
            command: "".to_string(),
            arguments: None,
        };
        let action = CodeActionOrCommand::CodeAction(CodeAction {
            title: "action place holder".to_string(),
            kind: None,
            diagnostics: None,
            edit: None,
            command: Some(command2),
            is_preferred: None,
            disabled: None,
            data: None,
        });
        Ok(Some(vec![action, command]))
    }
    async fn code_action_resolve(&self, params: CodeAction) -> Result<CodeAction> {
        Err(Error::method_not_found())
    }
    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        Err(Error::method_not_found())
    }
    async fn color_presentation(&self, params: ColorPresentationParams) -> Result<Vec<ColorPresentation>> {
        Err(Error::method_not_found())
    }
    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Err(Error::method_not_found())
    }
    async fn range_formatting(&self, params: DocumentRangeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Err(Error::method_not_found())
    }
    async fn on_type_formatting(&self, params: DocumentOnTypeFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        Err(Error::method_not_found())
    }
    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Err(Error::method_not_found())
    }
    async fn prepare_rename(&self, params: TextDocumentPositionParams) -> Result<Option<PrepareRenameResponse>> {
        Err(Error::method_not_found())
    }
    async fn linked_editing_range(&self, params: LinkedEditingRangeParams) -> Result<Option<LinkedEditingRanges>> {
        Err(Error::method_not_found())
    }
    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        Err(Error::method_not_found())
    }
    async fn symbol_resolve(&self, params: WorkspaceSymbol) -> Result<WorkspaceSymbol> {
        Err(Error::method_not_found())
    }
    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {

    }
    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {

    }
    async fn will_create_files(&self, params: CreateFilesParams) -> Result<Option<WorkspaceEdit>> {
        Err(Error::method_not_found())
    }
    async fn did_create_files(&self, params: CreateFilesParams) {

    }
    async fn will_rename_files(&self, params: RenameFilesParams) -> Result<Option<WorkspaceEdit>> {
        Err(Error::method_not_found())
    }
    async fn did_rename_files(&self, params: RenameFilesParams) {

    }
    async fn will_delete_files(&self, params: DeleteFilesParams) -> Result<Option<WorkspaceEdit>> {
        Err(Error::method_not_found())
    }
    async fn did_delete_files(&self, params: DeleteFilesParams) {

    }
    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {

    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<LSPAny>> {
        Err(Error::method_not_found())
    }
}
