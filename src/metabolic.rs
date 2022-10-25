use crate::maud_data::{MetaboliteInCompartment, Reaction, ReactionMechanism};
use core::fmt::Display;
use toml::Spanned;

pub trait Metabolic: Display {
    fn span(&self) -> &Spanned<&str>;
    fn identifier(&self) -> &str {
        self.span().get_ref()
    }
}

impl Display for MetaboliteInCompartment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metabolite = {}\nname = {}\ncompartment = {}\nbalanced = {}",
            self.metabolite.get_ref(),
            self.name,
            self.compartment,
            self.balanced
        )
    }
}

impl Metabolic for MetaboliteInCompartment<'_> {
    fn span(&self) -> &Spanned<&str> {
        &self.metabolite
    }
}

impl Display for ReactionMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReactionMechanism::ReversibleModularRateLaw => "reversible",
                ReactionMechanism::IrreversibleModularRateLaw => "irreversible",
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

pub enum Entity<'a> {
    Met(&'a MetaboliteInCompartment<'a>),
    Reac(&'a Reaction<'a>),
}

impl Display for Entity<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Met(m) => m.fmt(f),
            Entity::Reac(r) => r.fmt(f),
        }
    }
}

impl Metabolic for Entity<'_> {
    fn span(&self) -> &Spanned<&str> {
        match self {
            Entity::Met(m) => m.span(),
            Entity::Reac(r) => r.span(),
        }
    }
}
