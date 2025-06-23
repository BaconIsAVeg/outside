# Outside

A multi-purpose CLI weather client that uses the Open-Meteo API.

```
Usage: outside [OPTIONS]

Options:
  -l, --location <LOCATION>            'City, CA' or leave blank to auto-detect
  -u, --units <UNITS>                  Units of measurement [possible values: metric, imperial]
  -o, --output-format <OUTPUT_FORMAT>  Desired output format [possible values: simple, detailed, json, waybar]
      --use-cache <USE_CACHE>          Don't use cached location and weather data [possible values: true, false]
  -h, --help                           Print help
  -V, --version                        Print version
```

The `--location` should be a string with your city and country code, e.g. `London, GB` or `New York, US`. If this value is not provided, http://ip-api.com will be used to auto-detect your location based on your IP address.  Location data is cached for one hour, and weather data is cached for 10 minutes to reduce API calls. You can disable caching by setting `--use-cache false`.

# Configuration Options

As an alternative to passing the command line options, the application will look for the following configuration file:

```
~/.config/outside/config.yaml
```

An example configuration file:

```yaml
units: Metric
simple:
  template: "{weather_icon} {temperature_round}{temperature_unit} <U+F059D> {wind_speed_round}<U+EA9F>{wind_gusts_round}"
waybar:
  text: "{weather_icon} {temperature_round}{temperature_unit} <U+F059D> {wind_speed_round}<U+EA9F>{wind_gusts_round}"
  hot_temperature: 30
  cold_temperature: 0
```

### Available Template Variables

You can run `outside -o json` to see a list of all the current variables and their values.

```bash
$ outside -o json | jq
{
  "city": "Edmonton",
  "country": "CA",
  "temperature": 10.9,
  "temperature_round": "11",
  "feels_like": 10.0,
  "feels_like_round": "10",
  "temperature_unit": "°C",
  "wind_speed": 4.4,
  "wind_speed_round": "4",
  "wind_gusts": 11.5,
  "wind_gusts_round": "12",
  "wind_speed_unit": "km/h",
  "wind_direction": 351,
  "wind_compass": "N",
  "weather_code": 95,
  "weather_icon": "󰖓",
  "weather_description": "Thunderstorm",
  "openweather_code": "11d",
  "humidity": 89,
  "humidity_unit": "%",
  "pressure": 1015.7,
  "pressure_round": "1016",
  "pressure_unit": "hPa",
  "sunrise": "05:05am",
  "sunset": "10:07pm",
  "uv_index": 7.0,
  "precipitation_chance": 83,
  "precipitation_sum": 4.9,
  "precipitation_unit": "mm",
  "precipitation_hours": 8.0,
  "cache_age": 536
}
```

# Installation

## From Source

You can install the `outside` binary by checking out this repository and then using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):

```bash
cargo build --release
cargo install --path .
```

# Waybar Configuration

![outside as a waybar module](https://github.com/BaconIsAVeg/outside/blob/main/screenshot.png?raw=true)

Add the following configuration to your Waybar config file (usually located at `~/.config/waybar/config.jsonc`):

```jsonc
"custom/weather": {
  "exec": "/path/to/outside -o waybar",
  "return-type": "json",
  "interval": 60,
}
```

And the corresponding CSS to style the widget (usually located at `~/.config/waybar/style.css`). Feel free to adjust the CSS to your liking:

```css
#custom-weather {
  padding: 0.3rem 0.6rem;
  margin: 0.4rem 0.25rem;
  border-radius: 6px;
  background-color: #1a1a1f;
  color: #f9e2af;
}
```

**Important**: You will also need a nerd patched font to display the weather icons. You can find one at [Nerd Fonts](https://www.nerdfonts.com/). Many distributions already include these fonts, so you may not need to install anything extra.

## Conditional Styling

You can also add conditional styling based on the weather condition. For example, to change the background color based on the weather condition and have the module blink during adverse conditions, you can use the following CSS:

```css
#custom-weather {
  animation-timing-function: linear;
  animation-iteration-count: infinite;
  animation-direction: alternate;
}

@keyframes blink-condition {
  to {
    background-color: #dedede;
  }
}

#custom-weather.hot {
  background-color: #dd5050;
}

#custom-weather.cold {
  background-color: #5050dd;
}

#custom-weather.rain,
#custom-weather.snow,
#custom-weather.fog {
  color: #dedede;
  animation-name: blink-condition;
  animation-duration: 2s;
}

```

# License

This project is licensed under the AGPL V3 or Greater - see the [LICENSE](LICENSE) file for details.
