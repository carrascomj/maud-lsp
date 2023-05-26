use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Priors {
    pub kcat: Vec<KcatPrior>,
    pub km: Vec<KmPrior>,
}

#[derive(Debug, Deserialize)]
pub struct KcatPrior {
    enzyme: String,
    pub reaction: String,
    #[serde(default)]
    exploc: Option<f64>,
    #[serde(default)]
    scale: Option<f64>,
    #[serde(default)]
    pct1: Option<f64>,
    #[serde(default)]
    pct99: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct KmPrior {
    pub metabolite: String,
    compartment: String,
    pub enzyme: String,
    #[serde(default)]
    exploc: Option<f64>,
    #[serde(default)]
    scale: Option<f64>,
    #[serde(default)]
    pct1: Option<f64>,
    #[serde(default)]
    pct99: Option<f64>,
}
