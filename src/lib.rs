#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use num_derive::FromPrimitive;

include!("bindings.rs");
include!("status.rs");

#[macro_export]
macro_rules! sysex_config {
    ( $x:expr ) => {
        ($x[0] as isize) << 8 | ($x[1] as isize)
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, FromPrimitive)]
pub enum SysexConfiguration {
    AudioBitdepth = sysex_config!(SYSEX_CONFIGURATION_AUDIO_BITDEPTH),
    AudioBlocksize = sysex_config!(SYSEX_CONFIGURATION_AUDIO_BLOCKSIZE),
    AudioDataformat = sysex_config!(SYSEX_CONFIGURATION_AUDIO_DATAFORMAT),
    AudioRate = sysex_config!(SYSEX_CONFIGURATION_AUDIO_RATE),
    BootloaderLock = sysex_config!(SYSEX_CONFIGURATION_BOOTLOADER_LOCK),
    BusEnable = sysex_config!(SYSEX_CONFIGURATION_BUS_ENABLE),
    BusForwardMIDI = sysex_config!(SYSEX_CONFIGURATION_BUS_FORWARD_MIDI),
    CodecBypass = sysex_config!(SYSEX_CONFIGURATION_CODEC_BYPASS),
    CodecHipass = sysex_config!(SYSEX_CONFIGURATION_CODEC_HIGHPASS),
    CodecInputGain = sysex_config!(SYSEX_CONFIGURATION_CODEC_INPUT_GAIN),
    CodecOutputGain = sysex_config!(SYSEX_CONFIGURATION_CODEC_OUTPUT_GAIN),
    CodecSwap = sysex_config!(SYSEX_CONFIGURATION_CODEC_SWAP),
    ExpressionPedal = sysex_config!(SYSEX_CONFIGURATION_EXPRESSION_PEDAL),
    InputOffset = sysex_config!(SYSEX_CONFIGURATION_INPUT_OFFSET),
    InputScalar = sysex_config!(SYSEX_CONFIGURATION_INPUT_SCALAR),
    MIDIInputChannel = sysex_config!(SYSEX_CONFIGURATION_MIDI_INPUT_CHANNEL),
    MIDIOutputChannel = sysex_config!(SYSEX_CONFIGURATION_MIDI_OUTPUT_CHANNEL),
    OutputOffset = sysex_config!(SYSEX_CONFIGURATION_OUTPUT_OFFSET),
    OutputScalar = sysex_config!(SYSEX_CONFIGURATION_OUTPUT_SCALAR),
    PCButton = sysex_config!(SYSEX_CONFIGURATION_PC_BUTTON),
}

impl From<isize> for SysexConfiguration {
    fn from(value: isize) -> Self {
        num_traits::FromPrimitive::from_isize(value).unwrap()
    }
}

pub const SYSEX_CONFIGURATIONS: [SysexConfiguration; 20] = [
    SysexConfiguration::AudioBitdepth,
    SysexConfiguration::AudioBlocksize,
    SysexConfiguration::AudioDataformat,
    SysexConfiguration::AudioRate,
    SysexConfiguration::BootloaderLock,
    SysexConfiguration::BusEnable,
    SysexConfiguration::BusForwardMIDI,
    SysexConfiguration::CodecBypass,
    SysexConfiguration::CodecHipass,
    SysexConfiguration::CodecInputGain,
    SysexConfiguration::CodecOutputGain,
    SysexConfiguration::CodecSwap,
    SysexConfiguration::ExpressionPedal,
    SysexConfiguration::InputOffset,
    SysexConfiguration::InputScalar,
    SysexConfiguration::MIDIInputChannel,
    SysexConfiguration::MIDIOutputChannel,
    SysexConfiguration::OutputOffset,
    SysexConfiguration::OutputScalar,
    SysexConfiguration::PCButton,
];
const BUTTON_STRING_IDS: [&str; 11] = [
    "Push", "Green", "Red", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8",
];
impl PatchButtonId {
    pub fn string_id(&self) -> &str {
        BUTTON_STRING_IDS[usize::from(*self as u8 - 1)]
    }
}

impl From<isize> for PatchButtonId {
    fn from(value: isize) -> Self {
        num_traits::FromPrimitive::from_isize(value).unwrap()
    }
}

const PARAMETER_STRING_IDS: [&str; 40] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "AA", "AB", "AC", "AD", "AE", "AF", "AG", "AH", "BA",
    "BB", "BC", "BD", "BE", "BF", "BG", "BH", "CA", "CB", "CC", "CD", "CE", "CF", "CG", "CH", "DA",
    "DB", "DC", "DD", "DE", "DF", "DG", "DH",
];
impl PatchParameterId {
    pub fn string_id(&self) -> &str {
        PARAMETER_STRING_IDS[usize::from(*self as u8)]
    }
}

impl From<isize> for PatchParameterId {
    fn from(value: isize) -> Self {
        num_traits::FromPrimitive::from_isize(value).unwrap()
    }
}

impl From<isize> for OpenWareMidiSysexCommand {
    fn from(value: isize) -> Self {
        num_traits::FromPrimitive::from_isize(value).unwrap()
    }
}

impl From<isize> for OpenWareMidiControl {
    fn from(value: isize) -> Self {
        num_traits::FromPrimitive::from_isize(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sysex_config() {
        assert_eq!(SysexConfiguration::AudioRate as isize, 0x4653);
    }
    #[test]
    fn test_sysex_config_from_isize() {
        assert_eq!(
            num_traits::FromPrimitive::from_isize(0x4653),
            Some(SysexConfiguration::AudioRate)
        );
    }
    #[test]
    fn test_sysex_config_from_method() {
        assert_eq!(
            SysexConfiguration::from(0x4653),
            SysexConfiguration::AudioRate
        );
    }
    #[test]
    fn test_sysex_configs_list() {
        assert_eq!(SysexConfiguration::AudioRate, SYSEX_CONFIGURATIONS[3]);
    }
    #[test]
    fn test_parameter_id() {
        assert_eq!(PatchParameterId::PARAMETER_AA as u8, 8);
        assert_eq!(
            PatchParameterId::try_from(8).unwrap(),
            PatchParameterId::PARAMETER_AA
        );
        assert_eq!(PatchParameterId::PARAMETER_AA.string_id(), "AA");
    }
    #[test]
    fn test_button_id() {
        assert_eq!(PatchButtonId::BUTTON_1 as u8, 4);
        assert_eq!(PatchButtonId::try_from(4).unwrap(), PatchButtonId::BUTTON_1);
        assert_eq!(PatchButtonId::BUTTON_1.string_id(), "B1");
    }
    #[test]
    fn test_sysex_midi_command() {
        assert_eq!(
            OpenWareMidiSysexCommand::SYSEX_PARAMETER_NAME_COMMAND as u8,
            2
        );
        assert_eq!(
            OpenWareMidiSysexCommand::try_from(2).unwrap(),
            OpenWareMidiSysexCommand::SYSEX_PARAMETER_NAME_COMMAND
        );
    }
    #[test]
    fn test_midi_control() {
        assert_eq!(OpenWareMidiControl::LED as u8, 30);
        assert_eq!(
            OpenWareMidiControl::try_from(30).unwrap(),
            OpenWareMidiControl::LED
        );
    }
}
