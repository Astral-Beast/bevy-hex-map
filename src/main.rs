use std::f32::consts::PI;

use bevy::{
    ecs::bundle,
    log::tracing_subscriber::fmt::format,
    math::{vec2, vec3},
    prelude::*,
    render::{
        camera::RenderTarget,
        mesh::{Indices, PlaneMeshBuilder},
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, PrimitiveTopology, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        settings::*,
        RenderPlugin,
    },
    ui::node_bundles,
};
fn main() {
    App::new()
        .add_plugins((MyRenderPlugin))
        .add_systems(Startup, setup)
        //.add_systems(Update, rotate)
        .run();
}

pub struct MyRenderPlugin;
/// This plugin is copied from https://github.com/bevyengine/bevy/issues/9975 to fix an issue that was throwing an obscene number of
/// errors while the program was running
impl Plugin for MyRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1920.0, 1080.0).into(),
                        title: "Game".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        );
    }
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Hex {
    pub position: Vec3,
    pub index: Vec3,
    pub color: Color,
}
// HEX Const values
pub const HEX_OUTER_RADIUS: f32 = 2.0;
pub const HEX_INNER_RADIUS: f32 = HEX_OUTER_RADIUS * 0.866025404;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let shape = meshes.add(create_hex_mesh());

    for z in -10..10 {
        for x in -10..10 {
            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };
            //image = uv_debug_texture();
            // fill image.data with zeroes
            image.resize(size);

            let image_handle = images.add(image);
            let texture_camera = commands
                .spawn(Camera2dBundle {
                    camera: Camera {
                        // render before the "main pass" camera
                        order: -1,
                        target: RenderTarget::Image(image_handle.clone()),
                        clear_color: ClearColorConfig::Custom(Color::GOLD),
                        ..default()
                    },
                    ..default()
                })
                .id();

            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(image_handle),
                reflectance: 0.02,
                unlit: false,
                alpha_mode:AlphaMode::Mask(0.5),
                ..default()
            });
            let position = vec3(
                (x as f32 + z as f32 * 0.5 - (z / 2) as f32) * (HEX_INNER_RADIUS * 2.0),
                0.0,
                z as f32 * HEX_OUTER_RADIUS * 1.5,
            );
            let index = vec3(x as f32, (-x - z) as f32, z as f32);
            commands.spawn((
                PbrBundle {
                    mesh: shape.clone(),
                    material: materials.add(StandardMaterial {
                                    base_color_texture: Some(images.add(uv_debug_texture())),
                                    ..default()
                                }),
                    transform: Transform::from_xyz(position.x, position.y, position.z)
                        .with_rotation(Quat::from_rotation_x(0.0)),
                    ..default()
                },
                Hex {
                    position: { position },
                    index: { index },
                    color: Color::SILVER,
                },
            ));
            // commands.spawn((
            //     PbrBundle {
            //         mesh: shape.clone(),
            //         material: materials.add(StandardMaterial {
            //             base_color_texture: Some(images.add(uv_debug_texture())),
            //             ..default()
            //         }),
            //         transform: Transform::from_xyz(position.x, position.y, position.z)
            //             .with_rotation(Quat::from_rotation_x(0.0)),
            //         ..default()
            //     },
            //     Hex {
            //         position: { position },
            //         index: { index },
            //         color: Color::SILVER,
            //     },
            // ));
            commands
            .spawn((
                NodeBundle {
                    style: Style {
                        // Cover the whole image
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }),
                    ..default()
                },
                TargetCamera(texture_camera),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "This is a cube",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
            
            // commands.spawn(
            //     PbrBundle {
            //         mesh: meshes.add(Cuboid::new(HEX_INNER_RADIUS, 0.1, HEX_INNER_RADIUS)),
            //         material:material_handle,
            //         ..default()

            //     }
            // );
            commands.spawn(
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(HEX_INNER_RADIUS, 0.1, HEX_INNER_RADIUS)),
                    material: materials.add(StandardMaterial {
                                    base_color_texture: Some(images.add(uv_debug_texture())),
                                    ..default()
                                }),
                    ..default()

                }
            );
        }
    }
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn create_hex_mesh() -> Mesh {
    let opposite_leg:f32 = ((PI / 6.0).sin() * HEX_INNER_RADIUS) / HEX_OUTER_RADIUS;
    print!("{}, {}, {}",opposite_leg, 0.5 - opposite_leg, 0.5 + opposite_leg);
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        //Mesh vertices
        vec![
            [0.0, 0.0, 0.0],
            [0.0, 0.0, HEX_OUTER_RADIUS],
            [HEX_INNER_RADIUS, 0.0, 0.5 * HEX_OUTER_RADIUS],
            [HEX_INNER_RADIUS, 0.0, -0.5 * HEX_OUTER_RADIUS],
            [0.0, 0.0, -HEX_OUTER_RADIUS],
            [-HEX_INNER_RADIUS, 0.0, -0.5 * HEX_OUTER_RADIUS],
            [-HEX_INNER_RADIUS, 0.0, 0.5 * HEX_OUTER_RADIUS],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.5,0.5], 
            [0.0, 0.5], 
            [0.5-opposite_leg , 1.0], 
            [0.5+opposite_leg,1.0],
            [1.0,0.5],
            [0.5+opposite_leg,0.0], 
            [0.5-opposite_leg,0.0],
                ]
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1,
    ])) // triangles making up the top (+y) facing side.
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
