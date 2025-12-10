use eframe::egui;
use egui::{Frame, CornerRadius, Color32, Margin};
use crate::utils::{execute_action::execute_action, search::perform_search, utils::{SearchResult}};

pub struct QuickSearchApp {
    search_query: String,
    results: Vec<SearchResult>,
    selected_index: usize,
    first_frame: bool,
}

impl QuickSearchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            search_query: String::new(),
            results: Vec::new(),
            selected_index: 0,
            first_frame: true,
        }
    }
    
    fn search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.results.clear();
            return;
        }
        
        self.results = perform_search(&self.search_query);
        self.selected_index = 0;
    }
    
    fn execute_selected(&mut self, ctx: &egui::Context) {
        if self.selected_index < self.results.len() {
            let result = &self.results[self.selected_index];
            
            execute_action(result, &self.search_query);
            
            // Hide the search bar if opening a special window
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl eframe::App for QuickSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Define a modern, subtle blue color for highlighting
        const BLUE_HIGHLIGHT: Color32 = Color32::from_rgb(50, 140, 255);
        
        // Main window Frame: Almost fully transparent background
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgba_unmultiplied(20, 20, 24, 1),
                corner_radius: CornerRadius::ZERO,
                inner_margin: Margin::same(8),
                outer_margin: 0.0.into(),
                shadow: egui::epaint::Shadow::NONE,
                stroke: egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(60, 60, 70, 180)),
                ..Frame::default()
            })
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 4.0);
                
                // Search Input Frame
                let search_frame = Frame {
                    fill: Color32::from_rgba_unmultiplied(25, 25, 30, 255), 
                    corner_radius: CornerRadius::same(6),
                    inner_margin: Margin::symmetric(10, 6),
                    stroke: egui::Stroke::new(2.0, BLUE_HIGHLIGHT),
                    ..Frame::default()
                };
                
                let original_style = ui.style().clone(); 
                let mut custom_visuals = ui.style().visuals.clone();
                custom_visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(50, 140, 255, 100);
                custom_visuals.widgets.active.fg_stroke.color = BLUE_HIGHLIGHT;
                ui.style_mut().visuals = custom_visuals;

                let search_response = search_frame.show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .id_source("quick_search_input")
                            .font(egui::TextStyle::Heading)
                            .hint_text("🔎 Search or type @info for help")
                            .desired_width(f32::INFINITY)
                            .frame(false) 
                    );
                    response
                }).inner;
                
                ui.set_style(original_style); 
                
                if self.first_frame { 
                    search_response.request_focus(); 
                    self.first_frame = false; 
                }
                
                if search_response.changed() { 
                    self.search(); 
                }
                
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) { 
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close); 
                }
                
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) { 
                    self.execute_selected(ctx); 
                }
                
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if self.selected_index < self.results.len().saturating_sub(1) { 
                        self.selected_index += 1; 
                    } else if !self.results.is_empty() { 
                        self.selected_index = 0; 
                    }
                }
                
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if self.selected_index > 0 { 
                        self.selected_index = self.selected_index.saturating_sub(1); 
                    } else if !self.results.is_empty() { 
                        self.selected_index = self.results.len().saturating_sub(1); 
                    }
                }
                
                // --- Display Single Selected Result ---
                if !self.results.is_empty() {
                    ui.add_space(4.0);
                    ui.separator(); 
                    ui.add_space(4.0);
                    
                    if let Some(result) = self.results.get(self.selected_index) {
                        let is_selected = true; 
                        
                        let frame = Frame {
                            fill: if is_selected {
                                Color32::from_rgba_unmultiplied(80, 85, 110, 240) 
                            } else {
                                Color32::from_rgba_unmultiplied(20, 20, 24, 5) 
                            },
                            corner_radius: CornerRadius::ZERO,
                            inner_margin: Margin::symmetric(8, 8),
                            ..Frame::default()
                        };

                        let response = frame.show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                let icon_text = match result.icon.as_str() {
                                    "📱" => "[APP]", "🌐" => "[WEB]", "📁" => "[DIR]", 
                                    "📄" => "[FILE]", "🔢" => "[CALC]", "🔍" => "[SEARCH]",
                                    "⚙️" => "[SET]", "ℹ️" => "[INFO]", "📜" => "[HIST]",
                                    _ => "[?]",
                                };
                                
                                ui.label(egui::RichText::new(icon_text)
                                    .size(14.0)
                                    .color(BLUE_HIGHLIGHT)
                                    .monospace());
                                ui.add_space(8.0);
                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 1.0;
                                    ui.label(egui::RichText::new(&result.title)
                                        .size(14.0)
                                        .color(Color32::from_rgb(240, 240, 245)));
                                    ui.label(egui::RichText::new(&result.subtitle)
                                        .size(11.0)
                                        .color(Color32::from_rgb(150, 150, 160)));
                                });
                            });
                        });
                        
                        let rect = response.response.rect;
                        let hover_response = ui.interact(
                            rect,
                            egui::Id::new("single_result_interaction"),
                            egui::Sense::click()
                        );
                        
                        if hover_response.clicked() {
                            self.execute_selected(ctx);
                        }
                        
                        ui.add_space(4.0);
                    }
                }
            });
        
        ctx.request_repaint();
    }
}