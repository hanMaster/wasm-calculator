#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

use calculator_wasm_rust_pwa::{keyboard, math_exp};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "calculator-wasm-rust-pwa",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(CalcApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(CalcApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct CalcApp {
    math_exp: math_exp::MathExp,
}

impl CalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {
            math_exp: math_exp::MathExp::default(),
        }
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("screen_panel").show(ctx, |ui| {
            let expression = self.math_exp.to_string();
            ui.add_sized(
                [330.0, 70.0],
                egui::Label::new(
                    egui::RichText::new(expression).font(egui::FontId::monospace(25.0)),
                )
                .wrap(true),
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            keyboard::CalcKeyboard::from_buffer(&mut self.math_exp).show(ui)
        });
    }
}
