use crate::app::ScordApp;
use egui::Ui;

pub struct ControlsPanel;

impl ControlsPanel {
    pub fn show(ui: &mut Ui, app: &mut ScordApp) {
        ui.vertical(|ui| {
            // Contestant controls
            ui.group(|ui| {
                // Use allocate_ui_with_layout for proper baseline alignment
                let size = egui::Vec2::new(
                    ui.available_width(),
                    ui.spacing().interact_size.y + 6.0, // Add extra height for better alignment
                );
                ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("New:");
                    ui.text_edit_singleline(&mut app.new_contestant_name)
                        .on_hover_text("Contestant name");
                    if ui.button("Add Contestant").clicked() || 
                       (ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.memory(|m| m.has_focus(ui.id().with("contestant_input")))) {
                        app.add_contestant();
                    }
                });
            });

            ui.add_space(4.0);

            // Property controls
            ui.group(|ui| {
                // Use allocate_ui_with_layout for proper baseline alignment
                let size = egui::Vec2::new(
                    ui.available_width(),
                    ui.spacing().interact_size.y + 6.0, // Add extra height for better alignment
                );
                ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("New:");
                    ui.text_edit_singleline(&mut app.new_property_name)
                        .on_hover_text("Property name");
                    
                    ui.add(egui::DragValue::new(&mut app.new_property_weight)
                        .speed(0.1)
                        .range(0.1..=10.0)
                        .prefix("Weight: "));
                    
                    ui.checkbox(&mut app.new_property_higher_is_better, "Higher is better");
                    
                    if ui.button("Add Property").clicked() {
                        app.add_property();
                    }
                });
            });
        });
    }
}