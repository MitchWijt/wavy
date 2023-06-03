use crate::player::Player;
use crate::wav::Wav;

mod player;
mod wav;
mod playback_state;

#[derive(Debug)]
struct ByteSample (u8, u8);

fn main() {
    let wav = Wav::new("assets/track2.wav");
    let player = Player::new();

    player.play(wav);
}
