use std::sync::Arc;
use cpal::{Device, OutputCallbackInfo, SampleRate, Stream, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_queue::SegQueue;
use crate::{GuiToPlayerCommands, Player, PlayerToGuiCommands};

pub struct Output;

impl Output {
    pub fn new(from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>, to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>) -> Stream {
        let platform_settings = PlatformSettings::new();
        let mut player = Player::new(from_gui_queue, to_gui_queue);

        let stream = platform_settings.device.build_output_stream(
            &platform_settings.config,
            move | data: &mut [f32], _: &OutputCallbackInfo | player.process(data),
            move | err | {
                eprintln!("{}", err);
            }
        ).unwrap();

        stream.play().unwrap();

        stream
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