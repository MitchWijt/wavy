use std::fmt::{Display, Formatter};
use std::fs::{read_dir};
use std::path::{PathBuf};
use crate::wav::Wav;

pub struct Playlist {
    pub songs: Vec<Song>,
    pub indexes: Vec<usize>
}

impl Playlist {
    pub fn new() -> Self {
        let mut songs: Vec<Song> = Vec::new();
        let mut indexes: Vec<usize> = Vec::new();

        let song_paths: Vec<PathBuf> = read_dir("./playlist")
            .unwrap()
            .filter(|res| res.as_ref().unwrap().file_name().to_str().unwrap().contains(".wav"))
            .map(|res| res.unwrap().path())
            .collect();

        let mut index_count: usize = 0;
        for path in song_paths {
            let song = Song::from_path(path);
            songs.push(song);

            indexes.push(index_count);
            index_count += 1;
        }

        return Playlist {
            songs,
            indexes
        }
    }
}

pub struct Song {
    pub wav: Wav,
    pub artist: String,
    pub title: String,
    pub path: PathBuf
}

impl Song {
    pub fn from_path(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let splitted_file_name: Vec<&str> = file_name.split("-").collect();

        let artist = String::from(*splitted_file_name.get(0).unwrap());
        let title = String::from(*splitted_file_name.get(1).unwrap()).replace(".wav", "");

        Song {
            path: PathBuf::from(&path),
            wav: Wav::new(path),
            artist,
            title
        }
    }
}

impl Display for Song {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.title, self.artist)
    }
}