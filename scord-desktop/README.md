# Scord Desktop

A standalone desktop application for contestant scoring and ranking, ported from the original web application to Rust using egui/eframe.

## Features

- **Spreadsheet-like interface** for managing contestants and properties
- **Real-time scoring calculation** with weighted normalization
- **Ranking system** that identifies who's best at each property
- **Local data persistence** with automatic saving
- **Import/Export functionality** for JSON data
- **Dark charcoal theme** matching the original design
- **Cross-platform** desktop application

## Usage

### Adding Data
1. **Add Contestants**: Enter a name in the "New Contestant" field and click "Add Contestant"
2. **Add Properties**: Enter a property name, set weight (0.1-10.0), choose direction (higher/lower is better), and click "Add Property"
3. **Enter Values**: Use the drag values in the spreadsheet to set scores for each contestant-property combination

### Scoring System
- Values are normalized to a 0-100 scale per property
- Each property is weighted according to its weight setting
- Final scores are calculated as weighted averages
- "Best at" properties are shown for contestants who uniquely excel

### Data Management
- **Auto-save**: Data is automatically saved to `~/.scord/data.json`
- **Import**: Load data from a JSON file using the Import button
- **Export Data**: Save contestant and property data to JSON
- **Export Results**: Save current rankings and scores to JSON

## Building

```bash
cargo build --release
```

The executable will be created at `target/release/scord` (or `scord.exe` on Windows).

## Architecture

- **Models**: Core data structures (Contestant, Property, ScoreResult)
- **Scoring Engine**: Implements the weighted normalization algorithm
- **Persistence**: Local JSON file storage
- **UI**: egui-based interface with custom dark theme
- **Export**: JSON export functionality matching original format

## Original Web App

This desktop application is a faithful port of the original React/Convex web application, maintaining all core functionality while providing a native desktop experience with local data storage.