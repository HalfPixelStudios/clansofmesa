use super::assetloader::*;
use super::camera::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Tower;

pub struct SpawnTowerEvent {
    pub spawn_pos: Vec3,
    pub index: usize,
}

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>()
            .add_system(spawn_tower)
            .add_system(place_tower);
    }
}

pub fn spawn_tower(
    mut cmd: Commands,
    assets: Res<AssetSheet>,
    mut event: EventReader<SpawnTowerEvent>,
) {
    for ev in event.iter() {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: ev.index,
                ..default()
            },
            transform: Transform {
                translation: ev.spawn_pos,
                ..default()
            },
            texture_atlas: assets.0.clone(),
            ..default()
        })
        .insert(Tower);
    }
}

pub fn place_tower(
    mut cmd: Commands,
    input: Res<Input<KeyCode>>,
    mut spawn_event: EventWriter<SpawnTowerEvent>,
    cursor: Res<Cursor>,
) {
    if input.just_pressed(KeyCode::P) {
        spawn_event.send(SpawnTowerEvent {
            spawn_pos: cursor.0.extend(0.),
            index: 0,
        });
    }
}
