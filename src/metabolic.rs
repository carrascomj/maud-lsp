use crate::maud_data::{Enzyme, EnzymeReaction, Metabolite, Reaction, ReactionMechanism};
use core::fmt::Display;
use std::collections::HashMap;
use toml::Spanned;

pub trait Metabolic: Display {
    fn span(&self) -> &Spanned<&str>;
    fn identifier(&self) -> &str {
        self.span().get_ref()
    }
}

impl Display for Metabolite<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metabolite = {}\nname = {}\ninchi_key = {}",
            self.id.get_ref(),
            self.name,
            self.inchi_key,
        )
    }
}

impl Metabolic for Metabolite<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.id
    }
}

impl Display for ReactionMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReactionMechanism::ReversibleMichaelisMenten => "reversible",
                ReactionMechanism::IrreversibleMichaelisMenten => "irreversible",
                ReactionMechanism::Drain => "drain",
            }
        )
    }
}

fn to_reaction_str(st: &HashMap<&str, f32>) -> String {
    let reactants = st
        .iter()
        .filter(|(_k, &v)| v < 1.0e-6)
        .map(|(k, v)| {
            if v + 1. < -1.0e-6 {
                format!("{} {k}", f32::abs(*v))
            } else {
                k.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" + ");
    let products = st
        .iter()
        .filter(|(_k, &v)| v >= 1.0e-6)
        .map(|(k, v)| {
            if v - 1. > 1.0e-6 {
                format!("{v} {k}")
            } else {
                k.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" + ");
    format!("{reactants} <=> {products}")
}

impl Display for Reaction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reaction = {}\nname = {}\nmechanism = {}\nstoichiometry = '{}'",
            self.id.get_ref(),
            self.name,
            self.mechanism,
            to_reaction_str(&self.stoichiometry),
        )
    }
}

impl Metabolic for Reaction<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.id
    }
}

pub struct MetabolicEnzyme<'a> {
    pub enzyme: &'a Enzyme<'a>,
    pub reactions: Vec<&'a str>,
}

impl<'a> MetabolicEnzyme<'a> {
    pub fn from_enzyme(enzyme: &'a Enzyme<'a>, enzyme_reactions: &[EnzymeReaction<'a>]) -> Self {
        Self {
            enzyme,
            reactions: enzyme_reactions
                .iter()
                .filter(|enz_reac| &enz_reac.enzyme_id == enzyme.id.get_ref())
                .map(|enz_reac| enz_reac.reaction_id)
                .collect(),
        }
    }
}

impl Display for MetabolicEnzyme<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "enzyme = {}\nname = {}\nsubunits = {}\nreactions = {:?}",
            self.enzyme.id.get_ref(),
            self.enzyme.name,
            self.enzyme.subunits,
            self.reactions,
        )
    }
}

impl Metabolic for MetabolicEnzyme<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.enzyme.id
    }
}

pub enum Entity<'a> {
    Met(&'a Metabolite<'a>),
    Reac(&'a Reaction<'a>),
    Enz(MetabolicEnzyme<'a>),
}

impl Display for Entity<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Met(m) => m.fmt(f),
            Entity::Reac(r) => r.fmt(f),
            Entity::Enz(e) => e.fmt(f),
        }
    }
}

impl Metabolic for Entity<'_> {
    fn span(&self) -> &Spanned<&str> {
        match self {
            Entity::Met(m) => m.span(),
            Entity::Reac(r) => r.span(),
            Entity::Enz(e) => e.span(),
        }
    }
}
