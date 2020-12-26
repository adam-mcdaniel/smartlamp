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