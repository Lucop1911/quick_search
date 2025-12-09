mod gui;
mod utils;

use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Quick Search started.");
    
    let should_show = Arc::new(Mutex::new(true)); // Start visible
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 65.0]) 
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_position([660.0, 20.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Quick Search",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::QuickSearchApp::new(cc, should_show)))),
    )?;
    
    Ok(())
}