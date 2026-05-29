// Custom WGSL Chladni resonance shader for Codex Babel voxel faces.

struct FragmentInput {
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> time: f32;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let pi = 3.14159265359;
    
    // Normalize coordinates on voxel face [0, 1]
    let u = in.uv.x;
    let v = in.uv.y;
    
    // Wave parameters modulated by continuous time (frequency shifting)
    let n = 4.0 + sin(time * 0.5) * 2.0;
    let m = 3.0 + cos(time * 0.7) * 1.5;
    
    // Standard Chladni boundary wave equation:
    // w = cos(n*pi*x)*cos(m*pi*y) - cos(m*pi*x)*cos(n*pi*y)
    let wave_a = cos(n * pi * u) * cos(m * pi * v);
    let wave_b = cos(m * pi * u) * cos(n * pi * v);
    let resonance = wave_a - wave_b;
    
    // Create distinct resonant nodal lines
    let nodal_line = 1- smoothstep(0.0, 0.08, abs(resonance));
    
    // Dynamic color gradient driven by wave intensity
    let base_color = vec3<f32>(0.03, 0.08, 0.15); // Deep space teal
    let nodal_glow = vec3<f32>(0.95, 0.25, 0.72) * nodal_line; // Vivid magenta nodal bands
    let harmonic_bg = vec3<f32>(0.12, 0.10, 0.35) * abs(resonance); // Shifting indigo peaks
    
    let final_color = base_color + nodal_glow + harmonic_bg;
    
    return vec4<f32>(final_color, 1.0);
}
