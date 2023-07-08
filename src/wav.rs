use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek};
use std::path::Path;
use std::ptr::write;
use std::time::Duration;

pub struct Wav {
    pub header: WavHeader,
    pub duration: WavDuration,
    pub audio_data_reader: BufReader<File>,
}

impl Wav {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).expect("Unable to open WAV file");
        let mut reader = BufReader::new(file);

        let mut header_bytes = vec![0u8; 44];
        reader.read_exact(&mut header_bytes).expect("Error when reading header");

        let header = WavHeader::from_header_bytes(header_bytes);
        let duration = WavDuration::from_header(&header);

        Wav {
            header,
            duration,
            audio_data_reader: reader,
        }
    }

    pub fn load_data(&mut self) -> Result<Vec<u8>, io::Error> {
        let chunk_size = self.header.data.chunk_size as usize;
        let mut buffer = vec![0u8; chunk_size];
        self.audio_data_reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }

    pub fn read_buffer(&mut self, size: usize) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![0u8; size];
        self.audio_data_reader.read_exact(&mut buffer)?;

        Ok(buffer)
    }
}

pub struct WavDuration {
    pub raw_seconds: f32,
    pub seconds: f32,
    pub raw_minutes: f32,
    pub minutes: f32,
    pub milliseconds: f32,
}

impl WavDuration {
    pub fn from_header(header: &WavHeader) -> Self {
        let samples: f32 = (header.data.chunk_size / ((header.fmt.channels * header.fmt.bits_per_sample / 8) as u32)) as f32;

        let raw_seconds: f32 = samples / (header.fmt.sample_rate as f32);
        let raw_minutes: f32 = raw_seconds / 60.0;

        let minutes = raw_minutes.floor();

        let remaining_seconds = raw_minutes % minutes;
        let seconds = (remaining_seconds * 60.0).ceil();

        WavDuration {
            raw_seconds,
            seconds,
            raw_minutes,
            minutes,
            milliseconds: seconds * 1000.0
        }
    }
}

impl Display for WavDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let minutes = if self.minutes < 10.0 {
            format!("0{}", self.minutes)
        } else {
            format!("{}", self.minutes)
        };

        let seconds = if self.seconds < 10.0 {
            format!("0{}", self.seconds)
        } else {
            format!("{}", self.seconds)
        };

        write!(f, "{}:{}", minutes, seconds)
    }
}

impl Clone for WavDuration {
    fn clone(&self) -> Self {
        WavDuration {
            raw_seconds: self.raw_seconds,
            raw_minutes: self.raw_minutes,
            seconds: self.seconds,
            minutes: self.minutes,
            milliseconds: self.milliseconds
        }
    }
}

impl Copy for WavDuration {}

pub struct WavHeader {
    pub riff: RiffChunk,
    pub fmt: FmtSubChunk,
    pub data: DataSubChunk,
}

impl WavHeader {
    pub fn from_header_bytes(header: Vec<u8>) -> Self {
        let riff_chunk = RiffChunk::from_header_bytes(&header);
        let fmt_sub_chunk = FmtSubChunk::from_header_bytes(&header);
        let data_sub_chunk = DataSubChunk::from_header_bytes(&header);

        WavHeader {
            riff: riff_chunk,
            fmt: fmt_sub_chunk,
            data: data_sub_chunk
        }
    }
}

#[derive(Debug)]
pub struct RiffChunk {
    chunk_id: String,
    chunk_size: u32,
    format: String
}

impl RiffChunk {
    pub fn from_header_bytes(wav_header: &Vec<u8>) -> Self {
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

#[derive(Debug)]
pub struct FmtSubChunk {
    pub chunk_id: String,
    pub chunk_size: u32,
    pub audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16
}

impl FmtSubChunk {
    pub fn from_header_bytes(wav_header: &Vec<u8>) -> Self {
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

#[derive(Debug)]
pub struct DataSubChunk {
    pub chunk_id: String,
    pub chunk_size: u32,
}

impl DataSubChunk {
    pub fn from_header_bytes(wav_header: &Vec<u8>) -> Self {
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

