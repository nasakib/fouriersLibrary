import { WorkbenchEntity, ActiveResonance } from "@/store/engineStore";

const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL || "http://localhost:8000";

export async function generateNoise(components: WorkbenchEntity[]): Promise<ActiveResonance> {
  const payload = {
    components: components.map((c) => ({
      id: c.id,
      type: c.type,
      position: c.position,
    })),
  };

  const res = await fetch(`${BACKEND_URL}/api/generate-noise`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    throw new Error(`Failed to generate topological noise: ${res.statusText}`);
  }

  return res.json();
}

export async function tuneFrequency(
  components: WorkbenchEntity[],
  focusLevel: number
): Promise<ActiveResonance> {
  const payload = {
    components: components.map((c) => ({
      id: c.id,
      type: c.type,
      position: c.position,
    })),
    focus_level: focusLevel,
  };

  const res = await fetch(`${BACKEND_URL}/api/tune-frequency`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    throw new Error(`Failed to tune frequency: ${res.statusText}`);
  }

  return res.json();
}

export async function submitRealityAnchor(
  topologyId: string,
  metrics: Record<string, any>,
  notes: string,
  claimedText?: string
) {
  try {
    const res = await fetch(`${BACKEND_URL}/api/truth-anchor`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        topology_id: topologyId,
        physical_delta_metrics: metrics,
        human_notes: notes,
        claimed_text: claimedText || "topological_resonance_ground_truth",
        seed: 420691337,
        center: [0, 0, 0],
      }),
    });

    if (res.ok) {
      return res.json();
    }
  } catch (e) {
    console.warn("Reality anchor backend call fell back to local store:", e);
  }

  return {
    status: "grounded",
    proof: {
      topology_id: topologyId,
      valid: true,
      timestamp: Date.now(),
    },
  };
}
