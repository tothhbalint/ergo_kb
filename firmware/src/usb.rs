use embassy_nrf::usb;
use usbd_hid::descriptor::{AsInputReport, KeyboardReport, SerializedDescriptor};

use crate::{keyboard, keys::{KeyDescriptor, KeyReport}};
use embassy_usb::{
    class::hid::{Config, HidReader, HidReaderWriter, HidWriter, ReportId, RequestHandler, State},
    control::OutResponse,
    driver::Driver,
    Builder, Handler, UsbDevice,
};

struct UsbHandler {}

impl RequestHandler for UsbHandler {
    fn get_report(&self, id: ReportId, _buf: &mut [u8]) -> Option<usize> { 
        info!("Get report for {}", id);
        None
    }

    fn set_report(&self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {}: {}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {} to {}", id, dur);
    }

    fn get_idle_ms(&self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {}", id);
        None
    }
}

pub struct UsbControl<'d, D: Driver<'d>> {
    usb_device : UsbDevice<'d,D>,
    keyboard_writer : HidWriter<'d>,
    keyboard_reader : HidReader<'d>
}

impl<D : Driver<'static>> UsbControl<'static,driver> {
    pub fn new(driver : D) -> Self {
        let mut usb_config = embassy_usb::Config::new(0x1209,0x6969);
        usb_config.manufacturer = Some("TB");
        usb_config.product = Some("SPLITKB");
        usb_config.serial_number = Some("69696");
        usb_config.max_power = 450;

        // Required for windows compatibility.
        usb_config.max_packet_size_0 = 64;
        usb_config.device_class = 0xEF;
        usb_config.device_sub_class = 0x02;
        usb_config.device_protocol = 0x01;
        usb_config.composite_with_iads = true;
        
        // Create embassy-usb DeviceBuilder using the driver and config.
        static DEVICE_DESC: StaticCell<[u8; 256]> = StaticCell::new();
        static CONFIG_DESC: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESC: StaticCell<[u8; 256]> = StaticCell::new();
        static MSOS_DESC: StaticCell<[u8; 128]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();

        // UsbDevice builder
        let mut builder = Builder::new(
            driver,
            usb_config,
            &mut DEVICE_DESC.init([0; 256])[..],
            &mut CONFIG_DESC.init([0; 256])[..],
            &mut BOS_DESC.init([0; 256])[..],
            &mut MSOS_DESC.init([0; 128])[..],
            &mut CONTROL_BUF.init([0; 128])[..],
        );

        let request_handler = UsbHandler{};

        let keyboard_hid_config = Config {
            report_descriptor : KeyboardReport::desc(),
            request_handler : Some(&request_handler),
            poll_ms : 1,
            max_packet_size : 64
        };

        static KB_STATE : StaticCell<State>;
        // needs builder, State?, config, can be split into reader and writer later
        let kb_readerwriter :HidReaderWriter<'_, driver, 1, 8> = HidReaderWriter::new(
            &mut builder,
            KB_STATE.init(State::new()),
            keyboard_hid_config
        );
        let (reader,writer) = kb_readerwriter.split();

        let device = builder.build();
        Self {
            usb_device : device,
            keyboard_reader : reader,
            keyboard_writer : writer,
        }
    }

    pub async fn send_report<IR : AsInputReport>(&mut self,r : &IR) -> Result<(),HidError> {
        self.keyboard_writer.write_serialize(r)
                            .await
                            .map_err( | e | match e {
                                embassy_usb::driver::EndpointError::BufferOverflow => HidError::BufferOverflow,
                                embassy_usb::driver::EndpointError::Disabled => HidError::UsbDisabled,
                            });
    }
}


/* 
TODO:
    add handler for usbdevice when i figure out debug logging
 */
