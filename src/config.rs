use lsp_types::{ClientCapabilities, InitializeParams};
use std::path::PathBuf;

/// One-time initialized Config for the LSP.
#[derive(Clone)]
pub struct Config {
    pub caps: ClientCapabilities,
    pub root_dir: PathBuf,
}

impl Config {
    pub fn from_init(init_params: InitializeParams) -> std::io::Result<Self> {
        let root_path = match init_params
            .root_uri
            .and_then(|it| it.to_file_path().ok())
            .and_then(|it| PathBuf::try_from(it).ok())
        {
            Some(it) => it,
            None => std::env::current_dir()?,
        };
        Ok(Config {
            root_dir: root_path,
            caps: init_params.capabilities,
        })
    }
}
