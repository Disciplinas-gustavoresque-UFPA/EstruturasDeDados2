//! # Estatística descritiva
//!
//! Medidas-resumo de uma lista de números: média, moda, variância, desvio
//! padrão, coeficiente de variação, amplitude, quantis, quartis, intervalo
//! interquartílico, resumo de cinco números, covariância e correlação, e o
//! resumo completo (`summary`), que calcula todas as medidas de uma lista de
//! uma vez.
//!
//! Cada medida está num arquivo próprio desta pasta. O guia completo, com
//! exemplos de uso, como rodar os testes e a análise de complexidade, está no
//! arquivo `README.md` desta pasta.

mod helpers;

mod coefficient_of_variation;
mod correlation;
mod covariance;
mod five_number;
mod iqr;
mod mean;
mod median;
mod mode;
mod quantile;
mod quartiles;
mod range;
mod std_dev;
mod summary;
mod variance;

pub use coefficient_of_variation::coefficient_of_variation;
pub use correlation::correlation;
pub use covariance::covariance;
pub use five_number::{FiveNumberSummary, five_number_summary};
pub use iqr::iqr;
pub use mean::mean;
pub use median::median;
pub use mode::mode;
pub use quantile::quantile;
pub use quartiles::{Quartiles, quartiles};
pub use range::range;
pub use std_dev::std_dev;
pub use summary::{Summary, summary};
pub use variance::variance;
