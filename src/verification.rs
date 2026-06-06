//! Fleet-wide conservation verification.
//!
//! Given a collection of BridgeResults, verify that every Noether pair satisfies
//! the meta-law and produce a diagnostic report.

use crate::{BridgeResult, VerificationReport, Violation};

/// Verify a collection of bridge results and produce a report.
pub fn verify(results: &[BridgeResult]) -> VerificationReport {
    let total_pairs = results.len();
    let satisfied = results.iter().filter(|r| r.satisfied).count();

    let violations: Vec<Violation> = results
        .iter()
        .filter(|r| !r.satisfied)
        .map(|r| Violation {
            symmetry: r.pair.symmetry.clone(),
            expected: r.meta_params.constant,
            actual: r.meta_params.gamma + r.meta_params.h,
            error: r.error.abs(),
        })
        .collect();

    VerificationReport {
        total_pairs,
        satisfied,
        violations,
    }
}

/// Verify that a set of Noether pairs all conserve the same constant C
/// within tolerance. Shorthand that bridges first, then verifies.
pub fn verify_conservation(
    results: &[BridgeResult],
) -> VerificationReport {
    verify(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MetaLawParams, NoetherPair};

    #[test]
    fn all_satisfied_empty() {
        let report = verify(&[]);
        assert!(report.all_satisfied());
        assert_eq!(report.total_pairs, 0);
        assert_eq!(report.satisfaction_rate(), 1.0);
    }
}
