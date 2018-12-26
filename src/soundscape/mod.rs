use ears::{Sound, AudioController};

pub fn play_path(path: &str) {
    // Create a new Sound.
    println!("{}", path);
    let mut snd = Sound::new(path).unwrap();

    // Play the Sound
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}