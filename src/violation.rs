//! Violation detection and diagnostics.
//!
//! When a conservation law is violated, this module identifies which symmetry
//! is broken and quantifies the violation.

use crate::{BridgeResult, Violation};

/// Diagnose violations from a set of bridge results.
///
/// Returns all violations sorted by error magnitude (largest first).
pub fn diagnose(results: &[BridgeResult]) -> Vec<Violation> {
    let mut violations: Vec<Violation> = results
        .iter()
        .filter(|r| !r.satisfied)
        .map(|r| Violation {
            symmetry: r.pair.symmetry.clone(),
            expected: r.meta_params.constant,
            actual: r.meta_params.gamma + r.meta_params.h,
            error: r.error.abs(),
        })
        .collect();

    violations.sort_by(|a, b| b.error.partial_cmp(&a.error).unwrap_or(std::cmp::Ordering::Equal));
    violations
}

/// Identify the single worst violation (largest error).
pub fn worst_violation(results: &[BridgeResult]) -> Option<Violation> {
    diagnose(results).into_iter().next()
}

/// Count how many symmetries are broken.
pub fn count_violations(results: &[BridgeResult]) -> usize {
    results.iter().filter(|r| !r.satisfied).count()
}

/// Categorize violations by severity.
pub struct ViolationSummary {
    pub total: usize,
    pub minor: Vec<Violation>,  // error < 1.0
    pub major: Vec<Violation>,  // 1.0 <= error < 10.0
    pub critical: Vec<Violation>, // error >= 10.0
}

impl ViolationSummary {
    pub fn from_results(results: &[BridgeResult]) -> Self {
        let violations = diagnose(results);
        let mut minor = Vec::new();
        let mut major = Vec::new();
        let mut critical = Vec::new();

        for v in violations {
            if v.error < 1.0 {
                minor.push(v);
            } else if v.error < 10.0 {
                major.push(v);
            } else {
                critical.push(v);
            }
        }

        Self {
            total: minor.len() + major.len() + critical.len(),
            minor,
            major,
            critical,
        }
    }
}
