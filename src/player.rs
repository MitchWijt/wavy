use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::{io, thread};
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::{BufReader, Read};
use std::os::macos::raw::stat;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;
use cpal::{Device, Host, OutputCallbackInfo, Sample, SampleRate, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_queue::SegQueue;
use simple_bytes::{Bytes, BytesRead};
use crate::playback_duration::{PlaybackDuration};
use crate::{Commands, Wav};


pub struct Player {
    buffer_index: usize,
    bytes_read: usize,
    buffer: Option<Vec<u8>>,
    playback_state: PlaybackState,
    from_gui_queue: Arc<SegQueue<Commands>>,
    to_gui_queue: Arc<SegQueue<Commands>>
}

impl Player {
    pub fn new(from_gui_queue: Arc<SegQueue<Commands>>, to_gui_queue: Arc<SegQueue<Commands>>) -> Self {
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
                Commands::PLAY {
                    buffer
                } => {
                    self.playback_state = PlaybackState::Playing;
                    self.buffer = Some(buffer);
                    self.buffer_index = 0;
                    self.bytes_read = 0;
                },
                Commands::PAUSE => {
                    self.playback_state = PlaybackState::Paused;
                },
                Commands::PLAYRESUME => {
                    self.playback_state = PlaybackState::Playing;
                },
                _ => {}
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
            let sample_bytes = &self.buffer.as_ref().unwrap()[self.buffer_index..self.buffer_index + 2];
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