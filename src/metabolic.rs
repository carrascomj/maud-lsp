use crate::maud_data::{Enzyme, Metabolite, Reaction, ReactionMechanism};
use core::fmt::Display;
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

impl Display for Reaction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "reaction = {}\nname = {}\nstoichiometry = {:?}\nmechanism = {}",
            self.id.get_ref(),
            self.name,
            self.stoichiometry,
            self.mechanism
        )
    }
}

impl Metabolic for Reaction<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.id
    }
}

impl Display for Enzyme<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "enzyme = {}\nname = {}\nsubunits = {}",
            self.id.get_ref(),
            self.name,
            self.subunits,
        )
    }
}

impl Metabolic for Enzyme<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.id
    }
}

pub enum Entity<'a> {
    Met(&'a Metabolite<'a>),
    Reac(&'a Reaction<'a>),
    Enz(&'a Enzyme<'a>),
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
