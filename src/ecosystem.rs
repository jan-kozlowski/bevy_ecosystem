use bevy::prelude::*;

const TILEMAP_SIZE: usize = 11;
const TILE_SIZE: f32 = 2.;
const CENTER_TILE: (usize, usize) = (TILEMAP_SIZE / 2, TILEMAP_SIZE / 2);

pub struct EcosystemPlugin;
impl Plugin for EcosystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_tilemap, setup_lighting));
    }
}

fn spawn_tilemap(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut bundles = Vec::new();
    for x in 0..TILEMAP_SIZE {
        for z in 0..TILEMAP_SIZE {
            bundles.push((
                Mesh3d(meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 144, 0))),
                Transform::from_translation(tile_to_translation((x, z))),
            ));
        }
    }

    commands.spawn_batch(bundles);
}

fn setup_lighting(mut commands: Commands) {
    let translation = tile_to_translation(CENTER_TILE) + vec3(0., 5., 0.);

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(translation),
    ));
}

fn tile_to_translation(tile: (usize, usize)) -> Vec3 {
    Vec3 {
        x: tile.0 as f32 * TILE_SIZE,
        y: 0.,
        z: tile.1 as f32 * TILE_SIZE,
    }
}
