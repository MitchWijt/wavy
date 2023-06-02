use std::fs::File;
use std::io::BufReader;
use cpal::{Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};
use crate::Wav;

pub struct Player {
    host: Host,
    device: Device,
    config: StreamConfig,
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

        Player {
            host,
            device,
            config: output_config
        }
    }

    pub fn play(&self, mut wav: Wav) {
        let mut bytes_read = 0;
        let mut bytes = wav.read_buffer();
        let mut byte_index = 0;

        let stream = self.device.build_output_stream(
            &self.config,
            move | data: &mut [f32], info: &OutputCallbackInfo | {
                for sample in data.iter_mut() {
                    if bytes_read % 1024 == 0 {
                        bytes = wav.read_buffer();
                        byte_index = 0;
                    }

                    let byte_sample = &bytes[byte_index..byte_index + 2];

                    let mut bytes: Bytes = byte_sample.into();
                    let sample_value = bytes.read_le_i16();

                    *sample = Sample::from(&sample_value);

                    bytes_read += 2;
                    byte_index += 2;
                }
            },

            move | err | {
                eprintln!("{}", err);
            }
        ).unwrap();

        stream.play().unwrap();
        loop {

        }
    }
}