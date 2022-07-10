use super::assetloader::*;
use super::camera::*;
use super::input::*;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;
#[derive(Component, Clone)]
pub struct Tower;

pub struct SpawnStructureEvent {
    pub spawn_pos: Vec3,
    pub index: usize,
}

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStructureEvent>();
    }
}

pub fn spawn_structure(
    mut cmd: Commands,
    assets: Res<AssetSheet>,
    mut event: EventReader<SpawnStructureEvent>,
    mut rip: ResMut<RollbackIdProvider>,
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
        .insert(Tower)
        .insert(Rollback::new(rip.next_id()));
    }
}

pub fn place_structure(
    inputs: Res<Vec<(NetInput, InputStatus)>>,
    mut spawn_event: EventWriter<SpawnStructureEvent>,
) {
    let (input, _) = inputs[0];
    if input.pressed & PLACE != 0 {
        spawn_event.send(SpawnStructureEvent {
            spawn_pos: Vec3::new(input.grid_x as f32, input.grid_y as f32, 0.),
            index: 0,
        });
    }
}
