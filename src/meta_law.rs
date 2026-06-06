//! The γ + H = C meta-law.
//!
//! The conservation meta-law states that for each Noether pair, a specific
//! decomposition `γ + H = C` exists and must hold within numerical tolerance.
//! Here γ encodes the symmetry contribution and H the Hamiltonian contribution.

use crate::MetaLawParams;

/// Default tolerance for meta-law satisfaction checks.
pub const DEFAULT_TOLERANCE: f64 = 1e-9;

impl MetaLawParams {
    /// Create new meta-law parameters with explicit values.
    pub fn new(gamma: f64, h: f64, constant: f64, tolerance: f64) -> Self {
        Self {
            gamma,
            h,
            constant,
            tolerance,
        }
    }

    /// Create from γ and H, computing C = γ + H (exact satisfaction).
    pub fn exact(gamma: f64, h: f64) -> Self {
        Self {
            gamma,
            h,
            constant: gamma + h,
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    /// Create with default tolerance.
    pub fn with_tolerance(gamma: f64, h: f64, constant: f64) -> Self {
        Self::new(gamma, h, constant, DEFAULT_TOLERANCE)
    }

    /// Create with a custom tolerance.
    pub fn with_custom_tolerance(gamma: f64, h: f64, constant: f64, tolerance: f64) -> Self {
        Self::new(gamma, h, constant, tolerance)
    }

    /// Absolute error magnitude |γ + H − C|.
    pub fn abs_error(&self) -> f64 {
        self.error().abs()
    }
}

/// Compute the fleet-wide meta-law constant from a collection of (γ, H) pairs.
///
/// If the meta-law holds perfectly, every pair satisfies γᵢ + Hᵢ = C.
/// This function returns the mean of γᵢ + Hᵢ as the best-fit constant.
pub fn fleet_constant(pairs: &[(f64, f64)]) -> f64 {
    if pairs.is_empty() {
        return 0.0;
    }
    let sum: f64 = pairs.iter().map(|(g, h)| g + h).sum();
    sum / pairs.len() as f64
}

/// Check whether all (γ, H) pairs satisfy γ + H = C within tolerance.
pub fn all_satisfied(pairs: &[(f64, f64)], constant: f64, tolerance: f64) -> bool {
    pairs
        .iter()
        .all(|(g, h)| (g + h - constant).abs() <= tolerance)
}
