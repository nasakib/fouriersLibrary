# Codex Babel · The Symbiotic Truth Engine (`fouriersLibrary`)

An open-source, decentralized, spatial computing engine and ledger mapping mathematical physics, $K$-Space frequency transforms, and formal logical satisfiability into interactive 3D topological geometries.

```
                             ┌───────────────────────────────────────┐
                             │       1D Math, Physics & Logic        │
                             │  (First Principles / Conservation)    │
                             └──────────────────┬────────────────────┘
                                                │
                                                ▼
                             ┌───────────────────────────────────────┐
                             │          SMT Proof Engine             │
                             │    (Microsoft Z3 Theorem Prover)      │
                             └──────────────────┬────────────────────┘
                                                │
                                                ▼
                             ┌───────────────────────────────────────┐
                             │       Topological Graph State         │
                             │    (NetworkX Simplicial Complexes)    │
                             └──────────────────┬────────────────────┘
                                                │
                                                ▼
                             ┌───────────────────────────────────────┐
                             │     K-Space Frequency Domain          │
                             │    (babel-core 3D IFFT Pipeline)      │
                             └──────────────────┬────────────────────┘
                                                │
                                                ▼
                             ┌───────────────────────────────────────┐
                             │     Spatial Computing Viewports       │
                             │  • Next.js 15 / R3F (Vector Lens)     │
                             │  • Bevy Engine (vox-cymatic)          │
                             └──────────────────┬────────────────────┘
                                                │
                                                ▼
                             ┌───────────────────────────────────────┐
                             │       Truth Anchor Consensus          │
                             │  • signal-protocol Cryptographic Proof│
                             │  • Supabase Reality Anchor Store      │
                             │  • Solana Tipping / Proof-of-Meetup   │
                             └───────────────────────────────────────┘
```

---

## 🌌 System Philosophy & Boundary Axioms

The Symbiotic Truth Engine operates strictly upon **First Principles** (physics, formal logic, topological mathematics). It computes and renders falsifiable topological blueprints (**The Truth Frequency**).

### Core Boundaries:
1. **Algorithmic Objectivity:** All engine constraints, graph verification steps, and solver rules evaluate solely mathematical satisfiability, physical conservation laws (Kirchhoff loop laws, continuity of mass/energy, thermodynamic bounds), and topological invariants.
2. **Subjective & Ethical Offloading:** The engine explicitly excludes subjective preferences, moral weights, and sociopolitical value judgments from its core computational algorithms. Values and ethical objectives are strictly preserved within the human operator domain.
3. **The Symbiotic Reality Anchor:** The engine generates structured hypotheses in physical space; the human operator conducts empirical observation and grounds the model back into reality by recording empirical deltas ($\Delta$).

---

## 🔬 Scientific Foundations & References

1. **Topological Data Analysis (TDA):**
   *Carlsson, G. (2009). Topology and data. Bulletin of the American Mathematical Society, 46(2), 255-308.*
   Mapping chaotic, high-entropy configuration spaces to low-dimensional geometric invariants (Betti numbers $\beta_0, \beta_1$).
2. **Satisfiability Modulo Theories (SMT):**
   *de Moura, L., & Bjørner, N. (2008). Z3: An efficient SMT solver. TACAS, Springer.*
   Employing Microsoft Z3-Solver to calculate logical and physical satisfiability in real-time, physically rejecting impossible node topologies through tactile spring repulsion.
3. **Active Inference & The Free Energy Principle:**
   *Friston, K. (2010). The free-energy principle: a unified brain theory? Nature Reviews Neuroscience, 11(2), 127-138.*
   Minimizing prediction error via the Focus Dial entropy governor and Reality Tether feedback loop.
4. **Pictural Formalism & ZX-Calculus:**
   *Coecke, B., & Kissinger, A. (2017). Picturing Quantum Processes. Cambridge University Press.*
   Natively spatial diagrammatic mathematics compiled into refractive 3D geometries.

---

## 📦 Consolidated Monorepo Structure

*   **[`frontend`](file:///c:/Users/natsa/Documents/fouriersLibrary/frontend)**: Next.js 15 (App Router), React 19, React Three Fiber (R3F), `@react-three/drei`, `@react-three/spring`, Tailwind CSS, and Zustand. Features `AmbientCanvas`, `FocusDial`, `InventoryTray`, and `RealityTether`.
*   **[`backend`](file:///c:/Users/natsa/Documents/fouriersLibrary/backend)**: Python 3.11+ FastAPI service powering `/api/generate-noise`, `/api/tune-frequency`, `/api/truth-anchor`, and `/api/kspace-transform` using Microsoft `z3-solver` and `networkx`.
*   **[`database`](file:///c:/Users/natsa/Documents/fouriersLibrary/database)**: Supabase PostgreSQL DDL schema with Row Level Security (`first_principles`, `topologies`, `reality_anchors`).
*   **[`babel-core`](file:///c:/Users/natsa/Documents/fouriersLibrary/babel-core)**: High-performance Rust mathematical engine (3D IFFT, ChaCha8 PRNG, spatial frequency domain generation, and linguistic phase translation).
*   **[`signal-protocol`](file:///c:/Users/natsa/Documents/fouriersLibrary/signal-protocol)**: Decentralized state verification layer (`TruthAnchor` cryptographic proofs) and `connectome_listener` daemon.
*   **[`vox-cymatic`](file:///c:/Users/natsa/Documents/fouriersLibrary/vox-cymatic)**: Procedural 3D voxel renderer using Bevy Engine and custom WGSL cymatic shaders.
*   **[`solana-tipping`](file:///c:/Users/natsa/Documents/fouriersLibrary/solana-tipping)**: Solana on-chain program for micropayment tipping, community liquidity drops, and Proof-of-Connection meetup minting.

---

## 🛠️ Verification & Quick Start

### 1. Rust Mathematical Core
```bash
cargo test -p babel-core -p signal-protocol
```

### 2. SMT & Graph Backend (FastAPI + Z3)
```bash
cd backend
python -m venv .venv
source .venv/bin/activate # or .venv\Scripts\Activate.ps1 on Windows
pip install -r requirements.txt
uvicorn app.main:app --reload --port 8000
```

### 3. Spatial Web Viewport (Next.js 15 + R3F)
```bash
cd frontend
pnpm install
pnpm dev
```
Open [http://localhost:3000](http://localhost:3000) to enter the Vector Lens workspace.

### 4. Supabase Database
Run [`database/schema.sql`](file:///c:/Users/natsa/Documents/fouriersLibrary/database/schema.sql) in your Supabase SQL Editor.
