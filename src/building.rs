use bevy::prelude::*;
use crate::components;

pub fn add_building(mut commands: Commands) {
    commands
        .spawn()
        .insert(components::Building)
        .insert(components::Name("Kultakaivos".to_string()))
        .insert(components::Price(100.0))
        .insert(components::UpgradeLevel(1))
        .insert(components::Currency {
            name: "Kulta".to_string(),
            value: 10,
        })
        .insert(components::BaseAmount(10))
        .insert(components::AmountMultiplier(1.0))
        .insert(components::Cooldown(2));
}

pub fn get_name(query: Query<&components::Name, With<components::Building>>) {
    for entity in query.iter() {
        println!("Rakennus: {}", entity.0);
    }
}

pub fn get_price(query: Query<&components::Price, With<components::Building>>) {
    for entity in query.iter() {
        println!("Hinta: {}", entity.0);
    }
}

pub fn get_currency(query: Query<&components::Currency, With<components::Building>>) {
    for entity in query.iter() {
        println!("Valuutta: {} {}", entity.name, entity.value);
    }
}

pub fn get_base_amount(query: Query<&components::BaseAmount, With<components::Building>>) {
    for entity in query.iter() {
        println!("Base: {}", entity.0);
    }
}

pub fn get_amount_multiplier(query: Query<&components::AmountMultiplier, With<components::Building>>) {
    for entity in query.iter() {
        println!("Multi: {}", entity.0);
    }
}

pub fn get_upgrade_level(query: Query<&components::UpgradeLevel, With<components::Building>>) {
    for entity in query.iter() {
        println!("UpgLvl: {}", entity.0);
    }
}

pub fn get_cooldown(query: Query<&components::Cooldown, With<components::Building>>) {
    for entity in query.iter() {
        println!("CD: {}", entity.0);
    }
}

