//! Language Server for [Maud](https://github.com/biosustain/Maud).
//!
//! Supports Hover and GotoDefinition around the kinetic model.
use std::error::Error;

use lsp_types::OneOf;
use lsp_types::{HoverProviderCapability, InitializeParams, ServerCapabilities};

use lsp_server::Connection;
pub mod config;
mod looping;
mod maud_data;
mod symbol_parser;
mod state;

use config::Config;
pub use looping::main_loop;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = connection.initialize(server_capabilities)?;
    let params: InitializeParams = serde_json::from_value(initialization_params).unwrap();
    let config = Config::from_init(params)?;
    main_loop(connection, config)?;

    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}
