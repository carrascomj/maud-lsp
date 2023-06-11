use serde::Deserialize;

/// Experiment with measurements.
#[derive(Deserialize)]
pub struct Experiment<'a> {
    /// identifier, cannot contain underscores
    pub id: &'a str,
    pub is_train: bool,
    pub is_test: bool,
    pub temperature: f32,
    #[serde(default)]
    measurements: Vec<Measurement<'a>>,
}

#[derive(Deserialize)]
struct Measurement<'a> {
    target_type: &'a str,
    metabolite: Option<&'a str>,
    reaction: Option<&'a str>,
    compartment: Option<&'a str>,
    value: f32,
    error_scale: f32,
}

#[derive(Deserialize)]
pub(crate) struct ExperimentData<'a> {
    #[serde(rename = "experiment", borrow)]
    experiments: Vec<Experiment<'a>>,
}

impl ExperimentData<'_> {
    pub fn experiments(&self) -> Vec<String> {
        self.experiments.iter().map(|x| x.id.to_string()).collect()
    }
}
