use eframe::egui;
use egui::{Frame, CornerRadius, Color32, Margin, ScrollArea};
use crate::utils::settings_manager::{Settings, SettingsManager};

#[cfg(target_os = "windows")]
use egui::{KeyboardShortcut, Key, Modifiers};

const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);

pub struct SettingsApp {
    settings: Settings,
    settings_manager: SettingsManager,
    
    #[cfg(target_os = "windows")]
    is_recording_shortcut: bool,
    #[cfg(target_os = "windows")]
    temp_shortcut: Option<KeyboardShortcut>,
}

impl SettingsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.load_settings();
        
        Self {
            settings,
            settings_manager,
            
            #[cfg(target_os = "windows")]
            is_recording_shortcut: false,
            #[cfg(target_os = "windows")]
            temp_shortcut: None,
        }
    }
    
    fn save_settings(&mut self) {
        if let Err(e) = self.settings_manager.save_settings(&self.settings) {
            eprintln!("Failed to save settings: {}", e);
        }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgba_unmultiplied(20, 20, 24, 250),
                corner_radius: CornerRadius::same(8),
                inner_margin: Margin::same(16),
                stroke: egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(60, 60, 70, 180)),
                ..Frame::default()
            })
            .show(ctx, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("⚙️ Settings")
                        .size(20.0)
                        .color(BLUE_HIGHLIGHT));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("✕").size(16.0)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ScrollArea::vertical()
                    .max_height(500.0)
                    .show(ui, |ui| {
                        // Search Features Section
                        let mut settings_changed = false;
                        
                        Self::render_section_static(ui, "Search Features", |ui| {
                            settings_changed |= ui.checkbox(&mut self.settings.enable_app_search, "Enable Application Search")
                                .on_hover_text("Search for installed applications")
                                .changed();
                            ui.add_space(4.0);
                            
                            settings_changed |= ui.checkbox(&mut self.settings.enable_web_search, "Enable Web Search")
                                .on_hover_text("Fallback to web search when no results found")
                                .changed();
                            ui.add_space(4.0);
                            
                            settings_changed |= ui.checkbox(&mut self.settings.enable_math_eval, "Enable Math Evaluation")
                                .on_hover_text("Evaluate mathematical expressions")
                                .changed();
                            ui.add_space(4.0);
                            
                            settings_changed |= ui.checkbox(&mut self.settings.enable_file_search, "Enable File/Folder Search")
                                .on_hover_text("Search for files and folders by path")
                                .changed();
                            ui.add_space(4.0);
                            
                            settings_changed |= ui.checkbox(&mut self.settings.enable_history, "Enable Search History")
                                .on_hover_text("Save searches to history")
                                .changed();
                        });
                        
                        if settings_changed {
                            self.save_settings();
                        }

                        ui.add_space(12.0);

                        // Keyboard Shortcuts Section
                        #[cfg(target_os = "windows")]
                        {
                            Self::render_section_static(ui, "Keyboard Shortcuts", |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Open Quick Search:");
                                    ui.add_space(8.0);
                                    
                                    let current_shortcut = self.settings.open_shortcut.to_keyboard_shortcut();
                                    let shortcut_text = if self.is_recording_shortcut {
                                        "Press keys...".to_string()
                                    } else {
                                        self.format_shortcut(&current_shortcut)
                                    };
                                    
                                    let button = ui.button(
                                        egui::RichText::new(&shortcut_text)
                                            .monospace()
                                            .color(if self.is_recording_shortcut {
                                                Color32::from_rgb(255, 200, 100)
                                            } else {
                                                BLUE_HIGHLIGHT
                                            })
                                    );
                                    
                                    if button.clicked() {
                                        self.is_recording_shortcut = true;
                                        self.temp_shortcut = None;
                                    }
                                    
                                    if self.is_recording_shortcut {
                                        ui.label(
                                            egui::RichText::new("(Press Esc to cancel)")
                                                .size(10.0)
                                                .color(Color32::from_rgb(150, 150, 160))
                                        );
                                    }
                                });
                                
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new("⚠️ Note: Changing the shortcut requires restarting the application")
                                        .size(10.0)
                                        .color(Color32::from_rgb(200, 150, 100))
                                );
                                
                                // Handle shortcut recording
                                if self.is_recording_shortcut {
                                    self.handle_shortcut_recording(ctx, ui);
                                }
                            });
                        }

                        ui.add_space(24.0);

                        // Info
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("💾 Settings are saved automatically")
                                    .size(11.0)
                                    .color(Color32::from_rgb(120, 120, 130))
                            );
                        });
                    });

                // Keyboard shortcuts
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    #[cfg(target_os = "windows")]
                    if self.is_recording_shortcut {
                        self.is_recording_shortcut = false;
                        self.temp_shortcut = None;
                    } else {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    
                    #[cfg(not(target_os = "windows"))]
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

        ctx.request_repaint();
    }
}

impl SettingsApp {
    fn render_section_static<R>(
        ui: &mut egui::Ui,
        title: &str,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        Frame {
            fill: Color32::from_rgba_unmultiplied(30, 30, 35, 200),
            corner_radius: CornerRadius::same(6),
            inner_margin: Margin::same(12),
            stroke: egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(50, 50, 60, 150)),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.label(egui::RichText::new(title)
                .size(14.0)
                .strong()
                .color(BLUE_HIGHLIGHT));
            ui.add_space(8.0);
            content(ui)
        })
        .inner
    }
    
    #[cfg(target_os = "windows")]
    fn format_shortcut(&self, shortcut: &KeyboardShortcut) -> String {
        let mut parts = Vec::new();
        
        if shortcut.modifiers.ctrl {
            parts.push("Ctrl");
        }
        if shortcut.modifiers.alt {
            parts.push("Alt");
        }
        if shortcut.modifiers.shift {
            parts.push("Shift");
        }
        if shortcut.modifiers.command {
            parts.push("Cmd");
        }
        
        parts.push(&format!("{:?}", shortcut.logical_key));
        
        parts.join(" + ")
    }
    
    #[cfg(target_os = "windows")]
    fn handle_shortcut_recording(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Capture any key press
        ctx.input(|i| {
            let mut new_mods = Modifiers::NONE;
            
            if i.modifiers.ctrl {
                new_mods |= Modifiers::CTRL;
            }
            if i.modifiers.alt {
                new_mods |= Modifiers::ALT;
            }
            if i.modifiers.shift {
                new_mods |= Modifiers::SHIFT;
            }
            if i.modifiers.command {
                new_mods |= Modifiers::COMMAND;
            }
            
            // Check for any letter key press
            for key in [
                Key::A, Key::B, Key::C, Key::D, Key::E, Key::F, Key::G, Key::H,
                Key::I, Key::J, Key::K, Key::L, Key::M, Key::N, Key::O, Key::P,
                Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X,
                Key::Y, Key::Z, Key::Space,
            ] {
                if i.key_pressed(key) {
                    // Require at least one modifier
                    if new_mods != Modifiers::NONE {
                        let new_shortcut = KeyboardShortcut::new(new_mods, key);
                        self.temp_shortcut = Some(new_shortcut);
                        self.is_recording_shortcut = false;
                        
                        // Save the new shortcut
                        use crate::utils::settings_manager::SerializableShortcut;
                        self.settings.open_shortcut = SerializableShortcut::from_keyboard_shortcut(&new_shortcut);
                        self.save_settings();
                    }
                }
            }
        });
    }
}