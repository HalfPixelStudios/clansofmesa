use autodefault::autodefault;
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapPlugin;
pub const MAPWIDTH: f32 = 256.;
pub const MAPHEIGHT: f32 = 256.;

pub const TILEWIDTH: f32 = 16.;
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
        transform: Transform {
            translation: Vec3::new(-MAPWIDTH / 2., -MAPHEIGHT / 2., 0.),
            ..default()
        },
        ldtk_handle: asset_server.load("map/plains.ldtk"),
    });
}
// pub fn get_squares(mut query:Query<(&mut IntGridCell,&GridCoords)>){
//     for (mut cell,coords) in query.iter_mut(){

//         println!("{:?}",coords);

//     }
// }
pub fn to_grid_coords(x: i32, y: i32) -> (i32, i32) {
    return (x / TILEWIDTH as i32 + 7, y / TILEWIDTH as i32 + 7);
}
pub fn snap_to_grid(pos: Vec2) -> (i32, i32) {
    let mut snapped = ((pos / TILEWIDTH).ceil() * TILEWIDTH).round();
    (snapped.x as i32, snapped.y as i32)
}
