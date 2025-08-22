use crate::models::tab::{TabManager, Tab};
use crate::models::VisualConfig;
use egui::{Color32, Stroke, Rounding, Vec2, Pos2, Rect, Sense};

pub struct TabBar;

impl TabBar {
    pub fn show(ui: &mut egui::Ui, tab_manager: &mut TabManager, config: &VisualConfig) -> Option<TabAction> {
        let mut action = None;
        
        ui.horizontal(|ui| {
            // Tab bar styling with spacing
            ui.spacing_mut().item_spacing.x = config.tab_spacing; // Use configurable tab spacing
            
            let available_width = ui.available_width() - 40.0; // Reserve space for + button
            let tab_count = tab_manager.tab_count();
            let max_tab_width = 200.0;
            let min_tab_width = 120.0;
            let spacing_total = (tab_count.saturating_sub(1)) as f32 * config.tab_spacing; // Account for spacing
            let tab_width = ((available_width - spacing_total) / tab_count as f32).clamp(min_tab_width, max_tab_width);
            
            // Draw tabs
            for (index, tab) in tab_manager.tabs.iter().enumerate() {
                if let Some(tab_action) = Self::draw_tab(ui, tab, index, tab_width, tab_manager.active_tab_index == index, config) {
                    action = Some(tab_action);
                }
            }
            
            // Add new tab button
            ui.add_space(4.0);
            if ui.add_sized([32.0, 32.0], egui::Button::new("+")).clicked() {
                action = Some(TabAction::NewTab);
            }
        });
        
        action
    }
    
    fn draw_tab(ui: &mut egui::Ui, tab: &Tab, index: usize, width: f32, is_active: bool, config: &VisualConfig) -> Option<TabAction> {
        let height = 36.0; // Taller for dashboard style
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
        
        let mut action = None;
        
        if response.clicked() {
            action = Some(TabAction::SwitchTo(index));
        }
        
        // Dashboard-style tab with stronger visual hierarchy
        let has_unsaved = tab.content.has_unsaved_changes;
        
        let bg_color = if is_active {
            if has_unsaved {
                Color32::from_rgb(70, 45, 50) // Red tint for unsaved active tab
            } else {
                Color32::from_rgb(132, 126, 255) // Dashboard accent color for active tab
            }
        } else if response.hovered() {
            if has_unsaved {
                Color32::from_rgb(60, 40, 45) // Red tint for unsaved hovered tab
            } else {
                Color32::from_gray(35) // Lighter than surface for hover
            }
        } else {
            if has_unsaved {
                Color32::from_rgb(50, 35, 40) // Red tint for unsaved tab
            } else {
                Color32::from_gray(25) // Dashboard surface color
            }
        };
        
        let stroke = if is_active {
            if has_unsaved {
                Stroke::new(2.0, Color32::from_rgb(220, 120, 120)) // Red border for unsaved active tab
            } else {
                Stroke::new(2.0, Color32::from_rgb(132, 126, 255)) // Accent border for active tab
            }
        } else {
            if has_unsaved {
                Stroke::new(1.0, Color32::from_rgb(140, 90, 90)) // Dim red border for unsaved tab
            } else {
                Stroke::new(config.border_width, config.get_border_default()) // Default border for inactive tab
            }
        };
        
        // Draw tab background
        ui.painter().rect(
            rect,
            config.get_tab_rounding(),
            bg_color,
            stroke,
        );
        
        // Tab content area (leave space for close button)
        let content_rect = Rect::from_min_size(
            rect.min + Vec2::new(8.0, 0.0),
            Vec2::new(width - 32.0, height),
        );
        
        // Draw tab text with dashboard styling
        let text_color = if is_active {
            if has_unsaved {
                Color32::WHITE // White text for unsaved active
            } else {
                Color32::WHITE // White text on accent background
            }
        } else {
            Color32::from_gray(200) // Light gray for inactive tabs
        };
        
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(tab.display_name())
                        .color(text_color)
                        .size(14.0)
                        .strong() // Make text bold for better visibility
                ).truncate());
                
                // Unsaved indicator dot
                if has_unsaved {
                    ui.add_space(4.0);
                    let dot_color = if is_active {
                        Color32::from_rgb(255, 180, 180)
                    } else {
                        Color32::from_rgb(180, 120, 120)
                    };
                    ui.painter().circle_filled(
                        ui.next_widget_position() + Vec2::new(4.0, 0.0),
                        3.0,
                        dot_color
                    );
                }
            });
        });
        
        // Close button
        let close_button_rect = Rect::from_center_size(
            Pos2::new(rect.max.x - 16.0, rect.center().y),
            Vec2::new(16.0, 16.0),
        );
        
        let close_response = ui.allocate_rect(close_button_rect, Sense::click());
        
        if close_response.clicked() {
            action = Some(TabAction::Close(index));
        }
        
        // Draw close button
        let close_color = if close_response.hovered() {
            Color32::from_rgb(220, 80, 80)
        } else {
            Color32::from_gray(150)
        };
        
        ui.painter().text(
            close_button_rect.center(),
            egui::Align2::CENTER_CENTER,
            "×",
            egui::FontId::proportional(14.0),
            close_color,
        );
        
        // Add tooltips
        let tab_tooltip_text = format!("{}\n\nKeyboard shortcuts:\nCtrl+T: New tab\nCtrl+W: Close tab\nCtrl+Tab: Next tab\nCtrl+Shift+Tab: Previous tab\nCtrl+{}: Switch to this tab", 
            if tab.content.has_unsaved_changes { 
                format!("{} (has unsaved changes)", tab.name) 
            } else { 
                tab.name.clone() 
            },
            index + 1
        );
        if response.hovered() && !close_response.hovered() {
            response.on_hover_text(tab_tooltip_text);
        }
        if close_response.hovered() {
            close_response.on_hover_text("Close tab");
        }
        
        action
    }
}

#[derive(Debug, Clone)]
pub enum TabAction {
    SwitchTo(usize),
    Close(usize),
    NewTab,
}

