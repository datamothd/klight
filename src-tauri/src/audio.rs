use std::fs::File;
use std::sync::Mutex;

use rodio::{Decoder, MixerDeviceSink, Player, Source};

use lofty::prelude::*;
use lofty::probe::Probe;
use base64::{engine::general_purpose::STANDARD, Engine};

pub struct AudioState {
    #[allow(dead_code)]
    sink: MixerDeviceSink,
    player: Player,
    duration: Option<f32>,
}

impl AudioState {
    pub fn new(sink: MixerDeviceSink) -> Self {
        let player = Player::connect_new(sink.mixer());
        Self { sink, player, duration: None, }
    }
    pub fn play_file(&mut self, path: String) -> Result<(), String> {
        let file = File::open(&path)
            .map_err(|e| format!("Could not open file: {e}"))?;

    let source = Decoder::try_from(file)
        .map_err(|e| format!("Could not decode audio: {e}"))?;

    self.duration = source
        .total_duration()
        .map(|duration| duration.as_secs_f32());

    self.player.stop();
    self.player.append(source);
    self.player.play();
    Ok(())
    }
    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume);
    }
}

#[tauri::command]
pub fn get_artwork(path: String) -> Result<Option<String>, String> {
    let tagged_file = Probe::open(&path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;
    let Some(tag) = tagged_file.primary_tag() else {
        return Ok(None);
    };
    let Some(picture) = tag.pictures().first() else {
        return Ok(None);
    };
    let mime = picture
        .mime_type()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "image/jpeg".to_string());
    let encoded = STANDARD.encode(picture.data());
    Ok(Some(format!("data:{};base64,{}", mime, encoded)))
}

#[tauri::command]
pub fn play_audio(path: String, state: tauri::State<'_, Mutex<AudioState>>) -> Result<(), String> {
    let mut state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    state.play_file(path)
}

#[tauri::command]
pub fn toggle_pause(state: tauri::State<'_, Mutex<AudioState>>) -> Result<bool, String> {
    let state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    if state.player.is_paused() {
        state.player.play();
    } else {
        state.player.pause();
    }
    Ok(state.player.is_paused())
}

#[tauri::command]
pub fn get_position(
    state: tauri::State<'_, Mutex<AudioState>>,
) -> Result<f32, String> {
    let state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    Ok(state.player.get_pos().as_secs_f32())
}

#[tauri::command]
pub fn get_playback_info(
    state: tauri::State<'_, Mutex<AudioState>>,
) -> Result<(f32, Option<f32>), String> {
    let state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    let position = state.player.get_pos().as_secs_f32();
    Ok((position, state.duration))
}

#[tauri::command]
pub fn seek_audio(
    position: f32,
    state: tauri::State<'_, Mutex<AudioState>>,
) -> Result<(), String> {
    if !position.is_finite() || position < 0.0 {
        return Err("Invalid seek position".to_string());
    }
    let state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    state
        .player
        .try_seek(std::time::Duration::from_secs_f32(position))
        .map_err(|e| format!("Could not seek: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn set_volume(
    volume: f32,
    state: tauri::State<'_, Mutex<AudioState>>,
) -> Result<(), String> {
    if !volume.is_finite() || volume < 0.0 {
        return Err("Volume must be a finite, non-negative number".to_string());
    }
    let state = state
        .lock()
        .map_err(|e| format!("Could not access audio state: {e}"))?;
    state.set_volume(volume);
    Ok(())
}

#[tauri::command]
pub fn get_sample_rate(path: String) -> Result<Option<u32>, String> {
    let tagged_file = Probe::open(&path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;
    let properties = tagged_file.properties();
    Ok(properties.sample_rate())
}

#[tauri::command]
pub fn get_bit_depth(path: String) -> Result<Option<u8>, String> {
    let tagged_file = Probe::open(&path)
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;
    let properties = tagged_file.properties();
    Ok(properties.bit_depth())
}