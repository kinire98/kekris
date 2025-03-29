use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, Sink};
use tauri::{AppHandle, Manager};
use tokio::task;

const LOSS_SOUND: f32 = 1.0;
const RIGHT_LEFT_SOUND: f32 = 0.1;
const SOFT_DROP_SOUND: f32 = 5.0;
const TSPIN_SOUND: f32 = 2.0;
const PIECE_DROP_SOUND: f32 = 1.5;
const LINE_CLEAR_SOUND: f32 = 1.0;

async fn play_sound(sound: String, handle: AppHandle, volume: f32) {
    tokio::spawn(async move {
        let path = handle
            .path()
            .resolve(sound, tauri::path::BaseDirectory::Resource)
            .unwrap();
        task::spawn_blocking(move || {
            let (_stream, handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&handle).unwrap();
            sink.set_volume(volume);
            let file = BufReader::new(File::open(path).unwrap());
            let decoder = Decoder::new(file).unwrap();
            sink.append(decoder);
            sink.sleep_until_end();
        })
        .await
        .unwrap();
    });
}
pub async fn play_loss(handle: AppHandle) {
    // play_sound("assets/sound-fx/loss.mp3".to_string(), handle, LOSS_SOUND).await;
}
pub async fn play_right_left(handle: AppHandle) {
    // play_sound(
    //     "assets/sound-fx/move-right-left.wav".to_string(),
    //     handle,
    //     RIGHT_LEFT_SOUND,
    // )
    // .await;
}
pub async fn play_soft_drop(handle: AppHandle) {
    // play_sound(
    //     "assets/sound-fx/soft-drop.wav".to_string(),
    //     handle,
    //     SOFT_DROP_SOUND,
    // )
    // .await;
}
pub async fn play_tspin_tetris(handle: AppHandle) {
    // play_sound(
    //     "assets/sound-fx/t-spin-tetris.mp3".to_string(),
    //     handle,
    //     TSPIN_SOUND,
    // )
    // .await;
}
pub async fn play_piece_drop(handle: AppHandle) {
    // play_sound(
    //     "assets/sound-fx/piece-drop.wav".to_string(),
    //     handle,
    //     PIECE_DROP_SOUND,
    // )
    // .await;
}
pub async fn play_line_clear(handle: AppHandle) {
    // play_sound(
    //     "assets/sound-fx/line-clear.wav".to_string(),
    //     handle,
    //     LINE_CLEAR_SOUND,
    // )
    // .await;
}
