//=============================
// BVE ATS Plug-in Header File
//
//             Rock_On, mackoy
//=============================

// #ifdef ATS_EXPORTS
// #define ATS_API __declspec(dllexport)
// #else
// #define ATS_API __declspec(dllimport)
// #endif

#![allow(dead_code)]

// ATS Plug-in Version
pub const ATS_VERSION: i32 = 0x0002_0000;

// ATS Keys
pub const ATS_KEY_S: i32 = 0; // S Key
pub const ATS_KEY_A1: i32 = 1; // A1 Key
pub const ATS_KEY_A2: i32 = 2; // A2 Key
pub const ATS_KEY_B1: i32 = 3; // B1 Key
pub const ATS_KEY_B2: i32 = 4; // B2 Key
pub const ATS_KEY_C1: i32 = 5; // C1 Key
pub const ATS_KEY_C2: i32 = 6; // C2 Key
pub const ATS_KEY_D: i32 = 7; // D Key
pub const ATS_KEY_E: i32 = 8; // R Key
pub const ATS_KEY_F: i32 = 9; // F Key
pub const ATS_KEY_G: i32 = 10; // G Key
pub const ATS_KEY_H: i32 = 11; // H Key
pub const ATS_KEY_I: i32 = 12; // I Key
pub const ATS_KEY_J: i32 = 13; // J Key
pub const ATS_KEY_K: i32 = 14; // K Key
pub const ATS_KEY_L: i32 = 15; // L Key

// Initial Position of Handle
pub const ATS_INIT_REMOVED: i32 = 2; // Handle Removed
pub const ATS_INIT_EMG: i32 = 1; // Emergency Brake
pub const ATS_INIT_SVC: i32 = 0; // Service Brake

// Sound Control Instruction
pub const ATS_SOUND_STOP: i32 = -10000; // Stop
pub const ATS_SOUND_PLAY: i32 = 1; // Play Once
pub const ATS_SOUND_PLAYLOOPING: i32 = 0; // Play Repeatedly
pub const ATS_SOUND_CONTINUE: i32 = 2; // Continue

// Type of Horn
pub const ATS_HORN_PRIMARY: i32 = 0; // Horn 1
pub const ATS_HORN_SECONDARY: i32 = 1; // Horn 2
pub const ATS_HORN_MUSIC: i32 = 2; // Music Horn

// Constant Speed Control Instruction
pub const ATS_CONSTANTSPEED_CONTINUE: i32 = 0; // Continue
pub const ATS_CONSTANTSPEED_ENABLE: i32 = 1; // Enable
pub const ATS_CONSTANTSPEED_DISABLE: i32 = 2; // Disable

// Vehicle Specification
#[repr(C)]
pub struct AtsVehicleSpec {
    pub brake_notches: i32, // Number of Brake Notches
    pub power_notches: i32, // Number of Power Notches
    pub ats_notch: i32,     // ATS Cancel Notch
    pub b67_notch: i32,     // 80% Brake (67 degree)
    pub cars: i32,          // Number of Cars
}

// State Quantity of Vehicle
#[repr(C)]
pub struct AtsVehicleState {
    pub location: f64,     // Train Position (Z-axis) (m)
    pub speed: f32,        // Train Speed (km/h)
    pub time: i32,         // Time (ms)
    pub bc_pressure: f32,  // Pressure of Brake Cylinder (Pa)
    pub mr_pressure: f32,  // Pressure of MR (Pa)
    pub er_pressure: f32,  // Pressure of ER (Pa)
    pub bp_pressure: f32,  // Pressure of BP (Pa)
    pub sap_pressure: f32, // Pressure of SAP (Pa)
    pub current: f32,      // Current (A)
}

// Received Data from Beacon
#[repr(C)]
pub struct AtsBeaconData {
    pub beacon_type: i32, // Type of Beacon
    pub signal: i32,      // Signal of Connected Section
    pub distance: f32,    // Distance to Connected Section (m)
    pub optional: i32,    // Optional Data
}

// Train Operation Instruction
#[repr(C)]
pub struct AtsHandles {
    pub brake: i32,          // Brake Notch
    pub power: i32,          // Power Notch
    pub reverser: i32,       // Reverser Position
    pub constant_speed: i32, // Constant Speed Control
}
