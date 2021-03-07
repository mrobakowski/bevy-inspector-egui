use bevy::prelude::*;
use bevy_inspector_egui::{widgets::ResourceInspector, Inspectable, InspectorPlugin};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

#[derive(Inspectable, Default)]
struct Resources {
    ambient_light: ResourceInspector<bevy::pbr::AmbientLight>,
    clear_color: ResourceInspector<ClearColor>,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<Resources>::new())
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(Name::new("Camera"))
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb_u8(80, 233, 54).into()),
            ..Default::default()
        })
        .with(Name::new("Floor"))
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(Name::new("Cube"))
        .with_children(|commands| {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    transform: Transform::from_xyz(0.0, 0.8, 0.0),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .with(Name::new("Child"))
                .with_children(|commands| {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                            transform: Transform::from_xyz(0.0, 0.4, 0.0),
                            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                            ..Default::default()
                        })
                        .with(Name::new("Child"));
                });
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 20,
                radius: 0.5,
            })),
            transform: Transform::from_xyz(1.5, 1.5, 1.5),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .with(Name::new("Sphere"))
        .spawn(LightBundle {
            transform: Transform::from_xyz(10.3, 8.0, -2.3),
            light: Light {
                range: 20.0,
                intensity: 1237.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Name::new("Light"))
        .spawn(LightBundle {
            transform: Transform::from_xyz(-6.2, 8.0, 4.3),
            light: Light {
                range: 20.0,
                intensity: 245.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Name::new("Second Light"));
}
