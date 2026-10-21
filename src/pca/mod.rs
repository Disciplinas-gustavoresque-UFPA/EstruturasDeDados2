//! # Análise de Componentes Principais (PCA)
//!
//! O PCA gira os eixos de uma nuvem de dados: o primeiro eixo novo aponta
//! para a direção em que os dados mais variam, o segundo para a direção de
//! maior variação que sobra (perpendicular ao primeiro), e assim por diante.
//! A função principal é `pca(dados, k)`, que devolve as médias, os
//! autovalores, a proporção da variância explicada, os componentes e os
//! escores.
//!
//! A conta é feita em cinco etapas, cada uma num arquivo desta pasta:
//!
//! 1. `linear_algebra.rs`: operações básicas de álgebra linear (soma,
//!    subtração, produto escalar, produto matriz-vetor, produto externo,
//!    norma e normalização);
//! 2. `center.rs`: centralização dos dados;
//! 3. `covariance_matrix.rs`: matriz de covariância;
//! 4. `power_iteration.rs`: iteração de potências com deflação, para os k
//!    maiores autovalores e autovetores;
//! 5. `projection.rs`: projeção dos dados centralizados sobre os
//!    componentes.
//!
//! O arquivo `principal_components.rs` junta as etapas na função `pca`, e
//! `helpers.rs` guarda as regras usadas por todos.
//!
//! O guia completo, com o exemplo feito à mão, como rodar os testes e a
//! análise de complexidade, está no arquivo `README.md` desta pasta.

mod center;
mod covariance_matrix;
mod helpers;
mod power_iteration;
mod principal_components;
mod projection;

pub mod linear_algebra;

pub use center::{center, column_means};
pub use covariance_matrix::covariance_matrix;
pub use principal_components::{Pca, pca};
