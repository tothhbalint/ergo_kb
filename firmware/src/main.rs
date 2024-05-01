#![no_std]
#![no_main]

extern crate firmware;
use firmware::keyboard::Keyboard;

use cortex_m::prelude::{_embedded_hal_digital_InputPin, _embedded_hal_digital_OutputPin};
use embedded_hal::digital::{InputPin, OutputPin};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive},
    peripherals,
    usb::Out,
};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut kb = Keyboard::new(p);

    loop {
        kb.process_kb().await;
    }
}
