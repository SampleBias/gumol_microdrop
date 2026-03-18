# AGENTS.md - Gumol MicroDrop Design Studio

## Project Overview

Gumol MicroDrop Design Studio is a Rust native application built with Bevy that enables scientific researchers to convert molecular dynamics oxidative stress simulations from the Gumol engine into experimentally executable microdroplet assays using the Nuclera eProtein Discovery digital microfluidics platform.

**Technology Stack:**
- **Language:** Rust (native, no WebAssembly)
- **Visualization Engine:** Bevy (2D/3D rendering, not WebGL)
- **UI Framework:** bevy_egui (egui integration for Bevy)
- **Data Serialization:** serde, serde_json, serde_yaml
- **File I/O:** rfd (cross-platform file dialogs), csv, chrono

**Project Purpose:**
- Analyze Gumol simulation outputs
- Infer oxidative stress parameters
- Translate to microdroplet assay conditions
- Generate Nuclera cartridge configurations
- Produce wet-lab protocols and experiment matrices
- Output data schemas for correlation analysis

## Essential Commands

### Build
```bash
cargo build --release
```

### Run (Development)
```bash
cargo run
```

### Run (Release)
```bash
cargo run --release
```

### Check Code
```bash
cargo check
```

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

## Project Structure

```
src/
├── main.rs                  # Application entry point, Bevy app setup
├── lib.rs                   # Library exports
├── data_models.rs           # Core data structures (simulations, droplets, configs)
├── modules/
│   ├── mod.rs               # Module exports
│   ├── simulation_importer.rs   # Import and parse Gumol simulations
│   ├── parameter_translation.rs # Convert simulation to experimental params
│   ├── experiment_designer.rs   # Generate droplet matrices
│   ├── nuclera_generator.rs     # Create Nuclera cartridge configs
│   ├── protocol_generator.rs    # Generate wet-lab protocols
│   └── correlation_analyzer.rs # Analyze simulation vs experiment correlation
├── ui/
│   ├── mod.rs               # UI module exports and shared utilities
│   ├── simulation_importer.rs   # Panel 1: Import simulations
│   ├── card_editor.rs           # Panel 2: Visual card editing
│   ├── parameter_mapping.rs    # Panel 3: Parameter mapping
│   ├── experiment_design.rs    # Panel 4: Matrix design
│   ├── nuclera_config.rs       # Panel 5: Nuclera configuration
│   ├── protocol_generator.rs   # Panel 6: Protocol generation
│   └── data_viewer.rs          # Panel 7: Data and correlation analysis
└── visualization/
    ├── mod.rs               # Visualization module exports
    └── droplet_grid.rs      # Bevy-based droplet grid visualization
```

## Code Organization and Conventions

### Architecture Pattern

The application follows a modular architecture with clear separation of concerns:

1. **Data Models** (`data_models.rs`): All serializable data structures
2. **Core Logic Modules** (`modules/`): Business logic for each functional component
3. **UI Panels** (`ui/`): User interface implementations using egui
4. **Visualization** (`visualization/`): Bevy-based visual components

### Bevy Application Setup

The app is initialized in `main.rs` with:
- `DefaultPlugins` for core Bevy functionality
- `EguiPlugin` for UI rendering
- `DropletGridPlugin` for Bevy visualization
- Systems for startup and updates
- Resources for application state (`CurrentPanel`, `ApplicationState`)

### UI System

UI is rendered through 7 main panels:
1. Simulation Import - Load Gumol simulation data
2. Card Editor - Visually design experiment cards
3. Parameter Mapping - Map simulation to experimental parameters
4. Experiment Design - Configure droplet matrices
5. Nuclera Config - Generate cartridge configurations
6. Protocol Generator - Create wet-lab protocols
7. Data Viewer - Analyze correlations

Navigation is managed through the `CurrentPanel` resource.

### Visualization System

The `DropletGridPlugin` provides a visual representation of the experiment card:
- Grid layout (default 8x12 = 96 wells)
- Color-coded droplets based on experimental conditions
- Interactive selection via mouse clicks
- Multiple color modes (Oxidant, Antioxidant, ExposureTime, Treatment)
- Legend display

## Data Flow

```
Gumol Simulation (JSON/CSV)
    ↓
SimulationImporter (extract feature vectors)
    ↓
ParameterTranslationEngine (map to experimental ranges)
    ↓
ExperimentDesigner (generate droplet matrix)
    ↓
NucleraGenerator (create cartridge config)
    ↓
ProtocolGenerator (produce wet-lab protocol)
    ↓
CorrelationAnalyzer (validate against experimental data)
```

## Key Data Structures

### Core Models

- **GumolSimulation**: Input simulation data from Gumol engine
- **SimulationFeatureVector**: Extracted features from simulation
- **DropletCondition**: Single experimental condition
- **DropletMatrix**: Collection of droplet conditions
- **NucleraCartridgeConfig**: Nuclera platform configuration
- **ExperimentProtocol**: Wet-lab SOP-style protocol
- **CorrelationReport**: Validation metrics

### Card Design

- **CardDesign**: Visual card representation with grid layout
- Stores droplet conditions and metadata
- Configurable rows/columns (default 8x12)
- Total wells = rows * cols

## Naming Conventions

- **Struct names**: PascalCase (e.g., `GumolSimulation`, `DropletCondition`)
- **Function names**: snake_case (e.g., `import_from_json`, `generate_matrix`)
- **Constants/Modules**: snake_case (e.g., `CurrentPanel`, `simulation_importer`)
- **Resources**: PascalCase (e.g., `DropletGridState`, `ApplicationState`)
- **Components**: PascalCase (e.g., `Droplet`, `DropletGrid`)

## Important Patterns

### Bevy ECS Patterns

- Use `#[derive(Component)]` for Bevy components
- Use `#[derive(Resource)]` for application state
- Systems use `Query<>` for component access
- Resources for global state management

### Error Handling

- Use `anyhow::Result` for error propagation
- Use `anyhow::Context` for error context
- Methods return `Result<T>` for fallible operations

### Serialization

- Use `#[derive(Serialize, Deserialize)]` for data models
- Support JSON and YAML formats
- Use `serde_json` and `serde_yaml` crates

### UI Construction

- Use egui's fluent API for UI elements
- Follow hierarchical structure: panels → groups → widgets
- Use `ui.heading()`, `ui.label()`, `ui.button()` for common elements
- Use `egui::Grid` for tabular layouts
- Use `egui::ScrollArea` for scrollable content

## Testing Approach

**Note:** As of the initial commit, no tests are present in the codebase.

### Recommended Test Structure

When adding tests:

```
src/
├── data_models.rs
├── modules/
│   ├── simulation_importer.rs
│   ├── simulation_importer_tests.rs    # Unit tests for importer
│   ├── parameter_translation.rs
│   ├── parameter_translation_tests.rs  # Unit tests for translation
│   └── ...
```

### Test Patterns

- Unit tests for each module's core functions
- Integration tests for data flow across modules
- Use `cargo test` to run all tests
- Mock file I/O for testing without side effects

## Important Gotchas

### Bevy Integration

1. **UI + Visualization:** The app uses both egui (for panels) and Bevy ECS (for visualization). These run in parallel but must coordinate through resources.

2. **Resource Access:** Multiple systems may access the same resources. Use `Res` for read-only and `ResMut` for write access.

3. **System Ordering:** Ensure proper system ordering when one system depends on another's output.

### UI State Management

1. **Panel Navigation:** The `CurrentPanel` resource controls which UI panel is displayed. Only one panel is active at a time.

2. **Droplet Selection:** The `DropletGridState` resource tracks selected droplets in the visualization.

3. **State Updates:** UI state changes must be reflected in the Bevy visualization through resource updates.

### File I/O

1. **Async Considerations:** File dialogs use `rfd` which is async-capable, but the current implementation is synchronous. Consider async if performance becomes an issue.

2. **Path Handling:** Use `std::path::PathBuf` for cross-platform file paths.

### Data Validation

1. **Simulation Inputs:** Validate that imported simulations contain required fields before processing.

2. **Matrix Constraints:** Ensure droplet matrices respect cartridge capacity limits.

3. **Configuration Export:** Validate Nuclera configurations before export.

### Performance Considerations

1. **Grid Rendering:** The droplet grid renders 96 individual entities. Consider instancing for larger grids.

2. **UI Updates:** egui rebuilds UI each frame. Optimize by caching static elements where possible.

3. **File Operations:** Large simulation files may block the UI thread. Consider async loading for production.

## Color Modes in Visualization

The droplet grid supports multiple visualization modes via `ColorMode` enum:

- **Oxidant:** Color based on oxidant concentration (white to red gradient)
- **Antioxidant:** Color based on antioxidant presence (white to green gradient)
- **ExposureTime:** Color based on incubation time (blue gradient)
- **Treatment:** Color based on treatment type (categorical colors)

Change mode by updating `DropletGridState.color_mode` resource.

## Export Formats

### Nuclera Configuration
- Format: YAML
- Export function: `NucleraGenerator::export_to_yaml()`
- Contains droplet configs, routing, readout settings

### Protocol
- Format: Markdown
- Export function: `ProtocolGenerator::export_to_markdown()`
- Contains equipment, reagents, step-by-step procedures

### Data
- Formats: CSV, JSON, Parquet (planned)
- Schema includes droplet_id, concentrations, treatments, measurements

## Dependencies Summary

**Core:**
- `bevy` - Game engine and visualization
- `bevy_egui` - UI rendering
- `serde` - Serialization framework

**Data:**
- `serde_json` - JSON support
- `serde_yaml` - YAML support
- `csv` - CSV parsing
- `ndarray` - Array operations (planned use)

**Utilities:**
- `anyhow` - Error handling
- `thiserror` - Error types
- `rfd` - File dialogs
- `chrono` - Date/time handling
- `rand` - Random number generation

## Future Enhancement Areas

1. **Add comprehensive test suite** across all modules
2. **Implement async file loading** for large simulations
3. **Add HDF5 support** for simulation import
4. **Implement real-time visualization** updates when parameters change
5. **Add undo/redo** functionality for card editing
6. **Integrate with actual Nuclera API** when available
7. **Add Bayesian optimization** for experiment design
8. **Implement closed-loop simulation refinement**

## Working with This Codebase

When adding new features:

1. **Start with data models** in `data_models.rs` if new structures are needed
2. **Implement core logic** in appropriate module under `modules/`
3. **Create UI panel** in `ui/` if user interaction is required
4. **Add visualization** in `visualization/droplet_grid.rs` if visual feedback is needed
5. **Update navigation** in `main.rs` if new panel is added
6. **Export functionality** should be implemented in the relevant module

When modifying existing features:

1. **Read the entire file** before making changes
2. **Maintain the module boundaries** - don't mix concerns
3. **Update both UI and visualization** if state changes affect both
4. **Test the data flow** end-to-end if modifying pipeline components
5. **Update this documentation** if architectural patterns change
