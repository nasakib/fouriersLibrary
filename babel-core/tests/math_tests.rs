use babel_core::{reconstruct_block, KSpaceGenerator, SignalTransform, AmplitudeTranslator};

#[test]
fn test_determinism() {
    let seed = 42_069_1337;
    let center = (100, -200, 300);
    let n = 8;

    let res1 = reconstruct_block(seed, center, n).expect("Reconstruction failed");
    let res2 = reconstruct_block(seed, center, n).expect("Reconstruction failed");

    // Identical seeds and coordinates must produce identical outputs
    assert_eq!(res1.0, res2.0);
    assert_eq!(res1.1, res2.1);
}

#[test]
fn test_seed_sensitivity() {
    let center = (0, 0, 0);
    let n = 8;

    let (text1, _) = reconstruct_block(1111, center, n).unwrap();
    let (text2, _) = reconstruct_block(2222, center, n).unwrap();

    // Different seeds must generate different content
    assert_ne!(text1, text2);
}

#[test]
fn test_coordinate_sensitivity() {
    let seed = 9999;
    let n = 8;

    let (text1, _) = reconstruct_block(seed, (0, 0, 0), n).unwrap();
    let (text2, _) = reconstruct_block(seed, (100, 100, 100), n).unwrap();

    // Different coordinates must generate different content
    assert_ne!(text1, text2);
}

#[test]
fn test_extreme_inputs() {
    let n = 4;
    
    // Test large positive, negative, and zero values
    let coordinates = [
        (i64::MIN, i64::MIN, i64::MIN),
        (i64::MAX, i64::MAX, i64::MAX),
        (0, 0, 0),
        (-1, 5, -999999),
    ];

    for &coord in &coordinates {
        let res = reconstruct_block(0, coord, n);
        assert!(res.is_ok(), "Failed at coordinate {:?}", coord);
        
        let res_max = reconstruct_block(u64::MAX, coord, n);
        assert!(res_max.is_ok(), "Failed at coordinate {:?} with max seed", coord);
    }
}

#[test]
fn test_lsh_harmonic_correlation() {
    // Tests that adjacent coordinates show statistical phase correlation due to overlapping
    // frequency windows and our continuous LSH phase modulation.
    let seed = 12345;
    let n = 8;

    let generator = KSpaceGenerator::new(seed);
    
    // Evaluate adjacent frequencies in K-space
    let phase1 = generator.compute_lsh_phase(10, 10, 10);
    let phase2 = generator.compute_lsh_phase(10, 10, 11); // adjacent
    
    let diff = (phase1 - phase2).abs();
    
    // The phase field changes slowly, so adjacent cells should have small differences modulo 2*pi
    let wrapped_diff = (diff + std::f64::consts::PI) % (2.0 * std::f64::consts::PI) - std::f64::consts::PI;
    assert!(wrapped_diff.abs() < 0.2, "Phase changed too abruptly between adjacent coordinates: {}", wrapped_diff);
}

#[test]
fn test_transform_bounds() {
    let n = 8;
    let transformer = SignalTransform::new(n);
    let translator = AmplitudeTranslator::new();
    
    // Create an invalid size grid
    let bad_grid = ndarray::Array3::from_elem((n, n, n - 1), num_complex::Complex64::new(1.0, 0.0));
    let res = transformer.inverse_transform(bad_grid);
    
    assert!(res.is_err(), "Transformer accepted malformed input grid shape");
}
