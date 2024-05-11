use crate::{keys::{KeyMap, KeyReport}, usb::UsbControl};

use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive},
    peripherals,
    usb::Out,
};

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};

pub struct Keyboard<'a> {
    input_pins: [Input<'a, embassy_nrf::gpio::AnyPin>; 5],
    output_pins: [Output<'a, embassy_nrf::gpio::AnyPin>; 4],
    key_states: [[bool; 5]; 4],
    key_map: KeyMap,
    usbd : UsbControl,
}

impl<'a> Keyboard<'a> {
    pub fn new(p: embassy_nrf::Peripherals) -> Self {
        let (input_pins, output_pins) = config_matrix_pins_nrf!(peripherals : p,
            input : [P1_06,P1_04,P0_11,P1_00,P0_24],
            output : [P0_31,P0_29,P0_02,P1_15]
        );

        let key_states = [[false; 5]; 4];
        let key_map = KeyMap::new();
        let usbd = UsbControl::new();
        Keyboard {
            input_pins,
            output_pins,
            key_states,
            key_map,
            usbd
        }
    }

    //process kb one tick
    pub async fn process_kb(&mut self) {
        if !self.usbd.is_setup() {
            self.usbd.setup_device().await;
        }
        self.scan().await;
        let reported_keys : KeyReport;
        for (row_idx, row) in self.key_states.iter().enumerate() {
            for (col_idx, &col) in row.iter().enumerate() {
                if col {
                    reported_keys = self.key_map.get_key(row_idx, col_idx);
                }
            }
        }
    }

    pub(crate) async fn scan(&mut self) {
        for (out_idx, out_pin) in self.output_pins.iter_mut().enumerate() {
            out_pin.set_high();
            Timer::after_micros(1).await;
            for (in_idx, in_pin) in self.input_pins.iter_mut().enumerate() {
                let changed = in_pin.is_high().ok().unwrap_or_default();
                if changed {
                    self.key_states[out_idx][in_idx] = changed;
                }
            }
            out_pin.set_low();
        }
    }
}
