use eframe::egui;
use eframe::egui::Widget;

use crate::math_exp;

pub struct CustomKey {
    pub text: String,
    pub width: f32,
    pub height: f32,
}

impl CustomKey {
    /// Создать новую кнопку и установить заданный текст.
    pub fn from(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

impl Default for CustomKey {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            width: 58.0,
            height: 48.0,
        }
    }
}

impl Widget for CustomKey {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let btn = ui.add_sized(
            [self.width, self.height],
            egui::Button::new(self.text).small(),
        );
        btn
    }
}

static KEYS: [(&str, &str); 25] = [
    ("√", "√("),
    ("C", ""),
    ("(", "("),
    (")", ")"),
    ("<=", ""),
    ("sin", "sin("),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
    ("*", "*"),
    ("cos", "cos("),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("/", "/"),
    ("tg", "tg("),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("-", "-"),
    ("ctg", "ctg("),
    (".", "."),
    ("0", "0"),
    ("=", ""),
    ("+", "+"),
];
pub struct CalcKeyboard<'a> {
    buffer: &'a mut math_exp::MathExp,
    pub width: f32,
    pub height: f32,
}

impl<'a> CalcKeyboard<'a> {
    pub fn from_buffer(buffer: &'a mut math_exp::MathExp) -> Self {
        Self {
            buffer,
            width: 340.0,
            height: 320.0,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) {
        egui::Grid::new("keyboard")
            .num_columns(5)
            .max_col_width(self.width)
            .show(ui, |ui| {
                for (ind, (title, _)) in KEYS.iter().enumerate() {
                    if ind % 5 == 0 && ind != 0 {
                        ui.end_row();
                    }
                    if CustomKey::from(*title).ui(ui).clicked() {
                        match *title {
                            "C" => {
                                self.buffer.clear();
                            }
                            "<=" => {
                                self.buffer.pop();
                            }
                            "=" => {
                                self.buffer.calculate();
                            }
                            _ => {
                                self.buffer.add(title);
                            }
                        }
                    };
                }
            });
    }
}
