use nannou::prelude::*;

pub fn view(app: &App, frame: Frame<'_>) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(SLATEBLUE);

    let win_p = win.pad(25.0);
    let square = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);

    draw.rect().xy(square.xy()).wh(square.wh()).color(PLUM);

    let circle = square.below(square);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    draw.polygon()
        .points_colored((0..=360).step_by(45).map(|i| {
            let rad = deg_to_rad(i as f32 + 22.5);
            let x = rad.sin() * 150.0;
            let y = rad.cos() * 150.0;
            (pt2(x, y), STEELBLUE)
        }));

    draw.polyline().weight(3.0).points_colored((0..50).map(|i| {
        let x = i as f32 - 25.0;
        let point = pt2(x, x.sin()) * 20.0;
        (point, DEEPPINK)
    }));

    draw.ellipse().color(INDIGO);

    draw.to_frame(app, &frame).unwrap();
}
