use crate::maud_data::MetaboliteInCompartment;
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
            "metabolite = {}\nname = {}\ncomparment = {}\nbalanced ={}",
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
