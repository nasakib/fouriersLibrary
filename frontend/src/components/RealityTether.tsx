"use client";

import React, { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { CheckCircle2, AlertTriangle, Send, Code, Activity, ShieldCheck } from "lucide-react";
import { useEngineStore } from "@/store/engineStore";
import { submitRealityAnchor } from "@/lib/api";

export default function RealityTether() {
  const { entropyLevel, activeResonance, setAwaitingGroundTruth } = useEngineStore();
  const [metricValues, setMetricValues] = useState<Record<string, string>>({});
  const [notes, setNotes] = useState("");
  const [isSubmitted, setIsSubmitted] = useState(false);

  // Reality Tether appears when entropy drops to 0 (Singular Truth resolved)
  const isOptimal = entropyLevel <= 0.01;
  const hypothesis = activeResonance?.hypothesis;

  const handleMetricChange = (param: string, value: string) => {
    setMetricValues((prev) => ({ ...prev, [param]: value }));
  };

  const handleAnchorSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await submitRealityAnchor(
        "local-topology-active",
        metricValues,
        notes,
        hypothesis?.code_snippet || "topological_resonance_ground_truth"
      );
      setIsSubmitted(true);
      setTimeout(() => {
        setIsSubmitted(false);
        setAwaitingGroundTruth(false);
      }, 2500);
    } catch (err) {
      console.error("Failed to commit reality anchor:", err);
    }
  };

  return (
    <AnimatePresence>
      {isOptimal && (
        <motion.div
          initial={{ y: -60, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          exit={{ y: -60, opacity: 0 }}
          transition={{ duration: 0.5, ease: "easeInOut" }}
          className="fixed top-8 left-8 z-50 w-[420px] max-h-[85vh] overflow-y-auto rounded-2xl bg-surface-dark/95 backdrop-blur-2xl border border-neon-cyan/40 shadow-2xl shadow-cyan-950/40 p-5"
        >
          {/* Header */}
          <div className="flex items-center justify-between pb-3 border-b border-surface-border">
            <div className="flex items-center gap-2">
              <ShieldCheck className="w-5 h-5 text-neon-cyan" />
              <div>
                <h3 className="text-xs font-mono font-bold uppercase tracking-wider text-white">
                  Reality Tether Anchor
                </h3>
                <span className="text-[10px] font-mono text-neon-cyan/90">
                  Resonant Topological Blueprint Resolved
                </span>
              </div>
            </div>
            <div className="px-2 py-0.5 rounded bg-neon-cyan/10 border border-neon-cyan/30 text-[9px] font-mono text-neon-cyan">
              Hypothesis Only
            </div>
          </div>

          {/* Rosetta Stone Section */}
          <div className="mt-4 space-y-3">
            <div>
              <span className="text-[10px] font-mono uppercase text-slate-400">
                Singular Ground Truth
              </span>
              <p className="text-xs text-slate-200 font-medium mt-0.5">
                {hypothesis?.title || "Conservative Closed-Loop Energy Topology"}
              </p>
              <p className="text-[11px] text-slate-400 mt-1 leading-relaxed">
                {hypothesis?.formal_statement ||
                  "Topological manifold satisfies all Kirchhoff and continuity invariants without contradiction."}
              </p>
            </div>

            {/* Code / Schematic Translation */}
            <div className="rounded-xl bg-black/60 border border-surface-border p-3">
              <div className="flex items-center justify-between mb-2">
                <span className="text-[9px] font-mono uppercase text-slate-400 flex items-center gap-1.5">
                  <Code className="w-3.5 h-3.5 text-neon-cyan" />
                  Pictural Calculus Compilation
                </span>
                <span className="text-[9px] font-mono text-slate-500">ZX-Normal Form</span>
              </div>
              <pre className="text-[10px] font-mono text-slate-300 overflow-x-auto leading-relaxed">
                {hypothesis?.code_snippet ||
                  "// ZX Calculus Normal Form\nconnect(Battery, Pump);  // Electric Power bus\nconnect(Pump, Sensor);    // Fluid telemetry flow\nassert(total_dissipation < 15.0); // SAT"}
              </pre>
            </div>

            {/* Empirical Grounding Form */}
            <form onSubmit={handleAnchorSubmit} className="space-y-3 pt-2">
              <div className="flex items-center gap-1.5 text-slate-300">
                <Activity className="w-4 h-4 text-neon-amber" />
                <span className="text-[11px] font-mono uppercase tracking-wide">
                  Falsifiable Physical Delta (Active Inference)
                </span>
              </div>

              {(hypothesis?.falsifiable_metrics || [
                { parameter: "Loop Continuity Voltage", expected: "12.0 V ± 0.2", unit: "V" },
                { parameter: "Mass Flow Invariant", expected: "0.00 kg/s delta", unit: "kg/s" },
              ]).map((m, idx) => (
                <div key={idx} className="flex items-center justify-between gap-3 text-xs">
                  <div className="flex flex-col">
                    <span className="text-slate-300 font-mono text-[11px]">{m.parameter}</span>
                    <span className="text-slate-500 text-[9px] font-mono">Expected: {m.expected}</span>
                  </div>
                  <input
                    type="text"
                    placeholder={`e.g. 11.95`}
                    value={metricValues[m.parameter] || ""}
                    onChange={(e) => handleMetricChange(m.parameter, e.target.value)}
                    className="w-24 px-2 py-1 rounded bg-surface-elevated border border-surface-border text-right text-xs font-mono text-neon-cyan focus:outline-none focus:border-neon-cyan"
                  />
                </div>
              ))}

              <div>
                <label className="text-[10px] font-mono text-slate-400 block mb-1">
                  Empirical Lab Notes & Qualitative Anomalies
                </label>
                <textarea
                  rows={2}
                  value={notes}
                  onChange={(e) => setNotes(e.target.value)}
                  placeholder="Record real-world physical behavior, temperature rise, or acoustic hum..."
                  className="w-full px-3 py-2 rounded-lg bg-surface-elevated border border-surface-border text-xs text-slate-200 font-sans focus:outline-none focus:border-neon-cyan resize-none"
                />
              </div>

              <button
                type="submit"
                disabled={isSubmitted}
                className="w-full mt-2 py-2 px-4 rounded-xl bg-neon-cyan text-slate-950 font-mono text-xs font-bold uppercase tracking-wider flex items-center justify-center gap-2 hover:bg-cyan-300 transition-colors disabled:opacity-50"
              >
                {isSubmitted ? (
                  <>
                    <CheckCircle2 className="w-4 h-4" /> Ground Truth Committed
                  </>
                ) : (
                  <>
                    <Send className="w-4 h-4" /> Commit Reality Anchor
                  </>
                )}
              </button>
            </form>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
