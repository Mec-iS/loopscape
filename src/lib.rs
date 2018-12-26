// These are made public as they need to be accessed from
// other modules

///
/// Constants: core constants for logic
///
pub mod constants {
    // Primitive of a pattern
    // every primitive is assigned a sound, other settings may be add in future
    pub struct Primitive<'a> {
        path: &'a str,
        character: &'a str,
    }

    pub const DOT: Primitive = Primitive { 
          path: "assets/dot.wav", character: "." };
    pub const DASH: Primitive = Primitive {
          path: "assets/dash.wav", character: "-" };
    pub const PAUSE: Primitive = Primitive {
          path: "assets/pause.wav", character: "P" };

    const ELEMENTS: &[Primitive; 3] = &[DOT, DASH, PAUSE];

    impl<'b> Primitive<'_> {
        //
        // Return a elements' sound-path from a reference character
        //
        pub fn get_path_from_char(character: &'b char) -> &str {
            // default value is a pause
            let mut path: &str = self::PAUSE.path;
            for e in self::ELEMENTS {
                let iter: Vec<char> = e.character.chars().collect();
                for c in iter {
                    if c == *character {
                        path = e.path;
                        break
                    }
                }
            }
            
            // matching element's path is returned
            path
        }
    }
}

///
/// Basic hard-coded shapes
///
pub mod shapes {

    // This is a generic shape
    // they are used as "Primitives" to be basic building blocks
    #[derive(Debug)]
    pub struct Shape {
        pub name: String,
        pub pattern: String,
        pub sides: u32,
    }

    // define basic shapes and a `build` method for generic shapes
    // every "shape" matches to a "pattern"
    impl Shape {
        pub fn square() -> Shape {
            Shape::build (
                String::from("square"),
                String::from("...P"),
                4,
            )
        }

        pub fn triangle() -> Shape {
            Shape::build(
                String::from("triangle"),
                String::from(".-."),
                3,
            )
        }

        pub fn pentagon() -> Shape {
            Shape::build(
                String::from("pentagon"),
                String::from("-.P-"),
                4,
            )
        }

        pub fn build(name: String, pattern: String, sides: u32) -> Shape {
            Shape {
                name: name,
                pattern: pattern,
                sides: sides,
            }
        }
    }
}

///
/// Allowed operations on shapes
///
pub mod operations {
    use std::time::Duration;
    use std::thread;
    use std::thread::sleep;
    
    use super::soundscape::play_path;
    use super::constants;

    ///
    /// An operation is a coupling of a symbol to which the operation
    /// responds and a function that define how the patterns passed
    /// have to be played.
    pub struct Operation<'a> {
        symbol: &'a str,
        func: &'a Fn(&[Vec<char>; 2]),
    }

    ///
    /// An operation that plays patterns by adding sounds with the same
    /// position (index in a vector). The "plain" mode makes shoter patterns
    /// to repeat from the beggining to adapt to the lenght of longer patterns.
    pub fn stacking(pattern: &[Vec<char>; 2]) {}

    /// 
    /// An operations that plays patterns one after the other
    pub fn sequencing(pattern: &[Vec<char>; 2]) {}

    ///
    /// An operation that applies some kind of transformation over a pattern
    /// before it gets played. This a single-input operation.
    pub fn mutating(pattern: &[Vec<char>; 2]) {}

    ///
    /// An operation that plays the elements of patterns in a Z-curve (interleaving)
    /// fashion
    pub fn interleaving(pattern: &[Vec<char>; 2]) {}

    /// 
    /// Threading:
    /// An operation that plays patterns using machine's Thread
    /// implmentation. As Rust uses a non-green N-N model, sounds
    /// that make up a pattern are played according to machine specs
    /// and in an arbitrary order.
    pub fn threading(patterns: &[Vec<char>; 2]) {
        let Threading: Operation = Operation {
            symbol: "||",
            func: &threading,
        };

        let mut handles: Vec<_> = Vec::new();

        sleep(Duration::from_millis(5));
        
        for (i, pattern) in patterns.iter().enumerate() {
            let pattern = pattern.clone();
            for (j, ch) in pattern.iter().enumerate() {
                let ch = ch.clone();
                handles.push(thread::spawn(move || {
                   play_path(
                       constants::Primitive::get_path_from_char(&ch)
                   );
                }));
                sleep(Duration::from_millis(1));
            }
        }

        // Wait until the last sound is played, the main task own the ears context,
        // so we should kepp it alive
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

mod soundscape {
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
}