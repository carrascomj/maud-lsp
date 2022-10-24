use ouroboros::self_referencing;
use serde::Deserialize;
/// Maud data model of the kinetic_model file.
use std::io::Read;
use toml::Spanned;

/// Metabolites in a compartment in Maud.
#[derive(Deserialize)]
struct MetaboliteInCompartment<'a> {
    /// identifier, cannot contain underscores
    metabolite: Spanned<&'a str>,
    name: &'a str,
    compartment: &'a str,
    balanced: bool,
}

/// Compartments.
#[derive(Deserialize)]
struct Compartment<'a> {
    id: Spanned<&'a str>,
    name: &'a str,
    volume: f32,
}

/// Contains the metabolic model structural data.
#[derive(Deserialize)]
pub(crate) struct KineticModel<'a> {
    #[serde(rename = "metabolite-in-compartment", borrow)]
    metabolite_in_compartments: Vec<MetaboliteInCompartment<'a>>,
    #[serde(rename = "compartment", borrow)]
    compartments: Vec<Compartment<'a>>,
}

/// Both the data model and string representing the file.
#[self_referencing]
pub struct KineticModelState {
    file_str: String,
    #[borrows(file_str)]
    #[covariant]
    kinetic_model: KineticModel<'this>,
}

impl KineticModelState {
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

    /// Return the absolute spanned value of the symbol in the data model.
    /// TODO: return the actual symbol to render useful information
    pub fn find_symbol<'a>(&'a self, symbol: &str) -> &'a Spanned<&'a str> {
        let met = self.borrow_kinetic_model()
            .metabolite_in_compartments
            .iter()
            // TODO: handle this unwrap
            .find(|&met| met.metabolite.get_ref() == &symbol)
            .unwrap();
        &met.metabolite
    }

    /// Find the line where a symbol is defined (for GotoDefinition).
    pub fn find_symbol_line(&self, symbol: &str) -> usize {
        let met_metabolite = self.find_symbol(symbol);
        span_to_line_number(self.borrow_file_str(), &met_metabolite)
    }
}

#[derive(Deserialize)]
pub struct MaudConfig {
    pub kinetic_model: String,
}

fn span_to_line_number<T>(file_string: &str, span: &Spanned<T>) -> usize {
    file_string.get(0..span.start()).unwrap().lines().count()
}

#[cfg(test)]
mod tests {
    use super::{Compartment, KineticModel, KineticModelState};

    #[test]
    fn all_comp_metabolites_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.metabolite_in_compartments.len(), 8)
    }

    #[test]
    fn cytosol_is_found_in_examples() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert!(kinetic_model
            .compartments
            .iter()
            .any(|Compartment { id, .. }| *id.get_ref() == "c"))
    }

    #[test]
    fn cytosol_is_found_in_example_state() {
        let kinetic_model_state =
            KineticModelState::from_path("src/examples/ecoli_kinetic_model.toml");
        assert!(kinetic_model_state
            .borrow_kinetic_model()
            .compartments
            .iter()
            .any(|Compartment { id, .. }| *id.get_ref() == "c"))
    }

    #[test]
    fn finds_line_of_met_symbol() {
        let kinetic_model_state =
            KineticModelState::from_path("src/examples/ecoli_kinetic_model.toml");
        assert_eq!(kinetic_model_state.find_symbol_line("g3p"), 9);
        assert_eq!(kinetic_model_state.find_symbol_line("g6p"), 2)
    }
}
