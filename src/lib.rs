

use std::vec;

use egui::{Vec2, vec2};
use eframe::epi::App;


#[derive(Default)]
pub struct Event {
    input: String,
    total: String,
    positive: bool,
}

impl Event {
    pub fn new() -> Self {
        Event {
            input: Default::default(),
            total: "0".to_string(),
            positive: true
        }
    }

    pub fn reset(&mut self) {
        self.input = Default::default();
        self.total = "0".to_string();
        self.positive = true
    }

    pub fn calc(&mut self, s: &str) {
        match s {
            "+" => self.plus(),
            "-" => self.minus(),
            "X" => self.prod(),
            _ => self.divide(),
        };
        self.input = Default::default();
    }

    fn plus(&mut self) {
        let r = self.total.parse::<f64>().expect("cant convert to number") + self.input.parse::<f64>().expect("cant convert to number");
        self.total = r.to_string();
    }

    fn minus(&mut self) {
        let r = self.total.parse::<f64>().expect("cant convert to number") - self.input.parse::<f64>().expect("cant convert to number");
        self.total = r.to_string();
    }

    fn prod(&mut self) {
        let r = self.total.parse::<f64>().expect("cant convert to number") * self.input.parse::<f64>().expect("cant convert to number");
        self.total = r.to_string();
    }

    fn divide(&mut self) {
        let r = self.total.parse::<f64>().expect("cant convert to number") / self.input.parse::<f64>().expect("cant convert to number");
        self.total = r.to_string();
    }
}

impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(vec2(ui.available_width(), 50.0), |ui| {
                let _screen = ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.input));
                ui.label(format!("Total = {}", &self.total));
            });

            ui.columns(4, |ui| {
                ui[0].vertical_centered_justified(|ui| {
                    let seven = ui.add_sized(vec2(50.0,50.0), egui::Button::new("7"));
                    let four = ui.add_sized(vec2(50.0,50.0), egui::Button::new("4"));
                    let one = ui.add_sized(vec2(50.0,50.0), egui::Button::new("1"));
                    let sign = ui.add_sized(vec2(50.0,50.0), egui::Button::new("+/-"));
                    if seven.clicked() {self.input.push('7')};
                    if four.clicked() {self.input.push('4')};
                    if one.clicked() {self.input.push('1')};
                    if sign.clicked() {if self.positive {self.positive = false} else {self.positive = true}};
                });
                ui[1].vertical_centered_justified(|ui| {
                    let eight = ui.add_sized(vec2(50.0,50.0), egui::Button::new("8"));
                    let five = ui.add_sized(vec2(50.0,50.0), egui::Button::new("5"));
                    let two = ui.add_sized(vec2(50.0,50.0), egui::Button::new("2"));
                    let zero = ui.add_sized(vec2(50.0,50.0), egui::Button::new("0"));
                    if eight.clicked() {self.input.push('8')};
                    if five.clicked() {self.input.push('5')};
                    if two.clicked() {self.input.push('2')};
                    if zero.clicked() {self.input.push('0')};
                });
                ui[2].vertical_centered_justified(|ui| {
                    let nine = ui.add_sized(vec2(50.0,50.0), egui::Button::new("9"));
                    let six = ui.add_sized(vec2(50.0,50.0), egui::Button::new("6"));
                    let three = ui.add_sized(vec2(50.0,50.0), egui::Button::new("3"));
                    let point = ui.add_sized(vec2(50.0,50.0), egui::Button::new("."));
                    if nine.clicked() {self.input.push('9')};
                    if six.clicked() {self.input.push('6')};
                    if three.clicked() {self.input.push('3')};
                    if point.clicked() {if !self.input.contains('.') {self.input.push('.')}};
                });
                ui[3].vertical_centered_justified(|ui| {
                    let plus = ui.add_sized(vec2(50.0,50.0), egui::Button::new("+"));
                    let minus = ui.add_sized(vec2(50.0,50.0), egui::Button::new("-"));
                    let multiply = ui.add_sized(vec2(50.0,50.0), egui::Button::new("X"));
                    let divide = ui.add_sized(vec2(50.0,50.0), egui::Button::new("/"));
                    if plus.clicked() {self.calc("+")};
                    if minus.clicked() {self.calc("-")};
                    if multiply.clicked() {self.calc("X")};
                    if divide.clicked() {self.calc("/")};
                });
            });
            ui.allocate_ui(vec2(ui.available_width(), 50.0), |ui| {
                ui.columns(2, |ui| {
                    let clear = ui[0].add_sized(vec2(100.0, 50.0), egui::Button::new("C"));
                    let equal = ui[1].add_sized(vec2(100.0, 50.0), egui::Button::new("="));
                    if clear.clicked() {self.reset();}
                })
            });
        });
    }

    fn name(&self) -> &str {
        "awesome calculator"
    }
}


#[cfg(test)]
mod tests {
    #[test] 
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
