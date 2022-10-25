use serde::Deserialize;
/// Maud data model of the kinetic_model file.
use toml::Spanned;

/// Metabolites in a compartment in Maud.
#[derive(Deserialize)]
pub struct MetaboliteInCompartment<'a> {
    /// identifier, cannot contain underscores
    pub metabolite: Spanned<&'a str>,
    pub name: &'a str,
    pub compartment: &'a str,
    pub balanced: bool,
}

/// Compartments.
#[derive(Deserialize)]
pub struct Compartment<'a> {
    id: Spanned<&'a str>,
    name: &'a str,
    volume: f32,
}

/// Contains the metabolic model structural data.
#[derive(Deserialize)]
pub(crate) struct KineticModel<'a> {
    #[serde(rename = "metabolite-in-compartment", borrow)]
    pub metabolite_in_compartments: Vec<MetaboliteInCompartment<'a>>,
    #[serde(rename = "compartment", borrow)]
    pub compartments: Vec<Compartment<'a>>,
}

#[derive(Deserialize)]
pub struct MaudConfig {
    pub kinetic_model: String,
}

#[cfg(test)]
mod tests {
    use super::{Compartment, KineticModel};

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
}
