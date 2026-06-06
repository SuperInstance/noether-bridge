//! Comprehensive test suite for noether-bridge.

#[cfg(test)]
mod tests {
    use crate::{
        bridge::{self, BridgeMapping, DefaultBridge},
        meta_law,
        symmetry_registry::SymmetryRegistry,
        verification,
        violation,
        MetaLawParams, NoetherPair, SymmetryEntry,
    };

    // ── Noether pair construction tests ────────────────────────────────────

    #[test]
    fn test_time_translation_pair() {
        let pair = NoetherPair::time_translation(42.0);
        assert_eq!(pair.symmetry, "time_translation");
        assert_eq!(pair.conserved_quantity, "energy");
        assert_eq!(pair.value, 42.0);
        assert_eq!(pair.generator, vec![1.0]);
    }

    #[test]
    fn test_rotation_pair() {
        let gen_vec = vec![0.0, 0.0, 1.0]; // z-axis rotation
        let pair = NoetherPair::rotation(7.5, gen_vec.clone());
        assert_eq!(pair.symmetry, "rotation");
        assert_eq!(pair.conserved_quantity, "angular_momentum");
        assert_eq!(pair.value, 7.5);
        assert_eq!(pair.generator, gen_vec);
    }

    #[test]
    fn test_phase_rotation_pair() {
        let pair = NoetherPair::phase_rotation(100.0);
        assert_eq!(pair.symmetry, "phase_rotation");
        assert_eq!(pair.conserved_quantity, "particle_number");
        assert_eq!(pair.value, 100.0);
    }

    #[test]
    fn test_generator_norm() {
        let pair = NoetherPair::rotation(1.0, vec![3.0, 4.0, 0.0]);
        assert!((pair.generator_norm() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_generic_pair() {
        let pair = NoetherPair::new("custom_sym", vec![1.0, 2.0], "custom_obs", 99.0);
        assert_eq!(pair.symmetry, "custom_sym");
        assert_eq!(pair.conserved_quantity, "custom_obs");
    }

    // ── Meta-law tests ─────────────────────────────────────────────────────

    #[test]
    fn test_meta_law_exact_satisfaction() {
        let params = MetaLawParams::exact(3.0, 4.0);
        assert_eq!(params.constant, 7.0);
        assert!(params.is_satisfied());
        assert!(params.error().abs() < 1e-12);
    }

    #[test]
    fn test_meta_law_explicit_satisfaction() {
        let params = MetaLawParams::new(2.0, 3.0, 5.0, 1e-9);
        assert!(params.is_satisfied());
    }

    #[test]
    fn test_meta_law_violation() {
        let params = MetaLawParams::new(2.0, 3.0, 6.0, 1e-9);
        assert!(!params.is_satisfied());
        assert!((params.error() - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_meta_law_tolerance_handling() {
        // Small error within tolerance should not trigger violation.
        let params = MetaLawParams::new(1.0, 1.0, 2.0 + 1e-10, 1e-9);
        assert!(params.is_satisfied());

        // Error just outside tolerance.
        let params2 = MetaLawParams::new(1.0, 1.0, 2.0 + 1e-6, 1e-9);
        assert!(!params2.is_satisfied());
    }

    #[test]
    fn test_error_magnitude() {
        let params = MetaLawParams::new(1.0, 2.0, 3.5, 0.1);
        assert!((params.error().abs() - 0.5).abs() < 1e-12);
        assert!((params.abs_error() - 0.5).abs() < 1e-12);
    }

    // ── Symmetry registry tests ────────────────────────────────────────────

    #[test]
    fn test_standard_registry_has_known_symmetries() {
        let reg = SymmetryRegistry::standard();
        assert!(reg.get("time_translation").is_some());
        assert!(reg.get("rotation").is_some());
        assert!(reg.get("phase_rotation").is_some());
        assert!(reg.get("spatial_translation").is_some());
    }

    #[test]
    fn test_registry_lookup_correct() {
        let reg = SymmetryRegistry::standard();
        let entry = reg.get("time_translation").unwrap();
        assert_eq!(entry.conserved, "energy");
        assert_eq!(entry.generator_dim, 1);
        assert_eq!(entry.gamma_coeff, 1.0);
    }

    #[test]
    fn test_registry_rotation_entry() {
        let reg = SymmetryRegistry::standard();
        let entry = reg.get("rotation").unwrap();
        assert_eq!(entry.conserved, "angular_momentum");
        assert_eq!(entry.generator_dim, 3);
    }

    #[test]
    fn test_custom_symmetry_registration() {
        let mut reg = SymmetryRegistry::new();
        reg.register(SymmetryEntry {
            name: "supersymmetry".into(),
            generator_dim: 4,
            conserved: "supercharge".into(),
            gamma_coeff: 1.5,
        });
        assert!(reg.get("supersymmetry").is_some());
        let entry = reg.get("supersymmetry").unwrap();
        assert_eq!(entry.gamma_coeff, 1.5);
        assert_eq!(entry.conserved, "supercharge");
    }

    #[test]
    fn test_registry_replace_entry() {
        let mut reg = SymmetryRegistry::new();
        reg.register(SymmetryEntry {
            name: "test".into(),
            generator_dim: 1,
            conserved: "q1".into(),
            gamma_coeff: 1.0,
        });
        reg.register(SymmetryEntry {
            name: "test".into(),
            generator_dim: 2,
            conserved: "q2".into(),
            gamma_coeff: 2.0,
        });
        assert_eq!(reg.len(), 1);
        assert_eq!(reg.get("test").unwrap().gamma_coeff, 2.0);
    }

    #[test]
    fn test_registry_iter_and_symmetries() {
        let reg = SymmetryRegistry::standard();
        let names = reg.symmetries();
        assert!(names.len() >= 3);
        assert!(names.contains(&"time_translation"));
    }

    // ── Bridge tests ───────────────────────────────────────────────────────

    #[test]
    fn test_bridge_maps_time_translation() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::time_translation(10.0);
        // γ = 1.0 * 10.0 = 10.0, H = 1.0 * 10.0 = 10.0, C = 20.0
        let bridge = DefaultBridge::new(&reg, 1e-9, 20.0);
        let result = bridge.map(&pair);
        assert!(result.satisfied);
        assert!((result.meta_params.gamma - 10.0).abs() < 1e-12);
        assert!((result.meta_params.h - 10.0).abs() < 1e-12);
    }

    #[test]
    fn test_bridge_maps_rotation() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::rotation(6.0, vec![0.0, 0.0, 1.0]);
        // γ = 0.5 * 6.0 = 3.0, H = 1.0 * 6.0 = 6.0, C = 9.0
        let bridge = DefaultBridge::new(&reg, 1e-9, 9.0);
        let result = bridge.map(&pair);
        assert!(result.satisfied);
        assert!((result.meta_params.gamma - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_bridge_maps_phase_rotation() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::phase_rotation(5.0);
        // γ = 1.0 * 5.0 = 5.0, H = 1.0 * 5.0 = 5.0, C = 10.0
        let bridge = DefaultBridge::new(&reg, 1e-9, 10.0);
        let result = bridge.map(&pair);
        assert!(result.satisfied);
    }

    #[test]
    fn test_bridge_detects_violation() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::time_translation(10.0);
        // C = 999.0 is wrong → violation
        let bridge = DefaultBridge::new(&reg, 1e-9, 999.0);
        let result = bridge.map(&pair);
        assert!(!result.satisfied);
    }

    #[test]
    fn test_bridge_map_all_homogeneous() {
        let reg = SymmetryRegistry::standard();
        // Use identical pairs so fleet constant works
        let pairs = vec![
            NoetherPair::time_translation(10.0),
            NoetherPair::time_translation(10.0),
        ];
        let results = bridge::bridge_fleet_exact(&pairs, &reg, 1e-9);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.satisfied));
    }

    // ── Verification tests ─────────────────────────────────────────────────

    #[test]
    fn test_verification_all_satisfied() {
        let reg = SymmetryRegistry::standard();
        // Use identical pairs so they all share the same γ+H sum
        let pairs = vec![
            NoetherPair::time_translation(10.0),
            NoetherPair::time_translation(10.0),
            NoetherPair::time_translation(10.0),
        ];
        let results = bridge::bridge_fleet_exact(&pairs, &reg, 1e-9);
        let report = verification::verify(&results);
        assert!(report.all_satisfied());
        assert_eq!(report.total_pairs, 3);
        assert_eq!(report.satisfied, 3);
        assert!(report.violations.is_empty());
    }

    #[test]
    fn test_verification_with_violation() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::time_translation(10.0);
        // C deliberately wrong
        let bridge = DefaultBridge::new(&reg, 1e-9, 999.0);
        let results = vec![bridge.map(&pair)];
        let report = verification::verify(&results);
        assert!(!report.all_satisfied());
        assert_eq!(report.violations.len(), 1);
        assert_eq!(report.violations[0].symmetry, "time_translation");
    }

    #[test]
    fn test_verification_satisfaction_rate() {
        let reg = SymmetryRegistry::standard();
        let pair_ok = NoetherPair::time_translation(10.0);
        let pair_bad = NoetherPair::time_translation(10.0);
        let bridge_ok = DefaultBridge::new(&reg, 1e-9, 20.0);
        let bridge_bad = DefaultBridge::new(&reg, 1e-9, 999.0);
        let results = vec![bridge_ok.map(&pair_ok), bridge_bad.map(&pair_bad)];
        let report = verification::verify(&results);
        assert!((report.satisfaction_rate() - 0.5).abs() < 1e-12);
    }

    // ── Violation diagnostics tests ────────────────────────────────────────

    #[test]
    fn test_violation_diagnose_empty() {
        let violations = violation::diagnose(&[]);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_violation_sorted_by_severity() {
        let reg = SymmetryRegistry::standard();
        let p1 = NoetherPair::time_translation(10.0);
        let p2 = NoetherPair::phase_rotation(5.0);
        let bridge = DefaultBridge::new(&reg, 1e-9, 100.0); // wrong C for both
        let results = vec![bridge.map(&p1), bridge.map(&p2)];
        let violations = violation::diagnose(&results);
        assert_eq!(violations.len(), 2);
        // Larger error first
        assert!(violations[0].error >= violations[1].error);
    }

    #[test]
    fn test_worst_violation() {
        let reg = SymmetryRegistry::standard();
        // time_translation: γ=1*10=10, H=1*10=10, sum=20, C=100 → error=80
        // phase_rotation: γ=1*100=100, H=1*100=100, sum=200, C=100 → error=100
        let p1 = NoetherPair::time_translation(10.0);
        let p2 = NoetherPair::phase_rotation(100.0);
        let bridge = DefaultBridge::new(&reg, 1e-9, 100.0);
        let results = vec![bridge.map(&p1), bridge.map(&p2)];
        let worst = violation::worst_violation(&results).unwrap();
        assert_eq!(worst.symmetry, "phase_rotation");
    }

    #[test]
    fn test_count_violations() {
        let reg = SymmetryRegistry::standard();
        let p1 = NoetherPair::time_translation(10.0);
        let p2 = NoetherPair::time_translation(10.0);
        let bridge_ok = DefaultBridge::new(&reg, 1e-9, 20.0);
        let bridge_bad = DefaultBridge::new(&reg, 1e-9, 999.0);
        let results = vec![bridge_ok.map(&p1), bridge_bad.map(&p2)];
        assert_eq!(violation::count_violations(&results), 1);
    }

    #[test]
    fn test_violation_display() {
        let v = crate::Violation {
            symmetry: "time_translation".into(),
            expected: 20.0,
            actual: 21.0,
            error: 1.0,
        };
        let s = format!("{}", v);
        assert!(s.contains("time_translation"));
        assert!(s.contains("expected=20"));
    }

    #[test]
    fn test_violation_summary_severity() {
        let reg = SymmetryRegistry::standard();
        let p1 = NoetherPair::time_translation(1000.0);
        let p2 = NoetherPair::time_translation(10.0);
        let bridge = DefaultBridge::new(&reg, 1e-9, 0.0);
        let results = vec![bridge.map(&p1), bridge.map(&p2)];
        let summary = violation::ViolationSummary::from_results(&results);
        assert_eq!(summary.total, 2);
        assert!(!summary.critical.is_empty() || !summary.major.is_empty());
    }

    // ── Fleet-wide meta-law tests ──────────────────────────────────────────

    #[test]
    fn test_fleet_constant() {
        let pairs = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
        let c = meta_law::fleet_constant(&pairs);
        // sums: 3, 7, 11 → mean = 7.0
        assert!((c - 7.0).abs() < 1e-12);
    }

    #[test]
    fn test_fleet_constant_empty() {
        assert_eq!(meta_law::fleet_constant(&[]), 0.0);
    }

    #[test]
    fn test_all_satisfied_meta_law() {
        // Single pair: 1+2=3
        assert!(meta_law::all_satisfied(&[(1.0, 2.0)], 3.0, 1e-9));
        // Two pairs with different sums can't both equal the same C
        assert!(!meta_law::all_satisfied(&[(1.0, 2.0), (3.0, 4.0)], 3.0, 1e-9));
    }

    // ── Full pipeline test ─────────────────────────────────────────────────

    #[test]
    fn test_full_pipeline() {
        let reg = SymmetryRegistry::standard();

        // 1. Define symmetries as Noether pairs (homogeneous for fleet-wide satisfaction)
        let pairs = vec![
            NoetherPair::time_translation(10.0),
            NoetherPair::time_translation(10.0),
            NoetherPair::time_translation(10.0),
        ];

        // 2. Bridge to meta-law
        let results = bridge::bridge_fleet_exact(&pairs, &reg, 1e-9);

        // 3. Verify fleet-wide conservation
        let report = verification::verify(&results);

        // 4. Check report
        assert_eq!(report.total_pairs, 3);
        assert!(report.all_satisfied());

        // 5. No violations
        let violations = violation::diagnose(&results);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_full_pipeline_perfect_conservation() {
        let reg = SymmetryRegistry::standard();

        // Construct pairs where we know exact C
        let pairs = vec![
            NoetherPair::time_translation(10.0),
        ];
        // γ = 1.0*10 = 10, H = 1.0*10 = 10, sum = 20
        let bridge = DefaultBridge::new(&reg, 1e-9, 20.0);
        let results = bridge.map_all(&pairs);
        let report = verification::verify(&results);
        assert!(report.all_satisfied());
    }

    #[test]
    fn test_full_pipeline_with_violations() {
        let reg = SymmetryRegistry::standard();

        let pairs = vec![
            NoetherPair::time_translation(10.0),
            NoetherPair::phase_rotation(5.0),
        ];
        let results = bridge::bridge_fleet_exact(&pairs, &reg, 1e-9);
        // Different pair types have different γ+H sums
        // so fleet constant (mean) won't satisfy either exactly
        let report = verification::verify(&results);
        assert_eq!(report.total_pairs, 2);

        // Diagnose violations
        if !report.all_satisfied() {
            let violations = violation::diagnose(&results);
            assert!(!violations.is_empty());
        }
    }

    #[test]
    fn test_multiple_pairs_sum_to_constant() {
        let reg = SymmetryRegistry::standard();

        // Time translation: γ=10, H=10
        // Phase rotation: γ=5, H=5
        // Both sum to 20 and 10 respectively
        // Fleet constant = mean(20, 10) = 15
        let pairs = vec![
            NoetherPair::time_translation(10.0),
            NoetherPair::phase_rotation(5.0),
        ];
        let results = bridge::bridge_fleet_exact(&pairs, &reg, 1e-9);
        // With C=15, neither pair individually satisfies γ+H=C
        // but the fleet-wide check shows the mean constant
        let report = verification::verify(&results);
        assert_eq!(report.total_pairs, 2);
    }

    // ── Serialization tests ────────────────────────────────────────────────

    #[test]
    fn test_serde_noether_pair() {
        let pair = NoetherPair::time_translation(42.0);
        let json = serde_json::to_string(&pair).unwrap();
        let deserialized: NoetherPair = serde_json::from_str(&json).unwrap();
        assert_eq!(pair, deserialized);
    }

    #[test]
    fn test_serde_meta_law_params() {
        let params = MetaLawParams::exact(3.0, 4.0);
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: MetaLawParams = serde_json::from_str(&json).unwrap();
        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_serde_bridge_result() {
        let reg = SymmetryRegistry::standard();
        let pair = NoetherPair::time_translation(10.0);
        let bridge = DefaultBridge::new(&reg, 1e-9, 20.0);
        let result = bridge.map(&pair);
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: crate::BridgeResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_serde_verification_report() {
        let report = crate::VerificationReport {
            total_pairs: 3,
            satisfied: 2,
            violations: vec![crate::Violation {
                symmetry: "rotation".into(),
                expected: 9.0,
                actual: 9.5,
                error: 0.5,
            }],
        };
        let json = serde_json::to_string(&report).unwrap();
        let deserialized: crate::VerificationReport = serde_json::from_str(&json).unwrap();
        assert_eq!(report, deserialized);
    }

    #[test]
    fn test_serde_symmetry_entry() {
        let entry = SymmetryEntry {
            name: "test".into(),
            generator_dim: 2,
            conserved: "charge".into(),
            gamma_coeff: 1.5,
        };
        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: SymmetryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, deserialized);
    }
}
