use std::io::Read;
use ouroboros::self_referencing;
use toml::Spanned;
use crate::maud_data::KineticModel;
use crate::metabolic::Metabolic;

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
    /// TODO: return the actual symbol to make it more useful
    pub fn find_symbol<'a>(&'a self, symbol: &str) -> Option<&'a (impl Metabolic + 'a)> {
        self
            .borrow_kinetic_model()
            .metabolite_in_compartments
            .iter()
            // TODO: handle this unwrap
            .find(|&met| met.identifier() == symbol)
    }

    /// Render a symbol str.
    pub fn find_rendered_symbol<'a>(&'a self, symbol: &str) -> String {
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
        Some(span_to_line_number(self.borrow_file_str(), met_metabolite.span()))
    }
}

fn span_to_line_number<T>(file_string: &str, span: &Spanned<T>) -> usize {
    file_string.get(0..span.start()).unwrap().lines().count()
}

#[cfg(test)]
mod tests{
    use super::KineticModelState;

    #[test]
    fn finds_line_of_met_symbol() {
        let kinetic_model_state =
            KineticModelState::from_path("src/examples/ecoli_kinetic_model.toml");
        assert_eq!(kinetic_model_state.find_symbol_line("g3p"), Some(9));
        assert_eq!(kinetic_model_state.find_symbol_line("g6p"), Some(2))
    }
}
