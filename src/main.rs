use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::Chars;
use std::thread;
use std::time::Duration;
use cpal::{BufferSize, OutputCallbackInfo, Sample, SampleRate, StreamConfig, SupportedStreamConfig, SupportedStreamConfigRange};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use simple_bytes::{Bytes, BytesRead};

#[derive(Debug)]
struct RiffChunk {
    chunk_id: String,
    chunk_size: u32,
    format: String
}

#[derive(Debug)]
struct FmtSubChunk {
    chunk_id: String,
    chunk_size: u32,
    audio_format: u16,
    channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16
}

#[derive(Debug)]
struct DataSubChunk {
    chunk_id: String,
    chunk_size: u32,
}

#[derive(Debug)]
struct ByteSample (u8, u8);

fn main() {
    let (wav_header, wav_data) = read_wav_bytes();

    let riff_chunk = RiffChunk::new(&wav_header);
    let fmt_sub_chunk = FmtSubChunk::new(&wav_header);
    let data_sub_chunk = DataSubChunk::new(&wav_header);

    println!("{:?}", riff_chunk);
    println!("{:?}", fmt_sub_chunk);
    println!("{:?}", data_sub_chunk);

    let mut bytes_per_sample: Vec<ByteSample> = Vec::new();
    let samples = data_sub_chunk.chunk_size / fmt_sub_chunk.channels as u32 / ((fmt_sub_chunk.bits_per_sample / 8) as u32);

    let mut byte_index = 0;
    for s in 0..samples {
        for b in 0..fmt_sub_chunk.channels {
            let byte1 = *wav_data.get(byte_index).unwrap();
            let byte2 = *wav_data.get(byte_index + 1).unwrap();

            let byte_sample = ByteSample(byte1, byte2);
            bytes_per_sample.push(byte_sample);

            byte_index += 2;
        }
    }

    let host = cpal::default_host();
    let device = host.default_output_device().expect("No default output device was found");

    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.last().unwrap().with_max_sample_rate();
    let output_config = StreamConfig::from(supported_config);

    let mut wav_index = 0;
    let stream = device.build_output_stream(
        &output_config,
        move | data: &mut [f32], info: &OutputCallbackInfo | {
            for sample in data.iter_mut() {
                let byte_sample = bytes_per_sample.get(wav_index).unwrap();
                let bytes_slice = &vec![byte_sample.0, byte_sample.1][0..2];

                let mut bytes: Bytes = bytes_slice.into();
                let sample_value = bytes.read_le_i16();

                *sample = Sample::from(&sample_value);

                wav_index += 1;
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

pub fn read_wav_bytes() -> (Vec<u8>, Vec<u8>) {
    let file = File::open("assets/track.wav").unwrap();
    let mut reader = BufReader::new(file);
    let bytes = reader.bytes();

    let mut wav_header: Vec<u8> = Vec::new();
    let mut wav_data_bytes: Vec<u8> = Vec::new();

    let mut index = 0;
    for byte in bytes {
        if index >= 44 {
            wav_data_bytes.push(byte.unwrap());
        } else {
            wav_header.push(byte.unwrap());
        }
        index += 1;
    }

    (wav_header, wav_data_bytes)
}

impl RiffChunk {
    pub fn new(wav_header: &Vec<u8>) -> Self {
        let chunk_id_bytes = &wav_header[0..4];

        let chunk_size_bytes = &wav_header[4..8];
        let chunk_size_bytes_arr: [u8; 4] = chunk_size_bytes.try_into().expect("Incorrect amount of bytes");

        let format_bytes = &wav_header[8..12];

        let chunk_id: String = chunk_id_bytes.iter().map(|byte| *byte as char).collect();
        let chunk_size = u32::from_le_bytes(chunk_size_bytes_arr);
        let format: String = format_bytes.iter().map(|byte| *byte as char).collect();

        RiffChunk {
            chunk_id,
            chunk_size,
            format
        }
    }
}

impl FmtSubChunk {
    pub fn new(wav_header: &Vec<u8>) -> Self {
        let chunk_id_bytes = &wav_header[12..16];
        let chunk_size_bytes: [u8; 4] = (&wav_header[16..20]).try_into().expect("Incorrect amount of bytes");
        let audio_format_bytes: [u8; 2] = (&wav_header[20..22]).try_into().expect("Incorrect amount of bytes");
        let channels_bytes: [u8; 2] = (&wav_header[22..24]).try_into().expect("Incorrect amount of bytes");
        let sample_rate_bytes: [u8; 4] = (&wav_header[24..28]).try_into().expect("Incorrect amount of bytes");
        let byte_rate_bytes: [u8; 4] = (&wav_header[28..32]).try_into().expect("Incorrect amount of bytes");
        let block_align_bytes: [u8; 2] = (&wav_header[32..34]).try_into().expect("Incorrect amount of bytes");
        let bits_per_sample_bytes: [u8; 2] = (&wav_header[34..36]).try_into().expect("Incorrect amount of bytes");

        let chunk_id: String = chunk_id_bytes.iter().map(|byte| *byte as char).collect();
        let chunk_size = u32::from_le_bytes(chunk_size_bytes);
        let audio_format = u16::from_le_bytes(audio_format_bytes);
        let channels = u16::from_le_bytes(channels_bytes);
        let sample_rate = u32::from_le_bytes(sample_rate_bytes);
        let byte_rate = u32::from_le_bytes(byte_rate_bytes);
        let block_align = u16::from_le_bytes(block_align_bytes);
        let bits_per_sample = u16::from_le_bytes(bits_per_sample_bytes);

        FmtSubChunk {
            chunk_id,
            chunk_size,
            audio_format,
            channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample
        }
    }
}

impl DataSubChunk {
    pub fn new(wav_header: &Vec<u8>) -> Self {
        let chunk_id_bytes = &wav_header[36..40];
        let chunk_size_bytes: [u8; 4] = (&wav_header[40..44]).try_into().expect("Incorrect amount of bytes");

        let chunk_id: String = chunk_id_bytes.iter().map(|byte| *byte as char).collect();
        let chunk_size = u32::from_le_bytes(chunk_size_bytes);

        DataSubChunk {
            chunk_id,
            chunk_size
        }
    }
}
