# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Outside** is a CLI weather client written in Rust that fetches weather data from the Open-Meteo API and supports multiple output formats (simple, detailed, JSON, Waybar, and TUI). The application features intelligent caching, location detection, and customizable templating.

## Development Commands

### Build and Run
```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run the application
cargo run

# Run with specific arguments  
cargo run -- --location "New York, US" --units imperial --output detailed

# Run with TUI interface
cargo run -- --location "London, GB" --output tui

# Install from source
cargo install --path .
```

### Testing
```bash
# Run tests (no test files currently exist in the codebase)
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting and Formatting
```bash
# Format code
cargo fmt

# Check code without building
cargo check

# Run clippy for linting
cargo clippy
```

### Release Management
```bash
# Generate changelog (uses git-cliff)
git cliff

# Create release with git-cliff
git cliff --tag v<version>
```

## Code Architecture

### Core Application Flow
1. **Configuration Loading**: Settings are loaded from `~/.config/outside/config.yaml` and CLI arguments via `cli-settings-derive`
2. **Location Resolution**: Auto-detection via IP geolocation or explicit city/country parsing
3. **Weather Data Fetching**: Open-Meteo API calls with intelligent caching (10 minutes for weather, 4 hours for location)
4. **Context Building**: Raw API data is transformed into a unified context structure
5. **Output Rendering**: Template-based rendering using `tinytemplate` with trait-based output formatters

### Module Structure

- **`src/main.rs`**: Application entry point, coordinates the full pipeline
- **`src/settings.rs`**: Configuration management using `cli-settings-derive` for unified CLI/config file handling
- **`src/api/`**: External API integrations
  - `mod.rs`: API module organization
  - `client.rs`: HTTP client utilities 
  - `weather.rs`: Open-Meteo API client with caching
  - `location.rs`: Location data management and caching
  - `geolocation.rs`: City/country-based location lookup
  - `iplocation.rs`: IP-based location detection
- **`src/context.rs`**: Central data transformation layer that converts raw API responses into a unified context structure
- **`src/output/`**: Output formatting system
  - `mod.rs`: Trait-based output system with template helpers
  - `simple.rs`, `detailed.rs`, `json.rs`, `waybar.rs`: Format-specific renderers
- **`src/tui/`**: Terminal User Interface components
  - `mod.rs`: TUI module organization and main Output trait implementation
  - `async_operations.rs`: Weather data fetching with background threads
  - `constants.rs`: TUI-specific constants and configuration
  - `keyboard_handlers.rs`: Keybinding management and user input handling
  - `location_manager.rs`: Location list management and caching
  - `state_manager.rs`: Application state management across TUI lifecycle
  - `ui_components.rs`: UI layout creation and theme configuration
  - `weather_display.rs`: Weather data presentation components
- **`src/utils/`**: Utility functions
  - `cache.rs`: File-based caching with hashed filenames
  - `conversions.rs`: Date/time and data format conversions
  - `mappings.rs`: Weather code to icon/description mappings
  - `unitstrings.rs`: Metric/Imperial unit string management
  - `urls.rs`: URL construction utilities
  - `weather_classification.rs`: Weather condition categorization for styling and conditional logic

### Key Design Patterns

1. **Caching Strategy**: Uses `savefile` for binary serialization with location+units-based cache keys
2. **Template System**: Custom `TinyTemplate` formatters (e.g., `round` filter) for output rendering
3. **Trait-based Output**: Common `Output` trait enables polymorphic rendering across formats
4. **Error Handling**: Uses `anyhow` for error propagation with panic on critical failures
5. **Configuration**: Hybrid CLI/config file approach using `cli-settings-derive`

### Dependencies

- **API Client**: `isahc` with JSON features
- **Serialization**: `serde` ecosystem (JSON, YAML)
- **CLI**: `clap` with derive features
- **Caching**: `savefile` for binary serialization
- **Templates**: `tinytemplate` for output formatting
- **Time**: `chrono` for date/time handling
- **TUI**: `cursive` for terminal user interface

### Configuration Files

- **User Config**: `~/.config/outside/config.yaml` for persistent settings
- **Build Config**: `Cargo.toml` with conventional commit linting configuration
- **Changelog**: `cliff.toml` configured for conventional commits and automated changelog generation

### Testing and Quality

The project uses standard Rust tooling with cargo commands. Currently no test files exist in the codebase. When adding new functionality, consider adding appropriate tests.

### Notable Implementation Details

- Weather data cached for 10 minutes, location data for 4 hours
- Cache files stored in platform-specific cache directories with hashed filenames
- Units enum affects both API parameters and cache keys
- Template variables available via `outside -o json` for debugging
- Waybar integration includes conditional CSS classes for weather conditions
- Streaming mode available with `--stream` for continuous output updates (not compatible with TUI mode)
- Configuration supports custom templates for simple and waybar outputs
- TUI mode provides an interactive terminal interface with:
  - Location list management on the right side (cached using existing mechanism)
  - Dynamic weather updates: switching locations fetches and displays new weather data
  - Loading indicators during weather data fetching
  - Background threads for non-blocking API calls
  - Terminal-native theming: uses your terminal's default colors and transparency
  - Confirmation dialogs for location deletion
  - Keybinds: 'a' to add location, 'd' to delete, Enter to switch, 'u' to toggle units, q/Esc to quit
  - Full terminal layout with weather display on left, location list on right