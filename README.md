# Outside

![demo](https://github.com/BaconIsAVeg/outside/blob/main/demo/demo.gif?raw=true)

A multi-purpose weather client for your terminal.

    Usage: outside [OPTIONS]

    Options:
      -l, --location <LOCATION>  Location to fetch weather data for,
                                 leave blank to auto-detect using your IP address
      -u, --units <UNITS>        Units of measurement [possible values: metric, imperial]
      -o, --output <OUTPUT>      Display format [possible values: tui, simple, detailed, json, waybar]
      -s, --stream               Enable streaming mode for continuous output
      -i, --interval <INTERVAL>  Interval in seconds between streaming updates [default: 30]
      -h, --help                 Print help
      -V, --version              Print version

The `--location` should be a string with your city and country code, e.g. `London, GB` or `New York, US`. If this value is not provided, http://ip-api.com will be used to auto-detect your location based on your IP address.  Location data is cached for 4 hours, and weather data is cached for 10 minutes to reduce API calls.

## Example Outputs

### Simple

    Overcast 18°C | Wind 713 | Precipitation 53%

### Detailed


    $ outside -o detailed
    Edmonton, CA
    Current:     17.6°C Overcast
    Feels Like:  17.5°C
    Humidity:    72%
    Pressure:    1006.9hPa
    Wind:        6.6km/h with gusts up to 13.0km/h (W)
    UV Index:    6.2
    Precip:      0.8 mm (53% chance)
    Sunrise:     05:07am
    Sunset:      10:06pm

    Fri 06/27    9-22°C - Rain showers, slight
    Sat 06/28    13-21°C - Thunderstorm
    Sun 06/29    11-24°C - Overcast
    Mon 06/30    14-25°C - Overcast
    Tue 07/01    15-25°C - Overcast
    Wed 07/02    14-30°C - Overcast
    Thu 07/03    16-24°C - Rain showers, slight


    $ outside -o detailed -l 'Los Angeles, US' -u imperial
    Los Angeles, US
    Current:     67.1°F Clear sky
    Feels Like:  68.9°F
    Humidity:    80%
    Pressure:    1012.5hPa
    Wind:        4.6mp/h with gusts up to 5.8mp/h (W)
    UV Index:    8.5
    Precip:      0.0 inch (0% chance)
    Sunrise:     05:43am
    Sunset:      08:08pm

    Fri 06/27    61-85°F - Fog
    Sat 06/28    58-87°F - Fog
    Sun 06/29    58-85°F - Fog
    Mon 06/30    65-77°F - Clear sky
    Tue 07/01    64-79°F - Clear sky
    Wed 07/02    64-77°F - Clear sky
    Thu 07/03    63-74°F - Clear sky

### JSON

    $ outside -o json | jq
    {
      "city": "Edmonton",
      "country": "CA",
      "temperature": 17.6,
      "temperature_low": 9.1,
      "temperature_high": 21.7,
      "feels_like": 17.5,
      "temperature_unit": "°C",
      "wind_speed": 6.6,
      "wind_gusts": 13.0,
      "wind_speed_unit": "km/h",
      "wind_direction": 257,
      "wind_compass": "W",
      "weather_code": 3,
      "weather_icon": "󰖐",
      "weather_description": "Overcast",
      "openweather_code": "04d",
      "humidity": 72,
      "humidity_unit": "%",
      "pressure": 1006.9,
      "pressure_unit": "hPa",
      "sunrise": "05:07am",
      "sunset": "10:06pm",
      "uv_index": 6.2,
      "precipitation_chance": 53,
      "precipitation_sum": 0.8,
      "precipitation_unit": "mm",
      "precipitation_hours": 4.0,
      "forecast": [
        {
          "date": "Fri 06/27",
          "weather_code": 80,
          "weather_icon": "󰖗",
          "weather_description": "Rain showers, slight",
          "openweather_code": "09d",
          "uv_index": 6.2,
          "precipitation_sum": 0.8,
          "precipitation_hours": 4.0,
          "precipitation_chance": 53,
          "temperature_high": 21.7,
          "temperature_low": 9.1
        },
        ...
        {
          "date": "Thu 07/03",
          "weather_code": 80,
          "weather_icon": "󰖗",
          "weather_description": "Rain showers, slight",
          "openweather_code": "09d",
          "uv_index": 4.5,
          "precipitation_sum": 4.8,
          "precipitation_hours": 3.0,
          "precipitation_chance": 35,
          "temperature_high": 23.7,
          "temperature_low": 16.1
        }
      ],
      "cache_age": 355
    }

### Waybar

    $ outside -o waybar | jq
    {
      "text": "󰖐 18°C 󰖗 53%",
      "tooltip": "Edmonton, CA\nOvercast\nFeels Like  17.5 °C\nForecast    9-22 °C\nHumidity    72%\nPressure    1006.9 hPa\nWind        6.613.0 km/h (W)\nPrecip      0.8 mm (53% chance)\n\n 05:07am    10:06pm",
      "class": [],
      "percentage": 100
    }

# Installation

### From crates.io

```bash
cargo install outside
```

### From Source

```bash
cargo build --release
cargo install --path .
```

### Debian Package

You will need the `ca-certificates` and `openssl` packages if they're not already installed.

```bash
apt update
dpkg -i outside_0.4.1_amd64.deb
apt-get -f install
```

### Alpine Linux

```bash
apk add --allow-untrusted outside_0.4.1_x86_64.apk
```

### Redhat / RPM Based Distributions

```bash
rpm -i outside-0.4.1-1.x86_64.rpm
```

### Other Linux Systems

```bash
tar zxf outside-0.4.1_Linux_x86_64.tar.gz -C /usr/local/bin outside 
```

# Configuration Options

As an alternative to passing the command line options, the application will look for the following configuration file:

```
~/.config/outside/config.yaml
```

An example configuration file:

```yaml
units: Metric
simple:
  template: "{weather_icon} {temperature | round}{temperature_unit} <U+F059D> {wind_speed | round}<U+EA9F>{wind_gusts | round}"
waybar:
  text: "{weather_icon} {temperature | round}{temperature_unit} <U+F059D> {wind_speed | round}<U+EA9F>{wind_gusts | round}"
  hot_temperature: 30
  cold_temperature: 0
```

### Available Template Variables

You can run `outside -o json` to see a list of all the current variables and their values.

# Waybar Configuration

![outside as a waybar module](https://github.com/BaconIsAVeg/outside/blob/main/screenshot.png?raw=true)

Add the following configuration to your Waybar config file (usually located at `~/.config/waybar/config.jsonc`):

```jsonc
"custom/weather": {
  "exec": "/path/to/outside -o waybar -s",
  "format": "{text}",
  "tooltip": true,
  "return-type": "json",
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
