// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod audio;
use audio::{AudioState, get_artwork, play_audio, set_volume, toggle_pause, get_sample_rate, get_bit_depth, get_position, get_playback_info, seek_audio};

use std::sync::Mutex;

use rodio::DeviceSinkBuilder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_sink = DeviceSinkBuilder::open_default_sink().expect("Could not open default audio device");
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(AudioState::new(audio_sink)))
        .invoke_handler(tauri::generate_handler![get_artwork, play_audio, toggle_pause, set_volume, get_sample_rate, get_bit_depth, get_position, get_playback_info, seek_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}