/// Maud data model of the kinetic_model file.
use serde::Deserialize;
use std::collections::HashMap;
use toml::Spanned;

/// Metabolites in a compartment in Maud.
#[derive(Deserialize)]
pub struct Metabolite<'a> {
    /// identifier, cannot contain underscores
    pub id: Spanned<&'a str>,
    pub name: &'a str,
    pub inchi_key: &'a str,
}

#[derive(Deserialize)]
pub enum ReactionMechanism {
    IrreversibleMichaelisMenten,
    ReversibleMichaelisMenten,
    Drain,
}

fn deserialize_reaction_mechanism<'de, D>(de: D) -> Result<ReactionMechanism, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut deser_result: String = serde::Deserialize::deserialize(de)?;
    deser_result = deser_result.to_lowercase();
    match deser_result.as_str() {
        "irreversible_michaelis_menten" => Ok(ReactionMechanism::IrreversibleMichaelisMenten),
        "reversible_michaelis_menten" => Ok(ReactionMechanism::ReversibleMichaelisMenten),
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
    pub stoichiometry: HashMap<&'a str, f32>,
    #[serde(deserialize_with = "deserialize_reaction_mechanism")]
    pub mechanism: ReactionMechanism,
}

/// Enzyme that catalyzes 1 or more reactions.
#[derive(Deserialize)]
pub struct Enzyme<'a> {
    /// identifier, cannot contain underscores
    pub id: Spanned<&'a str>,
    pub name: &'a str,
    pub subunits: u16,
}

/// Table from enzyme to reaction
#[derive(Deserialize)]
pub struct EnzymeReaction<'a> {
    /// identifier, cannot contain underscores
    pub enzyme_id: &'a str,
    pub reaction_id: &'a str,
}

/// Table from enzyme to reaction
#[derive(Deserialize)]
pub struct MetaboliteInCompartment<'a> {
    /// identifier, cannot contain underscores
    pub metabolite_id: &'a str,
    pub compartment_id: &'a str,
    pub balanced: bool,
}

/// Contains the metabolic model structural data.
#[derive(Deserialize)]
pub(crate) struct KineticModel<'a> {
    #[serde(rename = "metabolite", borrow)]
    pub metabolites: Vec<Metabolite<'a>>,
    #[serde(rename = "reaction", borrow)]
    pub reactions: Vec<Reaction<'a>>,
    #[serde(rename = "enzyme", borrow)]
    pub enzymes: Vec<Enzyme<'a>>,
    #[serde(borrow)]
    pub enzyme_reaction: Vec<EnzymeReaction<'a>>,
    #[serde(borrow)]
    pub metabolite_in_compartment: Vec<MetaboliteInCompartment<'a>>,
}

#[derive(Deserialize)]
pub struct MaudConfig {
    pub kinetic_model_file: String,
    pub priors_file: String,
    pub experiments_file: String,
}

#[cfg(test)]
mod tests {
    use super::KineticModel;

    #[test]
    fn all_comp_metabolites_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.metabolites.len(), 8)
    }

    #[test]
    fn all_reactions_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.reactions.len(), 6)
    }

    #[test]
    fn all_enzymes_are_deserialized() {
        let kinetic_model: KineticModel =
            toml::from_str(include_str!("examples/ecoli_kinetic_model.toml")).unwrap();
        assert_eq!(kinetic_model.enzymes.len(), 2)
    }
}
