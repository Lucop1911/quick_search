mod gui;
mod utils;

use eframe::egui;
use named_lock::NamedLock;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "--v" => {
                println!("{}", VERSION);
                return Ok(());
            }
            "--history" => {
                return run_history_window();
            }
            "--settings" => {
                return run_settings_window();
            }
            "--info" => {
                return run_info_window();
            }
            "--force" => {
                return run_main_window(None);
            }
            _ => {}
        }
    }

    let lock = NamedLock::create("quick_search_single_instance")?;
    let guard = match lock.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Another instance is already running.");
            eprintln!("If you're sure no other instance is running, use: --force");
            return Ok(());
        }
    };

    run_main_window(Some(guard))
}

fn run_main_window(_guard: Option<named_lock::NamedLockGuard>) -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 130.0])
            .with_decorations(false)
            .with_transparent(false)
            .with_resizable(false)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Quick Search",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::search_bar::QuickSearchApp::new(cc)))),
    )?;

    Ok(())
}

fn run_history_window() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_resizable(true)
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Search History",
        native_options,
        Box::new(move |cc| {
            Ok(Box::new(gui::history::HistoryApp::new(cc)))
        }),
    )?;
    
    Ok(())
}

fn run_settings_window() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_resizable(true)
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Settings",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(gui::settings::SettingsApp::new(cc)))
        }),
    )?;
    
    Ok(())
}

fn run_info_window() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 550.0])
            .with_resizable(true)
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "About Quick Search",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(gui::info::InfoApp::new(cc)))
        }),
    )?;
    
    Ok(())
}