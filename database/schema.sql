-- ==============================================================================
-- The Symbiotic Truth Engine (fourier-truth-engine)
-- Database Schema: Supabase PostgreSQL DDL with Row Level Security (RLS)
-- ==============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ------------------------------------------------------------------------------
-- 1. Table: first_principles
-- Immutable repository of scientific axioms, physical conservation laws,
-- and formal logical constraints.
-- ------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.first_principles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    domain VARCHAR(100) NOT NULL, -- e.g., 'electromagnetism', 'thermodynamics', 'fluid_dynamics', 'formal_logic'
    axiom_logic JSONB NOT NULL,   -- SMT/Z3-compatible predicates, conservation formulas, typed port rules
    verified_status BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

COMMENT ON TABLE public.first_principles IS 'Axiomatic physical laws and logical formalisms utilized by the Z3 SMT solver.';
COMMENT ON COLUMN public.first_principles.axiom_logic IS 'JSON schema containing variables, constraints, and conservation equations.';

-- ------------------------------------------------------------------------------
-- 2. Table: topologies
-- User-generated or solver-tuned 3D topological graphs and blueprints.
-- ------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.topologies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    spatial_matrix_json JSONB NOT NULL, -- Coordinates, 3D simplicial complex, node graphs, edge tensors
    coherence_score DOUBLE PRECISION NOT NULL DEFAULT 0.0 CHECK (coherence_score >= 0.0 AND coherence_score <= 1.0),
    focus_level DOUBLE PRECISION NOT NULL DEFAULT 100.0 CHECK (focus_level >= 0.0 AND focus_level <= 100.0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

COMMENT ON TABLE public.topologies IS 'Computed 3D topological manifolds and resolved Truth Frequency blueprints.';
COMMENT ON COLUMN public.topologies.coherence_score IS 'Degree of mathematical satisfiability and harmonic resonance (0.0 = pure noise, 1.0 = optimal truth).';

-- ------------------------------------------------------------------------------
-- 3. Table: reality_anchors
-- Active Inference feedback records: User empirical observations, physical
-- sensor metrics, and falsification test logs grounded against the topology.
-- ------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.reality_anchors (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    topology_id UUID NOT NULL REFERENCES public.topologies(id) ON DELETE CASCADE,
    physical_delta_metrics JSONB NOT NULL, -- Measured vs. predicted physical values (e.g. voltage error, flow loss)
    human_notes TEXT,                      -- Qualitative laboratory observations, empirical falsification notes
    grounded_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

COMMENT ON TABLE public.reality_anchors IS 'Empirical ground truth feedback loop closing the Active Inference Free Energy cycle.';

-- ------------------------------------------------------------------------------
-- Indexes for High Performance Queries
-- ------------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_first_principles_domain ON public.first_principles(domain);
CREATE INDEX IF NOT EXISTS idx_topologies_user_id ON public.topologies(user_id);
CREATE INDEX IF NOT EXISTS idx_topologies_coherence ON public.topologies(coherence_score);
CREATE INDEX IF NOT EXISTS idx_reality_anchors_topology ON public.reality_anchors(topology_id);

-- ------------------------------------------------------------------------------
-- Row Level Security (RLS) Policies
-- ------------------------------------------------------------------------------
ALTER TABLE public.first_principles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.topologies ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.reality_anchors ENABLE ROW LEVEL SECURITY;

-- First Principles: Publicly readable by all authenticated and anonymous users
CREATE POLICY "Allow public read access to first principles"
    ON public.first_principles FOR SELECT
    USING (true);

-- Topologies: Users can read and write their own topologies; anonymous users can read public blueprints
CREATE POLICY "Users can manage their own topologies"
    ON public.topologies FOR ALL
    USING (auth.uid() = user_id OR user_id IS NULL)
    WITH CHECK (auth.uid() = user_id OR user_id IS NULL);

-- Reality Anchors: Users can read and write reality anchors linked to accessible topologies
CREATE POLICY "Users can manage reality anchors"
    ON public.reality_anchors FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM public.topologies t
            WHERE t.id = reality_anchors.topology_id
            AND (t.user_id = auth.uid() OR t.user_id IS NULL)
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM public.topologies t
            WHERE t.id = reality_anchors.topology_id
            AND (t.user_id = auth.uid() OR t.user_id IS NULL)
        )
    );

-- ------------------------------------------------------------------------------
-- Initial Seed Data: First Principles Baseline
-- ------------------------------------------------------------------------------
INSERT INTO public.first_principles (domain, axiom_logic, verified_status)
VALUES 
    (
        'electronics',
        '{
            "name": "Kirchhoff Current Law",
            "principle": "sum_currents_in = sum_currents_out",
            "forbidden_patterns": [
                {"type": "short_circuit", "rule": "battery_pos_direct_to_neg"},
                {"type": "floating_load", "rule": "load_without_ground_return"}
            ]
        }',
        true
    ),
    (
        'fluid_dynamics',
        '{
            "name": "Continuity Equation & Mass Conservation",
            "principle": "inflow_mass_rate = outflow_mass_rate",
            "forbidden_patterns": [
                {"type": "dead_end_pump", "rule": "pump_output_blocked_no_reservoir"},
                {"type": "cavitation_hazard", "rule": "negative_inlet_pressure"}
            ]
        }',
        true
    ),
    (
        'thermodynamics',
        '{
            "name": "First & Second Laws",
            "principle": "delta_U = Q - W and delta_S_universe >= 0",
            "forbidden_patterns": [
                {"type": "perpetual_motion", "rule": "work_extracted_exceeds_input_energy"}
            ]
        }',
        true
    );
