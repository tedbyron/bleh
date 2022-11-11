#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame<'_>) {
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time as f32;
    let diagonal = win.top_left().distance(win.bottom_right());

    draw.background().color(BLACK);

    let n = (0.1 * t * 2.0 * PI).sin().mul_add(100.0, 100.0) as usize;

    for i in 0..n {
        let f = i as f32 / n as f32;
        let max_weight = (1.0 / n as f32) * win.w();
        let x = win.x.lerp(f);
        let hz = 0.125;
        let tx = (t * hz * 2.0 * PI).sin() * win.right();
        let d = (tx - x).abs();
        let dn = d / win.w();
        let weight = max_weight * dn;

        // linear
        // let hue = 1.0;
        // let pa = pt2(x, win.top());
        // let pb = pt2(x, win.bottom());

        // radial
        let hue = t.mul_add(0.1, dn * 0.3);
        let rad = t.mul_add(0.05, f) * 2.0 * PI;
        let mag = diagonal;
        let pa = pt2(rad.cos() * mag, rad.sin() * mag);
        let pb = pt2(rad.cos() * -mag, rad.sin() * -mag);

        draw.line()
            .weight(weight)
            .points(pa, pb)
            .hsla(hue, 1.0, 1.0, dn);
    }

    draw.to_frame(app, &frame).unwrap();
}
