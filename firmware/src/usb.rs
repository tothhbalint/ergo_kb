pub struct UsbControl {
    setup : bool,
}

/*
Setup usb hid descriptor etc.
Send data through usb
 */
impl UsbControl {
    pub fn new() -> Self {
        let setup = false;
        UsbControl {
            setup
        }
    }

    pub async fn setup_device(&mut self) {

    }

    pub fn is_setup(&self) -> bool {
        return self.setup;
    }

}
