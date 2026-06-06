//! # noether-bridge
//!
//! Formal bridge from symplectic-fleet Noether pairs to conservation-law γ + H = C meta-law.
//!
//! Every continuous symmetry of the action yields a conserved quantity (Noether's theorem).
//! This library maps those symmetry–observable pairs into the meta-law framework
//! `γ + H = C`, verifying that each Noether contribution satisfies the conservation
//! constraint within a configurable tolerance.
//!
//! ## Modules
//!
//! - [`noether_pair`] — Symmetry-observable pairs from symplectic structure
//! - [`meta_law`] — The γ + H = C conservation law
//! - [`bridge`] — Formal mapping from Noether pairs to meta-law invariants
//! - [`verification`] — Fleet-wide conservation verification
//! - [`symmetry_registry`] — Extensible registry of known symmetries
//! - [`violation`] — Symmetry-breaking diagnostics

pub mod noether_pair;
pub mod meta_law;
pub mod bridge;
pub mod verification;
pub mod symmetry_registry;
pub mod violation;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ── Core types ──────────────────────────────────────────────────────────────

/// A Noether pair: a continuous symmetry and its associated conserved quantity.
///
/// Constructed from the symplectic structure of the fleet. The generator
/// is the Lie-algebra element (as a real vector), and `value` is the
/// current numerical value of the conserved observable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoetherPair {
    /// Human-readable name of the symmetry (e.g. "time_translation", "rotation").
    pub symmetry: String,
    /// Generator of the symmetry in Lie-algebra representation.
    pub generator: Vec<f64>,
    /// Name of the conserved quantity (e.g. "energy", "angular_momentum").
    pub conserved_quantity: String,
    /// Current measured value of the conserved quantity.
    pub value: f64,
}

/// Parameters for the meta-law γ + H = C.
///
/// For each Noether pair the bridge computes a specific `(γ, H)` such that
/// `γ + H = C` must hold within `tolerance`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaLawParams {
    /// γ contribution (symmetry-weighted term).
    pub gamma: f64,
    /// H contribution (Hamiltonian / energy-like term).
    pub h: f64,
    /// The constant C that the sum must equal.
    pub constant: f64,
    /// Absolute tolerance for satisfaction check.
    pub tolerance: f64,
}

impl MetaLawParams {
    /// Returns `true` when `|γ + H − C| ≤ tolerance`.
    pub fn is_satisfied(&self) -> bool {
        self.error().abs() <= self.tolerance
    }

    /// Returns the signed error `γ + H − C`.
    pub fn error(&self) -> f64 {
        self.gamma + self.h - self.constant
    }
}

/// Result of bridging a single Noether pair through the meta-law.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BridgeResult {
    /// The original Noether pair.
    pub pair: NoetherPair,
    /// Computed meta-law parameters.
    pub meta_params: MetaLawParams,
    /// Whether the meta-law is satisfied.
    pub satisfied: bool,
    /// Signed error γ + H − C.
    pub error: f64,
}

/// A conservation-law violation detected during fleet-wide verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Violation {
    /// Which symmetry is broken.
    pub symmetry: String,
    /// Expected value of the conserved quantity.
    pub expected: f64,
    /// Actual measured value.
    pub actual: f64,
    /// Error magnitude |actual − expected|.
    pub error: f64,
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Violation({}: expected={}, actual={}, error={:.6e})",
            self.symmetry, self.expected, self.actual, self.error
        )
    }
}

/// Fleet-wide verification report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationReport {
    /// Total number of Noether pairs checked.
    pub total_pairs: usize,
    /// Number that satisfied the meta-law.
    pub satisfied: usize,
    /// List of violations (empty if all satisfied).
    pub violations: Vec<Violation>,
}

impl VerificationReport {
    /// Returns `true` when every pair satisfies the meta-law.
    pub fn all_satisfied(&self) -> bool {
        self.violations.is_empty()
    }

    /// Fraction of pairs that satisfy the meta-law, in [0, 1].
    pub fn satisfaction_rate(&self) -> f64 {
        if self.total_pairs == 0 {
            1.0
        } else {
            self.satisfied as f64 / self.total_pairs as f64
        }
    }
}

/// An entry in the symmetry registry describing a known continuous symmetry
/// and how it maps into the meta-law framework.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymmetryEntry {
    /// Name of the symmetry (used as the registry key).
    pub name: String,
    /// Dimensionality of the Lie-algebra generator.
    pub generator_dim: usize,
    /// Name of the conserved quantity.
    pub conserved: String,
    /// Coefficient mapping the conserved-quantity value to the γ term.
    pub gamma_coeff: f64,
}
