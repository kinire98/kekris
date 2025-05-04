use tauri::{AppHandle, Emitter};
use tokio::{net::UdpSocket, sync::broadcast};

use crate::{
    globals::{LISTENING_DIRECTION_BROADCAST, LISTENING_RESPONSE_PORT_BROADCAST, SIZE_FOR_KB},
    models::{
        room_commands::client::ClientRoomNetCommands, room_commands::server::ServerRoomNetCommands,
        room_info::RoomInfo,
    },
};

const ERROR_OPENING_CONNECTION_EMIT: &str = "error_opening_connection";
pub fn listen_to_request(
    info: RoomInfo,
    app: AppHandle,
    mut stop_channel: broadcast::Receiver<bool>,
) {
    tokio::spawn(async move {
        let Ok(socket) = UdpSocket::bind(LISTENING_DIRECTION_BROADCAST).await else {
            finish_listening(&app);
            return;
        };
        socket
            .set_broadcast(true)
            .expect("Check for not broadcast allowance");
        let mut data = vec![0; SIZE_FOR_KB];
        loop {
            let Ok((valid_bytes, mut addr)) = socket.recv_from(&mut data).await else {
                finish_listening(&app);
                return;
            };
            let deserialized: Result<ClientRoomNetCommands, serde_json::Error> =
                serde_json::from_slice(&data[..valid_bytes]);
            let Ok(net_command) = deserialized else {
                return;
            };
            addr.set_port(LISTENING_RESPONSE_PORT_BROADCAST);
            if let ClientRoomNetCommands::RoomDiscover = net_command {
                let _ = socket
                    .send_to(
                        &serde_json::to_vec(&ServerRoomNetCommands::RoomDiscoverResponse(
                            info.clone(),
                        ))
                        .expect("It's reasonable to expect that this shouldn't panic"),
                        addr,
                    )
                    .await;
            }
            if stop_channel.try_recv().is_ok() {
                break;
            }
        }
    });
}
fn finish_listening(app: &AppHandle) {
    let _ = app.emit(ERROR_OPENING_CONNECTION_EMIT, false);
}
