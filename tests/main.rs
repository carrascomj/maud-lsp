mod support;

use std::path::PathBuf;

use crate::support::Project;

use lsp_types::{
    request::{GotoDefinition, HoverRequest},
    GotoDefinitionParams, HoverParams, PartialResultParams, Position, TextDocumentPositionParams,
    WorkDoneProgressParams,
};

#[test]
fn gotodefinition_of_metabolites() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("built");

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(2, 16),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    assert!(res.to_string().contains('3'));
}

#[test]
fn hover_of_metabolites() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("built");

    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(2, 16),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    assert!(res.to_string().contains("f6p"));
}
