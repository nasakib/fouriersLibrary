"use client";

import React from "react";
import { motion } from "framer-motion";
import { Zap, Gauge, Droplets, Database, Flame, Cpu } from "lucide-react";
import { useEngineStore } from "@/store/engineStore";

interface CatalogItem {
  type: string;
  domain: string;
  color: string;
  icon: React.ReactNode;
  label: string;
}

const INVENTORY_ITEMS: CatalogItem[] = [
  {
    type: "Battery",
    domain: "Electronics",
    color: "#00F0FF",
    label: "Power Cell",
    icon: <Zap className="w-5 h-5 text-[#00F0FF]" />,
  },
  {
    type: "Pump",
    domain: "Fluid Dynamics",
    color: "#FF007A",
    label: "Fluid Impeller",
    icon: <Droplets className="w-5 h-5 text-[#FF007A]" />,
  },
  {
    type: "Sensor",
    domain: "Electronics",
    color: "#FFB800",
    label: "Telemetry Probe",
    icon: <Gauge className="w-5 h-5 text-[#FFB800]" />,
  },
  {
    type: "Reservoir",
    domain: "Fluid Dynamics",
    color: "#00FF85",
    label: "Buffer Tank",
    icon: <Database className="w-5 h-5 text-[#00FF85]" />,
  },
  {
    type: "Radiator",
    domain: "Thermodynamics",
    color: "#9D00FF",
    label: "Thermal Exchanger",
    icon: <Flame className="w-5 h-5 text-[#9D00FF]" />,
  },
  {
    type: "Controller",
    domain: "Formal Logic",
    color: "#FFFFFF",
    label: "Logic Core",
    icon: <Cpu className="w-5 h-5 text-white" />,
  },
];

export default function InventoryTray() {
  const { addWorkbenchEntity } = useEngineStore();

  const handleSpawn = (item: CatalogItem) => {
    // Generate jittered position around origin
    const jitterX = (Math.random() - 0.5) * 4.0;
    const jitterY = (Math.random() - 0.5) * 2.0;
    const jitterZ = (Math.random() - 0.5) * 3.0;

    addWorkbenchEntity({
      type: item.type,
      position: [jitterX, jitterY, jitterZ],
      color: item.color,
      domain: item.domain,
    });
  };

  return (
    <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-40">
      <motion.div
        initial={{ y: 50, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ duration: 0.4, ease: "easeOut" }}
        className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-surface-dark/80 backdrop-blur-xl border border-surface-border shadow-2xl shadow-black/80"
      >
        <div className="flex flex-col pr-3 border-r border-surface-border">
          <span className="text-[10px] uppercase font-mono tracking-wider text-slate-400">
            Axiomatic Elements
          </span>
          <span className="text-xs font-semibold text-slate-200">
            Click to Materialize
          </span>
        </div>

        <div className="flex items-center gap-2">
          {INVENTORY_ITEMS.map((item) => (
            <motion.button
              key={item.type}
              whileHover={{ scale: 1.08, y: -2 }}
              whileTap={{ scale: 0.95 }}
              onClick={() => handleSpawn(item)}
              className="group relative flex flex-col items-center justify-center w-14 h-14 rounded-xl bg-surface-elevated/70 border border-surface-border/60 hover:border-slate-400 transition-colors"
            >
              <div className="p-1 rounded-lg transition-transform group-hover:scale-110">
                {item.icon}
              </div>
              <span className="text-[9px] font-mono text-slate-300 mt-0.5 tracking-tight">
                {item.label.split(" ")[0]}
              </span>

              {/* Tooltip */}
              <div className="absolute -top-9 left-1/2 -translate-x-1/2 px-2 py-0.5 rounded bg-slate-900 border border-slate-700 text-[10px] font-mono whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none text-slate-200 shadow-lg">
                + {item.label} ({item.domain})
              </div>
            </motion.button>
          ))}
        </div>
      </motion.div>
    </div>
  );
}
