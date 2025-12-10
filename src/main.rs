mod gui;
mod utils;

use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    // Detect if this instance is a temporary window (spawned by the hotkey listener).
    let _is_temp_window = args.iter().any(|a| a == "--temp-window");
    // Check for special command-line arguments
    if args.len() > 1 {
        match args[1].as_str() {
            "--history" => {
                return run_history_window();
            }
            "--settings" => {
                return run_settings_window();
            }
            "--info" => {
                return run_info_window();
            }
            _ => {}
        }
    }
    
    // Run main search window
    println!("Quick Search started.");

    #[cfg(not(target_os = "windows"))]
    {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 130.0])
                .with_decorations(false)
                .with_transparent(true)
                .with_resizable(false)
                .with_position([600.0, 20.0])
                .with_always_on_top(),
            ..Default::default()
        };

        eframe::run_native(
            "Quick Search",
            native_options,
            Box::new(|cc| Ok(Box::new(gui::search_bar::QuickSearchApp::new(cc)))),
        )?;
    }

    #[cfg(target_os = "windows")]
    {  
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 60.0])
                .with_decorations(false)
                .with_transparent(true)
                .with_resizable(false)
                .with_always_on_top(),
            ..Default::default()
        };

        eframe::run_native(
            "Quick Search",
            native_options,
            Box::new(|cc| Ok(Box::new(gui::search_bar::QuickSearchApp::new(cc)))),
        )?;
    }

    #[cfg(target_os = "windows")]
    {
        if !_is_temp_window {
            println!("Quick Search entering background listener (Windows). Press Alt+S to open the search bar.");
            // loop for the ALT+S hotkey
            crate::utils::hotkey_listener::start_hotkey_listener();
        }
    }

    Ok(())
}

fn run_history_window() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_resizable(true)
            .with_position([500.0, 200.0])
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
            .with_position([500.0, 200.0])
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
            .with_position([500.0, 200.0])
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