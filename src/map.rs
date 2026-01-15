use bevy::prelude::*;

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, logger);
    }
}

fn logger() {
    println!("Map plugin running!");
}
