use std::sync::{Arc, Mutex};
use std::time::Duration;
use cpal::{Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};
use crate::playback_state::{PlaybackDuration, PlaybackState};
use crate::Wav;

pub struct Player {
    device: Device,
    config: StreamConfig,
    playback_state: Arc<Mutex<PlaybackState>>
}

impl Player {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No default output device was found");

        let mut supported_configs_range = device.supported_output_configs().expect("error while querying configs");
        let supported_config = supported_configs_range
            .find(|d| d.max_sample_rate() == SampleRate(44100))
            .expect("No config with correct sample rate found")
            .with_max_sample_rate();

        let output_config = StreamConfig::from(supported_config);
        let playback_state = Arc::new(Mutex::new(PlaybackState::new()));

        Player {
            device,
            config: output_config,
            playback_state
        }
    }

    pub fn play(&mut self, mut wav: Wav) {
        let playback_state = self.playback_state.clone();
        playback_state.lock().unwrap().playing = true;

        let mut buffer: Vec<u8> = Vec::new();
        let mut buffer_index: usize = 0;
        let mut bytes_read: usize = 0;

        let stream = self.device.build_output_stream(
            &self.config,
            move | data: &mut [f32], _: &OutputCallbackInfo | {
                let buffer_size = data.len();
                for sample in data.iter_mut() {
                    if bytes_read % buffer_size == 0 {
                        buffer = wav.read_buffer(buffer_size);
                        buffer_index = 0;
                    }

                    let byte_sample = &buffer[buffer_index..buffer_index + 2];

                    let mut bytes: Bytes = byte_sample.into();
                    let sample_value = bytes.read_le_i16();

                    *sample = Sample::from(&sample_value);

                    bytes_read += 2;
                    buffer_index += 2;

                    let bytes_per_second: usize = (wav.header.fmt.sample_rate * (wav.header.fmt.block_align) as u32) as usize;
                    if bytes_read % bytes_per_second == 0 {
                        playback_state.lock().unwrap().playback_duration.advance();
                        println!("{}    {}", playback_state.lock().unwrap().playback_duration, wav.duration);
                    }
                }
            },
            move | err | {
                eprintln!("{}", err);
            }
        ).unwrap();

        stream.play().unwrap();
        loop {}
    }
}