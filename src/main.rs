use bevy::prelude::*;

//Generator building
#[derive(Component)]
struct Building;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Price(f32);

#[derive(Component)]
struct UpgradeLevel(i32);

//Currency generated
#[derive(Component)]
struct Currency {
    name: String,
    value: i32,
}

//Base amount of resources generated
#[derive(Component)]
struct BaseAmount(i32);

//Ex. Multiplier to increase the amount of resources generated
//when building is upgraded
#[derive(Component)]
struct AmountMultiplier(f32);

//Cooldown between cycles in seconds
#[derive(Component)]
struct Cooldown(i32);

fn add_building(mut commands: Commands) {
    commands
        .spawn()
        .insert(Building)
        .insert(Name("Kultakaivos".to_string()))
        .insert(Price(100.0))
        .insert(Currency {
            name: "Kulta".to_string(),
            value: 10,
        })
        .insert(BaseAmount(10))
        .insert(AmountMultiplier(1.0))
        .insert(Cooldown(2));
}

fn print_building(
    query: Query<
        (
            &Name,
            &Price,
            &Currency,
            &BaseAmount,
            &AmountMultiplier,
            &Cooldown,
        ),
        With<Building>,
    >,
) {
    for (name, price, currency, base_amount, amount_multiplier, cooldown) in query.iter() {
        println!("Rakennus: {}", name.0);
        println!("Hinta: {}", price.0);
        println!("Valuutan nimi: {}, Arvo: {}", currency.name, currency.value);
        println!("BaseAmount: {}", base_amount.0);
        println!("AmountMultiplier: {}x", amount_multiplier.0);
        println!("Cooldown: {} seconds", cooldown.0);
    }
}

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_startup_system(add_building)
        .add_system(print_building)
        .run();
}
