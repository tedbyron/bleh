#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::needless_pass_by_value)]
#![doc = include_str!("../README.md")]

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let w = app.new_window().view(view).build().unwrap();
    Model { _window: w }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame<'_>) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(SLATEBLUE);
    draw.ellipse().color(STEELBLUE);

    let win_p = win.pad(25.0);
    let square = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);

    draw.rect().xy(square.xy()).wh(square.wh()).color(PLUM);

    let circle = square.below(square);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    draw.to_frame(app, &frame).unwrap();
}
