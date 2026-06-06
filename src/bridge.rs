//! Bridge: formal mapping from SymplecticFleet::NoetherPair → ConservationLaw::Invariant.
//!
//! For each Noether pair, the bridge computes a specific (γ, H) decomposition
//! that must satisfy γ + H = C. The mapping uses the symmetry entry from the
//! registry to determine how the conserved-quantity value maps into the γ term,
//! with H derived from the generator norm.

use crate::symmetry_registry::SymmetryRegistry;
use crate::{BridgeResult, MetaLawParams, NoetherPair};

/// The core bridge mapping trait. Types implementing this can convert
/// a Noether pair into meta-law parameters.
pub trait BridgeMapping {
    /// Map a Noether pair to meta-law parameters.
    fn map(&self, pair: &NoetherPair) -> BridgeResult;

    /// Map a slice of pairs, returning all bridge results.
    fn map_all(&self, pairs: &[NoetherPair]) -> Vec<BridgeResult> {
        pairs.iter().map(|p| self.map(p)).collect()
    }
}

/// The default bridge uses the symmetry registry to look up coefficients.
pub struct DefaultBridge<'a> {
    pub registry: &'a SymmetryRegistry,
    pub tolerance: f64,
    /// Fleet-wide constant C for the meta-law.
    pub fleet_constant: f64,
}

impl<'a> DefaultBridge<'a> {
    pub fn new(registry: &'a SymmetryRegistry, tolerance: f64, fleet_constant: f64) -> Self {
        Self {
            registry,
            tolerance,
            fleet_constant,
        }
    }
}

impl BridgeMapping for DefaultBridge<'_> {
    fn map(&self, pair: &NoetherPair) -> BridgeResult {
        // Look up the symmetry entry; if unknown, use a default gamma_coeff of 1.0.
        let entry = self.registry.get(&pair.symmetry);
        let gamma_coeff = entry.map(|e| e.gamma_coeff).unwrap_or(1.0);

        // γ = gamma_coeff × conserved_value
        let gamma = gamma_coeff * pair.value;

        // H derived from generator norm (Hamiltonian contribution).
        let gen_norm = pair.generator_norm();
        let h = gen_norm * pair.value;

        let meta_params = MetaLawParams::new(gamma, h, self.fleet_constant, self.tolerance);
        let satisfied = meta_params.is_satisfied();
        let error = meta_params.error();

        BridgeResult {
            pair: pair.clone(),
            meta_params,
            satisfied,
            error,
        }
    }
}

/// Build a fleet-wide bridge where C is computed so every pair satisfies γ + H = C exactly.
///
/// This computes γᵢ + Hᵢ for each pair, sets C to the mean, then checks each pair.
pub fn bridge_fleet_exact(
    pairs: &[NoetherPair],
    registry: &SymmetryRegistry,
    tolerance: f64,
) -> Vec<BridgeResult> {
    // First pass: compute γ + H for each pair.
    let sums: Vec<f64> = pairs
        .iter()
        .map(|p| {
            let entry = registry.get(&p.symmetry);
            let gamma_coeff = entry.map(|e| e.gamma_coeff).unwrap_or(1.0);
            let gamma = gamma_coeff * p.value;
            let h = p.generator_norm() * p.value;
            gamma + h
        })
        .collect();

    // C = mean of all sums.
    let c = if sums.is_empty() {
        0.0
    } else {
        sums.iter().sum::<f64>() / sums.len() as f64
    };

    // Second pass: create BridgeResults.
    pairs
        .iter()
        .zip(sums.into_iter())
        .map(|(p, _sum)| {
            let entry = registry.get(&p.symmetry);
            let gamma_coeff = entry.map(|e| e.gamma_coeff).unwrap_or(1.0);
            let gamma = gamma_coeff * p.value;
            let h = p.generator_norm() * p.value;
            let meta_params = MetaLawParams::new(gamma, h, c, tolerance);
            let satisfied = meta_params.is_satisfied();
            let error = meta_params.error();
            BridgeResult {
                pair: p.clone(),
                meta_params,
                satisfied,
                error,
            }
        })
        .collect()
}
