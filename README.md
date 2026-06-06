# noether-bridge

Formal bridge from symplectic-fleet Noether pairs to the conservation-law **γ + H = C** meta-law.

Every continuous symmetry of the action yields a conserved quantity (Noether's theorem). This library maps those symmetry–observable pairs into the meta-law framework, verifying that each Noether contribution satisfies the conservation constraint within configurable tolerance.

## Architecture

```
Symmetry ──► NoetherPair ──► BridgeResult ──► VerificationReport
                │                │                  │
          (generator,       (γ, H, C,          (satisfied,
           conserved Q)      satisfied)         violations)
```

### Modules

| Module | Purpose |
|---|---|
| `noether_pair` | Symmetry-observable pairs from symplectic structure: `(generator, conserved_quantity)` |
| `meta_law` | The γ + H = C conservation law with tolerance checking |
| `bridge` | Formal mapping: `NoetherPair → MetaLawParams`. Proves each pair satisfies γ + H = C |
| `verification` | Fleet-wide conservation: sum all Noether contributions, check total meta-law satisfaction |
| `symmetry_registry` | Extensible registry of known symmetries and their conservation-law consequences |
| `violation` | Detect which symmetry is broken when conservation law is violated. Diagnostic tool |

### Standard Symmetries

| Symmetry | Generator | Conserved Quantity | γ coefficient |
|---|---|---|---|
| Time translation | 1D (time direction) | Energy | 1.0 |
| Rotation | 3D (axis) | Angular momentum | 0.5 |
| Phase rotation (U(1)) | 2D | Particle number | 1.0 |
| Spatial translation | 3D (direction) | Linear momentum | 1.0 |
| Galilean boost | 3D | Center-of-mass velocity | 0.5 |
| Scale (dilation) | 1D | Dilation charge | 2.0 |

## Quick Start

```rust
use noether_bridge::{
    NoetherPair, SymmetryRegistry,
    bridge::{self, DefaultBridge, BridgeMapping},
    verification, violation,
};

// Create a standard symmetry registry
let registry = SymmetryRegistry::standard();

// Define Noether pairs from the fleet
let pairs = vec![
    NoetherPair::time_translation(50.0),           // energy = 50
    NoetherPair::rotation(12.0, vec![0., 0., 1.]), // angular momentum = 12
    NoetherPair::phase_rotation(30.0),              // particle number = 30
];

// Bridge all pairs to meta-law parameters
let results = bridge::bridge_fleet_exact(&pairs, &registry, 1e-9);

// Verify fleet-wide conservation
let report = verification::verify(&results);
println!("Satisfied: {}/{}", report.satisfied, report.total_pairs);

// Diagnose violations if any
if !report.all_satisfied() {
    for v in violation::diagnose(&results) {
        println!("Broken symmetry: {} (error={:.6e})", v.symmetry, v.error);
    }
}
```

## Core Types

```rust
struct NoetherPair {
    symmetry: String,           // e.g. "time_translation"
    generator: Vec<f64>,        // Lie-algebra generator
    conserved_quantity: String, // e.g. "energy"
    value: f64,                 // current value of the conserved quantity
}

struct MetaLawParams {
    gamma: f64,       // symmetry-weighted contribution
    h: f64,           // Hamiltonian contribution
    constant: f64,    // C in γ + H = C
    tolerance: f64,   // |γ + H − C| ≤ tolerance
}

struct BridgeResult {
    pair: NoetherPair,
    meta_params: MetaLawParams,
    satisfied: bool,
    error: f64,       // signed error γ + H − C
}

struct VerificationReport {
    total_pairs: usize,
    satisfied: usize,
    violations: Vec<Violation>,
}

struct Violation {
    symmetry: String,
    expected: f64,
    actual: f64,
    error: f64,
}

struct SymmetryEntry {
    name: String,
    generator_dim: usize,
    conserved: String,
    gamma_coeff: f64,
}
```

## Custom Symmetries

```rust
let mut registry = SymmetryRegistry::standard();
registry.register(SymmetryEntry {
    name: "supersymmetry".into(),
    generator_dim: 4,
    conserved: "supercharge".into(),
    gamma_coeff: 1.5,
});

let pair = NoetherPair::new("supersymmetry", vec![1., 0., 0., 0.], "supercharge", 7.0);
```

## Features

- **Serde on all public types** — serialize/deserialize every struct
- **No external dependencies beyond serde** — lightweight
- **Edition 2024** — latest Rust idioms
- **43 tests** covering the full pipeline from symmetries through verification
- **Tolerance-aware** — small numerical errors don't trigger false violations
- **Extensible** — register custom symmetries with their own γ coefficients

## License

MIT
