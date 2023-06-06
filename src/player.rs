use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::{io, thread};
use std::process::exit;
use std::time::Duration;
use cpal::{Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};
use crate::playback_state::{PlaybackDuration};
use crate::Wav;

pub struct Player {
    pub state: Arc<Mutex<PlayerState>>,
    pub playback_duration: Arc<Mutex<PlaybackDuration>>,
    platform_settings: Arc<PlatformSettings>
}

impl Player {
    pub fn new(wav: Wav) -> Self {
        Player {
            state: Arc::new(Mutex::new(PlayerState::from_wav(wav))),
            playback_duration: Arc::new(Mutex::new(PlaybackDuration::new())),
            platform_settings: Arc::new(PlatformSettings::new())
        }
    }
    pub fn stream(&self, next_song_tx: Sender<bool>) -> Result<(), &'static str> {
        let playback_duration = self.playback_duration.clone();
        let platform_settings = self.platform_settings.clone();
        let state = self.state.clone();

        thread::spawn(move || {
            let stream = platform_settings.device.build_output_stream(
                &platform_settings.config,
                move | data: &mut [f32], _: &OutputCallbackInfo | {
                    let buffer_size = data.len();
                    for sample in data.iter_mut() {
                        let mut state = state.lock().unwrap();

                        if state.bytes_read % buffer_size == 0 {
                            match state.wav.read_buffer(buffer_size) {
                                Ok(v) => {
                                    state.buffer = v;
                                    state.buffer_index = 0;
                                },
                                Err(e) => {
                                    if e.kind() == io::ErrorKind::UnexpectedEof {
                                        next_song_tx.send(true).unwrap();
                                        thread::sleep(Duration::from_millis(500));
                                        break;
                                    }
                                }
                            }
                        }

                        let byte_sample = &state.buffer[state.buffer_index..state.buffer_index + 2];

                        let mut bytes: Bytes = byte_sample.into();
                        let sample_value = bytes.read_le_i16();

                        *sample = Sample::from(&sample_value);

                        state.bytes_read += 2;
                        state.buffer_index += 2;

                        let sample_rate = state.wav.header.fmt.sample_rate;
                        let block_align = state.wav.header.fmt.block_align as u32;

                        let bytes_per_second: usize = (sample_rate * block_align) as usize;
                        if state.bytes_read % bytes_per_second == 0 {
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

}

pub struct PlayerState {
    wav: Wav,
    buffer: Vec<u8>,
    buffer_index: usize,
    bytes_read: usize
}

impl PlayerState {
    pub fn from_wav(wav: Wav) -> Self {
        PlayerState {
            wav: wav,
            buffer: Vec::new(),
            buffer_index: 0,
            bytes_read: 0
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