use bevy::prelude::*;
mod building;
mod components;

struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(building::add_building)
            .add_system(building::get_name)
            .add_system(building::get_price)
            .add_system(building::get_upgrade_level)
            .add_system(building::get_base_amount)
            .add_system(building::get_amount_multiplier)
            .add_system(building::get_currency)
            .add_system(building::get_cooldown);
    }
}

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugin(BuildingPlugin)
        .run();
}
