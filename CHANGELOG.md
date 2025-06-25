# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-06-25

### 🐛 Bug Fixes

* Attempting to get cross working with `openssl`
* Missed updating the lock file
### 🚜 Refactor

* adding release build workflow
### ⚙️ Miscellaneous Tasks

* Update README to remove references to `_round` fields
* Add cliff to generate a changelog
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
* Refactor/cleanup
* Cleanup in preparation for releasing on Github
* Adding github workflows
* Add screenshot for waybar and template vars to the README
* Add more output examples
* Update README.md to clarify caching behavior and removal of the --use-cache option

