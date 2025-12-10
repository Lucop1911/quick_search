use eframe::egui;
use egui::{Frame, CornerRadius, Color32, Margin, ScrollArea};

const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);

pub struct SettingsApp {
    // Add settings state here as needed
    // For example:
    // theme: Theme,
    // max_results: usize,
    // enable_history: bool,
    // etc.
    
}

impl SettingsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            // Initialize default settings here
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
                        // Settings content will go here
                        ui.vertical_centered(|ui| {
                            ui.add_space(60.0);
                            ui.label(egui::RichText::new("⚙️")
                                .size(48.0));
                            ui.add_space(12.0);
                            ui.label(egui::RichText::new("Settings Coming Soon")
                                .size(18.0)
                                .color(Color32::from_rgb(240, 240, 245)));
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("This page is reserved for future configuration options")
                                .size(12.0)
                                .color(Color32::from_rgb(150, 150, 160)));
                            ui.add_space(60.0);
                        });

                        // Example settings sections (commented out for now)
                        /*
                        // Appearance Section
                        self.render_section(ui, "Appearance", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Theme:");
                                // Add theme selector
                            });
                        });

                        ui.add_space(12.0);

                        // Behavior Section
                        self.render_section(ui, "Behavior", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Max Results:");
                                // Add slider for max results
                            });
                        });

                        ui.add_space(12.0);

                        // Search Section
                        self.render_section(ui, "Search", |ui| {
                            ui.checkbox(&mut self.enable_history, "Enable search history");
                        });
                        */
                    });

                // Keyboard shortcuts
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

        ctx.request_repaint();
    }
}

impl SettingsApp {
    #[allow(dead_code)]
    fn render_section<R>(
        &self,
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
}