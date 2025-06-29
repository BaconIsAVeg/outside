# Changelog

All notable changes to this project will be documented in this file.

## [0.3.3] - 2025-06-29

### 💼 Other

* Fix geolocation API response handling to use `name` instead of `admin2`
### 🚜 Refactor

* Refactor API and Location Handling, update README
* Refactor API URL building into a utility module
## [0.3.2] - 2025-06-26

### 🚀 Features

* Additional context variables, including the 7 day forecast
## [0.3.1] - 2025-06-25

### 🐛 Bug Fixes

* Attempting to get cross working with `openssl`
### ⚙️ Miscellaneous Tasks

* Update README to remove references to `_round` fields
* Update badge name
## [0.2.0] - 2025-06-24

### 🚀 Features

* Most of the API lookup framework is now in place
* Building out the context used for the templating engine
* Add compass direction mapping and update context with new fields
* Cache location (1h) and weather data (~10m)
* Add the missing wind gusts field to the weather context
* Add cache age to context
* Add additional daily weather forecast data
* Adding additonal output default templates
* Add rounded units to the context, convert sunrise and sunset datetimes to times
* Add support for config.yaml templates
* Add context aware CSS classes to Waybar output
### 🐛 Bug Fixes

* Somehow, I messed up the range for Fog
### ⚙️ Miscellaneous Tasks

* Initial commit
* Cleanup in preparation for releasing on Github
* Adding github workflows
* Add screenshot for waybar and template vars to the README
* Add more output examples
* Update README.md to clarify caching behavior and removal of the --use-cache option

