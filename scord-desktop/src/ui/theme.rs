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
    
    // Dark charcoal theme colors - matching webapp exactly
    let charcoal_950 = Color32::from_rgb(13, 13, 13);    // #0d0d0d - primary background
    let charcoal_975 = Color32::from_rgb(10, 10, 10);    // #0a0a0a - deeper background  
    let charcoal_900 = Color32::from_rgb(25, 25, 25);    // #191919 - surface
    let charcoal_800 = Color32::from_rgb(45, 45, 45);    // #2d2d2d - elevated
    let charcoal_700 = Color32::from_rgb(63, 63, 63);    // #3f3f3f - borders
    let charcoal_600 = Color32::from_rgb(93, 93, 93);    // #5d5d5d
    let charcoal_500 = Color32::from_rgb(109, 109, 109); // #6d6d6d
    let charcoal_400 = Color32::from_rgb(136, 136, 136); // #888888 - muted text
    let charcoal_300 = Color32::from_rgb(176, 176, 176); // #b0b0b0 - secondary text
    let charcoal_200 = Color32::from_rgb(209, 209, 209); // #d1d1d1
    let charcoal_100 = Color32::from_rgb(232, 232, 232); // #e8e8e8 - primary text

    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(charcoal_100),
        
        // Window and panel colors - match webapp backgrounds
        window_fill: charcoal_950,
        panel_fill: charcoal_950, // Primary background like webapp
        
        // Widget colors - match webapp interactivity
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: charcoal_950,
                weak_bg_fill: charcoal_900,
                bg_stroke: Stroke::new(config.border_width, charcoal_700),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, charcoal_300),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: charcoal_900, // Surface color like webapp
                weak_bg_fill: charcoal_800,
                bg_stroke: Stroke::new(config.border_width, charcoal_800), // Darker border - less bright
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, charcoal_300),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: charcoal_800, // Elevated on hover like webapp
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(config.border_width, charcoal_600),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: config.hover_expansion,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: charcoal_700,
                weak_bg_fill: charcoal_600,
                bg_stroke: Stroke::new(config.active_border_width, charcoal_500),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: config.active_expansion,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: charcoal_800,
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(config.border_width, charcoal_600),
                rounding: config.get_rounding(),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: 0.0,
            },
        },
        
        // Selection colors - match webapp highlight
        selection: egui::style::Selection {
            bg_fill: charcoal_800,
            stroke: Stroke::new(1.0, charcoal_600),
        },
        
        // Hyperlink colors - use webapp accent colors
        hyperlink_color: charcoal_500,
        
        // Background colors - match webapp layering
        faint_bg_color: charcoal_900,
        extreme_bg_color: charcoal_975,
        code_bg_color: charcoal_900,
        
        // Warning colors
        warn_fg_color: Color32::from_rgb(255, 165, 0),
        error_fg_color: Color32::from_rgb(255, 100, 100),
        
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
            stroke: Stroke::new(2.0, charcoal_100),
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
        window_stroke: Stroke::new(1.0, charcoal_700),
        
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