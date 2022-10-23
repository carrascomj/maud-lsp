mod support;

use std::path::PathBuf;

use crate::support::Project;

use lsp_types::{
    request::GotoDefinition, GotoDefinitionParams, PartialResultParams, Position,
    TextDocumentPositionParams, WorkDoneProgressParams,
};

#[test]
fn gotodefinition_of_metabolites() {
    let server =
        Project::from_kinetic_model(PathBuf::from("src/examples/ecoli_kinetic_model.toml"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("src/examples/ecoli_kinetic_model.toml"),
            Position::new(1, 16),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    assert!(res.to_string().contains("g6p"));
}
