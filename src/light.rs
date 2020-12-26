use govee_rs::schema::{Color, Device, PowerState};
use std::fmt;

use super::{CLIENT, Error};

pub struct Light(Device);

impl Light {
    fn from_device(device: &Device) -> Self {
        Self(device.clone())
    }
    
    pub fn all_lights() -> Result<Vec<Self>, Error> {
        let device_request = CLIENT.devices();

        if let Ok(device_list) = device_request {
            let list = device_list.devices.iter().map(Self::from_device).collect::<Vec<Self>>();

            if list.is_empty() {
                Err(Error::NoDevicesFound)
            } else {
                Ok(list)
            }
        } else {
            Err(Error::NoDevicesFound)
        }
    }

    pub fn all_lights_with_names(names: Vec<String>) -> Result<Vec<Self>, Error> {
        // Get the list of all devices
        let lights = Self::all_lights()?;
        
        // Filter them for only the ones with given names
        let filtered_devices: Vec<Self> = lights
            .into_iter()
            .filter(|light| names.contains(&light.get_device().name))
            .collect();
        
        if filtered_devices.len() < 1 {
            // Return an error if no devices were found
            Err(Error::NoMatchingDevices)
        } else {
            // Otherwise, return the value devices
            Ok(filtered_devices)
        }
    }
    
    fn get_device(&self) -> &Device {
        &self.0
    }

    pub fn get_name(&self) -> &String {
        return &self.get_device().name
    }

    pub fn turn_on(&self) -> Result<(), Error> {
        Ok(CLIENT.toggle(self.get_device(), PowerState::On)?)
    }

    pub fn turn_off(&self) -> Result<(), Error> {
        Ok(CLIENT.toggle(self.get_device(), PowerState::Off)?)
    }

    pub fn set_color(&self, r: u32, b: u32, g: u32) -> Result<(), Error> {
        Ok(CLIENT.set_color(self.get_device(), &Color { r, b, g })?)
    }

    pub fn set_brightness(&self, value: u32) -> Result<(), Error> {
        Ok(CLIENT.set_brightness(self.get_device(), value)?)
    }

    pub fn set_color_temperature(&self, value: u32) -> Result<(), Error> {
        Ok(CLIENT.set_color_temperature(self.get_device(), value)?)
    }
}


impl fmt::Debug for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", CLIENT.state(&self.get_device()))
    }
}

