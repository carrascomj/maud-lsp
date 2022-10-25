use std::error::Error;
use std::fs::{read_to_string, File};
use std::io;
use std::io::BufRead;
use std::path::Path;

use lsp_types::{
    request::{GotoDefinition, HoverRequest},
    GotoDefinitionResponse, Hover, HoverContents, LanguageString, Location, MarkedString, Position,
    Range, Url,
};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response, ResponseError};

use crate::config::Config;
use crate::maud_data::MaudConfig;
use crate::state::KineticModelState;
use crate::symbol_parser::extract_symbol;

pub fn main_loop(
    connection: Connection,
    config: Config,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let maud_config: MaudConfig =
        toml::from_str(&read_to_string(config.root_dir.join("config.toml"))?)?;
    let kinetic_state =
        KineticModelState::from_path(config.root_dir.join(&maud_config.kinetic_model));
    let kinetic_model_uri =
        Url::from_file_path(config.root_dir.join(maud_config.kinetic_model)).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
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
                        let symbol = extract_symbol(&line_str, col as usize);
                        // handle this unwrap
                        let result_line = kinetic_state.find_symbol_line(symbol.unwrap()) - 1;
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
                        continue;
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
                        let symbol = extract_symbol(&line_str, col as usize);
                        // handle this unwrap
                        let result_symbol = kinetic_state.find_rendered_symbol(symbol.unwrap());
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
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{:?}", err),
                    Err(ExtractError::MethodMismatch(req)) => req.id,
                };
                // this should not really happen since we declare our capabilties beforehand
                let no_idea_resp = Response {
                    id: req_id,
                    result: None,
                    error: Some(ResponseError {
                        code: 500,
                        message: "Method not implemented".to_string(),
                        data: None,
                    }),
                };
                connection.sender.send(Message::Response(no_idea_resp))?;
            }
            Message::Response(resp) => {
                eprintln!("got response: {:?}", resp);
            }
            Message::Notification(not) => {
                eprintln!("got notification: {:?}", not);
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn read_line<P: AsRef<Path>>(file_path: P, line: u32) -> io::Result<String> {
    let input = File::open(file_path)?;
    let buffered = io::BufReader::new(input);
    buffered
        .lines()
        .nth(line as usize)
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?
}
