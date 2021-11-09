
use egui::vec2;
use eframe::epi::App;

pub enum Action {
    Plus,
    Minus,
    Divide,
    Multiply,
    None,
}

impl Default for Action {
    fn default() -> Self {
        Action::Plus
    }
}

#[derive(Default)]
pub struct Event {
    input: String,
    total: String,
    action: Action,
}

impl Event {
    pub fn new() -> Self {
        Event {
            input: Default::default(),
            total: Default::default(),
            action: Action::None,
        }
    }

    pub fn reset(&mut self) {
        self.input = Default::default();
        self.total = Default::default();
        self.action = Action::None;
    }

    pub fn calc(&mut self, s: Action) {

        match self.action {
            Action::Plus => self.plus(),
            Action::Minus => self.minus(),
            Action::Multiply => self.prod(),
            Action::Divide => self.divide(),
            Action::None => {self.total = self.input.clone()},
        };
        self.input = Default::default();
        self.action = s;
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

    fn equal(&mut self) {
        match self.action {
            Action::Plus => self.plus(),
            Action::Minus => self.minus(),
            Action::Multiply => self.prod(),
            Action::Divide => self.divide(),
            Action::None => {self.total = self.input.clone()},
        };
        self.input = self.total.clone();
        self.total = Default::default();
        self.action = Action::None;
    }

    fn pos_nev(&mut self) {
        if !self.input.contains('-') {
            self.input.insert(0, '-');
        } else {
            self.input.remove(0);
        }
    }
}

impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
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
                    if sign.clicked() {self.pos_nev()};
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
                    if plus.clicked() {self.calc(Action::Plus);};
                    if minus.clicked() {self.calc(Action::Minus);};
                    if multiply.clicked() {self.calc(Action::Multiply);};
                    if divide.clicked() {self.calc(Action::Divide);};
                });
            });
            ui.allocate_ui(vec2(ui.available_width(), 50.0), |ui| {
                ui.columns(2, |ui| {
                    let clear = ui[0].add_sized(vec2(100.0, 50.0), egui::Button::new("C"));
                    let equal = ui[1].add_sized(vec2(100.0, 50.0), egui::Button::new("="));
                    if clear.clicked() {self.reset();};
                    if equal.clicked() {self.equal();};
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
