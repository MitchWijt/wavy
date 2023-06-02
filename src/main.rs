mod player;
mod wav;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::Chars;
use std::thread;
use std::time::Duration;
use cpal::{BufferSize, OutputCallbackInfo, Sample, SampleFormat, SampleRate, StreamConfig, SupportedStreamConfig, SupportedStreamConfigRange};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};
use crate::player::Player;
use crate::wav::Wav;

#[derive(Debug)]
struct ByteSample (u8, u8);

fn main() {
    let mut wav = Wav::new("assets/track.wav");
    let player = Player::new();

    player.play(wav);
}
