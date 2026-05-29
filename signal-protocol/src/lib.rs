use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A certified cryptographic proof representing a discovered linguistic text structure
/// at a deterministic set of K-space coordinates.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TruthAnchor {
    /// Global generation seed
    pub seed: u64,
    /// The K-space starting coordinates (u, v, w) of the block
    pub center: (i64, i64, i64),
    /// Dimension of the block voxel cube
    pub n: usize,
    /// The legible decoded text string discovered
    pub claimed_text: String,
    /// Cryptographic SHA-256 hash of the decoded text
    pub text_hash: String,
}

impl TruthAnchor {
    /// Creates a new TruthAnchor structure and automatically calculates its cryptographic text hash.
    pub fn new(seed: u64, center: (i64, i64, i64), n: usize, claimed_text: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(claimed_text.as_bytes());
        let text_hash = hex::encode(hasher.finalize());

        Self {
            seed,
            center,
            n,
            claimed_text,
            text_hash,
        }
    }

    /// Validates the Truth Anchor discovery.
    ///
    /// Verification succeeds if and only if:
    /// 1. The stored `text_hash` matches the computed SHA-256 of the `claimed_text`.
    /// 2. Re-executing the deterministic K-Space wave transform pipeline produces exactly `claimed_text`.
    pub fn verify(&self) -> bool {
        // Step 1: Verify data integrity using SHA-256
        let mut hasher = Sha256::new();
        hasher.update(self.claimed_text.as_bytes());
        let computed_hash = hex::encode(hasher.finalize());
        if computed_hash != self.text_hash {
            return false;
        }

        // Step 2: Verify deterministic mathematical frequency-to-text transform
        match babel_core::reconstruct_block(self.seed, self.center, self.n) {
            Ok((reconstructed_text, _)) => reconstructed_text == self.claimed_text,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_anchor_valid_proof() {
        let seed = 777;
        let center = (5, -5, 10);
        let n = 4; // small block for rapid unit testing (64 characters)

        // 1. Core mathematical extraction
        let (reconstructed, _) = babel_core::reconstruct_block(seed, center, n)
            .expect("Failed to reconstruct block");

        // 2. Generate the consensus anchor
        let anchor = TruthAnchor::new(seed, center, n, reconstructed);

        // 3. Cryptographically verify
        assert!(anchor.verify(), "Valid anchor verification failed");
    }

    #[test]
    fn test_truth_anchor_invalid_hash() {
        let seed = 888;
        let center = (0, 0, 0);
        let n = 4;

        let (reconstructed, _) = babel_core::reconstruct_block(seed, center, n).unwrap();

        let mut anchor = TruthAnchor::new(seed, center, n, reconstructed);
        
        // Tamper with the cryptographic hash
        anchor.text_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

        assert!(!anchor.verify(), "Verifier approved tampered hash");
    }

    #[test]
    fn test_truth_anchor_tampered_text() {
        let seed = 999;
        let center = (12, 34, 56);
        let n = 4;

        let (reconstructed, _) = babel_core::reconstruct_block(seed, center, n).unwrap();

        let mut anchor = TruthAnchor::new(seed, center, n, reconstructed);
        
        // Tamper with the text while keeping the original valid hash
        anchor.claimed_text = "tampered".to_string();

        assert!(!anchor.verify(), "Verifier approved tampered text");
    }
}
