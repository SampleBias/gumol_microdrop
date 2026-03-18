# Gumol MicroDrop Design Studio

A native Rust application for converting molecular dynamics oxidative stress simulations from the **Gumol engine** into experimentally executable microdroplet assays on the **Nuclera eProtein Discovery** digital microfluidics platform.

---

## Purpose

Researchers running Gumol simulations of reactive oxygen species (ROS) dynamics need a way to validate computational predictions with real wet-lab experiments. This tool bridges that gap by:

1. Importing Gumol simulation outputs (JSON/CSV)
2. Extracting feature vectors (superoxide density, H2O2 peaks, diffusion rates, etc.)
3. Translating simulation parameters to experimental ranges
4. Generating droplet condition matrices for microdroplet assays
5. Producing Nuclera cartridge configurations (YAML)
6. Creating step-by-step wet-lab protocols (Markdown)
7. Analyzing correlation between predictions and measurements

---

## Technology Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (native, no WASM) |
| Window/App | Bevy 0.14 |
| UI Framework | bevy_egui (egui integration) |
| Serialization | serde, serde_json, serde_yaml |
| File Dialogs | rfd (cross-platform) |
| Data Export | csv crate |
| Error Handling | anyhow, thiserror |

---

## Quick Start

### Prerequisites

- Rust toolchain (1.75+ recommended): https://rustup.rs
- Linux: `sudo apt install libxkbcommon-dev libwayland-dev` (or equivalent for your distro)

### Build & Run

```bash
# Development (faster compile, dynamic linking)
cargo run

# Release (optimized)
cargo run --release

# Check without building
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

---

## Workflow

The application follows a 7-step pipeline accessible through the left navigation panel:

### Step 1: Simulation Import
Load a Gumol simulation file (JSON) or use the included sample data. The importer extracts a feature vector and computes initial experimental parameter ranges.

### Step 2: Card Editor (Main Design Palette)
The centerpiece of the app. Configure your experiment card:
- Set grid dimensions (default 8x12 = 96 wells)
- Choose oxidant type and concentration range
- Select antioxidant conditions
- Set exposure time points
- Click **Generate Matrix** to populate the grid

The color-coded grid shows droplet conditions at a glance. Switch between color modes (Oxidant, Antioxidant, ExposureTime, Treatment) to visualize different dimensions. Click any cell to see its details.

### Step 3: Parameter Mapping
View the translation from simulation parameters to experimental variables. Inspect computed feature values and recalculate ranges if needed.

### Step 4: Experiment Design
Fine-tune the matrix dimensions and constraints. Preview the full droplet list in tabular form. Export the matrix as CSV.

### Step 5: Nuclera Configuration
Configure cartridge-level settings (volume, mixing cycles, readout method/wavelength). Generate the Nuclera cartridge config and export as YAML.

### Step 6: Protocol Generator
Generate a complete wet-lab protocol with equipment lists, reagent lists, and step-by-step procedures with time estimates. Export as Markdown.

### Step 7: Data Viewer & Correlation
After running the experiment, analyze correlation between Gumol predictions and experimental measurements. View scatter plots, RMSE, Pearson r, and threshold accuracy. Export reports as JSON.

---

## Project Structure

```
src/
├── main.rs                          # App entry, Bevy setup, UI system dispatch
├── lib.rs                           # Library exports
├── data_models.rs                   # All data structures and application state
├── modules/
│   ├── mod.rs                       # Module exports
│   ├── simulation_importer.rs       # JSON/CSV import, feature extraction
│   ├── parameter_translation.rs     # Simulation → experimental range mapping
│   ├── experiment_designer.rs       # Droplet matrix generation
│   ├── nuclera_generator.rs         # Nuclera cartridge config generation
│   ├── protocol_generator.rs        # Wet-lab protocol generation
│   └── correlation_analyzer.rs      # RMSE, Pearson, threshold analysis
├── ui/
│   ├── mod.rs                       # Shared UI utilities, color functions
│   ├── simulation_importer.rs       # Panel 1: Import
│   ├── card_editor.rs               # Panel 2: Card design palette
│   ├── parameter_mapping.rs         # Panel 3: Param mapping view
│   ├── experiment_design.rs         # Panel 4: Matrix config & preview
│   ├── nuclera_config.rs            # Panel 5: Nuclera settings & export
│   ├── protocol_generator.rs        # Panel 6: Protocol view & export
│   └── data_viewer.rs               # Panel 7: Correlation analysis
└── visualization/
    ├── mod.rs                       # Visualization exports
    └── droplet_grid.rs              # Grid state resource (Bevy plugin)
```

---

## Sample Data

A sample Gumol simulation is included at `sample_simulation.json`. It contains:
- 3 radical species (O2-, H2O2, OH)
- 7 time points (0-60 units)
- Diffusion constants and reaction rates
- Damage threshold of 0.67 at 298.15 K

Use the **Load Sample Data** button in Step 1 to load it.

---

## Export Formats

| Output | Format | Step |
|--------|--------|------|
| Droplet matrix | CSV | Step 4 |
| Nuclera cartridge config | YAML | Step 5 |
| Wet-lab protocol | Markdown | Step 6 |
| Experiment data | CSV, JSON | Step 7 |
| Correlation report | JSON | Step 7 |

---

## Architecture

### State Management

All application state lives in a single Bevy `Resource` called `ApplicationState`. This is passed as `ResMut<ApplicationState>` to each UI panel, ensuring all panels share the same data and changes propagate immediately.

### UI Rendering

All UI is rendered through **egui** via the `bevy_egui` crate. The main `ui_system` function:
1. Renders the left navigation panel (160px)
2. Renders the bottom status bar
3. Dispatches to the active panel's render function

### Module Architecture

Core logic is separated from UI:
- `modules/` contains pure business logic (no UI dependencies)
- `ui/` contains panel rendering (calls into modules for operations)
- `visualization/` provides Bevy ECS state management

---

## Key Data Flow

```
Gumol Simulation JSON
    ↓  SimulationImporter::import_from_json()
GumolSimulation
    ↓  SimulationImporter::extract_feature_vector()
SimulationFeatureVector
    ↓  ParameterTranslationEngine::translate_simulation_to_experimental()
HashMap<String, Vec<f64>>  (parameter ranges)
    ↓  ExperimentDesigner::generate_matrix_from_config()
DropletMatrix
    ↓  NucleraGenerator::generate_config()
NucleraCartridgeConfig  →  YAML export
    ↓  ProtocolGenerator::generate_protocol()
ExperimentProtocol  →  Markdown export
    ↓  CorrelationAnalyzer::analyze_correlation()
CorrelationReport  →  JSON export
```

---

## License

See LICENSE file.
