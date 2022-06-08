use std::time::Duration;

use bevy::prelude::*;
use bevy_draw_debug::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_flycam::PlayerPlugin)
        .add_plugin(DrawDebugPlugin)
        .add_startup_system(startup)
        .add_system(update)
        .run();
}

#[derive(Component)]
struct ExampleTag;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert_bundle(DirectionalLightBundle::default());

    let entity = commands
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(Color::CYAN.into()),
            ..Default::default()
        })
        .insert(ExampleTag)
        .id();

    draw_debug!(entity, DrawDebugOptions { duration: Some(Duration::new(20, 0)), color: Some(Color::GREEN) });
}

fn update(mut query: Query<&mut Transform, With<ExampleTag>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += time.delta().as_secs_f32();
    }
}
