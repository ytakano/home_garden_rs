use bme280::i2c::BME280;
use chrono::prelude::*;
use linux_embedded_hal::{Delay, I2cdev};
use rppal::gpio::Gpio;
use std::{error::Error, time::Duration};

const GPIO5: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    // using Linux I2C Bus #1 in this example
    let i2c_bus = I2cdev::new("/dev/i2c-1")?;

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus);

    // initialize the sensor
    let mut delay = Delay;
    bme280.init(&mut delay).unwrap();

    let mut pin = Gpio::new()?.get(GPIO5)?.into_output_low();
    let dur = Duration::from_millis(1000);

    loop {
        // measure temperature, pressure, and humidity
        let measurements = bme280.measure(&mut delay).unwrap();

        let local: DateTime<Local> = Local::now();
        println!("{local}");

        println!("温度\t{:.2}[°C]", measurements.temperature);
        println!("湿度\t{:.2}[%]", measurements.humidity);
        println!("気圧\t{:.2}[hP]", measurements.pressure / 100.0);
        println!();

        if pin.is_set_high() {
            pin.set_low();
        } else {
            pin.set_high();
        }

        std::thread::sleep(dur);
    }
}
