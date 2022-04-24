#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::needless_pass_by_value)]
#![doc = include_str!("../README.md")]

mod shapes;

use shapes::*;

fn main() {
    nannou::app(model).update(update).run();
}
