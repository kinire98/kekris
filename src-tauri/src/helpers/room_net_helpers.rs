use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use crate::models::room_commands::{client::ClientRoomNetCommands, server::ServerRoomNetCommands};

/// Sends a `ClientRoomNetCommands` enum over a TCP stream.
///
/// Serializes the enum to JSON, prepends the length of the JSON as a u32 in big-endian byte order,
/// and then writes the length and the JSON to the stream.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
/// * `value` - A reference to the `ClientRoomNetCommands` enum to send.
pub async fn send_enum_from_client(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ClientRoomNetCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.write_all(&bytes).await?;
    Ok(())
}
/// Reads a `ClientRoomNetCommands` enum from a TCP stream.
///
/// Reads the length of the JSON as a u32 from the stream in big-endian byte order,
/// then reads that many bytes from the stream and deserializes them as a `ClientRoomNetCommands` enum.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
pub async fn read_enum_from_client(
    stream: &Arc<Mutex<TcpStream>>,
) -> Result<ClientRoomNetCommands, Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = stream.lock().await;
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;

    Ok(serde_json::from_slice(&buffer)?)
}

/// Sends a `ServerRoomNetCommands` enum over a TCP stream.
///
/// Serializes the enum to JSON, prepends the length of the JSON as a u32 in big-endian byte order,
/// and then writes the length and the JSON to the stream.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
/// * `value` - A reference to the `ServerRoomNetCommands` enum to send.
pub async fn send_enum_from_server(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ServerRoomNetCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.write_all(&bytes).await?;
    Ok(())
}
/// Reads a `ServerRoomNetCommands` enum from a TCP stream.
///
/// Reads the length of the JSON as a u32 from the stream in big-endian byte order,
/// then reads that many bytes from the stream and deserializes them as a `ServerRoomNetCommands` enum.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
pub async fn read_enum_from_server(
    stream: &Arc<Mutex<TcpStream>>,
) -> Result<ServerRoomNetCommands, Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = stream.lock().await;

    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;

    Ok(serde_json::from_slice(&buffer)?)
}
