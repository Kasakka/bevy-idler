use bevy::prelude::*;

#[derive(Component)]
struct Building;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Price(f32);

#[derive(Component)]
struct UpgradeLevel(i32);

#[derive(Component)]
struct Factor(f32);

fn add_building(mut commands: Commands) {
    commands.spawn()
        .insert(Building)
        .insert(Name("Kultakaivos".to_string()));
}

fn print_building(query: Query<&Name, With<Building>>) {
    for name in query.iter() {
        println!("Rakennus: {}", name.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_building)
        .add_system(print_building)
        .run();
}
