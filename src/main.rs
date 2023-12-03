mod sli_m;
mod wrc_telemetry;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    
    let api = hidapi::HidApi::new().unwrap();
    let hid_device = api.open(0x1dd2, 0x0102).unwrap();
    println!("HidDevice: {:?}", hid_device);
    
    sli_m::init_sli_m(&hid_device);

    {
        let socket = UdpSocket::bind("127.0.0.1:20777")?;

        loop {
            let mut buf: [u8; 128] = [0; 128];
            let (_amount_read, _) = socket.recv_from(&mut buf)?;

            sli_m::update_sli_m_with_telemetry(&hid_device, &wrc_telemetry::WrcTelemetry {
                gear: u8::from_ne_bytes([buf[0]]),
                rpm: f32::from_ne_bytes(buf[1..5].try_into().unwrap()),
                max_rpm: f32::from_ne_bytes(buf[5..9].try_into().unwrap()),
            });

        }
    } // the socket is closed here
}

