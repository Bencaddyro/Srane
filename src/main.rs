use gui::MyEguiApp;

mod config;
mod gpu;
mod gui;
mod simulation;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Srane Render",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}
