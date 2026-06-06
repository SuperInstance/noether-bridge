//! Noether pair construction and utilities.
//!
//! A Noether pair `(generator, conserved_quantity)` is the fundamental output of
//! Noether's theorem applied to a symplectic structure. This module provides
//! constructors for the standard physical symmetries.

use crate::NoetherPair;

/// Standard physical symmetry names.
pub mod symmetries {
    pub const TIME_TRANSLATION: &str = "time_translation";
    pub const ROTATION: &str = "rotation";
    pub const PHASE_ROTATION: &str = "phase_rotation";
    pub const SPATIAL_TRANSLATION: &str = "spatial_translation";
    pub const GALILEAN_BOOST: &str = "galilean_boost";
    pub const SCALE: &str = "scale";
}

/// Standard conserved quantity names.
pub mod conserved {
    pub const ENERGY: &str = "energy";
    pub const ANGULAR_MOMENTUM: &str = "angular_momentum";
    pub const PARTICLE_NUMBER: &str = "particle_number";
    pub const LINEAR_MOMENTUM: &str = "linear_momentum";
    pub const CENTER_OF_MASS_VELOCITY: &str = "center_of_mass_velocity";
    pub const DILATION_CHARGE: &str = "dilation_charge";
}

impl NoetherPair {
    /// Construct a time-translation Noether pair (energy conservation).
    pub fn time_translation(energy: f64) -> Self {
        Self {
            symmetry: symmetries::TIME_TRANSLATION.to_string(),
            generator: vec![1.0], // 1D generator (time direction)
            conserved_quantity: conserved::ENERGY.to_string(),
            value: energy,
        }
    }

    /// Construct a rotation Noether pair (angular-momentum conservation).
    ///
    /// `generator` is the 3D angular-momentum generator axis.
    pub fn rotation(angular_momentum: f64, generator: Vec<f64>) -> Self {
        Self {
            symmetry: symmetries::ROTATION.to_string(),
            generator,
            conserved_quantity: conserved::ANGULAR_MOMENTUM.to_string(),
            value: angular_momentum,
        }
    }

    /// Construct a phase-rotation Noether pair (particle-number conservation).
    pub fn phase_rotation(particle_number: f64) -> Self {
        Self {
            symmetry: symmetries::PHASE_ROTATION.to_string(),
            generator: vec![0.0, 1.0], // U(1) generator
            conserved_quantity: conserved::PARTICLE_NUMBER.to_string(),
            value: particle_number,
        }
    }

    /// Construct a spatial-translation Noether pair (linear-momentum conservation).
    pub fn spatial_translation(linear_momentum: f64, direction: Vec<f64>) -> Self {
        Self {
            symmetry: symmetries::SPATIAL_TRANSLATION.to_string(),
            generator: direction,
            conserved_quantity: conserved::LINEAR_MOMENTUM.to_string(),
            value: linear_momentum,
        }
    }

    /// Generic constructor.
    pub fn new(
        symmetry: impl Into<String>,
        generator: Vec<f64>,
        conserved_quantity: impl Into<String>,
        value: f64,
    ) -> Self {
        Self {
            symmetry: symmetry.into(),
            generator,
            conserved_quantity: conserved_quantity.into(),
            value,
        }
    }

    /// Norm of the generator vector.
    pub fn generator_norm(&self) -> f64 {
        self.generator.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}
