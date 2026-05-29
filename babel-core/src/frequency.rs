use ndarray::Array3;
use num_complex::Complex64;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Generator for mapping spatial frequency domain (K-Space) vectors to deterministic
/// complex frequency spectrums.
#[derive(Debug, Clone)]
pub struct KSpaceGenerator {
    seed: u64,
}

impl KSpaceGenerator {
    /// Creates a new K-space generator initialized with a global seed.
    pub const fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Computes a continuous, slowly-varying phase offset for phase-alignment and LSH harmony.
    /// Creates large-scale periodic "Semantic Fault Lines" and "Resonant Nodes".
    pub fn compute_lsh_phase(&self, u: i64, v: i64, w: i64) -> f64 {
        // Low-frequency spatial wave mixtures to generate deterministic phase variations
        let scale = 0.031_25; // 1 / 32
        let u_f = u as f64 * scale;
        let v_f = v as f64 * scale;
        let w_f = w as f64 * scale;

        // Wave equations establishing large-scale harmonics
        let wave_1 = (u_f.sin() * v_f.cos()).sin();
        let wave_2 = (v_f.sin() * w_f.cos()).cos();
        let wave_3 = (w_f.sin() * u_f.cos()).sin();

        (wave_1 + wave_2 + wave_3) * std::f64::consts::PI
    }

    /// Deterministically generates the complex frequency coefficient at a specific K-space coordinate.
    pub fn evaluate_frequency(&self, u: i64, v: i64, w: i64) -> Complex64 {
        // Construct a unique, deterministic 32-byte seed for ChaCha8
        let mut seed_bytes = [0u8; 32];
        seed_bytes[0..8].copy_from_slice(&self.seed.to_le_bytes());
        seed_bytes[8..16].copy_from_slice(&u.to_le_bytes());
        seed_bytes[16..24].copy_from_slice(&v.to_le_bytes());
        seed_bytes[24..32].copy_from_slice(&w.to_le_bytes());

        let mut rng = ChaCha8Rng::from_seed(seed_bytes);

        // Generate baseline complex coefficient inside the unit circle
        let r: f64 = rng.gen_range(-1.0..1.0);
        let i: f64 = rng.gen_range(-1.0..1.0);
        let base = Complex64::new(r, i);

        // Apply continuous LSH phase modulation to ensure nearby vectors exhibit mathematical harmony
        let phase_offset = self.compute_lsh_phase(u, v, w);
        let modulation = Complex64::from_polar(1.0, phase_offset);

        base * modulation
    }

    /// Generates a local frequency spectrum grid of size N x N x N centered at the specified coordinate.
    /// The coordinates generated are u_0 + i, v_0 + j, w_0 + k for i, j, k in [0, n).
    pub fn generate_grid(&self, center: (i64, i64, i64), n: usize) -> Array3<Complex64> {
        let (u0, v0, w0) = center;
        let mut grid = Array3::from_elem((n, n, n), Complex64::new(0.0, 0.0, 0.0));

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let u = u0 + i as i64;
                    let v = v0 + j as i64;
                    let w = w0 + k as i64;
                    grid[[i, j, k]] = self.evaluate_frequency(u, v, w);
                }
            }
        }

        grid
    }
}
