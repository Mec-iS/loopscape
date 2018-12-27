extern crate ears;
extern crate rand;

mod tests;

use std::error::Error;

use ears::{Sound, AudioController};
use rand::Rng;

use loopscope::shapes::Shape;
use loopscope::constants::{DOT, DASH, PAUSE};
use loopscope::operations::*;

///
/// initialize basic shapes
fn init_shapes() -> (Shape, Shape, Shape) {
    let sqr: Shape = Shape::square();
    let trn: Shape = Shape::triangle();
    let pnt: Shape = Shape::pentagon();

    (sqr, trn, pnt)
}

///
/// initialize available operations
fn init_operations() -> [&'static Operation<'static>; 5] {
    // create an array of references that points to Operation
    // constants
    use loopscope::operations::{Stacking, Sequencing, Stretching,
         Interleaving, Threading};
    [
        &Stacking,
        &Sequencing,
        &Stretching,
        &Interleaving,
        &Threading,
    ]
}

///
/// run an operation over a array N-2 of patterns
fn apply_operation(operation: &Operation, patterns: &[Vec<char>; 2]) {
    let func = operation.func;
    func(patterns)
}

fn main() {
    // available shapes
    let (sqr, trn, pnt) = init_shapes();

    // available operations
    let operations = init_operations();

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

    println!("{:?}", operations[2].symbol);

    apply_operation(operations[2], &patterns);


}
