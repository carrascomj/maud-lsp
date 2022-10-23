/// Maud data model of the kinetic_model file.
use serde::Deserialize;

/// Metabolites in a compartment in Maud.
#[derive(Deserialize)]
struct MetaboliteInCompartment<'a> {
    /// identifier, cannot contain underscores
    metabolite: &'a str,
    name: &'a str,
    compartment: &'a str,
    balanced: bool,
}

/// Compartments.
#[derive(Deserialize)]
struct Compartment<'a> {
    id: &'a str,
    name: &'a str,
    volume: f32,
}

#[derive(Deserialize)]
pub(crate) struct KineticModel<'a> {
    #[serde(rename = "metabolite-in-compartment", borrow)]
    metabolite_in_compartments: Vec<MetaboliteInCompartment<'a>>,
    #[serde(rename = "compartment", borrow)]
    compartments: Vec<Compartment<'a>>,
}

#[cfg(test)]
mod tests {
    use super::{Compartment, KineticModel};

    #[test]
    fn test_all_comp_metabolites_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.metabolite_in_compartments.len(), 8)
    }

    #[test]
    fn test_cytosol_is_found_in_examples() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert!(kinetic_model
            .compartments
            .iter()
            .any(|Compartment { id, .. }| *id == "c"))
    }
}
