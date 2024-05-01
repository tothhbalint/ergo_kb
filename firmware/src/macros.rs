//Taken from https://github.com/HaoboGu/rmk
macro_rules! config_matrix_pins_nrf {
    (peripherals: $p:ident, input: [$($in_pin:ident), *], output: [$($out_pin:ident), +]) => {
        {
            let mut output_pins = [$(Output::new(embassy_nrf::gpio::AnyPin::from($p.$out_pin), embassy_nrf::gpio::Level::Low, embassy_nrf::gpio::OutputDrive::Standard)), +];
            let input_pins = [$(Input::new(embassy_nrf::gpio::AnyPin::from($p.$in_pin), embassy_nrf::gpio::Pull::Down)), +];
            output_pins.iter_mut().for_each(|p| {
                p.set_low();
            });
            (input_pins, output_pins)
        }
    };
}
