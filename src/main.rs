extern crate ears;
extern crate rand;

mod soundscape;
mod tests;

use std::error::Error;

use ears::{Sound, AudioController};
use rand::Rng;

use loopscope::shapes::Shape;
use loopscope::constants::{DOT, DASH, PAUSE};
use loopscope::operations::*;



fn main() {
    let sqr: Shape = Shape::square();
    let trn: Shape = Shape::triangle();
    let pnt: Shape = Shape::pentagon();

    let operations = [
        stacking,
        sequencing,
        mutating,
        interleaving,
        threading
    ];

    // let shapes: [Shape; 3] = [
    //     sqr, trn, pnt,
    // ];

    // println!("shapes no: {}", shapes.len());

    // let rnd1: &Shape = &rand::thread_rng().choose(&shapes).unwrap();
    // let rnd2: &Shape = &rand::thread_rng().choose(&shapes).unwrap();

    let rnd1: &Shape = &sqr;
    let rnd2: &Shape = &pnt;

    println!("1: {:?} \n2: {:?}", rnd1, rnd2);

    let sounds: [&str; 2] = [&rnd1.pattern, &rnd2.pattern];

    println!("1: {:?} \n2: {:?}", sounds[0], sounds[1]);

    let mut pattern1: String = String::new();
    let mut pattern2: String = String::new();
    pattern1.push_str(sounds[0]);
    pattern2.push_str(sounds[1]);

    println!("1: {:?}", pattern1);
    println!("2: {:?}", pattern2);

    let patterns: [Vec<char>; 2] = [
        pattern1.chars().collect(),
        pattern2.chars().collect(),
    ];
    
    threading(&patterns);

}
