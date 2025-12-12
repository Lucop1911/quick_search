use eframe::egui;
use egui::{Align, Color32, CornerRadius, Frame, Margin, ScrollArea, TextEdit};
use crate::utils::settings_manager::{Settings, SettingsManager};

const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);

pub struct SettingsApp {
    settings: Settings,
    settings_manager: SettingsManager,
}

impl SettingsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.load_settings();
        
        Self {
            settings,
            settings_manager,
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
                fill: Color32::from_rgb(18, 18, 22),
                corner_radius: CornerRadius::same(12),
                inner_margin: Margin::same(20),
                stroke: egui::Stroke::new(1.5, Color32::from_rgb(60, 60, 70)),
                ..Frame::default()
            })
            .show(ctx, |ui| {
                // Header (No change)
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("⚙️  Settings")
                        .size(24.0)
                        .color(BLUE_HIGHLIGHT)
                        .strong());
                    
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(egui::Button::new(egui::RichText::new("✕").size(18.0))
                            .fill(Color32::from_rgb(60, 30, 30))
                            .stroke(egui::Stroke::NONE))
                            .on_hover_text("Close")
                            .clicked() 
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(16.0);

                // Centered scrollable content
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            let mut settings_changed = false;
                            
                            // 1. Search Features Frame (No Change)
                            // ... (Existing code for Search Features here)
                            Frame {
                                fill: Color32::from_rgb(28, 28, 32),
                                corner_radius: CornerRadius::same(8),
                                inner_margin: Margin::same(16),
                                stroke: egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
                                ..Frame::default()
                            }
                            .show(ui, |ui| {
                                ui.set_max_width(520.0);
                                
                                ui.label(egui::RichText::new("Search Features")
                                    .size(15.0)
                                    .strong()
                                    .color(BLUE_HIGHLIGHT));
                                ui.add_space(12.0);

                                // Application Search
                                Self::render_setting_item(ui, &mut settings_changed, 
                                    &mut self.settings.enable_app_search,
                                    "🔍  Application Search",
                                    "Search for installed applications on your system"
                                );

                                // Web Search
                                Self::render_setting_item(ui, &mut settings_changed,
                                    &mut self.settings.enable_web_search,
                                    "🌐  Web Search",
                                    "Fallback to web search when no local results found"
                                );

                                // Math Evaluation
                                Self::render_setting_item(ui, &mut settings_changed,
                                    &mut self.settings.enable_math_eval,
                                    "🔢  Math Evaluation",
                                    "Evaluate mathematical expressions directly"
                                );

                                // File Search
                                Self::render_setting_item(ui, &mut settings_changed,
                                    &mut self.settings.enable_file_search,
                                    "📁  File & Folder Search",
                                    "Search for files and folders by path"
                                );

                                // History
                                Self::render_setting_item(ui, &mut settings_changed,
                                    &mut self.settings.enable_history,
                                    "📜  Search History",
                                    "Save and access your search history"
                                );
                            });

                            ui.add_space(20.0);
                            
                            // 2. NEW: Editor & Terminal Settings Frame
                            Frame {
                                fill: Color32::from_rgb(28, 28, 32),
                                corner_radius: CornerRadius::same(8),
                                inner_margin: Margin::same(16),
                                stroke: egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
                                ..Frame::default()
                            }
                            .show(ui, |ui| {
                                ui.set_max_width(520.0);
                                
                                ui.label(egui::RichText::new("Editor & Terminal")
                                    .size(15.0)
                                    .strong()
                                    .color(BLUE_HIGHLIGHT));
                                ui.add_space(12.0);
                                
                                Self::render_input_setting(ui, &mut settings_changed,
                                    &mut self.settings.terminal_command,
                                    "💻  Terminal Command",
                                    "Command to launch your preferred terminal emulator (e.g., 'alacritty', 'kitty', 'gnome-terminal')"
                                );
                                
                                Self::render_input_setting(ui, &mut settings_changed,
                                    &mut self.settings.text_editor_command,
                                    "✏️  Default Text Editor",
                                    "Command to launch your preferred text editor (e.g., 'xed', 'nvim', 'subl'). Takes priority over system detection."
                                );
                            });

                            if settings_changed {
                                self.save_settings();
                            }

                            ui.add_space(20.0);

                            // Info footer (No change)
                            Frame {
                                fill: Color32::from_rgba_unmultiplied(40, 50, 70, 100),
                                corner_radius: CornerRadius::same(6),
                                inner_margin: Margin::symmetric(16, 10),
                                ..Frame::default()
                            }
                            .show(ui, |ui| {
                                ui.set_max_width(520.0);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("💾").size(14.0));
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new("Settings are saved automatically")
                                        .size(11.5)
                                        .color(Color32::from_rgb(160, 170, 190)));
                                });
                            });

                            ui.add_space(16.0);
                        });
                    });

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

        ctx.request_repaint();
    }
}

impl SettingsApp {
    fn render_setting_item(
        ui: &mut egui::Ui,
        settings_changed: &mut bool,
        setting: &mut bool,
        title: &str,
        description: &str,
    ) {
        Frame {
            fill: Color32::from_rgba_unmultiplied(35, 35, 42, 200),
            corner_radius: CornerRadius::same(6),
            inner_margin: Margin::symmetric(14, 12),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let checkbox_response = ui.checkbox(setting, "");
                *settings_changed |= checkbox_response.changed();
                
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(egui::RichText::new(title)
                        .size(13.5)
                        .strong()
                        .color(Color32::from_rgb(220, 220, 235)));
                    ui.label(egui::RichText::new(description)
                        .size(11.5)
                        .color(Color32::from_rgb(150, 150, 165)));
                });
            });
        });
        ui.add_space(8.0);
    }

    fn render_input_setting(
        ui: &mut egui::Ui,
        settings_changed: &mut bool,
        setting_value: &mut String,
        title: &str,
        description: &str,
    ) {
        Frame {
            fill: Color32::from_rgba_unmultiplied(35, 35, 42, 200),
            corner_radius: CornerRadius::same(6),
            inner_margin: Margin::symmetric(14, 12),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(egui::RichText::new(title)
                    .size(13.5)
                    .strong()
                    .color(Color32::from_rgb(220, 220, 235)));
                ui.label(egui::RichText::new(description)
                    .size(11.5)
                    .color(Color32::from_rgb(150, 150, 165)));
                
                ui.add_space(8.0);

                let text_edit = TextEdit::singleline(setting_value)
                    .desired_width(f32::INFINITY)
                    .frame(true)
                    .hint_text("Enter command name here (e.g., 'code')");
                
                let response = ui.add(text_edit);
                *settings_changed |= response.changed();
            });
        });
        ui.add_space(8.0);
    }
}