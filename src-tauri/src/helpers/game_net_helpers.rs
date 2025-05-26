use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use crate::models::online_game_commands::{
    client::ClientOnlineGameCommands, server::ServerOnlineGameCommands,
};

/// Sends an enum from the client to the server over a TCP stream.
///
/// Serializes the enum to JSON, prepends the length of the JSON as a u32 in big-endian byte order,
/// and then writes the length and the JSON to the stream.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
/// * `value` - A reference to the `ClientOnlineGameCommands` enum to send.
pub async fn send_enum_from_client(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ClientOnlineGameCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.write_all(&bytes).await?;
    Ok(())
}
/// Reads an enum from the client over a TCP stream.
///
/// Reads the length of the JSON as a u32 from the stream in big-endian byte order,
/// then reads that many bytes from the stream and deserializes them as a `ClientOnlineGameCommands` enum.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
pub async fn read_enum_from_client(
    stream: &Arc<Mutex<TcpStream>>,
) -> Result<ClientOnlineGameCommands, Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = stream.lock().await;
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;

    Ok(serde_json::from_slice(&buffer)?)
}

/// Sends an enum from the server to the client over a TCP stream.
///
/// Serializes the enum to JSON, prepends the length of the JSON as a u32 in big-endian byte order,
/// and then writes the length and the JSON to the stream.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
/// * `value` - A reference to the `ServerOnlineGameCommands` enum to send.
pub async fn send_enum_from_server(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ServerOnlineGameCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.write_all(&bytes).await?;
    Ok(())
}
/// Reads an enum from the server over a TCP stream.
///
/// Reads the length of the JSON as a u32 from the stream in big-endian byte order,
/// then reads that many bytes from the stream and deserializes them as a `ServerOnlineGameCommands` enum.
///
/// # Arguments
///
/// * `stream` - An `Arc<Mutex<TcpStream>>` representing the TCP stream.
pub async fn read_enum_from_server(
    stream: &Arc<Mutex<TcpStream>>,
) -> Result<ServerOnlineGameCommands, Box<dyn std::error::Error + Send + Sync>> {
    let mut stream = stream.lock().await;

    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;

    Ok(serde_json::from_slice(&buffer)?)
}
