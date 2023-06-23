use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::{io, thread};
use std::collections::HashMap;
use std::fs::read_dir;
use std::os::macos::raw::stat;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;
use cpal::{Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};
use crate::playback_duration::{PlaybackDuration};
use crate::Wav;

pub struct Player {
    pub player_state: Arc<Mutex<PlayerState>>,
    pub playback_state: Arc<Mutex<PlaybackState>>,
    pub playback_duration: Arc<Mutex<PlaybackDuration>>,
    platform_settings: Arc<PlatformSettings>
}

impl Player {
    pub fn new() -> Self {
        Player {
            player_state: Arc::new(Mutex::new(PlayerState::new())),
            playback_state: Arc::new(Mutex::new(PlaybackState::new())),
            playback_duration: Arc::new(Mutex::new(PlaybackDuration::new())),
            platform_settings: Arc::new(PlatformSettings::new())
        }
    }

    pub fn stream(&self) -> Result<(), &'static str> {
        let playback_duration = self.playback_duration.clone();
        let platform_settings = self.platform_settings.clone();
        let playback_state = self.playback_state.clone();
        let player_state = self.player_state.clone();

        self.next();

        thread::spawn(move || {
            let stream = platform_settings.device.build_output_stream(
                &platform_settings.config,
                move | data: &mut [f32], _: &OutputCallbackInfo | {
                    for sample in data.iter_mut() {
                        let mut state = playback_state.lock().unwrap();
                        if state.buffer.len() == 0 {
                            continue;
                        }

                        if state.bytes_read >= state.buffer.len() {
                            player_state.lock().unwrap().playlist_index += 1;
                        }

                        let byte_sample = &state.buffer[state.buffer_index..state.buffer_index + 2];

                        let mut bytes: Bytes = byte_sample.into();
                        let sample_value = bytes.read_le_i16();

                        *sample = Sample::from(&sample_value);

                        state.bytes_read += 2;
                        state.buffer_index += 2;

                        if state.bytes_read % state.bytes_per_ms == 0 {
                            playback_duration.lock().unwrap().milliseconds += 1;
                        }

                        if state.bytes_read % state.bytes_per_s == 0 {
                            playback_duration.lock().unwrap().advance();
                        }
                    }
                },
                move | err | {
                    eprintln!("{}", err);
                }
            ).unwrap();

            stream.play().unwrap();
            loop {}
        });

        Ok(())
    }

    pub fn next(&self) {
        let playback_duration = PlaybackDuration::new();
        *self.playback_duration.lock().unwrap() = playback_duration;

        let playback_state = PlaybackState::new();
        *self.playback_state.lock().unwrap() = playback_state;

        let mut player_state_binding = self.player_state.lock().unwrap();
        let playlist_index = player_state_binding.playlist_index;
        let current_path = player_state_binding.playlist.get(playlist_index).unwrap();
        let mut current_wav = Wav::new(current_path);

        let sample_rate = current_wav.header.fmt.sample_rate;
        let block_align = current_wav.header.fmt.block_align as u32;
        let bytes_per_second: usize = (sample_rate * block_align) as usize;
        let bytes_per_ms: usize = ((sample_rate * block_align) / 1000) as usize;

        self.playback_state.lock().unwrap().buffer = current_wav.load_data().unwrap();
        self.playback_state.lock().unwrap().bytes_per_s = bytes_per_second;
        self.playback_state.lock().unwrap().bytes_per_ms = bytes_per_ms;
        player_state_binding.active_song = current_wav;
    }
}

pub struct PlaybackState {
    buffer: Vec<u8>,
    buffer_index: usize,
    bytes_read: usize,
    bytes_per_s: usize,
    bytes_per_ms: usize,
}

impl PlaybackState {
    pub fn new() -> Self {
        PlaybackState {
            buffer: Vec::new(),
            buffer_index: 0,
            bytes_read: 0,
            bytes_per_s: 0,
            bytes_per_ms: 0,
        }
    }
}

pub struct PlayerState {
    pub playlist: Vec<PathBuf>,
    pub active_song: Wav,
    pub playlist_index: usize,
}

impl PlayerState {
    pub fn new() -> Self {
        let playlist: Vec<PathBuf> = read_dir("./assets").unwrap().map(|res| res.unwrap().path()).collect();
        let active_song = Wav::new(playlist.get(0).unwrap().clone());

        PlayerState {
            playlist,
            active_song,
            playlist_index: 0
        }
    }
}

struct PlatformSettings {
    device: Device,
    config: StreamConfig
}

impl PlatformSettings {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No default output device was found");

        let mut supported_configs_range = device.supported_output_configs().expect("error while querying configs");
        let supported_config = supported_configs_range
            .find(|d| d.max_sample_rate() == SampleRate(44100))
            .expect("No config with correct sample rate found")
            .with_max_sample_rate();

        let output_config = StreamConfig::from(supported_config);

        PlatformSettings {
            device,
            config: output_config,
        }
    }
}