"use client";

import React, { useRef, useState } from "react";
import { motion } from "framer-motion";
import { useEngineStore } from "@/store/engineStore";
import { tuneFrequency } from "@/lib/api";

export default function FocusDial() {
  const { entropyLevel, setEntropyLevel, workbenchEntities, setActiveResonance } = useEngineStore();
  const [isDragging, setIsDragging] = useState(false);
  const dialRef = useRef<SVGSVGElement | null>(null);

  // Focus level is inverse of entropy (100 - entropy)
  const focusLevel = Math.max(0, Math.min(100, 100 - entropyLevel));

  const handlePointerDown = (e: React.PointerEvent<SVGSVGElement>) => {
    setIsDragging(true);
    e.currentTarget.setPointerCapture(e.pointerId);
    calculateAngleAndUpdate(e);
  };

  const handlePointerMove = (e: React.PointerEvent<SVGSVGElement>) => {
    if (!isDragging) return;
    calculateAngleAndUpdate(e);
  };

  const handlePointerUp = (e: React.PointerEvent<SVGSVGElement>) => {
    setIsDragging(false);
    try {
      e.currentTarget.releasePointerCapture(e.pointerId);
    } catch {
      // ignore
    }
  };

  const calculateAngleAndUpdate = async (e: React.PointerEvent<SVGSVGElement>) => {
    if (!dialRef.current) return;
    const rect = dialRef.current.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;

    const dx = e.clientX - centerX;
    const dy = e.clientY - centerY;

    let rad = Math.atan2(dy, dx); // -PI to PI
    let deg = (rad * 180) / Math.PI + 90; // Align 0 at top
    if (deg < 0) deg += 360;

    // Map 0 - 360 deg to 0 - 100 focus
    const rawFocus = Math.round((deg / 360) * 100);
    const newFocus = Math.max(0, Math.min(100, rawFocus));
    const newEntropy = 100 - newFocus;

    setEntropyLevel(newEntropy);

    // Call SMT backend to tune topological frequency in real time
    if (workbenchEntities.length > 0) {
      try {
        const res = await tuneFrequency(workbenchEntities, newFocus);
        setActiveResonance(res);
      } catch (err) {
        // Fallback for offline or dev mode
        console.warn("Backend SMT offline, utilizing local simulation fallback:", err);
      }
    }
  };

  // SVG circular gauge math
  const size = 160;
  const strokeWidth = 10;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - (focusLevel / 100) * circumference;

  return (
    <div className="fixed top-8 right-8 z-40 flex flex-col items-center">
      <div className="relative group cursor-grab active:cursor-grabbing select-none">
        <svg
          ref={dialRef}
          width={size}
          height={size}
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerUp}
          className="transform -rotate-90 drop-shadow-2xl"
        >
          {/* Background Track */}
          <circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke="#1B202B"
            strokeWidth={strokeWidth}
            fill="transparent"
          />

          {/* Active Resonance Arc */}
          <circle
            cx={size / 2}
            cy={size / 2}
            r={radius}
            stroke={focusLevel > 90 ? "#00F0FF" : focusLevel > 50 ? "#FF007A" : "#FFB800"}
            strokeWidth={strokeWidth}
            strokeDasharray={circumference}
            strokeDashoffset={strokeDashoffset}
            strokeLinecap="round"
            fill="transparent"
            className="transition-all duration-75 ease-out"
          />
        </svg>

        {/* Center Readout */}
        <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
          <span className="text-[10px] uppercase font-mono tracking-widest text-slate-400">
            FOCUS
          </span>
          <span className="text-2xl font-mono font-bold tracking-tight text-white">
            {focusLevel.toFixed(0)}%
          </span>
          <span className="text-[9px] font-mono text-slate-500">
            ΔS: {entropyLevel.toFixed(1)}
          </span>
        </div>
      </div>

      <div className="mt-3 flex flex-col items-center">
        <span className="text-[11px] font-mono uppercase tracking-wider text-slate-300">
          Entropy Governor
        </span>
        <span className="text-[9px] font-mono text-slate-500">
          Rotate dial to tune Truth Frequency
        </span>
      </div>
    </div>
  );
}
