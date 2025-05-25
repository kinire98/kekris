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

const MILIS_TIMEOUT: u64 = 200;

const ROOM_UPDATES_EVENT: &str = "room-updates";
pub async fn listen_to_rooms(app: AppHandle, mut channel: tokio::sync::mpsc::Receiver<bool>) {
    let send_socket = UdpSocket::bind(DUMMY_SEND_BROADCAST)
        .await
        .expect("Check error");
    let listen_socket = UdpSocket::bind(LISTEN_BROADCAST_RESPONSE).await.unwrap();

    send_socket
        .set_broadcast(true)
        .expect("Reasonably expected to not panic");
    loop {
        let mut buf = vec![0; SIZE_FOR_KB];
        let mut rooms: Vec<RoomInfo> = vec![];

        let addr: SocketAddr = SENDING_BROADCAST.parse().unwrap();
        send_socket
            .send_to(
                &serde_json::to_vec(&ClientRoomNetCommands::RoomDiscover(false)).unwrap(),
                addr,
            )
            .await
            .unwrap();

        tokio::select! {
            _ = channel.recv() => {
                break;
            },
            result = listen_socket.recv_from(&mut buf) => {
                let Ok((amount, _)) = result else {
                    continue;
                };
                let response: Result<ServerRoomNetCommands, serde_json::Error> =
                    serde_json::from_slice(&buf[..amount]);
                let Ok(command) = response else {
                    continue;
                };
                if let ServerRoomNetCommands::RoomDiscoverResponse(info) = command {
                    rooms.push(info);
                }
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_millis(MILIS_TIMEOUT))  => {
                            break;
                        },
                        result = listen_socket.recv_from(&mut buf) => {
                            let Ok((amount, _)) = result else {
                                continue;
                            };
                            let response: Result<ServerRoomNetCommands, serde_json::Error> =
                                serde_json::from_slice(&buf[..amount]);
                            let Ok(command) = response else {
                                continue;
                            };
                            if let ServerRoomNetCommands::RoomDiscoverResponse(info) = command {
                                rooms.push(info);
                            }
                        },
                    }
                }
            }
        }
        app.emit(ROOM_UPDATES_EVENT, rooms).unwrap();
    }
}
