
// configured in custom_udp.json that WRC game reads
#[derive(Debug)]
pub struct WrcTelemetry {
    pub gear: u8,
    pub rpm: f32,
    pub max_rpm: f32,
}