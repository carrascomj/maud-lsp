mod support;

use std::path::PathBuf;

use crate::support::Project;

use lsp_types::{
    request::{GotoDefinition, HoverRequest},
    GotoDefinitionParams, HoverParams, PartialResultParams, Position, TextDocumentPositionParams,
    WorkDoneProgressParams,
};

#[test]
fn goestodef_of_metabolites() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(2, 8),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    // lines are 0-indexed!
    assert!(res.to_string().contains('2'));
}

#[test]
fn goestodef_of_metabolite_reactant() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(39, 18),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    // lines are 0-indexed!
    assert!(res.to_string().contains("\"line\":2"));
}

#[test]
fn goestodef_of_metabolites_in_csv() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("priors.csv"),
            Position::new(8, 5),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    // lines are 0-indexed!
    assert!(res.to_string().contains('2'));
}

#[test]
fn hovers_metabolites() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(2, 8),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    assert!(res.to_string().contains("f6p"));
}

#[test]
fn hovers_metabolite_reactant() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(24, 19),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    assert!(res.to_string().contains("g6p"));
}

#[test]
fn hovers_reaction_in_csv() {
    let server =
        Project::from_kinetic_model(PathBuf::from("/home/georg/git/maud-lsp/src/examples"))
            .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("priors.csv"),
            Position::new(4, 9),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    let res_str = res.to_string();
    assert!(res_str.contains("FBA"));
    assert!(res_str.contains("dhap_c"));
}
