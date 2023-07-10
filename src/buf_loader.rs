// use std::fs::File;
// use std::io::{BufReader, Read, Seek, SeekFrom};
// use std::sync::Arc;
// use crate::playlist::{Playlist, Song};
//
// pub struct BufLoader {
//     pre_loaded_buffer: Option<Vec<u8>>
// }
//
// impl BufLoader {
//     pub fn new() -> Self {
//         BufLoader {
//             pre_loaded_buffer: None
//         }
//     }
//
//     pub fn get_buffer(self, index: u16, playlist: Arc<Playlist>) -> Vec<u8> {
//         return match self.pre_loaded_buffer {
//             Some(buffer) => buffer,
//             None => self.load_buffer(index, playlist)
//         }
//     }
//
//     pub fn load_buffer(&self, index: u16, playlist: Arc<Playlist>) -> Vec<u8> {
//         println!("LOADING NOT CACHE");
//         let song: &Song = playlist.songs.get(index as usize).unwrap();
//         let file = File::open(&song.path).unwrap();
//
//         let mut reader = BufReader::new(file);
//
//         // set seek position after the RIFF header
//         reader.seek(SeekFrom::Start(44)).unwrap();
//
//         let mut buffer = vec![0u8; song.wav.header.data.chunk_size as usize];
//         reader.read_exact(&mut *buffer).unwrap();
//
//         buffer
//     }
//
//     pub fn pre_load_buffer(&mut self, index: u16, playlist: Arc<Playlist>) {
//         self.pre_loaded_buffer = Some(self.load_buffer(index, playlist));
//     }
// }