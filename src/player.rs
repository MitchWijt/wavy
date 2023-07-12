use std::sync::{Arc};
use cpal::{Sample};
use crossbeam_queue::SegQueue;
use simple_bytes::{Bytes, BytesRead};
use crate::{GuiToPlayerCommands, PlayerToGuiCommands};

pub struct Player {
    buffer_index: usize,
    bytes_read: usize,
    buffer: Option<Vec<u8>>,
    playback_state: PlaybackState,
    from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>,
    to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>
}

impl Player {
    pub fn new(from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>, to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>) -> Self {
        Player {
            buffer: None,
            buffer_index: 0,
            bytes_read: 0,
            playback_state: PlaybackState::Paused,
            from_gui_queue,
            to_gui_queue
        }
    }

    pub fn process(&mut self, data: &mut [f32]) {
        while let Some(command) = self.from_gui_queue.pop() {
            match command {
                GuiToPlayerCommands::Play {
                    buffer
                } => {
                    self.playback_state = PlaybackState::Playing;
                    self.buffer = Some(buffer);
                    self.buffer_index = 0;
                    self.bytes_read = 0;

                    self.to_gui_queue.push(PlayerToGuiCommands::Playing);
                },
                GuiToPlayerCommands::Pause => {
                    self.playback_state = PlaybackState::Paused;
                    self.to_gui_queue.push(PlayerToGuiCommands::Paused);
                },
                GuiToPlayerCommands::PlayResume => {
                    self.playback_state = PlaybackState::Playing;
                    self.to_gui_queue.push(PlayerToGuiCommands::Playing);
                },
                GuiToPlayerCommands::Forward => {
                    let sample_rate = 44100;
                    let bytes_per_s = sample_rate * 4;
                    let forwarded_amount = bytes_per_s * 15;
                    self.bytes_read += forwarded_amount;
                    self.buffer_index += forwarded_amount;
                }
                GuiToPlayerCommands::Rewind => {
                    let sample_rate = 44100;
                    let bytes_per_s = sample_rate * 4;
                    let forwarded_amount = bytes_per_s * 15;

                    if self.bytes_read < forwarded_amount {
                        self.bytes_read = 0;
                        self.buffer_index = 0;
                    } else {
                        self.bytes_read -= forwarded_amount;
                        self.buffer_index -= forwarded_amount;
                    }
                }
            }
        }

        if self.playback_state == PlaybackState::Paused {
            silence(data);
            return;
        }

        if self.buffer.is_none() {
            silence(data);
            return;
        }

        for sample in data.iter_mut() {
            let buffer = &self.buffer.as_ref().unwrap();
            if self.buffer_index + 1 > buffer.len() {
                self.to_gui_queue.push(PlayerToGuiCommands::End);
                self.buffer = None;
                return;
            }
            let sample_bytes = &buffer[self.buffer_index..self.buffer_index + 2];
            let mut bytes: Bytes = sample_bytes.into();
            let sample_value = bytes.read_le_i16();

            *sample = Sample::from(&sample_value);

            self.bytes_read += 2;
            self.buffer_index += 2;
        }
    }
}

pub fn silence(data: &mut [f32]) {
    for sample in data.iter_mut() {
        *sample = 0.0;
    }
}

#[derive(PartialEq)]
enum PlaybackState {
    Paused,
    Playing
}