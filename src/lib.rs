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
const ATS_VERSION: i32 = 0x0002_0000;

// ATS Keys
const ATS_KEY_S: i32 = 0;     // S Key
const ATS_KEY_A1: i32 = 1;    // A1 Key
const ATS_KEY_A2: i32 = 2;    // A2 Key
const ATS_KEY_B1: i32 = 3;    // B1 Key
const ATS_KEY_B2: i32 = 4;    // B2 Key
const ATS_KEY_C1: i32 = 5;    // C1 Key
const ATS_KEY_C2: i32 = 6;    // C2 Key
const ATS_KEY_D: i32 = 7;     // D Key
const ATS_KEY_E: i32 = 8;     // R Key
const ATS_KEY_F: i32 = 9;     // F Key
const ATS_KEY_G: i32 = 10;    // G Key
const ATS_KEY_H: i32 = 11;    // H Key
const ATS_KEY_I: i32 = 12;    // I Key
const ATS_KEY_J: i32 = 13;    // J Key
const ATS_KEY_K: i32 = 14;    // K Key
const ATS_KEY_L: i32 = 15;    // L Key

// Initial Position of Handle
const ATS_INIT_REMOVED: i32 = 2; // Handle Removed
const ATS_INIT_EMG: i32 = 1;     // Emergency Brake
const ATS_INIT_SVC: i32 = 0;     // Service Brake

// Sound Control Instruction
const ATS_SOUND_STOP: i32 = -10000;   // Stop
const ATS_SOUND_PLAY: i32 = 1;        // Play Once
const ATS_SOUND_PLAYLOOPING: i32 = 0; // Play Repeatedly
const ATS_SOUND_CONTINUE: i32 = 2;    // Continue

// Type of Horn
const ATS_HORN_PRIMARY: i32 = 0;   // Horn 1
const ATS_HORN_SECONDARY: i32 = 1; // Horn 2
const ATS_HORN_MUSIC: i32 = 2;     // Music Horn

// Constant Speed Control Instruction
const ATS_CONSTANTSPEED_CONTINUE: i32 = 0; // Continue
const ATS_CONSTANTSPEED_ENABLE: i32 = 1;   // Enable
const ATS_CONSTANTSPEED_DISABLE: i32 = 2;  // Disable

// Vehicle Specification
#[repr(C)]
pub struct AtsVehicleSpec {
    brake_notches: i32,    // Number of Brake Notches
    power_notches: i32,    // Number of Power Notches
    ats_notch: i32,        // ATS Cancel Notch
    b67_notch: i32,        // 80% Brake (67 degree)
    cars: i32,             // Number of Cars
}

// State Quantity of Vehicle
#[repr(C)]
pub struct AtsVehicleState {
    location: f64,     // Train Position (Z-axis) (m)
    speed: f32,        // Train Speed (km/h)
    time: i32,         // Time (ms)
    bc_pressure: f32,  // Pressure of Brake Cylinder (Pa)
    mr_pressure: f32,  // Pressure of MR (Pa)
    er_pressure: f32,  // Pressure of ER (Pa)
    bp_pressure: f32,  // Pressure of BP (Pa)
    sap_pressure: f32, // Pressure of SAP (Pa)
    current: f32,      // Current (A)
}

// Received Data from Beacon
#[repr(C)]
pub struct AtsBeaconData {
    beacon_type: i32, // Type of Beacon
    signal: i32,      // Signal of Connected Section
    distance: f32,    // Distance to Connected Section (m)
    optional: i32,    // Optional Data
}

// Train Operation Instruction
#[repr(C)]
pub struct AtsHandles {
    brake: i32,          // Brake Notch
    power: i32,          // Power Notch
    reverser: i32,       // Reverser Position
    constant_speed: i32, // Constant Speed Control
}

// Called when this plug-in is loaded
#[no_mangle]
pub extern "C" fn Load() {
}

// Called when this plug-in is unloaded
#[no_mangle]
pub extern "C" fn Dispose() {
}

// Returns the version numbers of ATS plug-in
#[no_mangle]
pub extern "C" fn GetPluginVersion() -> i32 {
    ATS_VERSION
}

// Called when the train is loaded
#[no_mangle]
pub extern "C" fn SetVehicleSpec(_vehicle_spec: AtsVehicleSpec) {
}

// Called when the game is started
#[no_mangle]
pub extern "C" fn Initialize(_brake: i32) {
}

// Called every frame
#[no_mangle]
pub extern "C" fn Elapse(_vehicle_state: AtsVehicleState, _panel: *const [i32; 256], _sound: *const [i32; 256]) -> AtsHandles {
    AtsHandles {
        brake: 0,
        power: 0,
        reverser: 1,
        constant_speed: ATS_CONSTANTSPEED_CONTINUE,
    }
}

// Called when the power is changed
#[no_mangle]
pub extern "C" fn SetPower(_notch: i32) {
}

// Called when the brake is changed
#[no_mangle]
pub extern "C" fn SetBrake(_notch: i32) {
}

// Called when the reverser is changed
#[no_mangle]
pub extern "C" fn SetReverser(_pos: i32) {
}

// Called when any ATS key is pressed
#[no_mangle]
pub extern "C" fn KeyDown(_ats_key_code: i32) {
}

// Called when any ATS key is released
#[no_mangle]
pub extern "C" fn KeyUp(_ats_key_code: i32) {
}

// Called when the horn is used
#[no_mangle]
pub extern "C" fn HornBlow(_horn_type: i32) {
}

// Called when the door is opened
#[no_mangle]
pub extern "C" fn DoorOpen() {
}

// Called when the door is closed
#[no_mangle]
pub extern "C" fn DoorClose() {
}

// Called when current signal is changed
#[no_mangle]
pub extern "C" fn SetSignal(_signal: i32) {
}

// Called when the beacon data is received
#[no_mangle]
pub extern "C" fn SetBeaconData(_beacon_data: AtsBeaconData) {
}