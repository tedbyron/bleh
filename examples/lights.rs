#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::prelude::*;

struct Model {
    points: Vec<Point>,
}

#[derive(Debug)]
struct Point {
    position: Point2,
    color: Rgba,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).size(720, 720).build().unwrap();
    let (w, h) = app.window(window).unwrap().rect().w_h();

    let positions = (0..w as u32).flat_map(|col| (0..h as u32).map(move |row| (row, col)));
    let points = positions
        .map(|coords| Point {
            position: vec2(coords.0 as f32, coords.1 as f32),
            color: rgba(0., 0., 0., 1.),
        })
        .collect();

    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let wh = app.main_window().rect().wh();
    let mut c = vec3(0., 0., 0.);
    let mut t = app.time;
    let mut l = 0.;

    for Point { position, color } in model.points.iter_mut() {
        for i in 0..3 {
            let mut p = *position / wh;
            p -= 0.5;
            p.x *= wh.x / wh.y;
            t += 0.07;
            l = p.length();
            let uv = p + (p / l * (t.sin() + 1.) * (l * 9. - 2. * t).sin().abs());
            c[i] = 0.01 / (vec2(uv.x.rem_euclid(1.), uv.y.rem_euclid(1.)) - 0.5).length();
        }

        let rgb = c / l;
        *color = rgba(rgb.x, rgb.y, rgb.z, 1.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame<'_>) {
    let draw = app.draw();
    let (w, h) = app.main_window().rect().w_h();

    draw.polyline()
        .weight(1.)
        .points_colored(model.points.iter().map(|&Point { position, color }| {
            (vec2(position.x - (w / 2.), position.y - (h / 2.)), color)
        }));

    draw.to_frame(app, &frame).unwrap();
}
