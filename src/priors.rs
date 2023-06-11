use serde::Deserialize;
use toml::Spanned;

#[derive(Debug, Deserialize, Clone)]
pub struct Priors {
    pub kcat: Vec<Spanned<KcatPrior>>,
    pub km: Vec<Spanned<KmPrior>>,
    pub conc_enzyme: Vec<Spanned<ConcEnzyme>>,
    pub conc_unbalanced: Vec<Spanned<ConcUnbalanced>>,
    pub drain: Vec<Spanned<Drain>>,
    pub dgf: Option<Spanned<Dgf>>,
}

pub trait Prior {
    /// Incomplete priors generate an diagnostic error.
    fn incomplete(&self) -> Option<&'static str> {
        None
    }
    /// Inconsistent priors generate an diagnostic error.
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

#[derive(Debug, Deserialize, Clone)]
pub struct ConcEnzyme {
    enzyme: String,
    pub experiment: String,
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
pub struct ConcUnbalanced {
    pub metabolite: String,
    pub compartment: String,
    experiment: String,
    #[serde(default)]
    pub exploc: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub pct1: Option<f64>,
    #[serde(default)]
    pub pct99: Option<f64>,
}

impl ConcUnbalanced {
    pub fn mean(&self) -> Option<f64> {
        match (self.exploc, self.scale, self.pct1, self.pct99) {
            (Some(mean), Some(_), _, _) => Some(mean),
            (_, _, Some(lower), Some(upper)) => Some((upper + lower) / 2.),
            _ => None,
        }
    }
}
impl KmPrior {
    pub fn mean(&self) -> Option<f64> {
        match (self.exploc, self.scale, self.pct1, self.pct99) {
            (Some(mean), Some(_), _, _) => Some(mean),
            (_, _, Some(lower), Some(upper)) => Some((upper + lower) / 2.),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Drain {
    pub reaction: String,
    pub experiment: String,
    #[serde(default)]
    pub location: Option<f64>,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub pct1: Option<f64>,
    #[serde(default)]
    pub pct99: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dgf {
    ids: Vec<String>,
    mean_vector: Vec<f64>,
    covariance_matrix: Vec<Vec<f64>>,
}

fn is_positive_definite(matrix: &Vec<Vec<f64>>) -> bool {
    let n = matrix.len();
    let mut chol = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut sum = matrix[i][j];
            for k in 0..j {
                sum -= chol[i][k] * chol[j][k];
            }
            if i == j {
                chol[i][j] = sum.sqrt();
            } else {
                chol[i][j] = sum / chol[j][j];
            }
        }
        if chol[i][i] <= 0.0 {
            return false;
        }
    }
    true
}

fn is_square(matrix: &Vec<Vec<f64>>) -> bool {
    let n = matrix.len();
    matrix.iter().all(|row| row.len() == n)
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

impl Prior for ConcEnzyme {
    fn incomplete(&self) -> Option<&'static str> {
        if !((self.exploc.is_some() & self.scale.is_some())
            || (self.pct1.is_some() & self.pct99.is_some()))
        {
            Some("Incomplete prior spec. Both exploc and scale must be specified.")
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
            Some("There are extra parameters specified. exploc and scale take precedence over percentiles!")
        } else {
            None
        }
    }
}

impl Prior for Drain {
    fn incomplete(&self) -> Option<&'static str> {
        if !((self.location.is_some() & self.scale.is_some())
            || (self.pct1.is_some() & self.pct99.is_some()))
        {
            Some("Incomplete prior spec. Both exploc and scale must be specified.")
        } else {
            None
        }
    }

    fn inconsistent(&self) -> Option<&'static str> {
        if (self.location.is_some() as usize
            + self.scale.is_some() as usize
            + self.pct1.is_some() as usize
            + self.pct99.is_some() as usize)
            > 2
        {
            Some("There are extra parameters specified. exploc and scale take precedence over percentiles!")
        } else {
            None
        }
    }
}

impl Prior for ConcUnbalanced {
    fn incomplete(&self) -> Option<&'static str> {
        if !((self.exploc.is_some() & self.scale.is_some())
            || (self.pct1.is_some() & self.pct99.is_some()))
        {
            Some("Incomplete prior spec. Both exploc and scale must be specified.")
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
            Some("There are extra parameters specified. exploc and scale take precedence over percentiles!")
        } else {
            None
        }
    }
}

impl Prior for Dgf {
    fn incomplete(&self) -> Option<&'static str> {
        if !is_square(&self.covariance_matrix) {
            Some("Covariance matrix is not square.")
        } else if !is_positive_definite(&self.covariance_matrix) {
            Some("Covariance matrix is not positive definite.")
        } else if self.ids.len() != self.mean_vector.len() {
            Some("Ids and mean vector have different lengths.")
        } else if self.ids.len() != self.covariance_matrix[0].len() {
            Some("Ids and covariance matrix have different lengths.")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive_definite() {
        let matrix = vec![vec![1.0, 0.5], vec![0.5, 1.0]];
        assert!(is_positive_definite(&matrix));

        //let matrix = vec![vec![1.0, 2.0], vec![2.5, 1.0]];
        // assert!(!is_positive_definite(&matrix));
    }
}
