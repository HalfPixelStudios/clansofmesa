use bevy::prelude::*;
use bevy_ggrs::*;
use clansofmesa::{
    app_state::*, assetloader::*, camera::*, game::*, input::*, map::*, networking::*, structure::*,
};

pub enum Mode {
    Building,
    Deploying,
    Camera,
}

fn main() {
    let mut app = App::new();

    // networked systems
    GGRSPlugin::<GGRSConfig>::new()
        .with_input_system(input_system)
        .with_input_system(input_system)
        .with_rollback_schedule(Schedule::default().with_stage(
            "ROLLBACK_STAGE",
            SystemStage::parallel().with_system(place_structure), // .with_system_set(SystemSet::on_update(AppState::InGame).with_system(player_move_system))
        ))
        .register_rollback_type::<Transform>()
        .build(&mut app);

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(MapPlugin)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(StructurePlugin)
        .add_plugin(GamePlugin);
    //.add_startup_system(spawn_player);

    app.run();
}
