use eframe::egui;
use egui::{Frame, CornerRadius, Color32, Margin, ScrollArea, RichText};

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
                corner_radius: CornerRadius::same(8),
                inner_margin: Margin::same(16),
                stroke: egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
                ..Default::default()
            })
            .show(ctx, |ui| {
                // --- Header ---
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("ℹ️ About Quick Search")
                        .size(22.0)
                        .color(BLUE_HIGHLIGHT)
                        .strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("✕").size(16.0).color(Color32::from_rgb(220, 80, 80))).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                ScrollArea::vertical().max_height(550.0).show(ui, |ui| {
                    // --- Application Info ---
                    self.render_section(ui, "Application", |ui| {
                        ui.label(RichText::new("Quick Search").size(16.0).strong().color(Color32::from_rgb(240, 240, 245)));
                        ui.label(RichText::new("Version 0.1.0").size(13.0).color(Color32::from_rgb(180, 180, 190)));
                        ui.add_space(6.0);
                        ui.label(RichText::new("A fast, lightweight launcher for your desktop")
                            .size(12.0)
                            .color(Color32::from_rgb(150, 150, 160)));
                    });

                    ui.add_space(16.0);

                    // --- Features ---
                    self.render_section(ui, "Features", |ui| {
                        let features = vec![
                            ("🔍", "Search Applications", "Find and launch apps instantly"),
                            ("🌐", "Web Search", "Search the web or open URLs directly"),
                            ("📁", "File Browser", "Open files and folders by path"),
                            ("🔢", "Calculator", "Evaluate math expressions on the fly"),
                            ("📜", "History", "Access your search history"),
                            ("⚙️", "Settings", "Customize your experience"),
                        ];

                        for (icon, title, desc) in features {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(icon).size(16.0));
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(title)
                                        .size(13.0)
                                        .strong()
                                        .color(Color32::from_rgb(220, 220, 230)));
                                    ui.label(RichText::new(desc)
                                        .size(11.0)
                                        .color(Color32::from_rgb(150, 150, 160)));
                                });
                            });
                            ui.add_space(6.0);
                        }
                    });

                    ui.add_space(16.0);

                    // --- Keyboard Shortcuts ---
                    self.render_section(ui, "Keyboard Shortcuts", |ui| {
                        let shortcuts = vec![
                            ("↵ Enter", "Execute selected action"),
                            ("↓ Down Arrow", "Navigate to next result"),
                            ("↑ Up Arrow", "Navigate to previous result"),
                            ("Esc", "Close window"),
                            ("@ + keyword", "Access special commands"),
                        ];

                        for (key, action) in shortcuts {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(key).size(12.0).monospace().color(BLUE_HIGHLIGHT));
                                ui.add_space(8.0);
                                ui.label(RichText::new(action).size(12.0).color(Color32::from_rgb(180, 180, 190)));
                            });
                            ui.add_space(4.0);
                        }
                    });

                    ui.add_space(16.0);

                    // --- Special Commands ---
                    self.render_section(ui, "Special Commands", |ui| {
                        let commands = vec![
                            ("@history", "Open search history"),
                            ("@settings", "Open settings"),
                            ("@info", "Show this information page"),
                        ];

                        for (cmd, desc) in commands {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(cmd).size(12.0).monospace().color(BLUE_HIGHLIGHT));
                                ui.add_space(8.0);
                                ui.label(RichText::new(desc).size(12.0).color(Color32::from_rgb(180, 180, 190)));
                            });
                            ui.add_space(4.0);
                        }
                    });

                    ui.add_space(16.0);

                    // --- System Information ---
                    self.render_section(ui, "System Information", |ui| {
                        self.render_sys_info_row(ui, "Platform", std::env::consts::OS);
                        self.render_sys_info_row(ui, "Architecture", std::env::consts::ARCH);
                        self.render_sys_info_row(ui, "Framework", "egui/eframe");
                    });

                    ui.add_space(24.0);

                    // --- Footer ---
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Built with ❤️ using Rust")
                            .size(11.0)
                            .color(Color32::from_rgb(120, 120, 130)));
                    });
                });

                // Close with Esc
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

        ctx.request_repaint();
    }
}

impl InfoApp {
    fn render_section<R>(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        Frame {
            fill: SECTION_BG,
            corner_radius: CornerRadius::same(6),
            inner_margin: Margin::same(12),
            stroke: egui::Stroke::new(1.0, SECTION_BORDER),
            ..Default::default()
        }
        .show(ui, |ui| {
            ui.label(RichText::new(title).size(14.0).strong().color(BLUE_HIGHLIGHT));
            ui.add_space(8.0);
            content(ui)
        })
        .inner
    }

    fn render_sys_info_row(&self, ui: &mut egui::Ui, label: &str, value: &str) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label)
                .size(12.0)
                .strong()
                .color(Color32::from_rgb(180, 180, 190)));
            ui.label(RichText::new(value)
                .size(12.0)
                .color(Color32::from_rgb(150, 150, 160)));
        });
        ui.add_space(2.0);
    }
}
