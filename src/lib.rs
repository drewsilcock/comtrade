pub mod parser;

use derive_builder::Builder;

pub use parser::{ComtradeParser, ComtradeParserBuilder, ParseError, ParseResult};

#[derive(Debug, Clone, PartialEq)]
enum FileType {
    Cfg,
    Dat,
    Hdr,
    Inf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatRevision {
    Revision1991,
    Revision1999,
    Revision2013,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Ascii,
    Binary16,
    Binary32,
    Float32,
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat::Ascii
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AnalogScalingMode {
    Primary,
    Secondary,
}

#[derive(Debug, Clone)]
pub struct AnalogChannel {
    /// 1-indexed counter used to determine which channel this is in a COMTRADE record.
    pub index: u32,
    pub name: String,
    pub phase: String,
    pub circuit_component_being_monitored: String,
    pub units: String,
    pub min_value: f64,
    pub max_value: f64,

    // Used to calculate real values from data points so don't need to be exposed.
    multiplier: f64,
    offset_adder: f64,

    /// Value in microseconds.
    skew: f64,

    /// Used to convert between primary and secondary values in channel.
    primary_factor: f64,

    /// Used to convert between primary and secondary values in channel.
    secondary_factor: f64,

    scaling_mode: AnalogScalingMode,
}

#[derive(Debug, Clone)]
pub struct StatusChannel {
    pub index: u32,
    pub name: String,
    pub phase: String,
    pub circuit_component_being_monitored: String,
    pub normal_status_value: u8,
}

#[derive(Debug, Clone)]
pub struct SamplingRate {
    pub rate_hz: f64,
    pub end_sample_number: u32,
}

#[derive(Default, Debug, Clone, Builder)]
pub struct Comtrade {
    pub station_name: String,
    pub recording_device_id: String,
    pub revision: FormatRevision,

    // Don't think it's really necessary for have these fields.
    pub num_total_channels: u32,
    pub num_analog_channels: u32,
    pub num_status_channels: u32,

    pub analog_channels: Vec<AnalogChannel>,
    pub status_channels: Vec<StatusChannel>,

    pub line_frequency: f64,

    // Don't think these is necessary either, it's just used to parse / process the data file.
    pub data_format: DataFormat,
    pub timestamp_multiplication_factor: f64,
}
