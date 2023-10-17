use std::ops::Deref;
use splatter::prelude::*;
use splatter_egui::{Egui};

fn main() {
    splatter::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    egui_demo_app: egui_demo_lib::DemoWindows,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let w_id = app
        .new_window()
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();
    let mut egui = Egui::from_window(&window);
    let mut egui_demo_app = egui_demo_lib::DemoWindows::default();
    let proxy = app.create_proxy();
    Model {
        egui,
        egui_demo_app,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &splatter::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut egui_demo_app,
        ..
    } = *model;
    egui.set_elapsed_time(update.since_start);
    let proxy = app.create_proxy();
    egui.begin_frame();
    egui_demo_app.ui(egui.ctx());
}

fn view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}
