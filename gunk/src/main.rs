#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame<'_>) {
    let t = app.time;
    let win = app.window_rect();
    let mid_to_corner = win.xy().distance(win.top_right());
    let pi2 = PI * 2.0;

    let draw = app.draw();

    draw.background().color(BLACK);

    let rad_hz = 0.1;
    let a_hz = 0.3;
    let n = 20;
    for i in 0..n {
        let f = i as f32 / n as f32;
        let t = (0.3 * t).sin().mul_add(0.5, t);
        let rad = (t.mul_add(rad_hz, f) % 1.0) * mid_to_corner;
        let a = (t.mul_add(a_hz, f) * pi2).sin().mul_add(0.1, 0.11);
        let a_fade_in = rad / mid_to_corner;
        let a_fade_out = (mid_to_corner - rad) / mid_to_corner;
        let hue = f.mul_add(3.0, t) % 1.0;
        let sat = 0.5;
        let lum = 0.5;
        draw.ellipse().resolution(200.0).radius(rad).hsla(
            hue,
            sat,
            lum,
            a_fade_in.powi(i) * a * a_fade_out,
        );
    }

    let rect = Rect::from_w_h(200.0, 200.0);
    for (j, r) in rect.subdivisions_iter().enumerate() {
        let jf = j as f32 / 4.0;
        for (i, r) in r.subdivisions_iter().enumerate() {
            let f = jf + i as f32 / 16.0;
            let offset = win.w() * 0.1;
            let x = (t + f % 1.0).sin().mul_add(offset, r.x());
            let y = (1.0 + t - f % 1.0).cos().mul_add(offset, r.y());
            let p = pt2(x, y) * 1.2;
            let from_mid = win.xy().distance(p);
            let pad = rect.w() * 0.5 * (from_mid / mid_to_corner);
            let r = r.pad(pad);
            let hue = f.mul_add(0.3, t) % 1.0;
            let sat = 0.5;
            let lum = 0.5;
            let a = 1.0;
            draw.rect().xy(p).wh(r.wh()).hsla(hue, sat, lum, a);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
