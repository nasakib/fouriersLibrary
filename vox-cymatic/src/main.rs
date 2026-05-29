use bevy::prelude::*;
use babel_core::reconstruct_block;
use ndarray::Array3;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Codex Babel - Vox Cymatic Renderer".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update_cymatic_resonance)
        .run();
}

/// Resource holding the active frequency and spatial amplitude buffers.
#[derive(Resource)]
struct ActiveFrequencyBuffer {
    _seed: u64,
    _center: (i64, i64, i64),
    amplitudes: Array3<f64>,
    time: f32,
}

/// Component marking individual voxels with their baseline amplitude and spatial indices.
#[derive(Component)]
struct VoxelNode {
    amplitude: f32,
    grid_pos: (usize, usize, usize),
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let seed = 42_069_1337;
    let center = (0, 0, 0);
    let grid_size = 8; // 8x8x8 voxel node grid

    // Reconstruct the spatial wave field from K-space coordinates
    let (_text, amplitudes) = reconstruct_block(seed, center, grid_size)
        .expect("Failed to execute K-space spatial wave reconstruction");

    commands.insert_resource(ActiveFrequencyBuffer {
        _seed: seed,
        _center: center,
        amplitudes: amplitudes.clone(),
        time: 0.0,
    });

    // Create a mesh and parent transform root
    let voxel_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.85 }));

    // Center offset to align the 8x8x8 voxel cube around (0,0,0)
    let offset = (grid_size as f32 - 1.0) * 0.5;

    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                let amp = amplitudes[[x, y, z]] as f32;

                // Thresholding to keep low-energy space empty, forming high-energy resonant lattices
                if amp > 0.08 {
                    // Harmonious visual color mapping:
                    // High energy -> Vibrant Magenta/Cymatic Rose
                    // Moderate energy -> Electric Indigo
                    // Low energy -> Deep Sea Teal
                    let color = Color::rgb(
                        amp * 1.5,
                        (1.0 - amp) * 0.4,
                        amp * 0.8 + 0.3,
                    );

                    let transform = Transform::from_xyz(
                        x as f32 - offset,
                        y as f32 - offset,
                        z as f32 - offset,
                    );

                    commands.spawn((
                        PbrBundle {
                            mesh: voxel_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                metallic: 0.6,
                                roughness: 0.2,
                                // Add emissive glow matching active energy
                                emissive: color * amp * 0.4,
                                ..default()
                            }),
                            transform,
                            ..default()
                        },
                        VoxelNode {
                            amplitude: amp,
                            grid_pos: (x, y, z),
                        },
                    ));
                }
            }
        }
    }

    // Spawn smooth directional lighting to outline voxel structures
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 12000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ambient background glow
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.02, 0.05, 0.08),
        brightness: 1.2,
    });

    // Spawn camera pointing at the voxel lattice
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Dynamically modulates voxel scale, rotation, and translation based on wave resonance
/// simulating shifting Chladni frequency behaviors on voxel nodes.
fn update_cymatic_resonance(
    time: Res<Time>,
    mut buffer: ResMut<ActiveFrequencyBuffer>,
    mut query: Query<(&mut Transform, &VoxelNode)>,
) {
    buffer.time += time.delta_seconds();
    let current_time = buffer.time;

    for (mut transform, voxel) in query.iter_mut() {
        // Base resonance coefficient
        let amp = voxel.amplitude;

        // Spatial indices modulate phase frequency
        let (x, y, z) = voxel.grid_pos;
        let phase = (x + y + z) as f32 * 0.25;

        // Dynamic scale oscillation driven by time and amplitude (Cymatic breathing)
        let scale_mod = 1.0 + (current_time * 2.0 + phase).sin() * amp * 0.15;
        transform.scale = Vec3::splat(scale_mod);

        // Dynamic coordinate jitter simulating wave propagation
        let displacement = (current_time * 3.0 + phase).cos() * amp * 0.08;
        transform.translation.y += displacement * 0.03; // micro-vibrations
    }
}
