/// Maud data model of the kinetic_model file.
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use std::collections::{BTreeMap, HashMap};
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

#[derive(Deserialize)]
pub enum ReactionMechanism {
    IrreversibleModularRateLaw,
    ReversibleModularRateLaw,
    Drain,
}

fn deserialize_reaction_mechanism<'de, D>(de: D) -> Result<ReactionMechanism, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut deser_result: String = serde::Deserialize::deserialize(de)?;
    deser_result = deser_result.to_lowercase();
    match deser_result.as_str() {
        "irreversible_modular_rate_law" => Ok(ReactionMechanism::IrreversibleModularRateLaw),
        "reversible_modular_rate_law" => Ok(ReactionMechanism::ReversibleModularRateLaw),
        "drain" => Ok(ReactionMechanism::Drain),
        _ => Err(serde::de::Error::custom("Invalid reaction mechanism")),
    }
}

/// Reaction that points to metabolites.
#[derive(Deserialize)]
pub struct Reaction<'a> {
    /// identifier, cannot contain underscores
    pub id: Spanned<&'a str>,
    pub name: &'a str,
    pub stoichiometry: HashMap<&'a str, i16>,
    #[serde(deserialize_with = "deserialize_reaction_mechanism")]
    pub mechanism: ReactionMechanism,
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
    #[serde(rename = "reaction", borrow)]
    pub reactions: Vec<Reaction<'a>>,
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
    fn all_reactions_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.reactions.len(), 5)
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
