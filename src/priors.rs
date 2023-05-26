use serde::Deserialize;
use toml::Spanned;

#[derive(Debug, Deserialize, Clone)]
pub struct Priors {
    pub kcat: Vec<Spanned<KcatPrior>>,
    pub km: Vec<Spanned<KmPrior>>,
}

pub trait Prior {
    fn incomplete(&self) -> Option<&'static str> {
        None
    }
    fn inconsistent(&self) -> Option<&'static str> {
        None
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct KcatPrior {
    enzyme: String,
    pub reaction: String,
    #[serde(default)]
    pub exploc: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub pct1: Option<f64>,
    #[serde(default)]
    pub pct99: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KmPrior {
    pub metabolite: String,
    pub compartment: String,
    pub enzyme: String,
    #[serde(default)]
    pub exploc: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub pct1: Option<f64>,
    #[serde(default)]
    pub pct99: Option<f64>,
}

impl Prior for KmPrior {
    fn incomplete(&self) -> Option<&'static str> {
        if !((self.exploc.is_some() & self.scale.is_some())
            || (self.pct1.is_some() & self.pct99.is_some()))
        {
            Some("Incomplete prior spec. Either exploc AND scale or pct1 AND pct99 must be specified.")
        } else {
            None
        }
    }
    fn inconsistent(&self) -> Option<&'static str> {
        if (self.exploc.is_some() as usize
            + self.scale.is_some() as usize
            + self.pct1.is_some() as usize
            + self.pct99.is_some() as usize)
            > 2
        {
            Some("There are extra parameters specified. Exploc and scale take precedence over percentiles!")
        } else {
            None
        }
    }
}

impl Prior for KcatPrior {
    fn incomplete(&self) -> Option<&'static str> {
        if !((self.exploc.is_some() & self.scale.is_some())
            || (self.pct1.is_some() & self.pct99.is_some()))
        {
            Some("Incomplete prior spec. Either exploc AND scale or pct1 AND pct99 must be specified.")
        } else {
            None
        }
    }
    fn inconsistent(&self) -> Option<&'static str> {
        if (self.exploc.is_some() as usize
            + self.scale.is_some() as usize
            + self.pct1.is_some() as usize
            + self.pct99.is_some() as usize)
            > 2
        {
            Some("There are extra parameters specified. Exploc and scale take precedence over percentiles!")
        } else {
            None
        }
    }
}
