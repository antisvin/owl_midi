/* automatically generated by rust-bindgen 0.59.2 */

pub const MIDI_SYSEX_MANUFACTURER: u32 = 125;
pub const MIDI_SYSEX_OMNI_DEVICE: u32 = 82;
pub const MIDI_SYSEX_OWL_DEVICE: u32 = 32;
pub const MIDI_SYSEX_VERSION: u32 = 3;
pub const SYSEX_CONFIGURATION_AUDIO_RATE: &[u8; 3usize] = b"FS\0";
pub const SYSEX_CONFIGURATION_AUDIO_BITDEPTH: &[u8; 3usize] = b"BD\0";
pub const SYSEX_CONFIGURATION_AUDIO_DATAFORMAT: &[u8; 3usize] = b"DF\0";
pub const SYSEX_CONFIGURATION_AUDIO_BLOCKSIZE: &[u8; 3usize] = b"BS\0";
pub const SYSEX_CONFIGURATION_CODEC_SWAP: &[u8; 3usize] = b"SW\0";
pub const SYSEX_CONFIGURATION_CODEC_BYPASS: &[u8; 3usize] = b"BY\0";
pub const SYSEX_CONFIGURATION_CODEC_INPUT_GAIN: &[u8; 3usize] = b"IG\0";
pub const SYSEX_CONFIGURATION_CODEC_OUTPUT_GAIN: &[u8; 3usize] = b"OG\0";
pub const SYSEX_CONFIGURATION_CODEC_HIGHPASS: &[u8; 3usize] = b"HP\0";
pub const SYSEX_CONFIGURATION_PC_BUTTON: &[u8; 3usize] = b"PC\0";
pub const SYSEX_CONFIGURATION_INPUT_OFFSET: &[u8; 3usize] = b"IO\0";
pub const SYSEX_CONFIGURATION_INPUT_SCALAR: &[u8; 3usize] = b"IS\0";
pub const SYSEX_CONFIGURATION_OUTPUT_OFFSET: &[u8; 3usize] = b"OO\0";
pub const SYSEX_CONFIGURATION_OUTPUT_SCALAR: &[u8; 3usize] = b"OS\0";
pub const SYSEX_CONFIGURATION_MIDI_INPUT_CHANNEL: &[u8; 3usize] = b"MI\0";
pub const SYSEX_CONFIGURATION_MIDI_OUTPUT_CHANNEL: &[u8; 3usize] = b"MO\0";
pub const SYSEX_CONFIGURATION_BUS_ENABLE: &[u8; 3usize] = b"BE\0";
pub const SYSEX_CONFIGURATION_BUS_FORWARD_MIDI: &[u8; 3usize] = b"BM\0";
pub const SYSEX_CONFIGURATION_BOOTLOADER_LOCK: &[u8; 3usize] = b"BL\0";
pub const SYSEX_CONFIGURATION_EXPRESSION_PEDAL: &[u8; 3usize] = b"EP\0";
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PatchParameterId {
    PARAMETER_A = 0,
    PARAMETER_B = 1,
    PARAMETER_C = 2,
    PARAMETER_D = 3,
    PARAMETER_E = 4,
    PARAMETER_F = 5,
    PARAMETER_G = 6,
    PARAMETER_H = 7,
    PARAMETER_AA = 8,
    PARAMETER_AB = 9,
    PARAMETER_AC = 10,
    PARAMETER_AD = 11,
    PARAMETER_AE = 12,
    PARAMETER_AF = 13,
    PARAMETER_AG = 14,
    PARAMETER_AH = 15,
    PARAMETER_BA = 16,
    PARAMETER_BB = 17,
    PARAMETER_BC = 18,
    PARAMETER_BD = 19,
    PARAMETER_BE = 20,
    PARAMETER_BF = 21,
    PARAMETER_BG = 22,
    PARAMETER_BH = 23,
    PARAMETER_CA = 24,
    PARAMETER_CB = 25,
    PARAMETER_CC = 26,
    PARAMETER_CD = 27,
    PARAMETER_CE = 28,
    PARAMETER_CF = 29,
    PARAMETER_CG = 30,
    PARAMETER_CH = 31,
    PARAMETER_DA = 32,
    PARAMETER_DB = 33,
    PARAMETER_DC = 34,
    PARAMETER_DD = 35,
    PARAMETER_DE = 36,
    PARAMETER_DF = 37,
    PARAMETER_DG = 38,
    PARAMETER_DH = 39,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PatchButtonId {
    PUSHBUTTON = 1,
    GREEN_BUTTON = 2,
    RED_BUTTON = 3,
    BUTTON_1 = 4,
    BUTTON_2 = 5,
    BUTTON_3 = 6,
    BUTTON_4 = 7,
    BUTTON_5 = 8,
    BUTTON_6 = 9,
    BUTTON_7 = 10,
    BUTTON_8 = 11,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OpenWareMidiSysexCommand {
    SYSEX_PRESET_NAME_COMMAND = 1,
    SYSEX_PARAMETER_NAME_COMMAND = 2,
    SYSEX_CONFIGURATION_COMMAND = 3,
    SYSEX_RESOURCE_NAME_COMMAND = 4,
    SYSEX_DEVICE_RESET_COMMAND = 125,
    SYSEX_BOOTLOADER_COMMAND = 126,
    SYSEX_FIRMWARE_UPLOAD = 16,
    SYSEX_FIRMWARE_STORE = 17,
    SYSEX_FIRMWARE_RUN = 18,
    SYSEX_FIRMWARE_FLASH = 19,
    SYSEX_FLASH_ERASE = 20,
    SYSEX_SETTINGS_RESET = 21,
    SYSEX_SETTINGS_STORE = 22,
    SYSEX_FIRMWARE_SAVE = 23,
    SYSEX_FIRMWARE_SEND = 24,
    SYSEX_FIRMWARE_VERSION = 32,
    SYSEX_DEVICE_ID = 33,
    SYSEX_PROGRAM_MESSAGE = 34,
    SYSEX_DEVICE_STATS = 35,
    SYSEX_PROGRAM_STATS = 36,
    SYSEX_BOOTLOADER_VERSION = 37,
    SYSEX_PROGRAM_ERROR = 48,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OpenWareMidiControl {
    PATCH_PARAMETER_A = 20,
    PATCH_PARAMETER_B = 21,
    PATCH_PARAMETER_C = 22,
    PATCH_PARAMETER_D = 23,
    PATCH_PARAMETER_E = 24,
    PATCH_PARAMETER_F = 1,
    PATCH_PARAMETER_G = 12,
    PATCH_PARAMETER_H = 13,
    PATCH_BUTTON = 25,
    PATCH_CONTROL = 26,
    PATCH_BUTTON_ON = 27,
    PATCH_BUTTON_OFF = 28,
    LED = 30,
    LEFT_INPUT_GAIN = 32,
    RIGHT_INPUT_GAIN = 33,
    LEFT_OUTPUT_GAIN = 34,
    RIGHT_OUTPUT_GAIN = 35,
    LEFT_INPUT_MUTE = 36,
    RIGHT_INPUT_MUTE = 37,
    LEFT_OUTPUT_MUTE = 38,
    RIGHT_OUTPUT_MUTE = 39,
    BYPASS = 40,
    REQUEST_SETTINGS = 67,
    SAVE_SETTINGS = 68,
    FACTORY_RESET = 70,
    DEVICE_STATUS = 71,
    PATCH_PARAMETER_AA = 75,
    PATCH_PARAMETER_AB = 76,
    PATCH_PARAMETER_AC = 77,
    PATCH_PARAMETER_AD = 78,
    PATCH_PARAMETER_AE = 79,
    PATCH_PARAMETER_AF = 80,
    PATCH_PARAMETER_AG = 81,
    PATCH_PARAMETER_AH = 82,
    PATCH_PARAMETER_BA = 83,
    PATCH_PARAMETER_BB = 84,
    PATCH_PARAMETER_BC = 85,
    PATCH_PARAMETER_BD = 86,
    PATCH_PARAMETER_BE = 87,
    PATCH_PARAMETER_BF = 88,
    PATCH_PARAMETER_BG = 89,
    PATCH_PARAMETER_BH = 90,
    PATCH_PARAMETER_CA = 91,
    PATCH_PARAMETER_CB = 92,
    PATCH_PARAMETER_CC = 93,
    PATCH_PARAMETER_CD = 94,
    PATCH_PARAMETER_CE = 95,
    PATCH_PARAMETER_CF = 96,
    PATCH_PARAMETER_CG = 97,
    PATCH_PARAMETER_CH = 98,
    PATCH_PARAMETER_DA = 99,
    PATCH_PARAMETER_DB = 100,
    PATCH_PARAMETER_DC = 101,
    PATCH_PARAMETER_DD = 102,
    PATCH_PARAMETER_DE = 103,
    PATCH_PARAMETER_DF = 104,
    PATCH_PARAMETER_DG = 105,
    PATCH_PARAMETER_DH = 106,
}
