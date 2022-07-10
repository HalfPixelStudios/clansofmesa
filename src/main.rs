use bevy::prelude::*;
use bevy_ggrs::*;
use clansofmesa::{assetloader::*, camera::*, networking::*, player::*};

fn main() {
    let mut app = App::new();

    // networked systems
    GGRSPlugin::<GGRSConfig>::new()
        // .with_rollback_schedule(
        //     Schedule::default().with_stage("ROLLBACK_STAGE",SystemStage::single_threaded())
        // )
        .build(&mut app);

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        });

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin);

    app.run();
}
