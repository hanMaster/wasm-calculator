//#![cfg_attr(not(debug_assertions), windows_subsystem = "macos")]

use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()>  {
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

struct CalcApp {}

impl CalcApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {}
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(r#"
            Это вымышленный калькулятор.
            Чтобы воспользоваться калькулятором вам необходимо воспользоваться воображением.
            Представте себе любой интерфейс, наберите в нем математическое выражение и нажмите '='.
            Увидели результат, Да? - Поздравляю, ваш калькулятор работает хорошо.
            "#);
        });
    }
}