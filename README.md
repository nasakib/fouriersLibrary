# Codex Babel (fouriersLibrary)

An open-source, decentralized, voxel-based universe mapping the Library of Babel through the spatial frequency domain ($K$-Space).

---

## Ecosystem Connectome Integration
Codex Babel serves as the **Companion R&D Visualization Engine** for Vector Core Holdings. It utilizes `connectome_listener` to subscribe to **Harmonic State Vectors** output by `babelForge`. It dynamically translates these mathematical representations of local, physical grids into visible voxel-cymatic architecture, allowing users to intuitively navigate and "see" real-world resource flows.

---

## 🌌 Core Mathematical Paradigm

Instead of storing static text blocks or physical voxel data, the coordinates in Codex Babel represent vectors in the spatial frequency domain ($K$-Space). Any client node can deterministically reconstruct local spatial structures and readable text matrices by running an Inverse Fast Fourier Transform (IFFT) over a local frequency grid.

```
                  ┌─────────────────────────────────┐
                  │      K-Space Coordinate         │
                  │      vec(C) = (u, v, w)         │
                  └────────────────┬────────────────┘
                                   │
                                   ▼
                  ┌─────────────────────────────────┐
                  │ Deterministic Spectrum Generator │
                  │       (ChaCha8 + LSH Phase)     │
                  └────────────────┬────────────────┘
                                   │
                                   ▼
                  ┌─────────────────────────────────┐
                  │    Frequency Grid F(u, v, w)     │
                  │         (Complex Array)         │
                  └────────────────┬────────────────┘
                                   │
                                   ▼
                  ┌─────────────────────────────────┐
                  │    3D Inverse FFT (IFFT_3D)     │
                  │      f(x,y,z) = IFFT{F}         │
                  └────────────────┬────────────────┘
                                   │
                                   ▼
                  ┌─────────────────┴────────────────┐
                  │     Decoded Spatial Voxel        │
                  │  Amplitude -> Visual Intensity   │
                  │  Phase -> 29-Character Alphabet   │
                  └──────────────────────────────────┘
```

### 1. Deterministic Frequency Generation
Given a 3D coordinate vector $\vec{C} = (u, v, w)$ in $K$-space and a global seed, the local frequency spectrum value $F(u, v, w)$ is computed deterministically:

$$F(u, v, w) = F_{\text{base}}(u, v, w) \cdot e^{i \theta(u, v, w)}$$

Where:
*   $F_{\text{base}}(u, v, w)$ is a complex number generated via a deterministic pseudo-random number generator (ChaCha8) seeded with the combined bytes of `seed`, $u$, $v$, and $w$.
*   $\theta(u, v, w)$ is a slowly-varying, continuous phase field that maps large-scale harmonics.

### 2. Locality-Sensitive Hashing (LSH) & Phase Harmony
To create navigable **Resonant Nodes** (coherent, high-amplitude text structures) and **Semantic Fault Lines** (high-entropy noise), we modulate the frequency phase with a slowly-varying harmonic field:

$$\theta(u, v, w) = \left( \sin\left(\frac{u}{32}\right)\cos\left(\frac{v}{32}\right) + \sin\left(\frac{w}{32}\right)\cos\left(\frac{u}{32}\right) \right) \cdot \pi$$

Because adjacent $K$-space coordinate clusters overlap significantly and share continuous phase bounds, adjacent spatial blocks exhibit high mathematical harmony, meaning spatial proximity yields semantically and statistically related text patterns.

### 3. The Signal Transform
Reconstruction of a physical voxel block is achieved by computing a 3D Inverse Fast Fourier Transform (IFFT) over the generated frequency spectrum:

$$f(x, y, z) = \mathcal{F}^{-1}\{F(u, v, w)\} = \frac{1}{N^3} \sum_{u=0}^{N-1} \sum_{v=0}^{N-1} \sum_{w=0}^{N-1} F(u, v, w) \cdot e^{i 2\pi \left(\frac{ux}{N} + \frac{vy}{N} + \frac{wz}{N}\right)}$$

### 4. Voxel & Linguistic Translation
Each element $c = r + i \cdot \text{img}$ in the reconstructed spatial grid $f(x, y, z)$ is split into its physical and semantic counterparts:

*   **Voxel Energy / Wave Amplitude (Magnitude):**
    $$A = |c| = \sqrt{r^2 + \text{img}^2}$$
    Maps to physical attributes: voxel scaling, visibility thresholding, emissive brightness, and cymatic node displacement.
*   **Linguistic Translation (Phase Angle):**
    $$\phi = \text{arg}(c) = \text{atan2}(\text{img}, r) \quad \in [-\pi, \pi]$$
    Mapped uniformly to the 29-character Babel alphabet:
    $$\text{Index} = \left\lfloor \frac{\phi + \pi}{2\pi} \cdot 29 \right\rfloor \quad \in [0, 28]$$
    
    The 29-character alphabet set corresponds to lowercase `a` to `z`, space ` `, comma `,`, and period `.`.

---

## 📦 Repository Structure

The workspace is organized as a Cargo multi-crate project:

*   **[`babel-core`](file:///c:/Users/natsa/Documents/fouriersLibrary/babel-core)**: Core Systems and Mathematical Engine. Contains 3D FFT pipelines, deterministic PRNG-based frequency generators, alphabet translation layers, and mathematical unit tests.
*   **[`vox-cymatic`](file:///c:/Users/natsa/Documents/fouriersLibrary/vox-cymatic)**: Dynamic Client and 3D Voxel Renderer. Uses the Bevy Engine to procedurally generate meshes and colors directly from real-time spatial wave values, introducing cymatic micro-oscillations.
*   **[`signal-protocol`](file:///c:/Users/natsa/Documents/fouriersLibrary/signal-protocol)**: P2P Consensus and State Verification Layer. Defines serialized `TruthAnchor` certificates validating discoveries mathematically across decentralized nodes.

---

## 🛠️ Verification & Build Instructions

Ensure you have Rust and Cargo installed.

### 1. Compile Workspace
To build all three crates concurrently:
```bash
cargo build
```

### 2. Execute Test Suites
To run mathematical, coordinate-sensitivity, and cryptographic consensus tests:
```bash
cargo test --all
```

### 3. Launch Voxel Client
To run the visual Bevy client rendering the active frequency universe:
```bash
cargo run -p vox-cymatic
```
