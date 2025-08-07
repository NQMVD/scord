# Font Baseline Alignment Fix

## Problem
When using PP Supply Sans Light font with egui, text appeared visually "shifted up" or misaligned when different UI widgets (buttons, text inputs, labels) were placed in horizontal layouts. This created an uneven, unprofessional appearance where text baselines didn't align properly.

## Root Cause
The original PP Supply Sans Light TTF font had vertical metrics (ascent, descent, baseline) that were incompatible with egui's text rendering system. egui uses the `ttf-parser` crate which relies on specific font metrics for proper text positioning, and the original font's metrics caused baseline misalignment.

## Solution
**Convert the font using FontSquirrel Webfont Generator with expert settings:**

1. Go to [FontSquirrel Webfont Generator](https://www.fontsquirrel.com/tools/webfont-generator)
2. Upload the original `PPSupplySans-Light.ttf` 
3. Select **"Expert"** mode
4. Enable **"Fix Vertical Metrics"** option
5. Choose output formats: **TrueType** and **WOFF**
6. Generate and download the webfont package
7. Use the generated `ppsupplysans-light-webfont.ttf` file

## Implementation
Replace the font loading in `src/ui/theme.rs`:

```rust
// Load PP Supply Sans Light webfont with corrected vertical metrics
fonts.font_data.insert(
    "supply_sans".to_owned(),
    FontData::from_static(include_bytes!("../../assets/fonts/ppsupplysans-light-webfont.ttf")),
);
```

## Result
✅ **Fixed baseline alignment** - All text now aligns properly on the same baseline
✅ **Preserved brand typography** - Still using PP Supply Sans Light font
✅ **No performance impact** - Font loading remains static and efficient
✅ **Cross-platform compatibility** - Works on all platforms egui supports

## Alternative Solutions Attempted
- **Layout adjustments**: Using `allocate_ui_with_layout()` instead of `horizontal()` - helped slightly but didn't fully fix the issue
- **Font size tweaking**: Adjusting font sizes to compensate - made UI too compact without solving baseline issue
- **Fallback fonts**: Adding Inter font as fallback - caused crashes due to similar ttf-parser incompatibility
- **Spacing adjustments**: Modifying spacing and padding - didn't address the core font metrics problem

## Technical Notes
- egui uses `ttf-parser` crate which has strict requirements for font metrics
- FontSquirrel's "Fix Vertical Metrics" option normalizes ascent/descent values for web compatibility
- The webfont conversion process maintains visual appearance while fixing technical compatibility
- Original variable fonts are not supported by egui's text rendering system

## Files Modified
- `assets/fonts/ppsupplysans-light-webfont.ttf` (new corrected font file)
- `src/ui/theme.rs` (updated font loading path)

This solution maintains the desired visual branding while ensuring proper technical implementation compatible with egui's text rendering system.