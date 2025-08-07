# Bug Fixes Applied

## ✅ **Layout Fixes**
- **Vertical stacking**: Create-new panels are now vertically stacked instead of horizontally
- **Centered alignment**: Both contestant and property creation panels now have properly centered content
- **Column width**: Spreadsheet columns are now wider (120px minimum, 150px for property columns) to prevent name wrapping

## ✅ **Font Size Improvements**
- **Increased base font sizes**: Body text now 16px, buttons 15px, headings 22px
- **Better readability**: Especially improved in the results panel and throughout the UI
- **Consistent sizing**: All text elements now use appropriate font sizes

## ✅ **Export Format Selection**
- **Format dropdown**: Added ComboBox in header to select JSON or CSV export format
- **Dual format support**: Both data export and results export now support JSON and CSV
- **Proper file extensions**: File dialogs now use correct extensions (.json/.csv)
- **CSV implementation**: Added proper CSV export with headers and formatted data

## ✅ **Interactive Editing**
- **Editable property names**: Click and edit property names directly in the spreadsheet
- **Editable weights**: Use drag values to adjust property weights (0.1-10.0 range)
- **Clickable direction toggle**: Button shows "↑ Higher" or "↓ Lower" and toggles on click
- **Editable contestant names**: Click and edit contestant names directly in cells
- **Real-time updates**: All changes trigger immediate score recalculation and auto-save

## ✅ **UI Polish**
- **Better button labels**: Direction buttons show clear "↑ Higher" / "↓ Lower" text
- **Improved spacing**: Added proper spacing between UI elements
- **Grid layout**: Uses egui Grid for better table structure and alignment
- **Wider columns**: No more awkward text wrapping after single words

## 🧪 **Testing**
All fixes have been tested and the application builds successfully. The UI should now feel much more polished and functional, with full editing capabilities matching the original webapp's functionality!