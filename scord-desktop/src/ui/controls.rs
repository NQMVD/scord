use crate::app::ScordApp;
use egui::Ui;

pub struct ControlsPanel;

impl ControlsPanel {
    pub fn show(ui: &mut Ui, app: &mut ScordApp) {
        ui.vertical(|ui| {
            // Reduced top padding
            ui.add_space(6.0);
            
            // Contestant controls - clean horizontal layout
            ui.horizontal(|ui| {
                ui.label("New:");
                ui.add_space(8.0);
                
                if let Some((new_contestant_name, _, _, _)) = app.get_active_tab_ui_state_mut() {
                    let text_edit = egui::TextEdit::singleline(new_contestant_name)
                        .desired_width(180.0)
                        .vertical_align(egui::Align::Center)
                        .margin(egui::Margin::symmetric(8.0, 0.0));
                    ui.add_sized([180.0, 32.0], text_edit);
                }
                
                ui.add_space(8.0);
                
                if ui.add_sized([120.0, 32.0], egui::Button::new("Add Contestant")).clicked() || 
                   (ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.memory(|m| m.has_focus(ui.id().with("contestant_input")))) {
                    app.add_contestant();
                }
            });

            ui.add_space(8.0); // Reduced spacing between panels

            // Property controls - clean horizontal layout
            ui.horizontal(|ui| {
                ui.label("New:");
                ui.add_space(8.0);
                
                if let Some((_, new_property_name, _, _)) = app.get_active_tab_ui_state_mut() {
                    let text_edit = egui::TextEdit::singleline(new_property_name)
                        .desired_width(180.0)
                        .vertical_align(egui::Align::Center)
                        .margin(egui::Margin::symmetric(8.0, 0.0));
                    ui.add_sized([180.0, 32.0], text_edit);
                }
                
                ui.add_space(8.0);
                
                if let Some((_, _, new_property_weight, _)) = app.get_active_tab_ui_state_mut() {
                    ui.add_sized([120.0, 32.0], egui::DragValue::new(new_property_weight)
                        .speed(0.1)
                        .range(0.1..=10.0)
                        .prefix("Weight: "));
                }
                
                ui.add_space(8.0);
                
                if let Some((_, _, _, new_property_higher_is_better)) = app.get_active_tab_ui_state_mut() {
                    ui.checkbox(new_property_higher_is_better, "Higher is better");
                }
                
                ui.add_space(8.0);
                
                if ui.add_sized([110.0, 32.0], egui::Button::new("Add Property")).clicked() {
                    app.add_property();
                }
            });
            
            // Reduced bottom padding
            ui.add_space(6.0);
        });
    }
}