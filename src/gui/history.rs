use crate::utils::history_manager::HistoryManager;
use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Margin, ScrollArea};

pub struct HistoryApp {
    pub(crate) history_manager: HistoryManager,
    pub(crate) selected_index: Option<usize>,
    pub(crate) search_filter: String,
}

impl HistoryApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            history_manager: HistoryManager::new(),
            selected_index: None,
            search_filter: String::new(),
        }
    }
}

const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);

impl eframe::App for HistoryApp {
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
                    ui.heading(
                        egui::RichText::new("📜 History")
                            .size(22.0)
                            .color(BLUE_HIGHLIGHT),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(
                                egui::RichText::new("✕")
                                    .size(16.0)
                                    .color(Color32::from_rgb(200, 80, 80)),
                            )
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                // Filter input
                Frame {
                    fill: Color32::from_rgba_unmultiplied(25, 25, 30, 255),
                    corner_radius: CornerRadius::same(6),
                    inner_margin: Margin::symmetric(10, 8),
                    stroke: egui::Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
                    ..Frame::default()
                }
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("🔍").size(16.0).color(BLUE_HIGHLIGHT));

                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_filter)
                                .hint_text("Filter history…")
                                .desired_width(f32::INFINITY)
                                .frame(false),
                        );
                    });
                });

                ui.add_space(12.0);

                // Action buttons
                ui.horizontal(|ui| {
                    if ui
                        .button(egui::RichText::new("🗑️ Clear All").size(12.0))
                        .clicked()
                    {
                        self.clear_history();
                    }

                    let total_items = self.history_manager.load_history().len();
                    ui.label(
                        egui::RichText::new(format!("Total: {} items", total_items))
                            .size(12.0)
                            .color(Color32::from_rgb(150, 150, 160)),
                    );
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // History list
                let filtered_history = self.get_filtered_history();

                if filtered_history.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            egui::RichText::new("📭 No history yet")
                                .size(18.0)
                                .color(Color32::from_rgb(150, 150, 160)),
                        );
                        ui.label(
                            egui::RichText::new("Your searches will appear here.")
                                .size(13.0)
                                .color(Color32::from_rgb(110, 110, 120)),
                        );
                    });
                } else {
                    ScrollArea::vertical()
                        .max_height(450.0)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.add_space(4.0);

                            for (idx, entry) in filtered_history.iter().enumerate() {
                                let is_selected = self.selected_index == Some(idx);

                                let mut delete_rect: Option<egui::Rect> = None;

                                // Background frame
                                let frame = Frame {
                                    fill: if is_selected {
                                        Color32::from_rgba_unmultiplied(60, 65, 90, 220)
                                    } else {
                                        Color32::from_rgba_unmultiplied(30, 30, 36, 200)
                                    },
                                    corner_radius: CornerRadius::same(6),
                                    inner_margin: Margin::symmetric(12, 10),
                                    stroke: if is_selected {
                                        egui::Stroke::new(1.0, BLUE_HIGHLIGHT)
                                    } else {
                                        egui::Stroke::NONE
                                    },
                                    ..Frame::default()
                                };

                                let response = frame.show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // ICON
                                        ui.label(
                                            egui::RichText::new(match entry.result_icon.as_str() {
                                                "📱" => "[APP]",
                                                "🌐" => "[WEB]",
                                                "📁" => "[DIR]",
                                                "📄" => "[FILE]",
                                                "🔢" => "[CALC]",
                                                _ => "[?]",
                                            })
                                            .size(12.0)
                                            .color(BLUE_HIGHLIGHT)
                                            .monospace(),
                                        );

                                        ui.add_space(8.0);

                                        // TITLE + TIMESTAMP
                                        ui.vertical(|ui| {
                                            ui.label(
                                                egui::RichText::new(&entry.query)
                                                    .size(14.0)
                                                    .color(Color32::from_rgb(240, 240, 245)),
                                            );

                                            ui.label(
                                                egui::RichText::new(&entry.result_title)
                                                    .size(12.0)
                                                    .color(Color32::from_rgb(180, 180, 190)),
                                            );

                                            ui.label(
                                                egui::RichText::new(&entry.timestamp)
                                                    .size(10.0)
                                                    .color(Color32::from_rgb(120, 120, 130)),
                                            );
                                        });

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                let delete_response = ui
                                                    .button(
                                                        egui::RichText::new("🗑")
                                                            .size(13.0)
                                                            .color(Color32::from_rgb(220, 80, 80)),
                                                    )
                                                    .on_hover_text("Delete entry");

                                                if delete_response.clicked() {
                                                    self.delete_entry(entry);
                                                }

                                                // capture the rect for click exclusion
                                                delete_rect = Some(delete_response.rect);
                                            },
                                        );
                                    });
                                });

                                let row_rect = response.response.rect;

                                // Make the row clickable (except for delete button)
                                let final_click_rect = if let Some(del) = delete_rect {
                                    egui::Rect::from_min_max(
                                        row_rect.min,
                                        egui::pos2(del.min.x - 4.0, row_rect.max.y),
                                    )
                                } else {
                                    row_rect
                                };

                                let interact = ui.interact(
                                    final_click_rect,
                                    egui::Id::new(format!("hist_item_{idx}")),
                                    egui::Sense::click(),
                                );

                                if interact.hovered() {
                                    self.selected_index = Some(idx);
                                }

                                if interact.clicked() {
                                    self.execute_history_item(entry, ctx);
                                }

                                ui.add_space(6.0);
                            }
                        });
                }

                // Keyboard shortcuts
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Some(idx) = self.selected_index {
                        let filtered = self.get_filtered_history();
                        if let Some(entry) = filtered.get(idx) {
                            self.execute_history_item(entry, ctx);
                        }
                    }
                }
            });

        ctx.request_repaint();
    }
}
