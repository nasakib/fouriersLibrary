use ndarray::Array3;
use num_complex::Complex64;

/// The 29-character alphabet of the Library of Babel.
pub const BABEL_ALPHABET: [char; 29] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ' ', ',', '.'
];

/// Translator for mapping complex spatial amplitudes to legible strings and intensities.
#[derive(Debug, Default, Clone, Copy)]
pub struct AmplitudeTranslator;

impl AmplitudeTranslator {
    /// Creates a new translator.
    pub const fn new() -> Self {
        Self
    }

    /// Maps a single complex number to a Babel character based on its phase argument.
    ///
    /// The phase lies in `[-pi, pi]`. We map this range to `[0, 29)` to achieve a uniform
    /// distribution for complex numbers with random phases.
    pub fn complex_to_char(&self, c: Complex64) -> char {
        let phase = c.arg(); // returns value in [-pi, pi]
        let normalized = (phase + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        
        // Clamp to safety
        let mut index = (normalized * 29.0).floor() as usize;
        if index >= 29 {
            index = 28;
        }
        
        BABEL_ALPHABET[index]
    }

    /// Translates a 3D spatial wave grid into a linear string using row-major ordering.
    pub fn grid_to_string(&self, grid: &Array3<Complex64>) -> String {
        let mut result = String::with_capacity(grid.len());
        
        // Traverse in X-Y-Z order
        for x in 0..grid.shape()[0] {
            for y in 0..grid.shape()[1] {
                for z in 0..grid.shape()[2] {
                    let val = grid[[x, y, z]];
                    result.push(self.complex_to_char(val));
                }
            }
        }
        
        result
    }

    /// Extracts the physical amplitude (magnitude) of the spatial wave at each voxel,
    /// which maps to voxel brightness, opacity, or cymatic height.
    pub fn grid_to_amplitudes(&self, grid: &Array3<Complex64>) -> Array3<f64> {
        grid.mapv(|c| c.norm())
    }
}
