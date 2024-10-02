use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::path::Path;

trait WavFile {
    fn is_wav(&mut self) -> bool;
}

impl WavFile for File {
    fn is_wav(&mut self) -> bool {
        let mut buffer = [0; 4];

        match self.read_exact(&mut buffer) {
            Ok(_) => matches!(&buffer, b"RIFF"),
            Err(_) => false,
        }
    }
}

#[derive(Debug)]
struct RiffChunk {
    riff_id: [u8; 4],
    file_size: u32,
    wave_id: [u8; 4],
}

#[derive(Debug)]
struct FmtChunk {
    format_id: [u8; 4],
    chunk_size: u32,
    audio_format_tag: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[derive(Debug)]
struct FactChunk {
    chunk_id: [u8; 4],
    chunk_size: u32,
    dw_sample_length: u32,
}

#[derive(Debug)]
struct DataChunk {
    data_id: [u8; 4],
    chunk_size: u32,
    sampled_data: Vec<u8>,
}

#[derive(Debug)]
struct WavHeader {
    riff_chunk: RiffChunk,
    fmt_chunk: FmtChunk,
    fact_chunk: Option<FactChunk>,
    data_chunk: DataChunk,
}

fn main() -> io::Result<()> {
    let path = Path::new("./foo.wav");
    let mut f = File::open(&path)?;

    if f.is_wav() {
        println!("The file is a valid WAV file.");
    } else {
        println!("The file is not a valid WAV file.");
    }

    Ok(())
}

fn create_wav_header() -> std::io::Result<()> {
    let riff_chunk = RiffChunk {
        riff_id: *b"RIFF",
        file_size: 44 - 8,
        wave_id: *b"WAVE",
    };

    let fmt_chunk = FmtChunk {
        format_id: *b"fmt ",
        chunk_size: 16,
        audio_format_tag: 1,
        num_channels: 2,
        sample_rate: 44100,
        byte_rate: 44100 * 2 * 16 / 8,
        block_align: 4,
        bits_per_sample: 16,
    };

    let data_chunk = DataChunk {
        data_id: *b"data",
        chunk_size: 44100 * 4,
        sampled_data: vec![0; 44100 * 4],
    };

    let wav_header = WavHeader {
        riff_chunk,
        fmt_chunk,
        fact_chunk: None,
        data_chunk,
    };

    println!("WAV chunks created successfully");

    Ok(())
}
