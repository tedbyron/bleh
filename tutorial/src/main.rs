#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

mod animation;

use animation::*;

fn main() {
    nannou::sketch(view).run();
}
