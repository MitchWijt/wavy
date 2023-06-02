use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub struct Wav {
    pub riff: RiffChunk,
    pub fmt: FmtSubChunk,
    pub data: DataSubChunk,
    audio_data_reader: BufReader<File>
}

impl Wav {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).expect("Unable to open WAV file");
        let mut reader = BufReader::new(file);

        let mut header = vec![0u8; 44];
        reader.read_exact(&mut header).expect("Error when reading header");

        let riff_chunk = RiffChunk::from_header(&header);
        let fmt_sub_chunk = FmtSubChunk::from_header(&header);
        let data_sub_chunk = DataSubChunk::from_header(&header);

        Wav {
            riff: riff_chunk,
            fmt: fmt_sub_chunk,
            data: data_sub_chunk,
            audio_data_reader: reader,
        }
    }

    pub fn read_buffer(&mut self) -> Vec<u8> {
        let mut buffer = vec![0u8; 1024];
        self.audio_data_reader.read_exact(&mut buffer).expect("Error when reading buffer");

        buffer
    }
}

#[derive(Debug)]
pub struct RiffChunk {
    chunk_id: String,
    chunk_size: u32,
    format: String
}

impl RiffChunk {
    pub fn from_header(wav_header: &Vec<u8>) -> Self {
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
    chunk_id: String,
    chunk_size: u32,
    audio_format: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16
}

impl FmtSubChunk {
    pub fn from_header(wav_header: &Vec<u8>) -> Self {
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
    chunk_id: String,
    chunk_size: u32,
}

impl DataSubChunk {
    pub fn from_header(wav_header: &Vec<u8>) -> Self {
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
