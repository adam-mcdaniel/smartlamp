# smartlamp

A library to control Govee IOT lights.

_This library is built upon [Matt Chun-Lum's library](https://github.com/mattcl/govee-rs), which is a work in progress._

## Usage

List of supported light models:
 - `H6160`, `H6163`, `H6104`, `H6109`, `H6110`,
 - `H6117`, `H6159`, `H7021`, `H7022`, `H6086`,
 - `H6089`, `H6182`, `H6085`, `H7014`, `H5081`,
 - `H6188`, `H6135`, `H6137`, `H6141`, `H6142`,
 - `H6195`, `H7005`, `H6083`, `H6002`, `H6003`,
 - `H6148`, `H6052`, `H6143`, `H6144`, `H6050`,
 - `H6199`, `H6054`, `H5001`


### Steps to follow

1. Download the "Govee Home" app.
2. Go to `My Profile` -> `About Us` -> `Apply for API Key`
3. Wait for the email, and copy your API key to an environment variable named `GOVEE_API_KEY`.
4. Clone this git repository, and write your code in `src/bin.rs`.
5. Finally, execute `cargo run`

And then you're done!

IMPORTANT NOTE: `No devices found` refers to the list of _valid_ devices associated with your account. _If you have a light that doesn't have a supported model number, this program will not find your light._

## Example Code

This code searches for all lights associated with your account, turns them on, sets their colors to red, sets their brightness to 50%, and their color temperature to balanced (perfectly between warm and cold).

```rust
use smartlamp::{Light, Error};

fn main() -> Result<(), Error> {
    let lights = Light::all_lights()?;

    for light in lights {
        light.turn_on()?;
        // red
        light.set_color(255, 0, 0)?;

        // brightness of bulb
        light.set_brightness(50);
        // warmer or colder temperature
        light.set_color_temperature(50);
    }

    Ok(())
}
```