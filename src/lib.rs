#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");
include!("status.rs");

/*
pub const SYSEX_CONFIGURATIONS: [&'static [u8; 3usize]; 1] =[
    SYSEX_CONFIGURATION_AUDIO_RATE,
    ];
     */

use num_derive::FromPrimitive;

#[macro_export]
macro_rules! sysex_config {
    ( $x:expr ) => {
        ($x[0] as isize) << 8 | ($x[1] as isize)
    };
}

#[derive(PartialEq, Debug, FromPrimitive)]
pub enum SysexConfiguration {
    AudioBitdepth     = sysex_config!(SYSEX_CONFIGURATION_AUDIO_BITDEPTH),
    AudioBlocksize    = sysex_config!(SYSEX_CONFIGURATION_AUDIO_BLOCKSIZE),
    AudioDataformat   = sysex_config!(SYSEX_CONFIGURATION_AUDIO_DATAFORMAT),
    AudioRate         = sysex_config!(SYSEX_CONFIGURATION_AUDIO_RATE),
    BootloaderLock    = sysex_config!(SYSEX_CONFIGURATION_BOOTLOADER_LOCK),
    BusEnable         = sysex_config!(SYSEX_CONFIGURATION_BUS_ENABLE),
    BusForwardMIDI    = sysex_config!(SYSEX_CONFIGURATION_BUS_FORWARD_MIDI),
    CodecBypass       = sysex_config!(SYSEX_CONFIGURATION_CODEC_BYPASS),
    CodecHipass       = sysex_config!(SYSEX_CONFIGURATION_CODEC_HIGHPASS),
    CodecInputGain    = sysex_config!(SYSEX_CONFIGURATION_CODEC_INPUT_GAIN),
    CodecOutputGain   = sysex_config!(SYSEX_CONFIGURATION_CODEC_OUTPUT_GAIN),
    CodecSwap         = sysex_config!(SYSEX_CONFIGURATION_CODEC_SWAP),
    ExpressionPedal   = sysex_config!(SYSEX_CONFIGURATION_EXPRESSION_PEDAL),
    InputOffset       = sysex_config!(SYSEX_CONFIGURATION_INPUT_OFFSET),
    InputScalar       = sysex_config!(SYSEX_CONFIGURATION_INPUT_SCALAR),
    MIDIInputChannel  = sysex_config!(SYSEX_CONFIGURATION_MIDI_INPUT_CHANNEL),
    MIDIOutputChannel = sysex_config!(SYSEX_CONFIGURATION_MIDI_OUTPUT_CHANNEL),
    OutputOffset      = sysex_config!(SYSEX_CONFIGURATION_OUTPUT_OFFSET),
    OutputScalar      = sysex_config!(SYSEX_CONFIGURATION_OUTPUT_SCALAR),
    PCButton          = sysex_config!(SYSEX_CONFIGURATION_PC_BUTTON),
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sysex_config() {
        assert_eq!(SysexConfiguration::AudioRate as isize, 0x4653);
    }
    #[test]
    fn test_sysex_config_from_isize() {
        assert_eq!(num_traits::FromPrimitive::from_isize(0x4653), Some(SysexConfiguration::AudioRate));
    }
    #[test]
    fn test_sysex_config_from_method() {
        assert_eq!(SysexConfiguration::from(0x4653), SysexConfiguration::AudioRate);
    }    
    #[test]
    fn test_sysex_configs_list() {
        assert_eq!(SysexConfiguration::AudioRate, SYSEX_CONFIGURATIONS[3]);
    }
}
