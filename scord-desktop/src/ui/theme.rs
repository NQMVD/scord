use crate::models::VisualConfig;
use egui::{Color32, Rounding, Shadow, Stroke, Style, Visuals, Margin, FontDefinitions, FontData, FontFamily};

pub fn setup_custom_style(ctx: &egui::Context) {
    setup_custom_style_with_config(ctx, &VisualConfig::default())
}

pub fn setup_custom_style_with_config(ctx: &egui::Context, config: &VisualConfig) {
    // Setup PP Supply Sans font (webfont version with corrected metrics)
    let mut fonts = FontDefinitions::default();
    
    // Load PP Supply Sans Light webfont with corrected vertical metrics
    fonts.font_data.insert(
        "supply_sans".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/ppsupplysans-light-webfont.ttf")),
    );
    
    // Set as primary font
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "supply_sans".to_owned());
    
    ctx.set_fonts(fonts);
    
    let mut style = Style::default();
    
    // Use configurable colors from VisualConfig
    let bg_primary = config.get_bg_primary();
    let bg_surface = config.get_bg_surface();
    let bg_elevated = config.get_bg_elevated();
    let bg_extreme = config.get_bg_extreme();
    let border_default = config.get_border_default();
    let border_active = config.get_border_active();
    let border_hover = config.get_border_hover();
    let text_primary = config.get_text_primary();
    let text_secondary = config.get_text_secondary();
    let text_muted = config.get_text_muted();
    let widget_bg = config.get_widget_bg();
    let widget_bg_hover = config.get_widget_bg_hover();
    let widget_bg_active = config.get_widget_bg_active();
    let selection_bg = config.get_selection_bg();

    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(text_primary),
        
        // Window and panel colors - use configurable backgrounds
        window_fill: bg_primary,
        panel_fill: bg_primary,
        
        // Widget colors - use configurable widget colors
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: bg_primary,
                weak_bg_fill: bg_surface,
                bg_stroke: Stroke::new(config.border_width, border_default),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, text_secondary),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: widget_bg,
                weak_bg_fill: bg_elevated,
                bg_stroke: Stroke::new(config.border_width, border_default),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, text_secondary),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: widget_bg_hover,
                weak_bg_fill: bg_elevated,
                bg_stroke: Stroke::new(config.border_width, border_hover),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, text_primary),
                expansion: config.hover_expansion,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: widget_bg_active,
                weak_bg_fill: bg_elevated,
                bg_stroke: Stroke::new(config.active_border_width, border_active),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, text_primary),
                expansion: config.active_expansion,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: bg_elevated,
                weak_bg_fill: bg_elevated,
                bg_stroke: Stroke::new(config.border_width, border_hover),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, text_primary),
                expansion: 0.0,
            },
        },
        
        // Selection colors - use configurable selection
        selection: egui::style::Selection {
            bg_fill: selection_bg,
            stroke: Stroke::new(1.0, border_hover),
        },
        
        // Hyperlink colors - use configurable text color
        hyperlink_color: text_muted,
        
        // Background colors - use configurable backgrounds
        faint_bg_color: bg_surface,
        extreme_bg_color: bg_extreme,
        code_bg_color: bg_surface,
        
        // Warning colors - use configurable status colors
        warn_fg_color: config.get_warning_color(),
        error_fg_color: config.get_error_color(),
        
        // Window shadow
        window_shadow: config.get_shadow(),
        
        // Popup shadow
        popup_shadow: if config.shadow_enabled {
            Shadow {
                offset: egui::vec2(8.0, 8.0),
                blur: config.shadow_blur * 2.0,
                spread: config.shadow_spread,
                color: Color32::from_black_alpha(96),
            }
        } else {
            Shadow::NONE
        },
        
        // Resize corner size
        resize_corner_size: 12.0,
        
        // Text cursor
        text_cursor: egui::style::TextCursorStyle {
            stroke: Stroke::new(2.0, text_primary),
            preview: false,
            blink: true,
            on_duration: 1.0,
            off_duration: 0.5,
        },
        
        // Clip rect margin
        clip_rect_margin: 3.0,
        
        // Button frame
        button_frame: true,
        
        // Collapsing header frame
        collapsing_header_frame: false,
        
        // Indent has left vline
        indent_has_left_vline: false,
        
        // Striped
        striped: false,
        
        // Slider trailing fill
        slider_trailing_fill: false,
        
        // Handle shape
        handle_shape: egui::style::HandleShape::Circle,
        
        // Menu rounding
        menu_rounding: config.get_rounding(),
        
        // Interact cursor
        interact_cursor: None,
        
        // Image loading spinners
        image_loading_spinners: true,
        
        // Window rounding
        window_rounding: Rounding::same(config.window_rounding),
        
        // Window stroke
        window_stroke: Stroke::new(1.0, border_default),
        
        // Window highlight topmost
        window_highlight_topmost: true,
        
        // Numeric color space
        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
    };
    
    // Font sizes from config
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(config.base_font_size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(config.button_font_size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(config.heading_font_size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(config.small_font_size, egui::FontFamily::Proportional),
    );
    
    // Spacing adjustments from config
    style.spacing.item_spacing = config.get_item_spacing();
    style.spacing.button_padding = config.get_button_padding();
    style.spacing.menu_margin = Margin::same(config.panel_margin);
    style.spacing.indent = 20.0;
    
    // Text input sizing - make textboxes taller
    style.spacing.text_edit_width = 200.0;
    
    // Adjust interact size for different components
    style.spacing.interact_size = egui::vec2(24.0, 24.0); // Default size
    
    // Use consistent button padding from config
    style.spacing.button_padding = config.get_button_padding();
    
    ctx.set_style(style);
}