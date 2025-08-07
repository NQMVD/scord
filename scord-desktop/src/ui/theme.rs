use egui::{Color32, Rounding, Shadow, Stroke, Style, Visuals, Margin, FontDefinitions, FontData, FontFamily};

pub fn setup_custom_style(ctx: &egui::Context) {
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
    
    // Dark charcoal theme colors
    let charcoal_950 = Color32::from_rgb(13, 13, 13);    // #0d0d0d
    let charcoal_900 = Color32::from_rgb(25, 25, 25);    // #191919
    let charcoal_800 = Color32::from_rgb(45, 45, 45);    // #2d2d2d
    let charcoal_700 = Color32::from_rgb(63, 63, 63);    // #3f3f3f
    let charcoal_400 = Color32::from_rgb(136, 136, 136); // #888888
    let charcoal_300 = Color32::from_rgb(176, 176, 176); // #b0b0b0
    let charcoal_100 = Color32::from_rgb(232, 232, 232); // #e8e8e8

    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(charcoal_100),
        
        // Window and panel colors
        window_fill: charcoal_950,
        panel_fill: charcoal_900,
        
        // Widget colors
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: charcoal_900,
                weak_bg_fill: charcoal_800,
                bg_stroke: Stroke::new(1.0, charcoal_700),
                rounding: Rounding::same(8.0),
                fg_stroke: Stroke::new(1.0, charcoal_300),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: charcoal_800,
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(1.0, charcoal_700),
                rounding: Rounding::same(8.0),
                fg_stroke: Stroke::new(1.0, charcoal_300),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: charcoal_700,
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(1.0, charcoal_400),
                rounding: Rounding::same(8.0),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: 2.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: charcoal_700,
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(2.0, charcoal_400),
                rounding: Rounding::same(8.0),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: 2.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: charcoal_700,
                weak_bg_fill: charcoal_700,
                bg_stroke: Stroke::new(1.0, charcoal_400),
                rounding: Rounding::same(8.0),
                fg_stroke: Stroke::new(1.0, charcoal_100),
                expansion: 0.0,
            },
        },
        
        // Selection colors
        selection: egui::style::Selection {
            bg_fill: charcoal_700,
            stroke: Stroke::new(1.0, charcoal_400),
        },
        
        // Hyperlink colors
        hyperlink_color: charcoal_400,
        
        // Faint background color
        faint_bg_color: charcoal_800,
        
        // Extreme background color
        extreme_bg_color: charcoal_950,
        
        // Code background
        code_bg_color: charcoal_800,
        
        // Warning colors
        warn_fg_color: Color32::from_rgb(255, 165, 0),
        error_fg_color: Color32::from_rgb(255, 100, 100),
        
        // Window shadow
        window_shadow: Shadow {
            offset: egui::vec2(4.0, 4.0),
            blur: 8.0,
            spread: 0.0,
            color: Color32::from_black_alpha(100),
        },
        
        // Popup shadow
        popup_shadow: Shadow {
            offset: egui::vec2(8.0, 8.0),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_black_alpha(96),
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
        menu_rounding: Rounding::same(8.0),
        
        // Interact cursor
        interact_cursor: None,
        
        // Image loading spinners
        image_loading_spinners: true,
        
        // Window rounding
        window_rounding: Rounding::same(8.0),
        
        // Window stroke
        window_stroke: Stroke::new(1.0, charcoal_700),
        
        // Window highlight topmost
        window_highlight_topmost: true,
        
        // Numeric color space
        numeric_color_space: egui::style::NumericColorSpace::GammaByte,
    };
    
    // Font sizes
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(22.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
    );
    
    // Spacing adjustments
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.menu_margin = Margin::same(8.0);
    style.spacing.indent = 20.0;
    
    ctx.set_style(style);
}