#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

const N_THINGS: usize = 1000;
const SIZE: usize = 500;

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let positions = vec![p];
        Self { positions }
    }
}

struct Parameters {
    speed: f64,
    sn_ratio: f64,
    loops: i32,
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
    parameters: Parameters,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        // Key::S => app
        //     .main_window()
        //     .capture_frame("images/".to_owned() + &app.exe_name().unwrap() + ".png"),
        Key::Up => model.parameters.sn_ratio += 0.001,
        Key::Down if model.parameters.sn_ratio > 0.0 => model.parameters.sn_ratio -= 0.001,
        Key::Right => model.parameters.speed += 1.0,
        Key::Left if model.parameters.speed > 0.0 => model.parameters.speed -= 0.1,
        _ => {}
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("swirly")
        .size(SIZE as u32 + 200, SIZE as u32 + 200)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut things = Vec::new();

    for _ in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * SIZE as f32,
            (random::<f32>() - 0.5) * SIZE as f32,
        ));

        things.push(thing);
    }

    Model {
        things,
        noise: Perlin::new(),
        parameters: Parameters {
            speed: 320.0,
            sn_ratio: 0.01,
            loops: 50,
        },
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f64 / model.parameters.speed;
    let sn = time.sin() as f64 * model.parameters.sn_ratio;

    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * SIZE as f32,
            (random::<f32>() - 0.5) * SIZE as f32,
        ));

        for _ in 0..model.parameters.loops {
            let last_position = thing.positions[0];
            thing.positions.insert(
                0,
                last_position
                    + vec2(
                        model.noise.get([
                            sn * last_position.x as f64,
                            sn * last_position.y as f64,
                            0.0,
                        ]) as f32,
                        model.noise.get([
                            sn * last_position.x as f64,
                            sn * last_position.y as f64,
                            1.0,
                        ]) as f32,
                    ),
            )
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame<'_>) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }

    // draw.rect()
    //     .w_h(SIZE as f32, SIZE as f32)
    //     .color(rgba(0.0, 0.0, 0.0, 0.09));

    draw.rect()
        .wh(app.window_rect().wh())
        .color(rgba(0.0, 0.0, 0.0, 0.1));

    for thing in &model.things {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(SNOW);
    }

    draw.to_frame(app, &frame).unwrap();
}
