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
fn init_shapes() -> Vec<Shape> {
    let mut shapes: Vec<Shape> = Vec::new();

    shapes.push(Shape::square());
    shapes.push(Shape::triangle());
    shapes.push(Shape::pentagon());

    let shapes = shapes;
    shapes
}

///
/// initialize available operations
fn init_operations() -> Vec<&'static Operation<'static>> {
    // create an array of references that points to Operation
    // constants
    use loopscope::operations::{Stacking, Sequencing, Stretching,
         Interleaving, Threading};
    vec![
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

#[macro_use] extern crate text_io;
fn main() {
    // available shapes
    let shapes = init_shapes();

    // available operations
    let operations = init_operations();

    println!("Choose 2 shapes by sides:");
    println!("| id | name | pattern | sides");
    for (i, s) in shapes.iter().enumerate() {
        println!("{} | {} |", i, s);
    }

    println!("Insert first shape (sides):");
    let sides_first: String = read!("{}\n");
    let sides_first: u8 = sides_first.parse::<u8>().unwrap();

    // https://stackoverflow.com/a/44662455
    let selected_first_sh: Vec<&Shape> = shapes.iter()
        .filter(|sh| sh.sides == sides_first).collect();
    let selected_first_sh = selected_first_sh[0];

    println!("Shape selected: {}", selected_first_sh.name);

    println!("Insert second shape (sides):");
    let sides_second: String = read!("{}\n");
    let sides_second: u8 = sides_second.parse::<u8>().unwrap();

    let selected_second_sh: Vec<&Shape> = shapes.iter()
        .filter(|sh| sh.sides == sides_second).collect();
    let selected_second_sh = selected_second_sh[0];

    println!("Shape selected: {}", selected_second_sh.name);

    println!("Choose by symbol an operation to apply to the shapes:");
    println!("| id | symbol | name ");
    for (i, op) in operations.iter().enumerate() {
        println!("{} | {} |", i, op);
    }

    let operation: String = read!("{}\n");

    let selected_op: Vec<&_> = operations.iter()
        .filter(|op| op.symbol == &operation).collect();
    let selected_op = selected_op[0];

    println!("{} on {} and {}",
        selected_op.name, selected_first_sh.name, selected_second_sh.name);

    let sounds: [&str; 2] = [
        &selected_first_sh.pattern,
        &selected_second_sh.pattern];

    let patterns: [Vec<char>; 2] = [
        sounds[0].chars().collect(),
        sounds[1].chars().collect(),
    ];

    apply_operation(
        selected_op,
        &patterns);
}
