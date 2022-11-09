#![warn(clippy::all, clippy::nursery, rust_2018_idioms)]

use nannou::prelude::*;
use nannou::rand::{self, prelude::ThreadRng, Rng};

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    poly: Vec<Vec2>,
    points: Vec<Point2>,
    angle: f32,
    color: Hsla,
    number_of_sides: usize,
    radius: f32,
}

fn corner(i: usize, radius: f32, number_of_sides: usize) -> Vec2 {
    let fract = i as f32 / number_of_sides as f32;
    let radian = TAU * fract;
    let x = radian.sin() * radius;
    let y = radian.cos() * radius;
    pt2(x, y)
}

fn poly_points(number_of_sides: usize, radius: f32) -> Vec<Vec2> {
    let mut poly: Vec<Vec2> = (0..number_of_sides)
        .map(|i| corner(i, radius, number_of_sides))
        .collect();
    poly.push(corner(0, radius, number_of_sides));
    poly
}

fn gen_line_points(step: f32, poly: &[Vec2]) -> Vec<Vec2> {
    let mut points = Vec::new();
    let mut ratio = step;

    while ratio < 1.0 {
        let mut previous_corner = None;
        let mut first_corner = Vec2::ZERO;

        for corner in poly {
            if previous_corner.is_none() {
                previous_corner = Some(corner);
                first_corner = *corner;
                continue;
            }

            let direction = *corner - *previous_corner.unwrap();

            points.push(*previous_corner.unwrap() + direction * ratio);
            previous_corner = Some(corner);
        }

        points.push(first_corner);
        ratio += step;
    }

    points
}

impl Thing {
    fn new(radius: f32, number_of_sides: usize, step: f32, angle: f32, color: Hsla) -> Self {
        let poly = poly_points(number_of_sides, radius);
        let points = gen_line_points(step, &poly);
        Self {
            poly,
            points,
            angle,
            color,
            number_of_sides,
            radius,
        }
    }

    fn set_angle(&mut self, new_angle: f32) {
        self.angle = new_angle;
    }

    fn update(&mut self, radius: f32, step: f32) {
        self.poly = poly_points(self.number_of_sides, radius);
        self.points = gen_line_points(step, &self.poly);
        self.radius = radius;
    }

    fn draw(&self, draw: &Draw) {
        draw.polyline()
            .join_round()
            .points(self.poly.clone())
            .rotate(self.angle)
            .color(self.color);
        draw.polyline()
            .join_round()
            .points(self.points.clone())
            .rotate(self.angle)
            .color(self.color);
    }
}

struct Settings {
    radius: f32,
    side_change: u64,
    min_sides: usize,
    max_sides: usize,
    step: f32,
    layer: u64,
}
struct Model {
    things: Vec<Thing>,
    settings: Settings,
    rng: ThreadRng,
}

const SIZE: usize = 500;

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    bleh::key_pressed(
        app,
        &mut model.settings.radius,
        &mut model.settings.step,
        key,
    );
}

fn new_things(radius: f32, step: f32, min_sides: usize, max_sides: usize) -> Vec<Thing> {
    let rand_angle = random::<f32>();
    (min_sides..max_sides)
        .map(|x| {
            Thing::new(
                radius,
                x,
                step,
                TAU * rand_angle * x as f32 % TAU,
                hsla(map_range(x, min_sides, max_sides, 0.1, 0.9), 0.6, 0.6, 0.6),
            )
        })
        .collect()
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("centroid")
        .size(SIZE as u32 + 100, SIZE as u32 + 100)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let step = 0.1;
    let radius = SIZE as f32 / 2.5;
    let settings = Settings {
        radius,
        side_change: 20,
        min_sides: 3,
        max_sides: 7,
        step,
        layer: 10,
    };

    let things = new_things(
        settings.radius,
        settings.step,
        settings.min_sides,
        settings.max_sides,
    );
    Model {
        things,
        settings,
        rng: rand::thread_rng(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let Model {
        ref mut settings,
        ref mut things,
        ref mut rng,
    } = *model;
    for thing in things {
        thing.set_angle(TAU.mul_add(rng.gen_range(0.0..0.05), thing.angle));
        if app.elapsed_frames() % settings.side_change == 0 {
            thing.update(
                thing.radius + rng.gen_range(-10.0..10.0),
                1.0 / rng.gen_range(4..20) as f32,
            );
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame<'_>) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }

    if app.elapsed_frames() % model.settings.layer == 0 {
        draw.rect()
            .wh(app.window_rect().wh())
            .color(rgba(0.0, 0.0, 0.0, 0.2));
    } else {
        draw.rect().wh(app.window_rect().wh()).color(srgba(
            0.0,
            0.0,
            0.0,
            0.02 * model.settings.layer as f64,
        ));
    }

    for thing in &model.things {
        thing.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
