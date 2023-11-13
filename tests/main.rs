mod support;

use std::path::PathBuf;

use crate::support::Project;

use lsp_types::{
    notification::DidSaveTextDocument,
    request::{GotoDefinition, HoverRequest},
    DidSaveTextDocumentParams, GotoDefinitionParams, HoverParams, PartialResultParams, Position,
    TextDocumentPositionParams, WorkDoneProgressParams,
};

#[test]
fn goestodef_of_metabolites() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
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
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model3.toml"),
            Position::new(38, 19),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    // lines are 0-indexed!
    assert!(res.to_string().contains("\"line\":2"));
}

#[test]
fn goestodef_of_metabolites_in_csv() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));

    let res = server.send_request::<GotoDefinition>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("priors.toml"),
            Position::new(9, 19),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    });
    // lines are 0-indexed!
    assert!(res.to_string().contains('2'));
}

#[test]
fn hovers_metabolites() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
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
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("ecoli_kinetic_model.toml"),
            Position::new(27, 19),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    assert!(res.to_string().contains("g6p"));
}

#[test]
fn hovers_reaction_in_csv() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("priors.toml"),
            Position::new(41, 16),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    let res_str = res.to_string();
    assert!(res_str.contains("g3pdrain"));
    assert!(res_str.contains("mechanism"));
    assert!(res_str.contains("reaction"));
}

#[test]
fn hovers_enzyme_in_csv() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    let res = server.send_request::<HoverRequest>(HoverParams {
        text_document_position_params: TextDocumentPositionParams::new(
            server.doc_id("priors.toml"),
            Position::new(2, 14),
        ),
        work_done_progress_params: WorkDoneProgressParams::default(),
    });
    let res_str = res.to_string();
    assert!(res_str.contains("E1"));
    assert!(res_str.contains("enzyme"));
}

#[test]
fn notifications_do_not_panic() {
    let server = Project::from_kinetic_model(
        std::env::current_dir()
            .unwrap()
            .join(PathBuf::from("tests/mock")),
    )
    .server();
    // waiting a bit for the server to initialize
    std::thread::sleep(std::time::Duration::from_secs(1));
    server.notification::<DidSaveTextDocument>(DidSaveTextDocumentParams {
        text_document: server.doc_id("ecoli_kinetic_model.toml"),
        text: None,
    });
}
