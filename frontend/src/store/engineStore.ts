import { create } from "zustand";

export type EngineMode = "curriculum" | "sandbox";

export interface WorkbenchEntity {
  id: string;
  type: string;
  position: [number, number, number];
  domain?: string;
  color?: string;
}

export interface TopologicalEdge {
  source: string;
  target: string;
  valid: boolean;
  reason?: string;
  weight?: number;
}

export interface FalsifiableMetric {
  parameter: string;
  expected: string;
  unit: string;
}

export interface RosettaHypothesis {
  title: string;
  formal_statement: string;
  code_snippet: string;
  falsifiable_metrics: FalsifiableMetric[];
}

export interface ActiveResonance {
  entropy: number;
  coherence: number;
  nodes: WorkbenchEntity[];
  edges: TopologicalEdge[];
  betti_numbers?: {
    beta_0: number;
    beta_1: number;
  };
  hypothesis?: RosettaHypothesis | null;
}

interface EngineState {
  engineMode: EngineMode;
  workbenchEntities: WorkbenchEntity[];
  entropyLevel: number; // 100.0 (total chaotic static) down to 0.0 (pure ground truth)
  activeResonance: ActiveResonance | null;
  awaitingGroundTruth: boolean;
  rejectedConnection: { source: string; target: string; reason: string } | null;

  // Actions
  setEngineMode: (mode: EngineMode) => void;
  setEntropyLevel: (level: number) => void;
  addWorkbenchEntity: (entity: Omit<WorkbenchEntity, "id">) => void;
  removeWorkbenchEntity: (id: string) => void;
  updateEntityPosition: (id: string, position: [number, number, number]) => void;
  setActiveResonance: (resonance: ActiveResonance | null) => void;
  setAwaitingGroundTruth: (status: boolean) => void;
  setRejectedConnection: (rejection: { source: string; target: string; reason: string } | null) => void;
  resetWorkspace: () => void;
}

export const useEngineStore = create<EngineState>((set, get) => ({
  engineMode: "sandbox",
  workbenchEntities: [
    { id: "node-1", type: "Battery", position: [-2.5, 0, 0], color: "#00F0FF", domain: "electronics" },
    { id: "node-2", type: "Pump", position: [0, 1.5, 0], color: "#FF007A", domain: "fluid_dynamics" },
    { id: "node-3", type: "Sensor", position: [2.5, 0, 0], color: "#FFB800", domain: "electronics" },
  ],
  entropyLevel: 100.0,
  activeResonance: null,
  awaitingGroundTruth: false,
  rejectedConnection: null,

  setEngineMode: (mode) => set({ engineMode: mode }),
  
  setEntropyLevel: (level) => {
    const clamped = Math.max(0, Math.min(100, level));
    set({
      entropyLevel: clamped,
      awaitingGroundTruth: clamped <= 0.01,
    });
  },

  addWorkbenchEntity: (entity) => {
    const newId = `node-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
    const newEntity: WorkbenchEntity = {
      ...entity,
      id: newId,
    };
    set((state) => ({
      workbenchEntities: [...state.workbenchEntities, newEntity],
      entropyLevel: 100.0, // Re-scramble when system composition changes
      awaitingGroundTruth: false,
    }));
  },

  removeWorkbenchEntity: (id) => {
    set((state) => ({
      workbenchEntities: state.workbenchEntities.filter((e) => e.id !== id),
      entropyLevel: 100.0,
      awaitingGroundTruth: false,
    }));
  },

  updateEntityPosition: (id, position) => {
    set((state) => ({
      workbenchEntities: state.workbenchEntities.map((e) =>
        e.id === id ? { ...e, position } : e
      ),
    }));
  },

  setActiveResonance: (resonance) => set({ activeResonance: resonance }),
  
  setAwaitingGroundTruth: (status) => set({ awaitingGroundTruth: status }),

  setRejectedConnection: (rejection) => set({ rejectedConnection: rejection }),

  resetWorkspace: () => {
    set({
      workbenchEntities: [],
      entropyLevel: 100.0,
      activeResonance: null,
      awaitingGroundTruth: false,
      rejectedConnection: null,
    });
  },
}));
