# Gumol MicroDrop Design Studio — Development TODO

Last updated: 2026-03-18

---

## Completed

- [x] Project scaffolding (Bevy + bevy_egui + modules)
- [x] Core data models (GumolSimulation, DropletMatrix, NucleraCartridgeConfig, etc.)
- [x] Simulation importer — JSON import and feature vector extraction
- [x] Parameter translation engine — simulation → experimental ranges
- [x] Experiment designer — config-driven matrix generation with cartridge optimization
- [x] Nuclera config generator — cartridge config + YAML export
- [x] Protocol generator — step-by-step protocol + Markdown export
- [x] Correlation analyzer — RMSE, Pearson r, threshold accuracy
- [x] Shared ApplicationState across all panels (ResMut)
- [x] Panel 1: Simulation Import — file dialog, sample data, feature display
- [x] Panel 2: Card Editor — centerpiece grid, color modes, Generate Matrix, cell selection
- [x] Panel 3: Parameter Mapping — feature values, translated ranges, recalculate
- [x] Panel 4: Experiment Design — matrix generation, preview table, CSV export
- [x] Panel 5: Nuclera Config — settings, YAML generation, preview, file export
- [x] Panel 6: Protocol Generator — protocol generation, equipment/reagent lists, Markdown export
- [x] Panel 7: Data Viewer — sample correlation analysis, scatter plot, JSON/CSV export
- [x] Left nav panel with status indicators
- [x] Bottom status bar
- [x] .gitignore (excludes target/)
- [x] Sample simulation data (sample_simulation.json)
- [x] README.md
- [x] End-to-end data flow wired up (import → design → export)

---

## In Progress

- [ ] **Testing** — Verify full workflow: import sample → generate matrix → export YAML/protocol

---

## High Priority

### Testing & Quality
- [ ] Add unit tests for `SimulationImporter` (JSON parsing, feature extraction edge cases)
- [ ] Add unit tests for `ParameterTranslationEngine` (range computation)
- [ ] Add unit tests for `ExperimentDesigner` (matrix generation, cartridge optimization)
- [ ] Add unit tests for `NucleraGenerator` (config generation, YAML serialization)
- [ ] Add unit tests for `ProtocolGenerator` (step generation, Markdown output)
- [ ] Add unit tests for `CorrelationAnalyzer` (RMSE, Pearson, threshold accuracy)
- [ ] Integration test: full pipeline from JSON import to YAML/Markdown export

### CSV Import
- [ ] Implement real CSV parsing in `SimulationImporter::import_from_csv()` (currently a stub)
- [ ] Define and document the expected CSV column format
- [ ] Add CSV format auto-detection

### Data Viewer Enhancements
- [ ] Load real experimental data from CSV files (not just synthetic sample data)
- [ ] Parse experimental CSV into `ExperimentalDataPoint` structs
- [ ] Correlation analysis against actual measurements

---

## Medium Priority

### UI Polish
- [ ] Add undo/redo for card editor changes
- [ ] Persist UI state to disk between sessions (recent files, last configuration)
- [ ] Add keyboard shortcuts for common actions (Ctrl+S export, Ctrl+G generate)
- [ ] Add confirmation dialogs before overwriting exported files
- [ ] Improve card editor: allow editing individual droplet conditions by clicking cells
- [ ] Add drag-select for multiple cell editing in card editor
- [ ] Dark/light theme toggle

### Card Editor Enhancements
- [ ] Custom antioxidant concentration per condition (currently fixed at 50 U/mL)
- [ ] Per-row and per-column condition assignment (batch editing)
- [ ] Template presets (e.g., "Standard ROS Panel", "Dose Response")
- [ ] Import card design from CSV

### Export Improvements
- [ ] PDF export for protocols (currently Markdown only)
- [ ] Excel (.xlsx) export for matrix data
- [ ] Parquet export for large datasets
- [ ] Batch export (all outputs in one operation)

### Nuclera Integration
- [ ] Validate cartridge config against Nuclera hardware constraints
- [ ] Support multiple cartridge sizes (not just 96-well)
- [ ] Routing optimization for droplet generation order

---

## Low Priority / Future

### Advanced Features
- [ ] HDF5 import support for large simulations
- [ ] Bayesian optimization for experiment design (minimize conditions needed)
- [ ] Async file loading for large simulation files (prevent UI blocking)
- [ ] Real-time visualization updates when parameters change (animated grid)
- [ ] Closed-loop simulation refinement (feed experimental data back to Gumol)
- [ ] Multi-simulation comparison view

### Visualization
- [ ] Restore Bevy 2D/3D droplet grid rendering (currently egui-only)
- [ ] Heatmap visualization in Data Viewer
- [ ] Error distribution histogram
- [ ] Residual plots
- [ ] Confidence interval overlays on scatter plot

### Infrastructure
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Cross-platform packaging (AppImage for Linux, .dmg for macOS, .msi for Windows)
- [ ] Automated release builds
- [ ] Performance benchmarks for large matrices (>1000 droplets)

### API & Integration
- [ ] REST API for headless operation (generate configs without GUI)
- [ ] Nuclera platform API integration (when available)
- [ ] Gumol engine direct integration (trigger simulations from the tool)

---

## Notes for Developers

### Build Commands
```bash
cargo run          # Dev build (fast compile, dynamic linking)
cargo run --release  # Release build (optimized)
cargo check        # Type-check only
cargo test         # Run tests
cargo fmt          # Format code
cargo clippy       # Lint
```

### Architecture Quick Reference
- All shared state: `ApplicationState` in `src/data_models.rs`
- Business logic: `src/modules/*.rs` (no UI deps)
- UI panels: `src/ui/*.rs` (each is a `render()` function)
- Entry point: `src/main.rs` → `ui_system()` dispatches to panels

### Adding a New Panel
1. Create `src/ui/new_panel.rs` with a `pub fn render(ctx, state, panel)` function
2. Add `pub mod new_panel;` to `src/ui/mod.rs`
3. Add variant to `CurrentPanel` enum in `src/data_models.rs`
4. Add match arm in `ui_system()` in `src/main.rs`
5. Add nav entry in the side panel

### Adding New State
1. Add field to `ApplicationState` in `src/data_models.rs`
2. Set default in the `Default` impl
3. Access via `state.field_name` in panel render functions
