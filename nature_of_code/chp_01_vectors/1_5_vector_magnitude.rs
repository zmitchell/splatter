// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 1-5: Vector Magnitude
use splatter::prelude::*;

fn main() {
    splatter::sketch(view).size(640, 360).run();
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    let mut mouse = vec2(app.mouse.x, app.mouse.y);
    let center = vec2(0.0, 0.0);
    mouse -= center;

    let m = mouse.length();

    let r = Rect::from_w_h(m, 10.0).top_left_of(win);
    draw.rect().xy(r.xy()).wh(r.wh()).color(BLACK);

    draw.line().weight(2.0).color(BLACK).points(center, mouse);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
