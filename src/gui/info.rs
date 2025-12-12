use eframe::egui;
use egui::{Frame, CornerRadius, Color32, Margin, ScrollArea, RichText, Align};

const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);
const SECTION_BG: Color32 = Color32::from_rgb(28, 28, 32);
const SECTION_BORDER: Color32 = Color32::from_rgb(60, 60, 70);

pub struct InfoApp {}

impl InfoApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for InfoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(18, 18, 22),
                corner_radius: CornerRadius::same(12),
                inner_margin: Margin::same(20),
                stroke: egui::Stroke::new(1.5, Color32::from_rgb(60, 60, 70)),
                ..Default::default()
            })
            .show(ctx, |ui| {
                // Header with close button
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.heading(RichText::new("ℹ️  About Quick Search")
                        .size(24.0)
                        .color(BLUE_HIGHLIGHT)
                        .strong());

                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(egui::Button::new(RichText::new("✕").size(18.0))
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

                // Centered scrollable content - use available height
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            // App info section
                            Frame {
                                fill: SECTION_BG,
                                corner_radius: CornerRadius::same(8),
                                inner_margin: Margin::same(16),
                                stroke: egui::Stroke::new(1.0, SECTION_BORDER),
                                ..Default::default()
                            }
                            .show(ui, |ui| {
                                ui.set_max_width(520.0);
                                ui.vertical_centered(|ui| {
                                    ui.label(RichText::new("Quick Search")
                                        .size(20.0)
                                        .strong()
                                        .color(Color32::from_rgb(240, 240, 250)));
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("Version 0.1.0")
                                        .size(13.0)
                                        .color(Color32::from_rgb(160, 160, 180)));
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("A fast, lightweight application launcher")
                                        .size(12.0)
                                        .color(Color32::from_rgb(140, 140, 160)));
                                });
                            });

                            ui.add_space(20.0);

                            // Features grid
                            self.render_section_centered(ui, "Features", 520.0, |ui| {
                                let features = vec![
                                    ("🔍", "Search Applications", "Find and launch apps instantly"),
                                    ("🌐", "Web Search", "Search the web or open URLs directly"),
                                    ("📁", "File Browser", "Open files and folders by path"),
                                    ("🔢", "Calculator", "Evaluate math expressions on the fly"),
                                    ("📜", "History", "Access your search history"),
                                    ("⚙️", "Settings", "Customize your experience"),
                                ];

                                for (icon, title, desc) in features {
                                    Frame {
                                        fill: Color32::from_rgba_unmultiplied(35, 35, 42, 200),
                                        corner_radius: CornerRadius::same(6),
                                        inner_margin: Margin::symmetric(12, 10),
                                        ..Default::default()
                                    }
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(icon).size(18.0));
                                            ui.add_space(10.0);
                                            ui.vertical(|ui| {
                                                ui.label(RichText::new(title)
                                                    .size(13.5)
                                                    .strong()
                                                    .color(Color32::from_rgb(220, 220, 235)));
                                                ui.label(RichText::new(desc)
                                                    .size(11.5)
                                                    .color(Color32::from_rgb(150, 150, 165)));
                                            });
                                        });
                                    });
                                    ui.add_space(8.0);
                                }
                            });

                            ui.add_space(20.0);

                            // Keyboard shortcuts
                            self.render_section_centered(ui, "Keyboard Shortcuts", 520.0, |ui| {
                                let shortcuts = vec![
                                    ("↵ Enter", "Execute selected action"),
                                    ("↓ Down", "Navigate to next result"),
                                    ("↑ Up", "Navigate to previous result"),
                                    ("Esc", "Close window"),
                                    ("@ + word", "Access special commands"),
                                ];

                                for (key, action) in shortcuts {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        Frame {
                                            fill: Color32::from_rgb(40, 45, 65),
                                            corner_radius: CornerRadius::same(4),
                                            inner_margin: Margin::symmetric(8, 4),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(key)
                                                .size(11.5)
                                                .monospace()
                                                .color(BLUE_HIGHLIGHT));
                                        });
                                        ui.add_space(12.0);
                                        ui.label(RichText::new(action)
                                            .size(12.0)
                                            .color(Color32::from_rgb(180, 180, 200)));
                                    });
                                    ui.add_space(6.0);
                                }
                            });

                            ui.add_space(20.0);

                            // Special commands
                            self.render_section_centered(ui, "Special Commands", 520.0, |ui| {
                                let commands = vec![
                                    ("@history", "View search history"),
                                    ("@settings", "Open settings panel"),
                                    ("@info", "Show this information"),
                                ];

                                for (cmd, desc) in commands {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        Frame {
                                            fill: Color32::from_rgb(40, 45, 65),
                                            corner_radius: CornerRadius::same(4),
                                            inner_margin: Margin::symmetric(8, 4),
                                            ..Default::default()
                                        }
                                        .show(ui, |ui| {
                                            ui.label(RichText::new(cmd)
                                                .size(11.5)
                                                .monospace()
                                                .color(BLUE_HIGHLIGHT));
                                        });
                                        ui.add_space(12.0);
                                        ui.label(RichText::new(desc)
                                            .size(12.0)
                                            .color(Color32::from_rgb(180, 180, 200)));
                                    });
                                    ui.add_space(6.0);
                                }
                            });

                            ui.add_space(20.0);

                            // System info
                            self.render_section_centered(ui, "System Information", 520.0, |ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    self.render_sys_info_item(ui, "Platform", std::env::consts::OS);
                                    ui.add_space(20.0);
                                    self.render_sys_info_item(ui, "Architecture", std::env::consts::ARCH);
                                    ui.add_space(20.0);
                                    self.render_sys_info_item(ui, "Framework", "egui/eframe");
                                });
                            });

                            ui.add_space(24.0);

                            // Footer
                            ui.label(RichText::new("🦀 Built with Rust")
                                .size(11.5)
                                .color(Color32::from_rgb(130, 130, 145)));
                            
                            ui.add_space(12.0); // Extra space at bottom
                        });
                    });

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

        ctx.request_repaint();
    }
}

impl InfoApp {
    fn render_section_centered<R>(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        max_width: f32,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        Frame {
            fill: SECTION_BG,
            corner_radius: CornerRadius::same(8),
            inner_margin: Margin::same(16),
            stroke: egui::Stroke::new(1.0, SECTION_BORDER),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.set_max_width(max_width);
            ui.label(RichText::new(title)
                .size(15.0)
                .strong()
                .color(BLUE_HIGHLIGHT));
            ui.add_space(12.0);
            content(ui)
        })
        .inner
    }

    fn render_sys_info_item(&self, ui: &mut egui::Ui, label: &str, value: &str) {
        ui.vertical(|ui| {
            ui.label(RichText::new(label)
                .size(11.0)
                .color(Color32::from_rgb(140, 140, 155)));
            ui.label(RichText::new(value)
                .size(12.5)
                .strong()
                .color(Color32::from_rgb(200, 200, 215)));
        });
    }
}