use eframe::{NativeOptions, run_native};
use calculator::Event;
use egui::Vec2;

fn main() {
    let app = Event::new();
    let mut native_option = NativeOptions::default();
    native_option.initial_window_size = std::option::Option::Some(Vec2 { x: 300.0, y: 400.0 });
    native_option.resizable = false;

    run_native(Box::new(app), native_option);
}