"use client";

import React, { useRef, useMemo, useState } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls, Float, MeshTransmissionMaterial, Text } from "@react-three/drei";
import * as THREE from "three";
import { useEngineStore, WorkbenchEntity } from "@/store/engineStore";

// Node Mesh in 3D Space with Glass-like Transmission Material and Neon Glow
function TopologicalNodeMesh({
  entity,
  isRepelled,
}: {
  entity: WorkbenchEntity;
  isRepelled: boolean;
}) {
  const meshRef = useRef<THREE.Mesh>(null);
  const glowRef = useRef<THREE.PointLight>(null);

  // Smooth vibration and tactile repulsion mechanics
  useFrame((state, delta) => {
    if (!meshRef.current) return;
    
    // Tactile Repulsion animation if Z3 SMT flagged violation
    if (isRepelled) {
      meshRef.current.position.y += Math.sin(state.clock.elapsedTime * 30.0) * 0.04;
      meshRef.current.position.x += Math.cos(state.clock.elapsedTime * 25.0) * 0.04;
    }

    meshRef.current.rotation.y += delta * 0.3;
    meshRef.current.rotation.x += delta * 0.15;
  });

  const nodeColor = entity.color || "#00F0FF";

  return (
    <group position={entity.position}>
      <Float speed={2} rotationIntensity={0.5} floatIntensity={0.5}>
        <mesh ref={meshRef}>
          <octahedronGeometry args={[0.7, 0]} />
          {/* Glass-like refractive material */}
          <meshPhysicalMaterial
            color={nodeColor}
            emissive={isRepelled ? "#FF0055" : nodeColor}
            emissiveIntensity={isRepelled ? 2.5 : 0.8}
            roughness={0.1}
            metalness={0.1}
            transmission={0.8}
            thickness={1.2}
            transparent
            opacity={0.9}
          />
        </mesh>
      </Float>

      {/* Internal point light casting neon ambience */}
      <pointLight
        ref={glowRef}
        color={isRepelled ? "#FF0055" : nodeColor}
        intensity={isRepelled ? 4.0 : 2.0}
        distance={4.0}
      />

      {/* Entity Type Label */}
      <Text
        position={[0, -1.0, 0]}
        fontSize={0.25}
        color="#E2E8F0"
        anchorX="center"
        anchorY="middle"
      >
        {entity.type}
      </Text>
    </group>
  );
}

// Chaotic Vibrating GLSL Shader Lines (High-Entropy Noise)
function ChaoticNoiseLines({
  entities,
  entropy,
}: {
  entities: WorkbenchEntity[];
  entropy: number;
}) {
  const lineCount = 30;
  const lines = useMemo(() => {
    if (entities.length < 2) return [];

    const curveList: THREE.CatmullRomCurve3[] = [];
    for (let i = 0; i < lineCount; i++) {
      const src = entities[i % entities.length].position;
      const tgt = entities[(i + 1) % entities.length].position;

      // Jittered midpoint creating turbulent chaotic strings
      const jitter = (entropy / 100.0) * 2.5;
      const mid: [number, number, number] = [
        (src[0] + tgt[0]) / 2 + (Math.random() - 0.5) * jitter,
        (src[1] + tgt[1]) / 2 + (Math.random() - 0.5) * jitter,
        (src[2] + tgt[2]) / 2 + (Math.random() - 0.5) * jitter,
      ];

      curveList.push(
        new THREE.CatmullRomCurve3([
          new THREE.Vector3(...src),
          new THREE.Vector3(...mid),
          new THREE.Vector3(...tgt),
        ])
      );
    }
    return curveList;
  }, [entities, entropy]);

  if (entropy <= 10.0 || lines.length === 0) return null;

  return (
    <group>
      {lines.map((curve, idx) => {
        const points = curve.getPoints(20);
        const geometry = new THREE.BufferGeometry().setFromPoints(points);

        return (
          <line key={idx} geometry={geometry}>
            <lineBasicMaterial
              color={idx % 2 === 0 ? "#FF007A" : "#00F0FF"}
              transparent
              opacity={Math.min(0.6, (entropy / 100.0) * 0.5)}
              linewidth={1}
            />
          </line>
        );
      })}
    </group>
  );
}

// Coherent Resolved Tube Geometries (The Truth Frequency)
function ResolvedTruthTubes({
  entities,
  edges,
  entropy,
}: {
  entities: WorkbenchEntity[];
  edges: Array<{ source: string; target: string; valid: boolean }>;
  entropy: number;
}) {
  const nodeMap = useMemo(() => {
    const map = new Map<string, [number, number, number]>();
    entities.forEach((e) => map.set(e.id, e.position));
    return map;
  }, [entities]);

  // Render clean glowing tubes as entropy drops
  if (entropy > 50.0) return null;

  const validEdges = edges.filter((e) => e.valid);

  return (
    <group>
      {validEdges.map((edge, idx) => {
        const p1 = nodeMap.get(edge.source);
        const p2 = nodeMap.get(edge.target);
        if (!p1 || !p2) return null;

        const curve = new THREE.CatmullRomCurve3([
          new THREE.Vector3(...p1),
          new THREE.Vector3(...p2),
        ]);
        const tubeGeom = new THREE.TubeGeometry(curve, 32, 0.08, 8, false);

        return (
          <mesh key={idx} geometry={tubeGeom}>
            <meshStandardMaterial
              color="#00F0FF"
              emissive="#00F0FF"
              emissiveIntensity={2.0 - (entropy / 50.0) * 1.5}
              roughness={0.2}
              metalness={0.8}
            />
          </mesh>
        );
      })}
    </group>
  );
}

export default function AmbientCanvas() {
  const { workbenchEntities, entropyLevel, activeResonance, rejectedConnection } = useEngineStore();

  const edges = useMemo(() => {
    if (activeResonance?.edges) return activeResonance.edges;
    // Default synthetic connections
    const list = [];
    for (let i = 0; i < workbenchEntities.length - 1; i++) {
      list.push({
        source: workbenchEntities[i].id,
        target: workbenchEntities[i + 1].id,
        valid: true,
      });
    }
    return list;
  }, [activeResonance, workbenchEntities]);

  return (
    <div className="absolute inset-0 w-full h-full bg-[#0B0C10]">
      <Canvas
        camera={{ position: [0, 4, 10], fov: 45 }}
        gl={{ antialias: true, alpha: false }}
      >
        <color attach="background" args={["#0B0C10"]} />
        <ambientLight intensity={0.4} />
        <directionalLight position={[10, 15, 10]} intensity={1.2} />

        <OrbitControls makeDefault enableDamping dampingFactor={0.05} />

        {/* 3D Nodes */}
        {workbenchEntities.map((entity) => {
          const isRepelled =
            rejectedConnection?.source === entity.id ||
            rejectedConnection?.target === entity.id;

          return (
            <TopologicalNodeMesh
              key={entity.id}
              entity={entity}
              isRepelled={isRepelled}
            />
          );
        })}

        {/* Chaotic GLSL Noise Lines (High Entropy) */}
        <ChaoticNoiseLines
          entities={workbenchEntities}
          entropy={entropyLevel}
        />

        {/* Coherent Tube Geometries (Ground Truth Frequency) */}
        <ResolvedTruthTubes
          entities={workbenchEntities}
          edges={edges}
          entropy={entropyLevel}
        />

        {/* Grid Floor */}
        <gridHelper args={[30, 30, "#293142", "#14171E"]} position={[0, -2.5, 0]} />
      </Canvas>
    </div>
  );
}
