use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader, path::PathBuf};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use tauri::{AppHandle, Manager};
use tokio::task;

const BASE_DIRECTORY: &str = "sound-fx";
pub struct SoundPlayer {
    stream: OutputStream,
    handle: OutputStreamHandle,
    paths: HashMap<String, PathBuf>,
}
impl Debug for SoundPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundPlayer").finish()
    }
}

impl SoundPlayer {
    pub fn new(handle: &AppHandle) -> Self {
        let mut paths = HashMap::new();
        paths.insert(
            "loss.mp3".to_string(),
            handle
                .path()
                .resolve("loss.mp3", tauri::path::BaseDirectory::Resource)
                .unwrap(),
        );
        paths.insert(
            "move-right-left.mp3".to_string(),
            handle
                .path()
                .resolve("move-right-left.mp3", tauri::path::BaseDirectory::Resource)
                .unwrap(),
        );
        paths.insert(
            "soft-drop.mp3".to_string(),
            handle
                .path()
                .resolve("soft-drop.mp3", tauri::path::BaseDirectory::Resource)
                .unwrap(),
        );
        paths.insert(
            "t-spin-tetris.mp3".to_string(),
            handle
                .path()
                .resolve("t-spin-tetris.mp3", tauri::path::BaseDirectory::Resource)
                .unwrap(),
        );
        paths.insert(
            "piece-drop.mp3".to_string(),
            handle
                .path()
                .resolve("piece-drop.mp3", tauri::path::BaseDirectory::Resource)
                .unwrap(),
        );
        let (stream, handle) = OutputStream::try_default().unwrap();
        SoundPlayer {
            stream,
            handle,
            paths,
        }
    }
    async fn play_sound(&self, sound: &str) {
        let handle = self.handle.clone();
        let sound_path = self.paths.get(&sound.to_string()).unwrap().clone();
        task::spawn_blocking(move || {
            let sink = Sink::try_new(&handle).unwrap();
            let file = BufReader::new(File::open(sound_path).unwrap());
            let decoder = Decoder::new(file).unwrap();
            sink.append(decoder);
            sink.sleep_until_end();
        })
        .await
        .unwrap();
    }
    pub async fn play_loss(&self) {
        self.play_sound("loss.mp3").await;
    }
    pub async fn play_right_left(&self) {
        self.play_sound("move-right-left.mp3").await;
    }
    pub async fn play_soft_drop(&self) {
        self.play_sound("soft-drop.mp3").await;
    }
    pub async fn play_tspin_tetris(&self) {
        self.play_sound("t-spin-tetris.mp3").await;
    }
    pub async fn play_piece_drop(&self) {
        self.play_sound("piece-drop.mp3").await;
    }
}
