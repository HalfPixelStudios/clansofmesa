// use autodefault::autodefault;
// use bevy::prelude::*;
// use bevy_ggrs::{Rollback, RollbackIdProvider};
// use ggrs::InputStatus;

// use crate::input::*;

// #[derive(Component, Default)]
// pub struct Player {
//     pub handle: usize,
// }

// #[derive(Component, Default)]
// pub struct Movement {
//     pub speed: f32,
// }

// #[autodefault]
// pub fn spawn_player(mut cmd: Commands, mut rip: ResMut<RollbackIdProvider>) {
//     cmd.spawn_bundle(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(0., 0.47, 1.),
//         },
//         transform: Transform {
//             scale: Vec3::splat(10.),
//         },
//     })
//     .insert(Player { handle: 0 })
//     .insert(Movement { speed: 100. })
//     .insert(Rollback::new(rip.next_id()));

//     cmd.spawn_bundle(SpriteBundle {
//         sprite: Sprite {
//             color: Color::rgb(1., 0.47, 1.),
//         },
//         transform: Transform {
//             translation: Vec3::new(10., 0., 0.),
//             scale: Vec3::splat(10.),
//         },
//     })
//     .insert(Player { handle: 1 })
//     .insert(Movement { speed: 100. })
//     .insert(Rollback::new(rip.next_id()));
// }

// pub fn player_move_system(
//     time: Res<Time>,
//     inputs: Res<Vec<(NetInput, InputStatus)>>,
//     mut query: Query<(&Player, &mut Transform, &Movement)>,
// ) {
//     for (player, mut transform, movement) in query.iter_mut() {
//         let (input, _) = inputs[player.handle as usize];

//         let mut move_dir = Vec2::ZERO;

//         if input.pressed & INPUT_UP != 0 {
//             move_dir += Vec2::Y;
//         } else if input.pressed & INPUT_DOWN != 0 {
//             move_dir -= Vec2::Y;
//         }
//         if input.pressed & INPUT_LEFT != 0 {
//             move_dir -= Vec2::X;
//         } else if input.pressed & INPUT_RIGHT != 0 {
//             move_dir += Vec2::X;
//         }

//         let move_vec = move_dir.normalize_or_zero();
//         transform.translation += move_vec.extend(0.) * movement.speed * time.delta_seconds();
//     }
// }
