# Test Datasets for Scord Desktop

I've created four test datasets to thoroughly test the Scord application:

## 0. Simple Test Dataset (`simple-test-dataset.json`)
**5 contestants, 3 properties**

A minimal dataset for quick testing and debugging:
- **Contestants**: Alpha Tester, Beta Runner, Gamma Force, Delta Prime, Epsilon Max
- **Properties**: Speed, Power, Cost (with different weights and directions)
- **Purpose**: Quick import test, verify basic functionality works

## 1. OpenTTD Trains Dataset (`openttd-trains-dataset.json`)
**30 contestants, 8 properties**

A realistic dataset featuring trains from OpenTTD (OpenTransport Tycoon Deluxe) and real-world trains:
- **Contestants**: Various train models from steam engines to high-speed rail
- **Properties**: Speed, Power, Tractive Effort, Weight, Cost, Running Cost, Reliability, Cargo Capacity
- **Interesting scoring scenarios**: Trade-offs between speed vs cost, power vs efficiency

## 2. Gaming Laptops Dataset (`gaming-laptops-dataset.json`)
**20 contestants, 10 properties**

A realistic dataset comparing gaming laptops with interesting trade-offs:
- **Contestants**: Popular gaming laptop models from ASUS, MSI, Razer, etc.
- **Properties**: CPU Score, GPU Score, RAM, Storage Speed, Display Quality, Battery Life, Weight, Price, Build Quality, Cooling
- **Interesting scenarios**: Performance vs portability, price vs quality trade-offs

## 4. Stress Test Dataset (`stress-test-dataset.json`)
**500 contestants, 15 properties**

A large dataset with gibberish names for performance testing:
- **Contestants**: 500 randomly generated names like "Zyx-omatic-4721", "Blarf-tron-892"
- **Properties**: Property_01 through Property_15 with random weights and directions
- **Purpose**: Test UI performance, scrolling, sorting with large datasets
- **Generated with**: Lua script (not Python!)

## How to Test

1. **Start the Scord desktop app**:
   ```bash
   cd scord-desktop
   cargo run --release
   ```

2. **Import a dataset**:
   - Click the "Import" button in the top-right
   - Select one of the JSON files
   - Watch the data populate and scores calculate
   - **Start with `simple-test-dataset.json`** to verify the import fix works!

3. **Test features**:
   - **Import Fix**: The app now handles both export format (property names) and internal format (UUIDs)
   - **Scrolling**: Try the stress test dataset to see how the UI handles 500+ rows
   - **Scoring**: Notice how different weight combinations affect rankings
   - **Editing**: Modify values and see real-time score updates
   - **Export**: Export the results to see the JSON output format

## Expected Behavior

- **OpenTTD Dataset**: High-speed trains like TGV and Shinkansen should rank highly due to speed/reliability weights
- **Gaming Laptops**: Should show interesting trade-offs between performance beasts vs portable/efficient models
- **Stress Test**: Should handle smoothly with good scrolling performance and instant score updates

## Performance Notes

The stress test dataset is designed to push the limits:
- 500 contestants × 15 properties = 7,500 data points
- Real-time scoring calculations on every change
- UI rendering with scrollable tables
- Should still feel responsive on modern hardware!

Enjoy testing! 🚂💻🎮