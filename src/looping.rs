use std::error::Error;
use std::fs::{read_to_string, File};
use std::io;
use std::io::BufRead;
use std::path::Path;

use lsp_types::{
    request::{GotoDefinition, HoverRequest},
    GotoDefinitionResponse, Hover, HoverContents, LanguageString, Location, MarkedString, Position,
    PublishDiagnosticsParams, Range, TextDocumentIdentifier, Url,
};

use lsp_server::{
    Connection, ExtractError, Message, Notification, Request, RequestId, Response, ResponseError,
};

use crate::config::Config;
use crate::maud_data::MaudConfig;
use crate::state::{gather_diagnostics, KineticModelState, PriorsState};
use crate::symbol_parser::extract_symbol;

pub fn main_loop(
    connection: Connection,
    config: Config,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let maud_config: MaudConfig =
        toml::from_str(&read_to_string(config.root_dir.join("config.toml"))?)?;
    let mut kinetic_state =
        KineticModelState::from_path(config.root_dir.join(&maud_config.kinetic_model_file));
    let mut priors_state = PriorsState::from_path(config.root_dir.join(&maud_config.priors_file));
    let kinetic_model_uri =
        Url::from_file_path(config.root_dir.join(&maud_config.kinetic_model_file)).unwrap();
    let _priors_uri = Url::from_file_path(config.root_dir.join(&maud_config.priors_file)).unwrap();
    for msg in &connection.receiver {
        match match_message(msg, &connection, &kinetic_state, &kinetic_model_uri) {
            Ok(Some(OkMsg::OkNotFound { id, msg })) => {
                let no_idea_resp = Response {
                    id,
                    result: None,
                    error: Some(ResponseError {
                        code: 500,
                        message: msg,
                        data: None,
                    }),
                };
                connection.sender.send(Message::Response(no_idea_resp))?;
            }
            Ok(Some(OkMsg::Shutdown)) => return Ok(()),
            Ok(Some(OkMsg::DidSave(text_document))) => {
                if Some(text_document.uri.path())
                    == config
                        .root_dir
                        .join(&maud_config.kinetic_model_file)
                        .to_str()
                {
                    // the model file may have been saved in an invalid state
                    // only update the data model if it is valid
                    if let Ok(state) = KineticModelState::try_from_path(
                        config.root_dir.join(&maud_config.kinetic_model_file),
                    ) {
                        kinetic_state = state;
                        let diagnostics = gather_diagnostics(&kinetic_state, &priors_state);
                        let diagnostics = PublishDiagnosticsParams {
                            uri: kinetic_model_uri.clone(),
                            diagnostics,
                            version: None,
                        };
                        connection.sender.send(Message::Notification(Notification {
                            method: "textDocument/publishDiagnostics".to_string(),
                            params: serde_json::to_value(diagnostics).unwrap(),
                        }))?;
                    }
                } else if Some(text_document.uri.path())
                    == config.root_dir.join(&maud_config.priors_file).to_str()
                {
                    // the priors file may have been saved in an invalid state
                    // only update the data model if it is valid
                    if let Ok(state) =
                        PriorsState::try_from_path(config.root_dir.join(&maud_config.priors_file))
                    {
                        priors_state = state;
                        let diagnostics = gather_diagnostics(&kinetic_state, &priors_state);
                        let diagnostics = PublishDiagnosticsParams {
                            uri: kinetic_model_uri.clone(),
                            diagnostics,
                            version: None,
                        };
                        connection.sender.send(Message::Notification(Notification {
                            method: "textDocument/publishDiagnostics".to_string(),
                            params: serde_json::to_value(diagnostics).unwrap(),
                        }))?;
                    }
                }
            }
            Err(e) => panic!("{:?}", e),
            _ => (),
        }
    }
    Ok(())
}

enum OkMsg {
    /// Something was not found but the server does not have to crash.
    OkNotFound { id: RequestId, msg: String },
    /// Received shutdown, return Ok
    Shutdown,
    /// Kinetic Model was saved
    DidSave(TextDocumentIdentifier),
}

fn match_message(
    msg: Message,
    connection: &Connection,
    kinetic_state: &KineticModelState,
    kinetic_model_uri: &Url,
) -> Result<Option<OkMsg>, Box<dyn Error + Sync + Send>> {
    match msg {
        Message::Request(req) => {
            if connection.handle_shutdown(&req)? {
                return Ok(Some(OkMsg::Shutdown));
            }
            let passed_req = match cast::<GotoDefinition>(req) {
                Ok((id, params)) => {
                    let (row, col) = (
                        params.text_document_position_params.position.line,
                        params.text_document_position_params.position.character,
                    );
                    // TOD check the uri is a valid absolute path
                    let line_str = read_line(
                        params
                            .text_document_position_params
                            .text_document
                            .uri
                            .path(),
                        row,
                    )?;
                    let symbol = match extract_symbol(&line_str, col as usize) {
                        Some(s) => s,
                        None => {
                            return Ok(Some(OkMsg::OkNotFound {
                                id,
                                msg: format!("Valid symbol at {},{} Not Found", line_str, col),
                            }))
                        }
                    };
                    let result_line = match kinetic_state.find_symbol_line(symbol) {
                        Some(line) => line - 1,
                        None => {
                            return Ok(Some(OkMsg::OkNotFound {
                                id,
                                msg: format!("Symbol {} Not Found in Kinetic Model", symbol),
                            }))
                        }
                    };
                    // the way of finding the symbol on the cursor changes between
                    // maud CSVs and kinetic models.
                    // TODO(carrascomj): we are only handling the kinetic model
                    let result = Some(GotoDefinitionResponse::Scalar(Location {
                        uri: kinetic_model_uri.clone(),
                        range: Range {
                            start: Position {
                                line: result_line as u32,
                                character: 0,
                            },
                            end: Position {
                                line: result_line as u32,
                                character: 0,
                            },
                        },
                    }));
                    let result = serde_json::to_value(&result).unwrap();
                    let resp = Response {
                        id,
                        result: Some(result),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    return Ok(None);
                }
                Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                Err(ExtractError::MethodMismatch(req)) => req,
            };
            let req_id = match cast::<HoverRequest>(passed_req) {
                Ok((id, params)) => {
                    let (row, col) = (
                        params.text_document_position_params.position.line,
                        params.text_document_position_params.position.character,
                    );
                    // TODO: check the uri is a valid absolute path
                    let line_str = read_line(
                        params
                            .text_document_position_params
                            .text_document
                            .uri
                            .path(),
                        row,
                    )?;
                    let symbol = match extract_symbol(&line_str, col as usize) {
                        Some(s) => s,
                        None => {
                            return Ok(Some(OkMsg::OkNotFound {
                                id,
                                msg: format!("Valid symbol at {},{} Not Found", line_str, col),
                            }))
                        }
                    };
                    let result_symbol = kinetic_state.find_rendered_symbol(symbol);
                    // the way of finding the symbol on the cursor changes between
                    // maud CSVs and kinetic models.
                    let result = Some(Hover {
                        contents: HoverContents::Scalar(MarkedString::LanguageString(
                            LanguageString {
                                language: "toml".to_string(),
                                value: result_symbol,
                            },
                        )),
                        range: None,
                    });
                    // TODO: handle this unwrap
                    let result = serde_json::to_value(&result)?;
                    let resp = Response {
                        id,
                        result: Some(result),
                        error: None,
                    };
                    connection.sender.send(Message::Response(resp))?;
                    return Ok(None);
                }
                Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                Err(ExtractError::MethodMismatch(req)) => req.id,
            };
            // this should not really happen since we declare our capabilties beforehand
            Ok(Some(OkMsg::OkNotFound {
                id: req_id,
                msg: "Method not implemented".to_string(),
            }))
        }
        Message::Response(resp) => {
            eprintln!("got response: {:?}", resp);
            Ok(None)
        }
        Message::Notification(not) => {
            match cast_not::<lsp_types::notification::DidSaveTextDocument>(not) {
                Ok(params) => return Ok(Some(OkMsg::DidSave(params.text_document))),
                _ => {
                    eprintln!("got unhandled notification");
                }
            }

            Ok(None)
        }
    }
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_not<N>(not: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned + Send,
{
    not.extract(N::METHOD)
}

fn read_line<P: AsRef<Path>>(file_path: P, line: u32) -> io::Result<String> {
    let input = File::open(file_path)?;
    let buffered = io::BufReader::new(input);
    buffered
        .lines()
        .nth(line as usize)
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?
}
