mod core;
use core::json_handler::{self, load_json, save_json};
use core::app;
use eframe::egui::ViewportBuilder;





fn main() {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    
    eframe::run_native("Todo app", options, Box::new(|cc| {
        Ok(Box::<app::MyApp>::default())
    })).unwrap();

}


