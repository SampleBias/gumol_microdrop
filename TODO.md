# Gumol MicroDrop / eProtein Design Studio — Refactor & Roadmap

**Last updated:** 2026-03-18  
**Strategy:** Keep the Bevy + bevy_egui shell; add a parallel **eProtein Discovery** workflow aligned with Nuclera’s design → express/purify → scale-up story. Gumol ROS workflow remains available.

---

## Refactor phases (current plan)

### Phase 0 — Foundation ✅ **done** (2026-03-21)
- [x] This `TODO.md` as single source of truth for refactor tasks
- [x] `WorkflowMode`: `GumolRos` | `EproteinDiscovery` in `ApplicationState`
- [x] `eprotein` module: core types (`Construct`, `CellFreeBlend`, screen kinds, matrix slots)
- [x] `EproteinNavPanel` + dual navigation in `main.rs` (window title: **MicroDrop Design Studio**)
- [x] `modules/eprotein_screen.rs`: build **192** (soluble) / **88** (membrane) slots; unit tests
- [x] Placeholder UI: `ui/eprotein/design.rs`, `screen_palette.rs`, `placeholders.rs`

### Phase 1 — eProtein Design panel (constructs + predictions prep)
- [ ] FASTA / CSV import for variant lists
- [ ] Edit construct metadata (name, linear vs circular, tags)
- [ ] Add `reqwest` + optional `tokio` (or blocking HTTP) for prediction APIs
- [ ] `predictions` module: trait `PredictionClient` + first provider (e.g. BioLM solubility or DNA metrics — TBD with your API keys)
- [ ] Cache prediction results on `Construct` / sidecar map in `EproteinProjectState`

### Phase 2 — Screen matrix UI (microdrop palette)
- [ ] Visual grid for 192 or 88 slots (construct index × blend index), reusing egui patterns from `card_editor`
- [ ] Color modes: by construct, by blend, expression tier (placeholder until real data)
- [ ] Export screen layout CSV aligned with Nuclera handoff (validate against real Nuclera export samples when available)

### Phase 3 — Instrument export & protocol copy
- [ ] Extend `NucleraGenerator` or add `EproteinCartridgeConfig` YAML schema documented against Nuclera docs
- [ ] Protocol text: eGene prep → cartridge load → expression screen → purification ranking → scale-up hints
- [ ] File dialogs for YAML / Markdown (reuse existing patterns)

### Phase 4 — Results & scale-up
- [ ] Import expression / purification results (CSV/JSON) from cloud export or manual file
- [ ] Rank expressors; table + simple charts (egui)
- [ ] Scale-up panel: selected construct + blend, volumes, checklist

### Phase 5 — Quality & docs
- [ ] Unit tests: `eprotein_screen`, parsers, API client (mocked)
- [ ] README section: workflow switch, API keys, Nuclera disclaimer (no public third-party API guaranteed)
- [ ] Optional: rename binary / crate to reflect dual product (later)

---

## Legacy product (Gumol ROS) — backlog

See sections below for tests, CSV import, and UI polish that still apply to **GumolRos** mode only.

### Completed (historical)
- [x] Gumol pipeline end-to-end (import → matrix → YAML → protocol → correlation)
- [x] Sample `sample_simulation.json` + `sample_experimental_results.csv`
- [x] README platform-agnostic note

### High priority (Gumol)
- [ ] Unit tests for core `modules/*` (importer, translation, designer, nuclera, protocol, correlation)
- [ ] Real CSV import for Gumol simulations

### Medium priority
- [ ] Per-cell editing, presets, theme, undo/redo

---

## Decision log

| Date | Decision |
|------|----------|
| 2026-03-18 | **Refactor, not scrap:** shared Bevy/egui shell; new `eprotein` domain + `WorkflowMode` toggle. |
| 2026-03-18 | Nuclera integration assumed **file + cloud UX handoff** until a partner API is confirmed. |

---

## Quick links (code)

| Area | Path |
|------|------|
| Workflow + Gumol state | `src/data_models.rs` |
| eProtein types | `src/eprotein/models.rs` |
| 192 / 88 matrix builder | `src/modules/eprotein_screen.rs` |
| eProtein UI | `src/ui/eprotein/*.rs` |
| App entry / nav | `src/main.rs` |
