use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};
use crate::wav::Wav;

pub struct Playlist {
    pub songs: Vec<Song>
}

impl Playlist {
    pub fn new() -> Self {
        let mut songs: Vec<Song> = Vec::new();

        let song_paths: Vec<PathBuf> = read_dir("./playlist")
            .unwrap()
            .filter(|res| res.as_ref().unwrap().file_name().to_str().unwrap().contains(".wav"))
            .map(|res| res.unwrap().path())
            .collect();

        for path in song_paths {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let splitted_file_name: Vec<&str> = file_name.split("-").collect();

            let artist = String::from(*splitted_file_name.get(0).unwrap());
            let title = String::from(*splitted_file_name.get(1).unwrap()).replace(".wav", "");

            songs.push(Song {
                path: PathBuf::from(&path),
                wav: Wav::new(path),
                artist,
                title
            })
        }

        return Playlist {
            songs
        }
    }
}

pub struct Song {
    pub wav: Wav,
    pub artist: String,
    pub title: String,
    pub path: PathBuf
}