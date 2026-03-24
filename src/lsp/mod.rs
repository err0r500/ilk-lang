mod completions;
mod diagnostics;
mod hover;
mod navigate;

use crate::{formatter, Compiler};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct Backend {
    client: Client,
    compiler: Arc<RwLock<Compiler>>,
    documents: Arc<RwLock<HashMap<Url, String>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            compiler: Arc::new(RwLock::new(Compiler::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn update_diagnostics(&self, uri: &Url, text: &str) {
        let path = uri_to_path(uri);
        let mut compiler = self.compiler.write().await;
        compiler.invalidate(&path);

        // Load the file with its source text (imports loaded from disk)
        let diags = match compiler.load(&path, text) {
            Ok(_) => match compiler.validate(&path) {
                Ok(()) => vec![],
                Err(errs) => diagnostics::convert(&errs, text),
            },
            Err(errs) => diagnostics::convert(&errs, text),
        };

        self.client
            .publish_diagnostics(uri.clone(), diags, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "{".to_string(),
                        "=".to_string(),
                        ":".to_string(),
                        " ".to_string(),
                    ]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ilk LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents
            .write()
            .await
            .insert(uri.clone(), text.clone());
        self.update_diagnostics(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents
                .write()
                .await
                .insert(uri.clone(), change.text.clone());
            self.update_diagnostics(&uri, &change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);

        let path = uri_to_path(&uri);
        self.compiler.write().await.invalidate(&path);

        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let docs = self.documents.read().await;
        let Some(text) = docs.get(uri) else {
            return Ok(None);
        };

        let path = uri_to_path(uri);
        let compiler = self.compiler.read().await;

        let items = completions::complete(&compiler, &path, text, pos);
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(text) = docs.get(uri) else {
            return Ok(None);
        };

        let path = uri_to_path(uri);
        let compiler = self.compiler.read().await;

        let Some(file) = compiler.get_file(&path) else {
            return Ok(None);
        };
        let Some(env) = compiler.get_env(&path) else {
            return Ok(None);
        };

        let offset = diagnostics::position_to_offset(pos, text);
        let Some(def_span) = navigate::find_definition(file, env, offset) else {
            return Ok(None);
        };

        let range = diagnostics::span_to_range(def_span.start, def_span.end, text);
        Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
            uri.clone(),
            range,
        ))))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(text) = docs.get(uri) else {
            return Ok(None);
        };

        let path = uri_to_path(uri);
        let compiler = self.compiler.read().await;

        let Some(file) = compiler.get_file(&path) else {
            return Ok(None);
        };
        let Some(env) = compiler.get_env(&path) else {
            return Ok(None);
        };

        let offset = diagnostics::position_to_offset(pos, text);
        let Some(info) = hover::hover_info(file, env, offset) else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info,
            }),
            range: None,
        }))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;

        let docs = self.documents.read().await;
        let Some(text) = docs.get(uri) else {
            return Ok(None);
        };

        let path = uri_to_path(uri);

        // Parse the file to get the AST with comments
        let Ok(file) = crate::parser::parse(text, &path) else {
            return Ok(None);
        };

        let formatted = formatter::format(&file, text);

        // Return a single edit replacing the entire document
        let line_count = text.lines().count();
        let last_line_len = text.lines().last().map(|l| l.len()).unwrap_or(0);

        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(line_count as u32, last_line_len as u32),
            },
            new_text: formatted,
        }]))
    }
}

fn uri_to_path(uri: &Url) -> PathBuf {
    uri.to_file_path()
        .unwrap_or_else(|_| PathBuf::from(uri.path()))
}

pub async fn run() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
