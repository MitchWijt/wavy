struct PlaybackState {
    playing: bool,
    seconds_playing: u32,
    minutes_playing: u32,
    buffer: Vec<u8>,
    buffer_index: u32,
    bytes_read: u32
}