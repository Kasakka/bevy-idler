use bevy::prelude::*;

//Generator building
#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Price(pub f32);

#[derive(Component)]
pub struct UpgradeLevel(pub i32);

//Currency generated
#[derive(Component)]
pub struct Currency {
    pub name: String,
    pub value: i32,
}

//Base amount of resources generated
#[derive(Component)]
pub struct BaseAmount(pub i32);

//Ex. Multiplier to increase the amount of resources generated
//when building is upgraded
#[derive(Component)]
pub struct AmountMultiplier(pub f32);

//Cooldown between cycles in seconds
#[derive(Component)]
pub struct Cooldown(pub i32);

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
