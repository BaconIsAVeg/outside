# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2025-07-04

### 🚀 Features

* Adding a TUI output mode
* Adding cache age/refresh time to the TUI
### 🚜 Refactor

* cleaning up TUI.rs
* centralize weather classifiction into it's own file
## [0.3.5] - 2025-07-03

### 🚀 Features

* Support streaming output at specified intervals
## [0.3.4] - 2025-07-03

### 🚜 Refactor

* refactor: cleanup enum handling and add a shared http client for
## [0.3.3] - 2025-06-30

### 🐛 Bug Fixes

* Fix geolocation API response handling to use `name` instead of `admin2`
### 🚜 Refactor

* Refactor API and Location Handling, update README
* Refactor API URL building into a utility module
## [0.3.2] - 2025-06-30

### 🚀 Features

* Additional context variables, including the 7 day forecast
## [0.3.1] - 2025-06-30

### 🚜 Refactor

* adding release build workflow
## [0.3.0] - 2025-06-30

### 🚀 Features

* Most of the API lookup framework is now in place
* Building out the context used for the templating engine
* Add compass direction mapping and update context with new fields
* Cache location (1h) and weather data (~10m)
* Add the missing wind gusts field to the weather context
* Add cache age to context
* Add additional daily weather forecast data
* feat: Add cache control for location and weather data, enabled by
* Adding additonal output default templates
* Add rounded units to the context, convert sunrise and sunset datetimes to times
* Add support for config.yaml templates
* Add context aware CSS classes to Waybar output
* feat: Switch to `savefile` instead of `disk` for location and weather data caching, which
* feat: Switch from `xdg` to `dirs-next` for cache directory management,
* feat: Add a custom template formatter `| round` to round numbers in templates
### 🐛 Bug Fixes

* Somehow, I messed up the range for Fog

