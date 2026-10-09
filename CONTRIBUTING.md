# Contributing to The Symbiotic Truth Engine

Thank you for contributing to **The Symbiotic Truth Engine (`fourier-truth-engine`)**.

To ensure this tool remains a rigorously verifiable scientific instrument and pedagogical discovery platform, all contributors are required to respect the fundamental architectural boundaries defined below.

---

## 🏛️ The Non-Negotiable Boundary Principle

> **Objective Mathematics is Allowed in Algorithms. Subjective Ethics is Forbidden in Algorithms.**

### What May Exist in the Computational Engine:
- **First Principles of Physics:** Conservation of energy, conservation of charge, conservation of momentum, Navier-Stokes fluid continuity, thermodynamic limits.
- **Formal Logic & SMT Provers:** Z3 satisfiability, Horn clauses, boolean satisfiability, relational calculus, type theories.
- **Topological Invariants:** Betti numbers, simplicial homology, persistence diagrams, manifold boundaries, graph connectivity metrics.
- **Topological Rewrites (ZX-Calculus):** Morphisms, string diagrams, spatial tensor contractions.

### What is Forbidden from Core Algorithmic Code:
- **Moral & Ethical Weightings:** Do not bake "moral utility functions", "ethical alignments", or subjective social judgments into node satisfaction logic, edge penalties, or graph scoring.
- **Paternalistic Heuristics:** Do not hide or censor mathematically valid physical topologies because of speculative misuse. 
- **Prescriptive Human Values:** The engine calculates *what is physically and logically possible* (The Truth Frequency). The human operator determines *what ought to be built and why*.

Any PR attempting to embed moral heuristics or subjective filters directly into `z3-solver` rules or NetworkX topologies will be rejected.

---

## 🔬 Scientific Validation Guidelines

When contributing new nodes, axioms, or spatial operators:
1. **Falsifiability:** Every node or connection constraint must include a formal definition that can be refuted by empirical physical measurement.
2. **SMT Proof Invariant:** Ensure your constraints compile cleanly to `z3.Solver()` without non-terminating quantifier loops.
3. **Active Inference Grounding:** If a new topology variant is introduced, map its parameters so they can be written back into the `reality_anchors` table with a measurable physical delta ($\Delta$).

---

## 💻 Development Workflow

1. Fork the repository and create your feature branch from `main`:
   ```bash
   git checkout -b feature/conservation-law-topology
   ```
2. Verify all Python solver unit tests:
   ```bash
   cd backend
   pytest
   ```
3. Verify all TypeScript types and frontend tests:
   ```bash
   cd frontend
   pnpm lint
   pnpm test
   ```
4. Submit your Pull Request detailing the mathematical/physical derivation of your changes.
