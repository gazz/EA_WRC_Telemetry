use hidapi::HidDevice;

use crate::wrc_telemetry;

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(SLIMBoardOUT {
    report_id : u8,
    report_type : u8,
    gear : u8,
    rpm_leds : [u8; 13],
    led14 : u8,
    led15 : u8,
    led16 : u8,
    led17 : u8,
    led18 : u8,
    led19 : u8,
    led_external : [u8; 5],
    spare : [u8; 27],
});

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

pub fn update_sli_m_hid(hid_device: &HidDevice, report: &SLIMBoardOUT) {
    let buf: &[u8] = unsafe { any_as_u8_slice(report) };
    // println!("Writing bytes: {:?}", buf);
    let _res = hid_device.write(&buf).unwrap();
    // println!("Wrote: {:?} byte(s)", res);
}


pub fn update_sli_m_with_telemetry(hid_device: &HidDevice, telemetry: &wrc_telemetry::WrcTelemetry) {
    let mut report = SLIMBoardOUT {
        report_id: 0,
        report_type : 1,
        gear : if telemetry.gear == 10 { 82 } else { telemetry.gear },
        rpm_leds : [0; 13],
        led14 : 0,
        led15 : 0,
        led16 : 0,
        led17 : 0,
        led18 : 0,
        led19 : 0,
        led_external : [0; 5],
        spare : [0; 27],
    };

    let start_revs = telemetry.max_rpm / 2.0;
    let revs_per_light = start_revs / 13.0;

    for i in 0..13 {
        let revs_to_light_up = start_revs + (i as f32) * revs_per_light;
        let light_up = if telemetry.rpm > revs_to_light_up { 1 } else { 0 };
        // println!("LED: {i} Current revs: {}, min revs for light: {}", telemetry.rpm, revs_to_light_up);
        report.rpm_leds[i] = light_up;
    }
    // println!("Start revs: {start_revs} Revs: {:?}", report.rpm_leds);

    // shift lights
    let shift_lights = if telemetry.rpm > telemetry.max_rpm * 0.9 { 1 } else { 0 };  
    report.led14 = shift_lights;
    // report.led15 = shift_lights;
    // report.led16 = shift_lights;
    // report.led17 = shift_lights;
    // report.led18 = shift_lights;
    report.led19 = shift_lights;

    update_sli_m_hid(hid_device, &report);
}

pub fn init_sli_m(hid_device: &HidDevice) {
    update_sli_m_with_telemetry(hid_device, &wrc_telemetry::WrcTelemetry {
        gear: 45,
        rpm: 0.0,
        max_rpm: 0.0
    });
}

/*
// OUTPUT
typedef struct _SLI_MboardOUT {
	BYTE ReportID;      //  always 0
	BYTE ReportType;      //  type of report  1
	BYTE Gear;                 //  gear (revert display not implemented)
	BYTE RPMLED[13];      //  RPM leds (value = 0 or 1)
	BYTE LED14;         //  RED (Damage/heat/Hard Warning/...)
	BYTE LED15;         //  BLUE (optimal shiftpoint feedback)
	BYTE LED16;         //  YELLOW (low fuel/Yellow flag/...)
	BYTE LED17;         //  BLUE (optimal shiftpoint feedback)
	BYTE LED18;         //  YELLOW (Speedlimiter/Yellow Flag/...)
	BYTE LED19;         //  RED (TC/...)
	BYTE LEDExternal[5];           //  External leds 5
	BYTE Spare[27];
} _SLI_MboardOUT, *P_SLI_MboardOUT;


// Global Brightness
typedef struct _SLI_MboardOUT2 {
	BYTE ReportID;      // always 0
	BYTE ReportType;   // type of report  2
	BYTE GlobalBrightness;   // Value from 0 to 255
	BYTE Spare[51];
} _SLI_MboardOUT2, *P_SLI_MboardOUT2;

// INPUT (Btns)
typedef struct _SLI_MboardIN {
	BYTE ID; // zero
	BYTE Btn[2];   // digital IN first byte from 1 to 8 and second from 9 to 16
} _SLI_MboardIN, *P_SLI_MboardIN;
 */