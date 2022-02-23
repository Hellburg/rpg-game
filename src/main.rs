use bevy::prelude::*;
use std::ops::Range;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(movement_system)
        .run();
}

#[derive(Component)]
struct Cube;

fn movement_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&Cube, &mut Transform)>) {
    let (_cube, mut transform) = query.single_mut();
    let mut translation_vec = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::R) {
        transform.translation = Vec3::new(0.0, 0.5, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        translation_vec -= Vec3::Z
    }
    if keyboard_input.pressed(KeyCode::Down) {
        translation_vec += Vec3::Z
    }
    if keyboard_input.pressed(KeyCode::Left) {
        translation_vec -= Vec3::X
    }
    if keyboard_input.pressed(KeyCode::Right) {
        translation_vec += Vec3::X
    }

    translate_with_bounds(&mut transform, translation_vec*0.1, Bound {
        x_range: Range { start: -10.0, end: 10.0 },
        y_range: Range { start: -10.0, end: 10.0 },
        z_range: Range { start: -10.0, end: 10.0 }
    });
}

struct Bound {
    x_range: Range<f32>,
    y_range: Range<f32>,
    z_range: Range<f32>
}

fn translate_with_bounds(transform: &mut Transform, translation_vec: Vec3, bounds: Bound) {
    let translation = transform.translation + translation_vec;
    transform.translation = translation;
    transform.translation.x = transform.translation.x.min(bounds.x_range.end).max(bounds.x_range.start);
    transform.translation.y = transform.translation.y.min(bounds.y_range.end).max(bounds.y_range.start);
    transform.translation.z = transform.translation.z.min(bounds.z_range.end).max(bounds.z_range.start);
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    let cube_start_transform = Transform::from_xyz(0.0, 0.5, 0.0);

    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(cube_start_transform.translation, Vec3::Y);

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: cube_start_transform,
        ..Default::default()
    }).insert(Cube).with_children(|parent| {
        parent.spawn_bundle(camera);
    });    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });


    // camera
  //  commands.spawn_bundle(PerspectiveCameraBundle {
    //    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    ..Default::default()
   //  });
}