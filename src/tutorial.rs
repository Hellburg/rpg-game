use bevy::prelude::*;

#[derive(Component)]
struct Person;

// Name is a Tuple Struct! Name("johan") for example
#[derive(Component)]
struct Name(String);

fn populate_world(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Johan Hillborg".to_string()));
    commands.spawn().insert(Person).insert(Name("Annika Norell".to_string()));
}

struct GreetTimer(Timer);

// You can interpret the Query above as: "iterate over every Name component for entities that also have a Person component",
// Resources are accessed in much the same way that we access components (i.e Name, Person etc). You can access the Time resource in your system.
// To make this easier, Bevy provides the Time and Timer type. Let's create a new Resource for our system to track elapsed time with a Timer.
fn shout_outs(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}

// Entities and Components are great for representing complex,
// query-able groups of data. But most Apps will also require "globally unique" data of some kind.
// In Bevy ECS, we represent globally unique data using Resources.


pub struct PopulatePlugin;

impl Plugin for PopulatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(populate_world)
            .add_system(shout_outs).run();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulatePlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
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
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
