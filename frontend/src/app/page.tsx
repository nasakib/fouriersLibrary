"use client";

import React, { useEffect } from "react";
import AmbientCanvas from "@/components/AmbientCanvas";
import FocusDial from "@/components/FocusDial";
import InventoryTray from "@/components/InventoryTray";
import RealityTether from "@/components/RealityTether";
import { useEngineStore } from "@/store/engineStore";
import { generateNoise } from "@/lib/api";
import { Compass, RotateCcw, Activity } from "lucide-react";

export default function WorkspacePage() {
  const {
    engineMode,
    setEngineMode,
    workbenchEntities,
    entropyLevel,
    activeResonance,
    setActiveResonance,
    resetWorkspace,
  } = useEngineStore();

  // Initial noise synthesis on entity changes
  useEffect(() => {
    if (workbenchEntities.length > 0 && entropyLevel >= 90) {
      generateNoise(workbenchEntities)
        .then((res) => setActiveResonance(res))
        .catch((err) => console.warn("Using offline noise fallback:", err));
    }
  }, [workbenchEntities.length]);

  return (
    <main className="relative w-full h-full min-h-screen overflow-hidden bg-void select-none">
      {/* 3D WebGL Spatial Canvas */}
      <AmbientCanvas />

      {/* Top Header / Mode Switcher */}
      <header className="fixed top-6 left-8 z-40 flex items-center gap-4">
        <div className="flex items-center gap-3 px-4 py-2 rounded-2xl bg-surface-dark/80 backdrop-blur-xl border border-surface-border">
          <div className="w-2.5 h-2.5 rounded-full bg-neon-cyan animate-pulse shadow-neon-cyan" />
          <div className="flex flex-col">
            <h1 className="text-xs font-mono font-bold uppercase tracking-widest text-white">
              Symbiotic Truth Engine
            </h1>
            <span className="text-[10px] font-mono text-slate-400">
              Vector Lens v1.0.0 · First Principles
            </span>
          </div>
        </div>

        {/* Engine Mode Toggle */}
        <div className="flex items-center p-1 rounded-xl bg-surface-dark/80 backdrop-blur-xl border border-surface-border">
          <button
            onClick={() => setEngineMode("curriculum")}
            className={`px-3 py-1 rounded-lg text-xs font-mono uppercase tracking-wider transition-all ${
              engineMode === "curriculum"
                ? "bg-neon-cyan text-slate-950 font-bold shadow-neon-cyan"
                : "text-slate-400 hover:text-white"
            }`}
          >
            Curriculum
          </button>
          <button
            onClick={() => setEngineMode("sandbox")}
            className={`px-3 py-1 rounded-lg text-xs font-mono uppercase tracking-wider transition-all ${
              engineMode === "sandbox"
                ? "bg-neon-cyan text-slate-950 font-bold shadow-neon-cyan"
                : "text-slate-400 hover:text-white"
            }`}
          >
            Sandbox
          </button>
        </div>

        {/* Reset Button */}
        <button
          onClick={resetWorkspace}
          title="Reset Workspace"
          className="p-2.5 rounded-xl bg-surface-dark/80 backdrop-blur-xl border border-surface-border text-slate-400 hover:text-white hover:border-slate-400 transition-colors"
        >
          <RotateCcw className="w-4 h-4" />
        </button>
      </header>

      {/* Real-time Topological Invariants Pill */}
      <div className="fixed bottom-28 left-8 z-40 flex items-center gap-3 px-4 py-2.5 rounded-xl bg-surface-dark/80 backdrop-blur-xl border border-surface-border text-[11px] font-mono text-slate-300">
        <div className="flex items-center gap-1.5 text-neon-cyan">
          <Compass className="w-4 h-4" />
          <span>Homology:</span>
        </div>
        <span>β₀: {activeResonance?.betti_numbers?.beta_0 ?? 1}</span>
        <span className="text-slate-600">|</span>
        <span>β₁: {activeResonance?.betti_numbers?.beta_1 ?? 0}</span>
        <span className="text-slate-600">|</span>
        <span className="text-neon-amber">
          Coherence: {((activeResonance?.coherence ?? 0.15) * 100).toFixed(0)}%
        </span>
      </div>

      {/* Interactive Controls & HUD Overlays */}
      <FocusDial />
      <RealityTether />
      <InventoryTray />
    </main>
  );
}
