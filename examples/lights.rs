#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::color::IntoColor;
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
    let rect = app.window(window).unwrap().rect();

    let positions = ((rect.x.start as i32)..(rect.x.end as i32))
        .flat_map(|x| ((rect.y.start as i32)..(rect.y.end as i32)).map(move |y| (x, y)));
    let points = positions
        .map(|(x, y)| Point {
            position: vec2(x as f32 + 0.5, y as f32 + 0.5),
            color: rgba(0., 0., 0., 1.),
        })
        .collect();

    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let wh = app.main_window().rect().wh();
    // let mut c = vec3(0., 0., 0.);
    // let mut t = app.time;
    // let mut l = 0.;

    // for Point { position, color } in model.points.iter_mut() {
    //     for i in 0..3 {
    //         let mut p = (*position + wh / 2.) / wh;
    //         p -= 0.5;
    //         p.x *= wh.x / wh.y;
    //         t += 0.07;
    //         l = p.length();
    //         let uv = p + (p / l * (t.sin() + 1.) * (l * 9. - 2. * t).sin().abs());
    //         c[i] = 0.01 / (vec2(uv.x.rem_euclid(1.), uv.y.rem_euclid(1.)) - 0.5).length();
    //     }

    //     let rgb = c / l;
    //     *color = rgba(rgb.x, rgb.y, rgb.z, 1.0);
    // }

    // for Point { position, color } in model.points.iter_mut() {
    //     let uv = (*position + wh / 2.) / wh;
    //     *color = rgba(uv.x, uv.y, 0., 1.);
    // }

    for Point { position, color } in model.points.iter_mut() {
        let st = (*position + wh / 2.) / wh;
        let rgb = hsv(st.x, 1.0, st.y).into_rgb::<nannou::color::encoding::Srgb>();
        *color = rgba(rgb.red, rgb.green, rgb.blue, 1.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame<'_>) {
    let draw = app.draw();

    draw.polyline()
        .weight(app.main_window().scale_factor())
        .points_colored(
            model
                .points
                .iter()
                .map(|&Point { position, color }| (vec2(position.x, position.y), color)),
        );

    draw.to_frame(app, &frame).unwrap();
}
