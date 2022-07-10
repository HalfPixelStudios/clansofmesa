use autodefault::autodefault;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_startup_system(setup);
    }
}

#[autodefault]
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/plains.ldtk"),
    });
}
