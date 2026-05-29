use ndarray::Array3;
use num_complex::Complex64;
use rustfft::FftPlanner;
use std::sync::Arc;

/// Processor for executing 3D Fourier Transforms over voxel grids.
#[derive(Clone)]
pub struct SignalTransform {
    n: usize,
    fft_inverse: Arc<dyn rustfft::Fft<f64>>,
}

impl std::fmt::Debug for SignalTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalTransform")
            .field("n", &self.n)
            .finish()
    }
}

impl SignalTransform {
    /// Creates a new SignalTransform processor for grids of size N x N x N.
    pub fn new(n: usize) -> Self {
        let mut planner = FftPlanner::new();
        let fft_inverse = planner.plan_fft_inverse(n);
        Self { n, fft_inverse }
    }

    /// Computes the 3D Inverse Fast Fourier Transform (IFFT) on the given frequency grid.
    ///
    /// Evaluates:
    /// f(x, y, z) = IFFT_3D { F(u, v, w) }
    ///
    /// Applies 1D IFFT sweeps along all three axes, then scales by 1 / (N^3) to normalize.
    pub fn inverse_transform(&self, mut grid: Array3<Complex64>) -> Result<Array3<Complex64>, &'static str> {
        let shape = grid.shape();
        if shape[0] != self.n || shape[1] != self.n || shape[2] != self.n {
            return Err("Input grid dimensions must match planned dimensions (N, N, N)");
        }

        let mut buffer = vec![Complex64::new(0.0, 0.0); self.n];

        // 1. Transform along Axis 0 (rows)
        for j in 0..self.n {
            for k in 0..self.n {
                // Copy into buffer
                for i in 0..self.n {
                    buffer[i] = grid[[i, j, k]];
                }

                // Process in-place 1D IFFT
                self.fft_inverse.process(&mut buffer);

                // Write back
                for i in 0..self.n {
                    grid[[i, j, k]] = buffer[i];
                }
            }
        }

        // 2. Transform along Axis 1 (columns)
        for i in 0..self.n {
            for k in 0..self.n {
                // Copy into buffer
                for j in 0..self.n {
                    buffer[j] = grid[[i, j, k]];
                }

                // Process in-place 1D IFFT
                self.fft_inverse.process(&mut buffer);

                // Write back
                for j in 0..self.n {
                    grid[[i, j, k]] = buffer[j];
                }
            }
        }

        // 3. Transform along Axis 2 (depth)
        for i in 0..self.n {
            for j in 0..self.n {
                // Copy into buffer
                for k in 0..self.n {
                    buffer[k] = grid[[i, j, k]];
                }

                // Process in-place 1D IFFT
                self.fft_inverse.process(&mut buffer);

                // Write back
                for k in 0..self.n {
                    grid[[i, j, k]] = buffer[k];
                }
            }
        }

        // Normalize the 3D IFFT output by dividing by N^3 (the volume of the grid)
        let norm_factor = 1.0 / (self.n as f64).powi(3);
        grid.mapv_inplace(|val| val * norm_factor);

        Ok(grid)
    }
}
