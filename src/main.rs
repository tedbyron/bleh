#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]
#![allow(clippy::needless_pass_by_value)]
#![doc = include_str!("../README.md")]

mod animation;

use animation::*;

fn main() {
    nannou::sketch(view).run();
}
