use bevy::prelude::*;
use building::*;

struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_building)
            .add_system(print_building);
    }
}

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugin(BuildingPlugin)
        .run();
}
