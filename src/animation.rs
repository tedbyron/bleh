use nannou::prelude::*;

pub fn view(app: &App, frame: Frame<'_>) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let sine = app.time.sin();
    let slow_sine = (app.time / 2.0).sin();
    let bound = app.window_rect();

    let x = map_range(sine, -1.0, 1.0, bound.left(), bound.right());
    let y = map_range(slow_sine, -1.0, 1.0, bound.bottom(), bound.top());

    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
