use crate::maud_data::KineticModel;
use crate::metabolic::{Entity, Metabolic, MetabolicEnzyme};
use crate::priors::Priors;
use ouroboros::self_referencing;
use std::fs::File;
use std::io::Read;
use toml::Spanned;

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
