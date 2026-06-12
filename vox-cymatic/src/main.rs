use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use babel_core::{KSpaceGenerator, SignalTransform, AmplitudeTranslator, BABEL_ALPHABET};
use ndarray::Array3;
use rand::Rng;
use num_complex::Complex64;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(KSpaceConfig {
            seed: 42_069_1337,
            center: (0, 0, 0),
            n: 8, // 8x8x8 voxel block dimension
            needs_update: true,
        })
        .insert_resource(VisualMode::Lattice)
        .insert_resource(Presets::default())
        .insert_resource(ZenMode(false))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Codex Babel - Frequency Domain Explorer".into(),
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
                update_stars,
                inspect_voxels,
                spawn_voxel_particles,
                update_particles,
                draw_connections,
                toggle_zen_mode,
                handle_presets_and_modes,
                pulsate_central_glow,
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

/// Active visualization mode resource.
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
enum VisualMode {
    Lattice,
    CymaticSheet,
    SphericalShell,
}

/// Zen Mode toggles visibility of the HUD overlay.
#[derive(Resource)]
struct ZenMode(bool);

/// Presets/bookmarks in K-space.
#[derive(Resource)]
struct Presets {
    list: Vec<((i64, i64, i64), &'static str)>,
    index: usize,
}

impl Default for Presets {
    fn default() -> Self {
        Self {
            list: vec![
                ((0, 0, 0), "Genesis Hub"),
                ((100, -200, 300), "Chladni Singularity"),
                ((-420, 69, 1337), "Resonant Core"),
                ((9999, 9999, 9999), "Deep Semantic Fault"),
                ((88888, -77777, 66666), "Acoustic Superposition"),
            ],
            index: 0,
        }
    }
}

/// Component marking individual voxels with their baseline amplitude, phase, character, and indices.
#[derive(Component)]
struct VoxelNode {
    amplitude: f32,
    phase: f32,
    character: char,
    grid_pos: (usize, usize, usize),
    base_pos: Vec3,
}

/// Component marking the player's fly camera and its look orientation.
#[derive(Component)]
struct Player {
    yaw: f32,
    pitch: f32,
}

/// Component marking background stars.
#[derive(Component)]
struct StarNode {
    orbit_speed: f32,
    orbit_axis: Vec3,
    distance: f32,
    angle: f32,
}

/// Component marking floating frequency particles.
#[derive(Component)]
struct FrequencyParticle {
    velocity: Vec3,
    lifetime: f32,
    max_lifetime: f32,
}

/// Component marking the central point light.
#[derive(Component)]
struct CentralLight;

/// Components for UI text fields.
#[derive(Component)]
struct CoordinateText;

#[derive(Component)]
struct ModeStatusText;

#[derive(Component)]
struct BabelPageText;

#[derive(Component)]
struct VoxelInspectorText;

#[derive(Component)]
struct PresetText;

#[derive(Component)]
struct HudRoot;

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn directional lighting outlining voxel structures
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 15000.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 25.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ambient background glow
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.01, 0.03, 0.06),
        brightness: 1.5,
    });

    // Central pulsing point light
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::rgb(0.0, 0.9, 1.0),
                intensity: 5000.0,
                radius: 10.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CentralLight,
    ));

    // Spawn player camera pointing at the voxel lattice
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8.0, 10.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player {
            yaw: -0.78,
            pitch: -0.4,
        },
    ));

    // Spawn a starry background
    let mut rng = rand::thread_rng();
    let star_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.08 }));
    let star_mat = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.85, 1.0),
        emissive: Color::rgb(1.0, 1.3, 1.7) * 2.0,
        unlit: true,
        ..default()
    });

    for _ in 0..400 {
        let dist = rng.gen_range(28.0..65.0);
        let axis = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ).normalize();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(0.01..0.04);

        let initial_pos = Quat::from_axis_angle(axis, angle) * (Vec3::X * dist);

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_mat.clone(),
                transform: Transform::from_translation(initial_pos),
                ..default()
            },
            StarNode {
                orbit_speed: speed,
                orbit_axis: axis,
                distance: dist,
                angle,
            },
        ));
    }
}

fn setup_ui(mut commands: Commands) {
    // Spawn full-screen HUD container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // Top Bar: Coordinate readout and Mode
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            }).with_children(|top_bar| {
                top_bar.spawn((
                    TextBundle::from_section(
                        "K-SPACE COORDINATES: [U: 0, V: 0, W: 0] | SEED: 420691337",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::rgb(0.0, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    CoordinateText,
                ));

                top_bar.spawn((
                    TextBundle::from_section(
                        "VISUAL MODE: LATTICE (Press 1/2/3)",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::rgb(0.95, 0.25, 0.72),
                            ..default()
                        },
                    ),
                    ModeStatusText,
                ));
            });

            // Middle: Left (Babel Decrypted Text), Right (Voxel Inspector)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(75.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|middle_row| {
                // Left Column: Babel Page Text Box
                middle_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(60.0),
                            height: Val::Percent(85.0),
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.005, 0.01, 0.03, 0.90)),
                        border_color: BorderColor(Color::rgb(0.95, 0.25, 0.72)),
                        ..default()
                    })
                    .with_children(|box_parent| {
                        box_parent.spawn((
                            TextBundle::from_section(
                                "Loading Decoded Babel Page...",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::rgb(0.9, 0.95, 1.0),
                                    ..default()
                                },
                            ),
                            BabelPageText,
                        ));
                    });

                // Right Column: Voxel Inspector Box
                middle_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(35.0),
                            height: Val::Percent(85.0),
                            padding: UiRect::all(Val::Px(20.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.005, 0.01, 0.03, 0.90)),
                        border_color: BorderColor(Color::rgb(0.0, 0.9, 1.0)),
                        ..default()
                    })
                    .with_children(|inspector_parent| {
                        inspector_parent.spawn((
                            TextBundle::from_section(
                                "VOXEL INSPECTOR\n\nPoint reticle at a node to inspect...\n\n- Position: N/A\n- Amplitude: N/A\n- Phase: N/A\n- Character: N/A\n- Resonance: N/A",
                                TextStyle {
                                    font_size: 14.0,
                                    color: Color::rgb(0.75, 0.85, 0.95),
                                    ..default()
                                },
                            ),
                            VoxelInspectorText,
                        ));
                    });
            });

            // Bottom Bar: Help mappings & Bookmarks
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            }).with_children(|bottom_bar| {
                bottom_bar.spawn(TextBundle::from_section(
                    "FLY BINDINGS: [W/S/A/D] Move | [Space] Up | [C] Down | [Mouse] Look\nK-SPACE Traversal: [Arrow Keys] Shift U/V | [PageUp/PageDown] Shift W",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::rgb(0.5, 0.6, 0.7),
                        ..default()
                    },
                ));

                bottom_bar.spawn((
                    TextBundle::from_section(
                        "PRESET [Tab]: Genesis Hub\n[R] Randomize | [Backspace] Zen Mode | [Enter] Reset",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::rgb(0.0, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    PresetText,
                ));
            });
        });

    // Spawn central crosshair reticle
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Px(8.0),
            height: Val::Px(8.0),
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect {
                left: Val::Px(-4.0),
                top: Val::Px(-4.0),
                ..default()
            },
            border: UiRect::all(Val::Px(1.5)),
            ..default()
        },
        border_color: BorderColor(Color::rgb(0.0, 0.9, 1.0)),
        ..default()
    });
}

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
            
            player.pitch = player.pitch.clamp(-1.54, 1.54);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, player.yaw)
                * Quat::from_axis_angle(Vec3::X, player.pitch);
        }
    }
}

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

fn handle_presets_and_modes(
    keyboard: Res<Input<KeyCode>>,
    mut config: ResMut<KSpaceConfig>,
    mut presets: ResMut<Presets>,
    mut mode: ResMut<VisualMode>,
    mut mode_text_query: Query<&mut Text, (With<ModeStatusText>, Without<PresetText>)>,
    mut preset_text_query: Query<&mut Text, (With<PresetText>, Without<ModeStatusText>)>,
) {
    // Switch modes via 1, 2, 3 keys
    let mut mode_changed = false;
    if keyboard.just_pressed(KeyCode::Key1) {
        *mode = VisualMode::Lattice;
        mode_changed = true;
    }
    if keyboard.just_pressed(KeyCode::Key2) {
        *mode = VisualMode::CymaticSheet;
        mode_changed = true;
    }
    if keyboard.just_pressed(KeyCode::Key3) {
        *mode = VisualMode::SphericalShell;
        mode_changed = true;
    }

    if mode_changed {
        config.needs_update = true;
        for mut text in mode_text_query.iter_mut() {
            let label = match *mode {
                VisualMode::Lattice => "LATTICE",
                VisualMode::CymaticSheet => "CYMATIC SHEET",
                VisualMode::SphericalShell => "SPHERICAL SHELL",
            };
            text.sections[0].value = format!("VISUAL MODE: {} (Press 1/2/3)", label);
        }
    }

    // Cycle presets via Tab
    if keyboard.just_pressed(KeyCode::Tab) {
        presets.index = (presets.index + 1) % presets.list.len();
        let preset = presets.list[presets.index];
        config.center = preset.0;
        config.needs_update = true;

        for mut text in preset_text_query.iter_mut() {
            text.sections[0].value = format!(
                "PRESET [Tab]: {}\n[R] Randomize | [Backspace] Zen Mode | [Enter] Reset",
                preset.1
            );
        }
    }

    // Randomize coordinates via R
    if keyboard.just_pressed(KeyCode::R) {
        let mut rng = rand::thread_rng();
        config.center = (
            rng.gen_range(-500..500),
            rng.gen_range(-500..500),
            rng.gen_range(-500..500),
        );
        config.needs_update = true;
    }

    // Reset to [0, 0, 0] via Enter
    if keyboard.just_pressed(KeyCode::Return) {
        config.center = (0, 0, 0);
        config.needs_update = true;
    }
}

fn rebuild_grid(
    mut commands: Commands,
    mut config: ResMut<KSpaceConfig>,
    mode: Res<VisualMode>,
    query: Query<Entity, With<VoxelNode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut coordinate_text_query: Query<&mut Text, (With<CoordinateText>, Without<BabelPageText>)>,
    mut babel_text_query: Query<&mut Text, (With<BabelPageText>, Without<CoordinateText>)>,
) {
    if !config.needs_update {
        return;
    }

    // 1. Remove old voxels
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // 2. Perform 3D IFFT via direct core integration
    let generator = KSpaceGenerator::new(config.seed);
    let transformer = SignalTransform::new(config.n);
    let translator = AmplitudeTranslator::new();

    let freq_grid = generator.generate_grid(config.center, config.n);
    if let Ok(spatial_grid) = transformer.inverse_transform(freq_grid) {
        let voxel_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.85 }));
        let offset = (config.n as f32 - 1.0) * 0.5;

        // 3. Spawning rebuilt voxels
        for x in 0..config.n {
            for y in 0..config.n {
                for z in 0..config.n {
                    let val = spatial_grid[[x, y, z]];
                    let amp = val.norm() as f32;
                    let phase = val.arg() as f32;
                    let character = translator.complex_to_char(val);

                    if amp > 0.08 {
                        // Color palette matching the visual mode
                        let color = match *mode {
                            VisualMode::Lattice => Color::rgb(
                                amp * 1.5,
                                (1.0 - amp) * 0.4,
                                amp * 0.8 + 0.3,
                            ),
                            VisualMode::CymaticSheet => Color::rgb(
                                (phase + std::f32::consts::PI) / 6.28,
                                amp,
                                0.8,
                            ),
                            VisualMode::SphericalShell => Color::rgb(
                                0.9,
                                0.2 + amp * 0.6,
                                0.4 + (phase.cos() + 1.0) * 0.3,
                            ),
                        };

                        // Position mapping matching the visual mode
                        let base_pos = match *mode {
                            VisualMode::Lattice => Vec3::new(
                                x as f32 - offset,
                                y as f32 - offset,
                                z as f32 - offset,
                            ),
                            VisualMode::CymaticSheet => {
                                let idx = x * config.n * config.n + y * config.n + z;
                                let grid_x = idx % 32;
                                let grid_z = idx / 32;
                                Vec3::new(
                                    grid_x as f32 - 15.5,
                                    0.0,
                                    grid_z as f32 - 7.5,
                                )
                            }
                            VisualMode::SphericalShell => {
                                let idx = x * config.n * config.n + y * config.n + z;
                                let i = idx as f32 + 0.5;
                                let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
                                let theta = 2.0 * std::f32::consts::PI * i / phi;
                                let s_phi = (1.0 - 2.0 * i / 512.0).acos();
                                let radius = 6.0 + amp * 2.0;
                                Vec3::new(
                                    radius * s_phi.sin() * theta.cos(),
                                    radius * s_phi.sin() * theta.sin(),
                                    radius * s_phi.cos(),
                                )
                            }
                        };

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
                                transform: Transform::from_translation(base_pos),
                                ..default()
                            },
                            VoxelNode {
                                amplitude: amp,
                                phase,
                                character,
                                grid_pos: (x, y, z),
                                base_pos,
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

        let decoded_text = translator.grid_to_string(&spatial_grid);
        for mut text_node in babel_text_query.iter_mut() {
            let mut formatted_page = String::new();
            let chars: Vec<char> = decoded_text.chars().collect();
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

fn update_cymatic_resonance(
    time: Res<Time>,
    mode: Res<VisualMode>,
    mut query: Query<(&mut Transform, &VoxelNode)>,
) {
    let current_time = time.elapsed_seconds();

    for (mut transform, voxel) in query.iter_mut() {
        let amp = voxel.amplitude;
        let (x, y, z) = voxel.grid_pos;
        let phase = (x + y + z) as f32 * 0.25;

        match *mode {
            VisualMode::Lattice => {
                let scale_mod = 1.0 + (current_time * 2.0 + phase).sin() * amp * 0.15;
                transform.scale = Vec3::splat(scale_mod);
                let vibration = (current_time * 4.0 + phase).cos() * amp * 0.05;
                transform.translation.y = voxel.base_pos.y + vibration;
            }
            VisualMode::CymaticSheet => {
                let scale_mod = 0.85 + amp * 5.0 * (current_time * 5.0 + phase).sin().abs();
                transform.scale = Vec3::new(0.85, scale_mod, 0.85);
                transform.translation.y = amp * (current_time * 6.0 + phase).cos() * 2.5;
            }
            VisualMode::SphericalShell => {
                let scale_mod = 0.7 * (1.0 + (current_time * 3.0 + phase).sin() * amp * 0.15);
                transform.scale = Vec3::splat(scale_mod);
                let radius = 6.0 + amp * 2.0 + (current_time * 4.0 + phase).sin() * 0.8 * amp;
                let dir = voxel.base_pos.normalize_or_zero();
                transform.translation = dir * radius;
            }
        }
    }
}

fn update_stars(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut StarNode)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut star) in query.iter_mut() {
        star.angle += star.orbit_speed * dt;
        transform.translation = Quat::from_axis_angle(star.orbit_axis, star.angle) * (Vec3::X * star.distance);
    }
}

fn ray_aabb_intersection(ray_origin: Vec3, ray_dir: Vec3, box_min: Vec3, box_max: Vec3) -> Option<f32> {
    let mut tmin = f32::NEG_INFINITY;
    let mut tmax = f32::INFINITY;

    for i in 0..3 {
        let inv_d = 1.0 / ray_dir[i];
        let mut t0 = (box_min[i] - ray_origin[i]) * inv_d;
        let mut t1 = (box_max[i] - ray_origin[i]) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = tmin.max(t0);
        tmax = tmax.min(t1);
        if tmax < tmin {
            return None;
        }
    }
    if tmax < 0.0 {
        return None;
    }
    Some(tmin)
}

fn inspect_voxels(
    camera_query: Query<&Transform, With<Player>>,
    mut voxel_query: Query<(Entity, &Transform, &VoxelNode, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut inspector_text_query: Query<&mut Text, With<VoxelInspectorText>>,
    config: Res<KSpaceConfig>,
    mode: Res<VisualMode>,
) {
    let Ok(cam_transform) = camera_query.get_single() else { return; };
    let ray_origin = cam_transform.translation;
    let ray_dir = cam_transform.forward();

    let mut closest_entity = None;
    let mut closest_dist = f32::INFINITY;
    let mut closest_node_data = None;

    // Raycast check
    for (entity, transform, voxel, _) in voxel_query.iter() {
        let size = 0.85;
        let box_min = transform.translation - Vec3::splat(size / 2.0);
        let box_max = transform.translation + Vec3::splat(size / 2.0);

        if let Some(t) = ray_aabb_intersection(ray_origin, *ray_dir, box_min, box_max) {
            if t < closest_dist {
                closest_dist = t;
                closest_entity = Some(entity);
                closest_node_data = Some(voxel);
            }
        }
    }

    // Reset all standard material emissives back to baseline
    for (_, _, voxel, handle) in voxel_query.iter_mut() {
        if let Some(mat) = materials.get_mut(handle) {
            let color = match *mode {
                VisualMode::Lattice => Color::rgb(
                    voxel.amplitude * 1.5,
                    (1.0 - voxel.amplitude) * 0.4,
                    voxel.amplitude * 0.8 + 0.3,
                ),
                VisualMode::CymaticSheet => Color::rgb(
                    (voxel.phase + std::f32::consts::PI) / 6.28,
                    voxel.amplitude,
                    0.8,
                ),
                VisualMode::SphericalShell => Color::rgb(
                    0.9,
                    0.2 + voxel.amplitude * 0.6,
                    0.4 + (voxel.phase.cos() + 1.0) * 0.3,
                ),
            };
            mat.emissive = color * voxel.amplitude * 0.4;
        }
    }

    let mut inspector_value = String::from("VOXEL INSPECTOR\n\nPoint reticle at a node to inspect...\n\n- Position: N/A\n- Amplitude: N/A\n- Phase: N/A\n- Character: N/A\n- Resonance: N/A");

    if let Some(voxel) = closest_node_data {
        let (x, y, z) = voxel.grid_pos;
        let absolute_pos = (
            config.center.0 + x as i64,
            config.center.1 + y as i64,
            config.center.2 + z as i64,
        );
        let resonance_freq = 432.0 + (voxel.phase * 50.0);

        inspector_value = format!(
            "VOXEL INSPECTOR\n\n\
             - Coordinate: [X: {}, Y: {}, Z: {}]\n\
             - K-Space: [U: {}, V: {}, W: {}]\n\
             - Amplitude: {:.4}\n\
             - Phase Arg: {:.4} rad\n\
             - Babel Character: '{}'\n\
             - Resonance: {:.1} Hz",
            x, y, z,
            absolute_pos.0, absolute_pos.1, absolute_pos.2,
            voxel.amplitude,
            voxel.phase,
            voxel.character,
            resonance_freq
        );

        // Apply visual highlight to closest entity
        if let Some(entity) = closest_entity {
            if let Ok((_, _, _, handle)) = voxel_query.get(entity) {
                if let Some(mat) = materials.get_mut(handle) {
                    mat.emissive = Color::rgb(3.0, 2.5, 0.0);
                }
            }
        }
    }

    for mut text in inspector_text_query.iter_mut() {
        text.sections[0].value = inspector_value.clone();
    }
}

fn spawn_voxel_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxel_query: Query<(&Transform, &VoxelNode)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    let particle_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.15 }));

    for (transform, voxel) in voxel_query.iter() {
        if voxel.amplitude > 0.65 && rng.gen_bool(0.015) {
            let offset_pos = transform.translation + Vec3::new(
                rng.gen_range(-0.2..0.2),
                rng.gen_range(0.1..0.3),
                rng.gen_range(-0.2..0.2),
            );

            let color = Color::rgb(0.0, 0.9, 1.0);

            commands.spawn((
                PbrBundle {
                    mesh: particle_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive: color * 2.5,
                        metallic: 0.1,
                        roughness: 0.1,
                        ..default()
                    }),
                    transform: Transform::from_translation(offset_pos),
                    ..default()
                },
                FrequencyParticle {
                    velocity: Vec3::new(
                        rng.gen_range(-0.4..0.4),
                        rng.gen_range(0.8..1.8),
                        rng.gen_range(-0.4..0.4),
                    ),
                    lifetime: 0.0,
                    max_lifetime: rng.gen_range(0.8..2.2),
                },
            ));
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut FrequencyParticle)>,
) {
    let dt = time.delta_seconds();

    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime += dt;
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
        } else {
            transform.translation += particle.velocity * dt;
            let ratio = particle.lifetime / particle.max_lifetime;
            transform.scale = Vec3::splat(1.0 - ratio);
        }
    }
}

fn draw_connections(
    mut gizmos: Gizmos,
    mode: Res<VisualMode>,
    voxel_query: Query<(&Transform, &VoxelNode)>,
) {
    if *mode != VisualMode::SphericalShell {
        return;
    }

    let mut active_nodes = Vec::new();
    for (transform, voxel) in voxel_query.iter() {
        if voxel.amplitude > 0.55 {
            active_nodes.push((transform.translation, voxel.phase, voxel.amplitude));
        }
    }

    let mut lines_drawn = 0;
    for i in 0..active_nodes.len() {
        for j in (i + 1)..active_nodes.len() {
            if lines_drawn > 80 {
                break;
            }
            let (pos1, phase1, amp1) = active_nodes[i];
            let (pos2, phase2, _) = active_nodes[j];

            let dist = pos1.distance(pos2);
            if dist < 9.0 {
                let phase_diff = (phase1 - phase2).abs();
                if phase_diff < 0.18 || (std::f32::consts::TAU - phase_diff).abs() < 0.18 {
                    let color = Color::rgba(
                        0.0,
                        0.7 + amp1 * 0.3,
                        1.0,
                        (1.0 - dist / 9.0) * 0.45,
                    );
                    gizmos.line(pos1, pos2, color);
                    lines_drawn += 1;
                }
            }
        }
    }
}

fn toggle_zen_mode(
    keyboard: Res<Input<KeyCode>>,
    mut zen: ResMut<ZenMode>,
    mut hud_query: Query<&mut Visibility, With<HudRoot>>,
) {
    if keyboard.just_pressed(KeyCode::BackSpace) {
        zen.0 = !zen.0;
        for mut visibility in hud_query.iter_mut() {
            *visibility = if zen.0 { Visibility::Hidden } else { Visibility::Visible };
        }
    }
}

fn pulsate_central_glow(
    time: Res<Time>,
    voxel_query: Query<&VoxelNode>,
    mut light_query: Query<&mut PointLight, With<CentralLight>>,
    mut ambient: ResMut<AmbientLight>,
) {
    let current_time = time.elapsed_seconds();

    let mut total_amp = 0.0;
    let mut count = 0;
    for voxel in voxel_query.iter() {
        total_amp += voxel.amplitude;
        count += 1;
    }

    let avg_amp = if count > 0 { total_amp / count as f32 } else { 0.0 };
    let pulse = (current_time * 3.0).sin() * 0.15;

    for mut light in light_query.iter_mut() {
        light.intensity = 3000.0 + (avg_amp + pulse) * 12000.0;
        light.color = Color::rgb(0.0, 0.8 + avg_amp * 0.2, 0.9 + pulse * 0.1);
    }

    ambient.brightness = 1.2 + (avg_amp + pulse) * 0.6;
}
