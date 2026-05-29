pub mod frequency;
pub mod transform;
pub mod translation;

pub use frequency::KSpaceGenerator;
pub use transform::SignalTransform;
pub use translation::{AmplitudeTranslator, BABEL_ALPHABET};

use ndarray::Array3;

/// Reconstructs a spatial voxel block from its K-space frequency representation.
///
/// Given a seed, a 3D frequency center coordinate, and a block dimension N:
/// 1. Generates the deterministic complex frequency spectrum.
/// 2. Performs a 3D Inverse Fast Fourier Transform.
/// 3. Translates spatial phases to a 29-character string and amplitudes to physical intensities.
pub fn reconstruct_block(
    seed: u64,
    center: (i64, i64, i64),
    n: usize,
) -> Result<(String, Array3<f64>), &'static str> {
    let generator = KSpaceGenerator::new(seed);
    let transformer = SignalTransform::new(n);
    let translator = AmplitudeTranslator::new();

    // 1. Generate deterministic K-space frequency grid
    let freq_grid = generator.generate_grid(center, n);

    // 2. Perform 3D IFFT to retrieve spatial wave components
    let spatial_grid = transformer.inverse_transform(freq_grid)?;

    // 3. Decode characters (phases) and intensities (amplitudes)
    let text = translator.grid_to_string(&spatial_grid);
    let amplitudes = translator.grid_to_amplitudes(&spatial_grid);

    Ok((text, amplitudes))
}
