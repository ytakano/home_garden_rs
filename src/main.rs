#[macro_use]
extern crate diesel;

use bme280::i2c::BME280;
use chrono::prelude::*;
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use linux_embedded_hal::{Delay, I2cdev};
use rppal::gpio::{Gpio, OutputPin};
use std::{env, error::Error, thread, time::Duration};

pub type DynError = Box<dyn Error + Send + Sync + 'static>;

mod schema;

const GPIO5: u8 = 5;

fn main() -> Result<(), DynError> {
    dotenv()?;

    let pin = Gpio::new()?.get(GPIO5)?.into_output_low();
    let th1 = thread::spawn(move || blink_led(pin));
    let th2 = thread::spawn(observe_env);

    th1.join().unwrap();
    th2.join().unwrap()?;

    Ok(())
}

fn blink_led(mut pin: OutputPin) {
    loop {
        let dur = Duration::from_millis(5);
        std::thread::sleep(dur);

        let hour = Local::now().hour();
        if 2 <= hour && hour <= 6 {
            pin.set_low();
            let dur = Duration::from_secs(60);
            std::thread::sleep(dur);
            continue;
        }

        if pin.is_set_high() {
            pin.set_low();
        } else {
            pin.set_high();
        }
    }
}

fn observe_env() -> Result<(), DynError> {
    // connect to a PostgreSQL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)?;

    // using Linux I2C Bus #1 in this example
    let i2c_bus = I2cdev::new("/dev/i2c-1")?;

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus);

    // initialize the sensor
    let mut delay = Delay;
    bme280.init(&mut delay).unwrap();

    let dur = Duration::from_secs(60);

    loop {
        // measure temperature, pressure, and humidity
        let measurements = bme280.measure(&mut delay).unwrap();

        let local: DateTime<Local> = Local::now();
        println!("{local}");

        let pressure = measurements.pressure / 100.0;

        println!("温度\t{:.2}[°C]", measurements.temperature);
        println!("湿度\t{:.2}[%]", measurements.humidity);
        println!("気圧\t{:.2}[hP]", pressure);
        println!();

        // save the data to the database
        diesel::insert_into(schema::posts::table)
            .values((
                schema::posts::datetime.eq(diesel::dsl::now),
                schema::posts::temperature.eq(measurements.temperature),
                schema::posts::relative_humidity.eq(measurements.humidity),
                schema::posts::atmospheric_pressure.eq(pressure),
            ))
            .execute(&connection)?;

        std::thread::sleep(dur);
    }
}
