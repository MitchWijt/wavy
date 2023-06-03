use crate::player::Player;
use crate::wav::Wav;

mod player;
mod wav;

#[derive(Debug)]
struct ByteSample (u8, u8);

fn main() {
    let wav = Wav::new("assets/track2.wav");
    println!("{:?}", wav.data);
    println!("{:?}", wav.fmt);
    let player = Player::new();

    player.play(wav);
}
