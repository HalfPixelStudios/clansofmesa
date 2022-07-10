use bevy::prelude::*;
use bevy_ggrs::*;
use clansofmesa::{app_state::*, assetloader::*, camera::*, input::*, networking::*, player::*};

fn main() {
    let mut app = App::new();

    // networked systems
    GGRSPlugin::<GGRSConfig>::new()
        .with_input_system(input_system)
        .with_rollback_schedule(Schedule::default().with_stage(
            "ROLLBACK_STAGE",
            SystemStage::single_threaded().with_system(player_move_system), // .with_system_set(State::<AppState>::get_driver())
                                                                            // .with_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
                                                                            // .with_system_set(SystemSet::on_update(AppState::InGame).with_system(player_move_system))
        ))
        .build(&mut app);

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_startup_system(spawn_player);

    app.run();
}
