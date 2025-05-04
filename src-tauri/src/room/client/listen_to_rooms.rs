use std::{net::SocketAddr, time::Duration};
use tokio::net::UdpSocket;

use tauri::{AppHandle, Emitter};

use crate::{
    globals::{DUMMY_SEND_BROADCAST, LISTEN_BROADCAST_RESPONSE, SENDING_BROADCAST, SIZE_FOR_KB},
    models::{
        room_commands::client::ClientRoomNetCommands, room_commands::server::ServerRoomNetCommands,
        room_info::RoomInfo,
    },
};

const SECONDS_TO_LISTEN: u64 = 10;
const MILIS_TIMEOUT: u64 = 50;

const ROOM_UPDATES_EVENT: &str = "room-updates";
pub async fn listen_to_rooms(app: AppHandle, mut channel: tokio::sync::mpsc::Receiver<bool>) {
    let send_socket = UdpSocket::bind(DUMMY_SEND_BROADCAST)
        .await
        .expect("Check error");
    let listen_socket = UdpSocket::bind(LISTEN_BROADCAST_RESPONSE).await.unwrap();
    loop {
        if channel.try_recv().is_ok() {
            drop(send_socket);
            drop(listen_socket);
            break;
        }
        send_socket
            .set_broadcast(true)
            .expect("Reasonably expected to not panic");
        let addr: SocketAddr = SENDING_BROADCAST.parse().unwrap();
        send_socket
            .send_to(
                &serde_json::to_vec(&ClientRoomNetCommands::RoomDiscover).unwrap(),
                addr,
            )
            .await
            .unwrap();

        let mut buf = vec![0; SIZE_FOR_KB];

        let mut rooms: Vec<RoomInfo> = vec![];
        if channel.try_recv().is_ok() {
            drop(send_socket);
            drop(listen_socket);
            break;
        }
        let _ = tokio::time::timeout(Duration::from_millis(MILIS_TIMEOUT), async {
            loop {
                let Ok((len, _)) = listen_socket.recv_from(&mut buf).await else {
                    continue;
                };
                let response: Result<ServerRoomNetCommands, serde_json::Error> =
                    serde_json::from_slice(&buf[..len]);
                let Ok(command) = response else {
                    continue;
                };
                if let ServerRoomNetCommands::RoomDiscoverResponse(info) = command {
                    rooms.push(info);
                }
                app.emit(ROOM_UPDATES_EVENT, &rooms).unwrap();
            }
        })
        .await;
        if channel.try_recv().is_ok() {
            drop(send_socket);
            drop(listen_socket);
            break;
        }
        app.emit(ROOM_UPDATES_EVENT, rooms).unwrap();
        tokio::time::sleep(Duration::from_secs(SECONDS_TO_LISTEN)).await;
    }
}
