use components::*;
use bevy::prelude::*;

pub fn add_building(mut commands: Commands) {
    commands
        .spawn()
        .insert(Building)
        .insert(Name("Kultakaivos".to_string()))
        .insert(Price(100.0))
        .insert(UpgradeLevel(1))
        .insert(Currency {
            name: "Kulta".to_string(),
            value: 10,
        })
        .insert(BaseAmount(10))
        .insert(AmountMultiplier(1.0))
        .insert(Cooldown(2));
}

pub fn print_building(query: Query<
    (
        &components::Name,
        &Price,
        &Currency,
        &UpgradeLevel,
        &BaseAmount,
        &AmountMultiplier,
        &Cooldown,
    ),
    With<Building>,
>,) {
    for (name, price, currency, upgrade_level, base_amount, amount_multiplier, cooldown) in query.iter() {
        println!("Rakennus: {}", name.0);
        println!("Hinta: {}", price.0);
        println!("Taso: {}", upgrade_level.0);
        println!("Valuutan nimi: {}, Arvo: {}", currency.name, currency.value);
        println!("BaseAmount: {}", base_amount.0);
        println!("AmountMultiplier: {}x", amount_multiplier.0);
        println!("Cooldown: {} seconds", cooldown.0);
    }
}