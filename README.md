# Gumol MicroDrop Design Studio

A native Rust application for converting molecular dynamics oxidative stress simulations from the **Gumol engine** into experimentally executable microdroplet assays. The tool generates platform-agnostic experiment designs — droplet matrices, cartridge configurations, and wet-lab protocols — that can be adapted to a range of digital microfluidics systems and standard laboratory equipment.

### Dual workflow (sidebar)

Use the **Workflow** selector in the left panel:

| Mode | Purpose |
|------|---------|
| **Gumol ROS** | Original pipeline: Gumol simulation → oxidative stress droplet matrix → exports → correlation |
| **eProtein** | Nuclera-aligned path: construct design → **192** (soluble) or **88** (membrane) expression screen → export / results / scale-up (panels expand in upcoming phases) |

See `TODO.md` for the refactor roadmap and phase checklist.

---

## Purpose

Researchers running Gumol simulations of reactive oxygen species (ROS) dynamics need a way to validate computational predictions with real wet-lab experiments. This tool bridges that gap by:

1. Importing Gumol simulation outputs (JSON/CSV)
2. Extracting feature vectors (superoxide density, H2O2 peaks, diffusion rates, etc.)
3. Translating simulation parameters to experimental ranges
4. Generating droplet condition matrices for microdroplet assays
5. Producing cartridge/plate configurations (YAML)
6. Creating step-by-step wet-lab protocols (Markdown)
7. Analyzing correlation between predictions and measurements

---

## Platform Compatibility

This tool is designed to be **platform-agnostic**. While the configuration output uses the Nuclera eProtein Discovery system as a reference format, the generated experiment designs (droplet matrices, concentration gradients, exposure time series, and readout parameters) are portable to any system that can execute a combinatorial microdroplet or microplate assay.

### Supported Execution Platforms

| Platform | How It Maps | Notes |
|----------|-------------|-------|
| **General-purpose EWOD platforms** (e.g., Sci-Bots DropBot, academic research-grade systems) | Direct match — the YAML config describes droplet volumes, reagent wells, mixing cycles, and incubation times that map to programmable EWOD protocols | Full control over cartridge design and assay chemistry; ideal for custom ROS/oxidative stress assays |
| **Droplet microfluidics (flow-based)** | The droplet condition matrix defines the combinatorial space; each row becomes a droplet generation recipe | High-throughput screening; well-suited for large factorial designs exceeding 96 conditions |
| **Standard 96-well plate reader + liquid handler** | The default 8×12 grid maps directly to a standard microplate; oxidant/antioxidant concentrations become pipetting instructions; fluorescence readout at 520nm matches standard plate reader optics | Most accessible option — no specialized microfluidics hardware required |
| **Nuclera eProtein Discovery** | Reference target for YAML export format | Nuclera's platform is purpose-built for cell-free protein expression screening, not ROS assays; however, its EWOD cartridge format serves as a well-documented reference for the configuration schema |

### Why Platform-Agnostic?

The core value of this tool is **translating computational simulation parameters into experimental designs** — not controlling a specific instrument. The pipeline from Gumol simulation → feature extraction → parameter mapping → factorial matrix is entirely independent of execution hardware. The "Nuclera Config" step (Step 5) is best understood as a **generic digital microfluidics configuration** that uses Nuclera's schema as a concrete, well-structured output format. Adapting the YAML output to another platform's protocol format is straightforward since all the essential parameters (reagent identities, concentrations, volumes, mixing steps, incubation times, and readout settings) are explicitly defined.

### Recommended Setup for ROS/Oxidative Stress Assays

For the specific use case of validating Gumol oxidative stress simulations:

- **Reagents:** H2O2 solutions (0–250 µM), antioxidant enzymes (SOD3, Catalase, GPx), ROS fluorescent probe (e.g., H2DCFDA or CellROX)
- **Readout:** Fluorescence at Ex/Em 485/520 nm — compatible with virtually all plate readers and fluorescence detectors
- **Format:** 96-well microplate (8 rows × 12 columns) or equivalent digital microfluidics cartridge
- **Volume:** 5 nL per droplet (microfluidics) or 100–200 µL per well (microplate, scaled up)

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

### Step 5: Platform Configuration (Nuclera Format)
Configure cartridge/plate-level settings (volume, mixing cycles, readout method/wavelength). Generate a configuration in Nuclera-compatible YAML format — adaptable to any EWOD platform, droplet system, or standard microplate setup (see Platform Compatibility above).

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

Two sample files are included for demonstrating the full workflow:

**`sample_simulation.json`** — Gumol oxidative stress simulation:
- 3 radical species (O2-, H2O2, OH)
- 20 time points (0–120 units) with progressive ROS accumulation
- 5 reaction rate constants (dismutation, decomposition, scavenging, lipid peroxidation, protein oxidation)
- Damage threshold of 0.67 at 310.15 K (physiological temperature)

**`sample_experimental_results.csv`** — Simulated wet-lab measurements:
- 40 droplet conditions (5 oxidant levels × 2 treatments × 4 time points)
- Fluorescence intensity (RFU), viability (%), and Gumol-predicted damage
- Demonstrates SOD3 attenuation of oxidative damage across the dose-response curve

Use **Load Sample Data** (Step 1) and **Load Sample Results** (Step 7) to walk through the complete pipeline.

---

## Export Formats

| Output | Format | Step |
|--------|--------|------|
| Droplet matrix | CSV | Step 4 |
| Platform config (Nuclera format) | YAML | Step 5 |
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

## A Note on the Nuclera Reference

The Nuclera eProtein Discovery system is a cell-free protein expression screening platform — its primary purpose is not ROS or oxidative stress assays. We use its YAML configuration schema as a **reference format** because it is a well-documented, real-world digital microfluidics specification that captures all the parameters needed to describe a combinatorial droplet experiment (reagent locations, volumes, mixing, incubation, and fluorescence readout). The intent is to keep the tool general enough that as the right platform emerges — whether that's a repurposed EWOD system, a custom microfluidics setup, or a standard plate reader workflow — the experimental designs generated here translate directly.

---

## License

See LICENSE file.
