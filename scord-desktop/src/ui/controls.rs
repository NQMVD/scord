use crate::app::ScordApp;
use egui::Ui;

pub struct ControlsPanel;

impl ControlsPanel {
    pub fn show(ui: &mut Ui, app: &mut ScordApp) {
        ui.vertical(|ui| {
            // Reduced top padding
            ui.add_space(6.0);
            
            // Contestant controls - remove group border, center-align vertically
            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), ui.spacing().interact_size.y + 16.0), // More height for better spacing
                egui::Layout::left_to_right(egui::Align::Center), 
                |ui| {
                    ui.label("New:");
                    // Same width as property textbox with better vertical centering
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(180.0, 28.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // Use a frame with proper margin to fix text positioning
                            egui::Frame::none()
                                .inner_margin(egui::Margin::symmetric(0.0, 3.0)) // Add vertical margin to center text
                                .show(ui, |ui| {
                                    let mut textbox = egui::TextEdit::singleline(&mut app.new_contestant_name)
                                        .desired_width(180.0);
                                    ui.add_sized([180.0, 22.0], textbox).on_hover_text("Contestant name");
                                });
                        }
                    );
                    
                    if ui.button("Add Contestant").clicked() || 
                       (ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.memory(|m| m.has_focus(ui.id().with("contestant_input")))) {
                        app.add_contestant();
                    }
                }
            );

            ui.add_space(8.0); // Reduced spacing between panels

            // Property controls - remove group border, center-align vertically
            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), ui.spacing().interact_size.y + 16.0), // More height for better spacing
                egui::Layout::left_to_right(egui::Align::Center), 
                |ui| {
                    ui.label("New:");
                    // Same width as contestant textbox with better vertical centering
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(180.0, 28.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            // Use a frame with proper margin to fix text positioning
                            egui::Frame::none()
                                .inner_margin(egui::Margin::symmetric(0.0, 3.0)) // Add vertical margin to center text
                                .show(ui, |ui| {
                                    let mut textbox = egui::TextEdit::singleline(&mut app.new_property_name)
                                        .desired_width(180.0);
                                    ui.add_sized([180.0, 22.0], textbox).on_hover_text("Property name");
                                });
                        }
                    );
                    
                    // Shorter, darker number input
                    ui.visuals_mut().widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 45); // charcoal-800 - darker
                    ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgb(63, 63, 63); // charcoal-700
                    
                    ui.add_sized([120.0, 20.0], egui::DragValue::new(&mut app.new_property_weight)
                        .speed(0.1)
                        .range(0.1..=10.0)
                        .prefix("Weight: "));
                    
                    ui.checkbox(&mut app.new_property_higher_is_better, "Higher is better");
                    
                    if ui.button("Add Property").clicked() {
                        app.add_property();
                    }
                }
            );
            
            // Reduced bottom padding
            ui.add_space(6.0);
        });
    }
}