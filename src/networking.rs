use bevy::{prelude::*, tasks::IoTaskPool};
use matchbox_socket::WebRtcNonBlockingSocket;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_server);
    }
}

fn start_server(mut cmd: Commands, task_pool: Res<IoTaskPool>) {
    let room_url = "ws://127.0.0.1:3536/next_2";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcNonBlockingSocket::new(room_url);

    task_pool.spawn(message_loop).detach();

    cmd.insert_resource(Some(socket));
}
