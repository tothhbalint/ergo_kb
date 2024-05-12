use crate::{keys::{KeyCode, KeyMap, KeyReport}, usb::UsbControl};

use embassy_nrf::{
    gpio::{AnyPin, Input, Level, Output, OutputDrive},
    peripherals,
    usb::{vbus_detect::HardwareVbusDetect, Out},
};

use embassy_time::Timer;
use embedded_hal::digital::{InputPin, OutputPin};
use usbd_hid::descriptor::KeyboardReport;

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<peripherals::USBD>;
    POWER_CLOCK => usb::vbus_detect::InterruptHandler;
});


pub struct Keyboard<'a> {
    input_pins: [Input<'a, embassy_nrf::gpio::AnyPin>; 5],
    output_pins: [Output<'a, embassy_nrf::gpio::AnyPin>; 4],
    key_states: [[bool; 5]; 4],
    key_map: KeyMap,
    report : KeyBoardReport,
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
        let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));
        let usbd = UsbControl::new(driver);
        Keyboard {
            input_pins,
            output_pins,
            key_states,
            key_map,
            report : KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            },
            usbd
        }
    }

    //process kb one tick
    pub async fn process_kb(&mut self) {
        if !self.usbd.is_setup() {
            self.usbd.setup_device().await;
        }
        self.scan().await;
        match usbd.send_report(&self.report).await {
            Ok(()) => {}
            Err(e) => error!("Send keyboard report error: {}", e),
        };
        // Reset report key states
        for bit in &mut self.report.keycodes {
            *bit = 0;
        }
    }

    pub(crate) async fn scan(&mut self) {
        // TODO: update key report, Read upon N-key rollover, add debouncing
        for (out_idx, out_pin) in self.output_pins.iter_mut().enumerate() {
            out_pin.set_high();
            Timer::after_micros(1).await;
            for (in_idx, in_pin) in self.input_pins.iter_mut().enumerate() {
                let changed = in_pin.is_high().ok().unwrap_or_default();
                if changed {
                    let code : KeyCode = self.key_map[in_idx][out_idx];
                }
            }
            out_pin.set_low();
        }
    }
}
