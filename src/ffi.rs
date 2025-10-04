#![allow(nonstandard_style)]

pub use core::ffi::{c_char, c_int, c_float, c_void};
pub type size_t = usize;

pub enum xm_context {}
pub type xm_context_t = xm_context;

#[derive(Default)]
#[repr(C)]
pub struct xm_prescan_data {
    context_size: u32,
    format: u8,
    num_rows: u32,
    smaples_data_length: u32,
    num_patterns: u16,
    num_samples: u16,
    pot_length: u16,
    num_channels: u8,
    num_instruments: u8,
}
pub type xm_prescan_data_t = xm_prescan_data;
static_assertions::const_assert_eq!(::core::mem::size_of::<xm_prescan_data>(), 0x18);

extern "C" {
    // TODO:
    // xm_seek
    // xm_mute_channel
    // xm_mute_instrument
    // xm_get_sample_waveform
    // xm_is_channel_active
    // xm_get_instrument_of_channel
    // xm_get_frequency_of_channel
    // xm_get_volume_of_channel
    // xm_get_panning_of_channel

    pub fn xm_prescan_module(moddata: *const u8, moddata_length: u32, out: *mut xm_prescan_data_t) -> bool;
    pub fn xm_size_for_context(p: *const xm_prescan_data_t) -> u32;
    pub fn xm_create_context(pool: *mut u8, p: *const xm_prescan_data_t, moddata: *const u8, moddata_length: u32) -> *mut xm_context_t;
    pub fn xm_context_size(context: *const xm_context_t) -> u32;

    pub fn xm_generate_samples(context: *mut xm_context_t, output: *mut c_float, numsamples: u16);
    pub fn xm_generate_samples_noninterleaved(context: *mut xm_context_t, output_left: *mut c_float, output_right: *mut c_float, numsamples: u16);
    pub fn xm_generate_samples_unmixed(context: *mut xm_context_t, output: *mut c_float, numsamples: u16);

    pub fn xm_set_max_loop_count(context: *mut xm_context_t, loopcnt: u8);
    pub fn xm_set_sample_rate(context: *mut xm_context_t, rate: u16);

    pub fn xm_get_loop_count(context: *const xm_context_t) -> u8;
    pub fn xm_get_sample_rate(context: *const xm_context_t) -> u16;
    pub fn xm_get_module_name(context: *const xm_context_t) -> *const c_char;
    pub fn xm_get_tracker_name(context: *const xm_context_t) -> *const c_char;
    pub fn xm_get_number_of_channels(context: *const xm_context_t) -> u8;
    pub fn xm_get_module_length(context: *const xm_context_t) -> u16;
    pub fn xm_get_number_of_patterns(context: *const xm_context_t) -> u16;

    pub fn xm_get_number_of_rows(context: *const xm_context_t, pattern: u16) -> u16;
    pub fn xm_get_number_of_instruments(context: *const xm_context_t) -> u8;
    pub fn xm_get_number_of_samples(context: *const xm_context_t, instrument: u8) -> u8;

    pub fn xm_get_playing_speed(context: *const xm_context_t, bpm: *mut u8, tempo: *mut u8);
    pub fn xm_get_position(context: *const xm_context_t, pattern_index: *mut u8, pattern: *mut u8, row: *mut u8, samples: *mut u32);
    pub fn xm_get_latest_trigger_of_instrument(context: *const xm_context_t, instrument: u8) -> u32;
    pub fn xm_get_latest_trigger_of_sample(context: *const xm_context_t, instr: u8, sample: u8) -> u32;
    pub fn xm_get_latest_trigger_of_channel(context: *const xm_context_t, channel: u8) -> u32;
}
