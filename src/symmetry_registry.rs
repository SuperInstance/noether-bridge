//! Extensible registry of known symmetries and their conservation-law consequences.
//!
//! The registry maps symmetry names to [`SymmetryEntry`] records that describe
//! how each symmetry maps into the meta-law framework.

use crate::SymmetryEntry;
use std::collections::HashMap;

/// The symmetry registry: an extensible map from symmetry names to entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymmetryRegistry {
    entries: HashMap<String, SymmetryEntry>,
}

impl SymmetryRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Create a registry pre-populated with the standard physical symmetries.
    pub fn standard() -> Self {
        let mut reg = Self::new();
        reg.register(SymmetryEntry {
            name: "time_translation".into(),
            generator_dim: 1,
            conserved: "energy".into(),
            gamma_coeff: 1.0,
        });
        reg.register(SymmetryEntry {
            name: "rotation".into(),
            generator_dim: 3,
            conserved: "angular_momentum".into(),
            gamma_coeff: 0.5,
        });
        reg.register(SymmetryEntry {
            name: "phase_rotation".into(),
            generator_dim: 2,
            conserved: "particle_number".into(),
            gamma_coeff: 1.0,
        });
        reg.register(SymmetryEntry {
            name: "spatial_translation".into(),
            generator_dim: 3,
            conserved: "linear_momentum".into(),
            gamma_coeff: 1.0,
        });
        reg.register(SymmetryEntry {
            name: "galilean_boost".into(),
            generator_dim: 3,
            conserved: "center_of_mass_velocity".into(),
            gamma_coeff: 0.5,
        });
        reg.register(SymmetryEntry {
            name: "scale".into(),
            generator_dim: 1,
            conserved: "dilation_charge".into(),
            gamma_coeff: 2.0,
        });
        reg
    }

    /// Register a new symmetry entry (or replace an existing one).
    pub fn register(&mut self, entry: SymmetryEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    /// Look up a symmetry by name.
    pub fn get(&self, name: &str) -> Option<&SymmetryEntry> {
        self.entries.get(name)
    }

    /// List all registered symmetry names.
    pub fn symmetries(&self) -> Vec<&str> {
        self.entries.keys().map(|s| s.as_str()).collect()
    }

    /// Number of registered symmetries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Is the registry empty?
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over all entries.
    pub fn iter(&self) -> impl Iterator<Item = &SymmetryEntry> {
        self.entries.values()
    }
}

impl Default for SymmetryRegistry {
    fn default() -> Self {
        Self::standard()
    }
}

use serde::{Deserialize, Serialize};
