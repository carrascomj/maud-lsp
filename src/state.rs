use crate::maud_data::{KineticModel, ReactionMechanism};
use crate::metabolic::{Entity, Metabolic, MetabolicEnzyme};
use crate::priors::{Prior, Priors};
use lsp_types::{Diagnostic, Position};

use ouroboros::self_referencing;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use toml::Spanned;

const OFF: u32 = 5;

/// Both the data model and string representing the file.
#[self_referencing]
pub struct KineticModelState {
    file_str: String,
    #[borrows(file_str)]
    #[covariant]
    kinetic_model: KineticModel<'this>,
}

impl KineticModelState {
    /// Can panic. Used first time the server reads the document
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        let mut file = std::fs::File::open(path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        KineticModelStateBuilder {
            file_str: contents,
            kinetic_model_builder: |file_str| toml::from_str(file_str.as_str()).unwrap(),
        }
        .build()
    }

    /// Do not panic.
    pub fn try_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = File::open(path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        KineticModelStateTryBuilder {
            file_str: contents,
            kinetic_model_builder: |file_str| {
                toml::from_str(file_str.as_str())
                    // the error is changed because of lifetime bounds of toml::from_str in
                    // conjuction with ouroboros
                    .map_err(|_| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid kinetic model.",
                        )
                    })
            },
        }
        .try_build()
    }

    /// Return the absolute spanned value of the symbol in the data model.
    /// TODO: return the actual symbol to make it more useful
    pub fn find_symbol<'a>(&'a self, symbol: &str) -> Option<Entity<'a>> {
        let some_met = self
            .borrow_kinetic_model()
            .metabolites
            .iter()
            // TODO: handle this unwrap
            .find(|&met| met.identifier() == symbol);
        if some_met.is_some() {
            return some_met.map(Entity::Met);
        }

        let some_reac = self
            .borrow_kinetic_model()
            .reactions
            .iter()
            // TODO: handle this unwrap
            .find(|&reac| reac.identifier() == symbol)
            .map(Entity::Reac);
        if some_reac.is_some() {
            return some_reac;
        }
        self.borrow_kinetic_model()
            .enzymes
            .iter()
            // TODO: handle this unwrap
            .find(|enz| enz.id.get_ref() == &symbol)
            .map(|enz| {
                Entity::Enz(MetabolicEnzyme::from_enzyme(
                    enz,
                    self.borrow_kinetic_model().enzyme_reaction.as_slice(),
                ))
            })
    }

    /// Render a symbol str.
    pub fn find_rendered_symbol(&self, symbol: &str) -> String {
        let metabolic_entity = self.find_symbol(symbol);
        if let Some(met) = metabolic_entity {
            met.to_string()
        } else {
            String::from("")
        }
    }

    /// Find the line where a symbol is defined (for GotoDefinition).
    pub fn find_symbol_line(&self, symbol: &str) -> Option<usize> {
        let met_metabolite = self.find_symbol(symbol)?;
        Some(span_to_line_number(
            self.borrow_file_str(),
            met_metabolite.span(),
        ))
    }
}

fn span_to_line_number<T>(file_string: &str, span: &Spanned<T>) -> usize {
    file_string.get(0..span.start()).unwrap().lines().count()
}

#[self_referencing]
pub struct PriorsState {
    file_str: String,
    #[borrows(file_str)]
    pub priors: Priors,
}

impl PriorsState {
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        let mut file = File::open(path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        PriorsStateBuilder {
            file_str: contents,
            priors_builder: |file_str| toml::from_str(file_str.as_str()).unwrap(),
        }
        .build()
    }

    pub fn try_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        PriorsStateTryBuilder {
            file_str: contents,
            priors_builder: |file_str| {
                toml::from_str(file_str.as_str()).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid priors.")
                })
            },
        }
        .try_build()
    }
}

pub fn gather_diagnostics(
    kinetic_state: &KineticModelState,
    priors: &PriorsState,
) -> Vec<Diagnostic> {
    let kinetic_model = kinetic_state.borrow_kinetic_model();
    let priors = priors.borrow_priors();
    // offset to apply to the diagnostic range ("id = ")
    // check that all reactions have a corresponding enzyme
    kinetic_model
        .reactions
        .iter()
        .filter(|reac| !matches!(reac.mechanism, ReactionMechanism::Drain))
        .filter(|reac| {
            kinetic_model
                .enzyme_reaction
                .iter()
                .all(|er| er.reaction_id != reac.id.clone().into_inner())
        })
        .map(|reac| {
            let result_line = span_to_line_number(kinetic_state.borrow_file_str(), reac.span()) - 1;
            let span = reac.id.span();
            let end = (span.1 - span.0) as u32;
            Diagnostic {
                range: lsp_types::Range {
                    start: Position {
                        line: result_line as u32,
                        character: OFF,
                    },
                    end: Position {
                        line: result_line as u32,
                        character: end + OFF,
                    },
                },
                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                code: Some(lsp_types::NumberOrString::Number(0)),
                message: "Missing enzyme for reaction.".to_string(),
                ..Default::default()
            }
        })
        .chain(
            // check that all enzymes have a corresponding reaction
            kinetic_model
                .enzymes
                .iter()
                .filter(|reac| {
                    kinetic_model
                        .enzyme_reaction
                        .iter()
                        .all(|er| er.enzyme_id != reac.id.clone().into_inner())
                })
                .map(|enz: &crate::maud_data::Enzyme| {
                    let result_line =
                        span_to_line_number(kinetic_state.borrow_file_str(), &enz.id) - 1;
                    let span = enz.id.span();
                    let end = (span.1 - span.0) as u32;
                    Diagnostic {
                        range: lsp_types::Range {
                            start: Position {
                                line: result_line as u32,
                                character: OFF,
                            },
                            end: Position {
                                line: result_line as u32,
                                character: end + OFF,
                            },
                        },
                        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                        code: Some(lsp_types::NumberOrString::Number(0)),
                        message: "Missing kcat for reaction.".to_string(),
                        ..Default::default()
                    }
                }),
        )
        .chain(
            // check that all reactions have a corresponding kcat
            kinetic_model
                .reactions
                .iter()
                .filter(|reac| !matches!(reac.mechanism, ReactionMechanism::Drain))
                .filter(|reac| {
                    priors
                        .kcat
                        .iter()
                        .all(|kc| &kc.get_ref().reaction.as_str() != reac.id.get_ref())
                })
                .map(|reac| {
                    let result_line =
                        span_to_line_number(kinetic_state.borrow_file_str(), reac.span()) - 1;
                    let span = reac.id.span();
                    let end = (span.1 - span.0) as u32;
                    Diagnostic {
                        range: lsp_types::Range {
                            start: Position {
                                line: result_line as u32,
                                character: OFF,
                            },
                            end: Position {
                                line: result_line as u32,
                                character: end + OFF,
                            },
                        },
                        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                        code: Some(lsp_types::NumberOrString::Number(0)),
                        message: "Missing kcat for reaction!".to_string(),
                        ..Default::default()
                    }
                }),
        )
        .chain(
            // check that all reactions have all kms
            kinetic_model
                .reactions
                .iter()
                .filter(|reac| !matches!(reac.mechanism, ReactionMechanism::Drain))
                .map(|reac| {
                    (
                        reac,
                        kinetic_model
                            .enzyme_reaction
                            .iter()
                            .find(|er| er.reaction_id == reac.id.clone().into_inner()),
                        reac.stoichiometry.keys().copied().collect::<HashSet<_>>(),
                    )
                })
                .filter(|(_, er, _)| er.is_some())
                .filter_map(|(reac, er, st)| {
                    let er = er.as_ref().unwrap();
                    let defined_km = priors
                        .km
                        .iter()
                        .filter(|km| km.get_ref().enzyme == er.enzyme_id)
                        .map(|km| {
                            format!("{}_{}", km.get_ref().metabolite, km.get_ref().compartment)
                        })
                        .collect::<HashSet<_>>();
                    let def_km = defined_km
                        .iter()
                        .map(|x| x.as_str())
                        .collect::<HashSet<_>>();
                    let missing_km = st
                        .difference(&def_km)
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    if missing_km.is_empty() {
                        None
                    } else {
                        Some((reac, missing_km))
                    }
                })
                .map(|(reac, missing_km)| {
                    let result_line =
                        span_to_line_number(kinetic_state.borrow_file_str(), reac.span()) - 1;
                    let span = reac.id.span();
                    let end = (span.1 - span.0) as u32;
                    Diagnostic {
                        range: lsp_types::Range {
                            start: Position {
                                line: result_line as u32,
                                character: OFF,
                            },
                            end: Position {
                                line: result_line as u32,
                                character: end + OFF,
                            },
                        },
                        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                        code: Some(lsp_types::NumberOrString::Number(0)),
                        message: format!(
                            "Missing kms for reaction {}: {:?}.",
                            reac.id.clone().into_inner(),
                            missing_km
                        ),
                        ..Default::default()
                    }
                }),
        )
        .collect()
}

pub fn gather_diagnostics_priors(priors_state: &PriorsState) -> Vec<Diagnostic> {
    let km_info = priors_state.borrow_priors().km.iter().map(|km| {
        let result_line = span_to_line_number(priors_state.borrow_file_str(), km) - 1;
        let span = km.span();

        (
            result_line,
            span,
            km.get_ref().incomplete(),
            km.get_ref().inconsistent(),
        )
    });
    let kcat_info = priors_state.borrow_priors().kcat.iter().map(|kcat| {
        let result_line = span_to_line_number(priors_state.borrow_file_str(), kcat) - 1;
        let span = kcat.span();
        (
            result_line,
            span,
            kcat.get_ref().incomplete(),
            kcat.get_ref().inconsistent(),
        )
    });
    km_info
        .chain(kcat_info)
        .filter(|(_, _, err, warn)| err.is_some() || warn.is_some())
        .flat_map(|(result_line, span, err, warn)| {
            let end = (span.1 - span.0) as u32;
            [
                err.map(|err| Diagnostic {
                    range: lsp_types::Range {
                        start: Position {
                            line: result_line as u32,
                            character: OFF,
                        },
                        end: Position {
                            line: result_line as u32,
                            character: end + OFF,
                        },
                    },
                    severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                    code: Some(lsp_types::NumberOrString::Number(1)),
                    message: err.to_string(),
                    ..Default::default()
                }),
                warn.map(|warn| Diagnostic {
                    range: lsp_types::Range {
                        start: Position {
                            line: result_line as u32,
                            character: OFF,
                        },
                        end: Position {
                            line: result_line as u32,
                            character: end + OFF,
                        },
                    },
                    severity: Some(lsp_types::DiagnosticSeverity::WARNING),
                    code: Some(lsp_types::NumberOrString::Number(1)),
                    message: warn.to_string(),
                    ..Default::default()
                }),
            ]
            .into_iter()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::KineticModelState;

    #[test]
    fn finds_line_of_met_symbol() {
        let kinetic_model_state =
            KineticModelState::from_path("src/examples/ecoli_kinetic_model.toml");
        assert_eq!(kinetic_model_state.find_symbol_line("g3p"), Some(9));
        assert_eq!(kinetic_model_state.find_symbol_line("g6p"), Some(2))
    }
}
