use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use crate::models::online_game_commands::{
    client::ClientOnlineGameCommands, server::ServerOnlineGameCommands,
};

pub async fn send_enum_from_client(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ClientOnlineGameCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.flush().await?;
    guard.write_all(&bytes).await?;
    guard.flush().await?;
    Ok(())
}
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

pub async fn send_enum_from_server(
    stream: &Arc<Mutex<TcpStream>>,
    value: &ServerOnlineGameCommands,
) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut guard = stream.lock().await;
    guard.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
    guard.flush().await?;
    guard.write_all(&bytes).await?;
    guard.flush().await?;
    Ok(())
}
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
