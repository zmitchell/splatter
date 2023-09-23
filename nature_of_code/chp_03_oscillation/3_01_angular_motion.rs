// The Nature Of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 3-01: Angular Motion(using rotate())

use splatter::prelude::*;

fn main() {
    splatter::app(model).update(update).run();
}

struct Model {
    angle: f32,
    angle_velocity: f32,
    angle_acceleration: f32,
}

fn model(app: &App) -> Model {
    let angle = 0.0;
    let angle_velocity = 0.0;
    let angle_acceleration = -0.0001;
    app.new_window().size(800, 200).view(view).build().unwrap();
    Model {
        angle,
        angle_velocity,
        angle_acceleration,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += model.angle_velocity;
    model.angle_velocity += model.angle_acceleration;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.rect().wh(app.window_rect().wh()).color(WHITE);

    draw.line()
        .start(pt2(-60.0, 0.0))
        .end(pt2(60.0, 0.0))
        .color(BLACK)
        .stroke_weight(2.0)
        .rotate(model.angle);

    draw.ellipse()
        .xy(pt2(60.0, 0.0).rotate(model.angle))
        .w_h(16.0, 16.0)
        .gray(0.5)
        .stroke_weight(2.0)
        .stroke_color(BLACK);

    draw.ellipse()
        .xy(pt2(-60.0, 0.0).rotate(model.angle))
        .w_h(16.0, 16.0)
        .gray(0.5)
        .stroke_weight(2.0)
        .stroke_color(BLACK);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
