use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use babel_core::reconstruct_block;
use ndarray::Array3;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(KSpaceConfig {
            seed: 42_069_1337,
            center: (0, 0, 0),
            n: 8, // 8x8x8 voxel block dimension
            needs_update: true,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Codex Babel - Interactive Voxel Explorer".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_environment, setup_ui))
        .add_systems(
            Update,
            (
                player_look,
                player_move,
                kspace_navigation,
                rebuild_grid,
                update_cymatic_resonance,
            ),
        )
        .run();
}

/// Dynamic K-space configuration resource.
#[derive(Resource)]
struct KSpaceConfig {
    seed: u64,
    center: (i64, i64, i64),
    n: usize,
    needs_update: bool,
}

/// Component marking individual voxels with their baseline amplitude and spatial indices.
#[derive(Component)]
struct VoxelNode {
    amplitude: f32,
    grid_pos: (usize, usize, usize),
    base_y: f32,
}

/// Component marking the player's fly camera and its look orientation.
#[derive(Component)]
struct Player {
    yaw: f32,
    pitch: f32,
}

/// Component marking the UI Coordinate Text section.
#[derive(Component)]
struct CoordinateText;

/// Component marking the UI Decrypted Babel Page section.
#[derive(Component)]
struct BabelPageText;

fn setup_environment(mut commands: Commands) {
    // Spawn directional lighting outlining voxel structures
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 12000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 12.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ambient background glow
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.02, 0.05, 0.08),
        brightness: 1.2,
    });

    // Spawn player camera pointing at the voxel lattice
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player {
            yaw: -0.78, // start rotated slightly
            pitch: -0.4,
        },
    ));
}

fn setup_ui(mut commands: Commands) {
    // Spawn full-screen HUD container
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top Bar: Coordinate readout
            parent.spawn((
                TextBundle::from_section(
                    "K-SPACE COORDINATES: [U: 0, V: 0, W: 0] | SEED: 420691337",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.0, 0.9, 1.0),
                        ..default()
                    },
                ),
                CoordinateText,
            ));

            // Main central region containing the formatted Babel string block
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|center_parent| {
                    center_parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(20.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgba(0.01, 0.02, 0.06, 0.90)),
                            border_color: BorderColor(Color::rgb(0.95, 0.25, 0.72)),
                            ..default()
                        })
                        .with_children(|box_parent| {
                            box_parent.spawn((
                                TextBundle::from_section(
                                    "Loading Decoded Babel Page...",
                                    TextStyle {
                                        font_size: 18.0,
                                        color: Color::rgb(0.9, 0.95, 1.0),
                                        ..default()
                                    },
                                ),
                                BabelPageText,
                            ));
                        });
                });

            // Bottom Bar: Fly controls & traverser keyboard mappings
            parent.spawn(TextBundle::from_section(
                "FLY BINDINGS: [W/S/A/D] Move | [Space] Up | [C] Down | [Mouse] Look\nK-SPACE Traversal: [Arrow Keys] Shift U/V | [PageUp/PageDown] Shift W",
                TextStyle {
                    font_size: 14.0,
                    color: Color::rgb(0.5, 0.6, 0.7),
                    ..default()
                },
            ));
        });
}

/// Allows the player to fly around using keyboard actions.
fn player_move(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    if let Ok((mut transform, _player)) = query.get_single_mut() {
        let speed = if keyboard.pressed(KeyCode::ShiftLeft) { 15.0 } else { 5.0 };
        let mut direction = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if keyboard.pressed(KeyCode::W) {
            direction += forward;
        }
        if keyboard.pressed(KeyCode::S) {
            direction -= forward;
        }
        if keyboard.pressed(KeyCode::A) {
            direction -= right;
        }
        if keyboard.pressed(KeyCode::D) {
            direction += right;
        }
        if keyboard.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }
        if keyboard.pressed(KeyCode::C) {
            direction -= Vec3::Y;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * speed * time.delta_seconds();
        }
    }
}

/// Allows the player to rotate the camera orientation using mouse drift.
fn player_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let mut rotation_delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        rotation_delta += event.delta;
    }

    if rotation_delta.length_squared() > 0.0 {
        if let Ok((mut transform, mut player)) = query.get_single_mut() {
            let sensitivity = 0.0025;
            player.yaw -= rotation_delta.x * sensitivity;
            player.pitch -= rotation_delta.y * sensitivity;
            
            // Clamp pitch to avoid full inversion
            player.pitch = player.pitch.clamp(-1.54, 1.54);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw)
                * Quat::from_axis_angle(Vec3::X, player.pitch);
        }
    }
}

/// Shifts active coordinates inside the K-space generator upon keyboard triggers.
fn kspace_navigation(
    keyboard: Res<Input<KeyCode>>,
    mut config: ResMut<KSpaceConfig>,
) {
    let mut moved = false;

    if keyboard.just_pressed(KeyCode::Right) {
        config.center.0 += 1;
        moved = true;
    }
    if keyboard.just_pressed(KeyCode::Left) {
        config.center.0 -= 1;
        moved = true;
    }
    if keyboard.just_pressed(KeyCode::Up) {
        config.center.1 += 1;
        moved = true;
    }
    if keyboard.just_pressed(KeyCode::Down) {
        config.center.1 -= 1;
        moved = true;
    }
    if keyboard.just_pressed(KeyCode::PageUp) {
        config.center.2 += 1;
        moved = true;
    }
    if keyboard.just_pressed(KeyCode::PageDown) {
        config.center.2 -= 1;
        moved = true;
    }

    if moved {
        config.needs_update = true;
    }
}

/// Despawns existing grids, recalculates values, and generates the new physical lattice
/// while updating coordinates and readable outputs in the UI panels.
fn rebuild_grid(
    mut commands: Commands,
    mut config: ResMut<KSpaceConfig>,
    query: Query<Entity, With<VoxelNode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut coordinate_text_query: Query<&mut Text, (With<CoordinateText>, Without<BabelPageText>)>,
    mut babel_text_query: Query<&mut Text, (With<BabelPageText>, Without<CoordinateText>)>,
) {
    if !config.needs_update {
        return;
    }

    // 1. Remove all old voxel representations
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // 2. Perform 3D IFFT and alpha translation from coordinates
    if let Ok((text, amplitudes)) = reconstruct_block(config.seed, config.center, config.n) {
        let voxel_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.85 }));
        let offset = (config.n as f32 - 1.0) * 0.5;

        // 3. Rebuild voxel entities
        for x in 0..config.n {
            for y in 0..config.n {
                for z in 0..config.n {
                    let amp = amplitudes[[x, y, z]] as f32;

                    if amp > 0.08 {
                        let color = Color::rgb(
                            amp * 1.5,
                            (1.0 - amp) * 0.4,
                            amp * 0.8 + 0.3,
                        );

                        let x_pos = x as f32 - offset;
                        let y_pos = y as f32 - offset;
                        let z_pos = z as f32 - offset;

                        commands.spawn((
                            PbrBundle {
                                mesh: voxel_mesh.clone(),
                                material: materials.add(StandardMaterial {
                                    base_color: color,
                                    metallic: 0.6,
                                    roughness: 0.2,
                                    emissive: color * amp * 0.4,
                                    ..default()
                                }),
                                transform: Transform::from_xyz(x_pos, y_pos, z_pos),
                                ..default()
                            },
                            VoxelNode {
                                amplitude: amp,
                                grid_pos: (x, y, z),
                                base_y: y_pos,
                            },
                        ));
                    }
                }
            }
        }

        // 4. Update HUD text values
        for mut text_node in coordinate_text_query.iter_mut() {
            text_node.sections[0].value = format!(
                "K-SPACE COORDINATES: [U: {}, V: {}, W: {}] | SEED: {}",
                config.center.0, config.center.1, config.center.2, config.seed
            );
        }

        for mut text_node in babel_text_query.iter_mut() {
            // Format 512 characters into a neat 8 lines x 64 characters layout resembling a page
            let mut formatted_page = String::new();
            let chars: Vec<char> = text.chars().collect();
            for chunk in chars.chunks(64) {
                let line: String = chunk.iter().collect();
                formatted_page.push_str(&line);
                formatted_page.push('\n');
            }
            text_node.sections[0].value = formatted_page;
        }
    }

    config.needs_update = false;
}

/// Implements shifting cymatic scales and wave jitters based on time and physical amplitudes.
fn update_cymatic_resonance(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &VoxelNode)>,
) {
    let current_time = time.elapsed_seconds();

    for (mut transform, voxel) in query.iter_mut() {
        let amp = voxel.amplitude;
        let (x, y, z) = voxel.grid_pos;
        let phase = (x + y + z) as f32 * 0.25;

        // Dynamic scale oscillation (breathing)
        let scale_mod = 1.0 + (current_time * 2.0 + phase).sin() * amp * 0.15;
        transform.scale = Vec3::splat(scale_mod);

        // Micro-displacement vibrations simulating sound waves
        let vibration = (current_time * 4.0 + phase).cos() * amp * 0.05;
        transform.translation.y = voxel.base_y + vibration;
    }
}
