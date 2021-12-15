pub mod parser;

use chrono::{FixedOffset, NaiveDateTime};
use derive_builder::Builder;

pub use parser::{ComtradeParser, ComtradeParserBuilder, ParseError, ParseResult};

#[derive(Debug, Clone, PartialEq)]
enum FileType {
    Cfg,
    Dat,
    Hdr,
    Inf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
pub enum AnalogScalingMode {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, PartialEq)]
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
    pub multiplier: f64,
    pub offset_adder: f64,

    /// Value in microseconds.
    pub skew: f64,

    /// Used to convert between primary and secondary values in channel.
    pub primary_factor: f64,

    /// Used to convert between primary and secondary values in channel.
    pub secondary_factor: f64,

    pub scaling_mode: AnalogScalingMode,

    pub data: Vec<f64>,
}

impl AnalogChannel {
    fn push_datum(&mut self, value: f64) {
        self.data.push(value);
    }

    // TODO: Method for retrieving datum at index / sample number including value and time calculations.
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusChannel {
    pub index: u32,
    pub name: String,
    pub phase: String,
    pub circuit_component_being_monitored: String,
    pub normal_status_value: u8,

    pub data: Vec<u8>, // Values are 0 or 1.
}

impl StatusChannel {
    fn push_datum(&mut self, value: u8) {
        self.data.push(value);
    }

    // TODO: Method for retrieving datum at index / sample number including time calculations.
}

#[derive(Debug, Clone, PartialEq)]
pub struct SamplingRate {
    pub rate_hz: f64,
    pub end_sample_number: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeQuality {
    /// Clock in locked and in normal operation.
    ClockLocked,

    /// Clock is unlocked and reliable to a specified precision. Value given is
    /// reliability of time as power of 10. For instances:
    ///
    /// ```rust
    /// use comtrade::TimeQuality;
    ///
    /// // Device clock time is reliable to 1 nanosecond (10^-9).
    /// let q1 = TimeQuality::ClockUnlocked(-9);
    ///
    /// // Device clock time is reliable to 10 microseconds (10^-5).
    /// let q2 = TimeQuality::ClockUnlocked(-5);
    ///
    /// // Device clock time is reliable to 10 seconds (10^1).
    /// let q3 = TimeQuality::ClockUnlocked(1);
    /// ```
    ///
    /// COMTRADE format specification expects values between -9 and 1.
    ClockUnlocked(i32),

    /// There is a fault in the clock and the time it gives is not reliable.
    ClockFailure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeapSecondStatus {
    /// Time source does not have capability to address presence of leap seconds.
    NoCapability,

    /// A leap second has been subtracted from the record.
    Subtracted,

    /// A leap second has been added to the record.
    Added,

    /// No leap second is present in the record.
    NotPresent,
}

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Comtrade {
    pub station_name: String,
    pub recording_device_id: String,
    pub revision: FormatRevision,

    // Don't think it's really necessary for have these fields.
    pub num_total_channels: u32,
    pub num_analog_channels: u32,
    pub num_status_channels: u32,

    pub sample_numbers: Vec<u32>,
    pub timestamps: Vec<Option<u32>>,
    pub analog_channels: Vec<AnalogChannel>,
    pub status_channels: Vec<StatusChannel>,

    pub line_frequency: f64,

    pub sampling_rates: Vec<SamplingRate>,
    pub start_time: NaiveDateTime,
    pub trigger_time: NaiveDateTime,

    // Don't think these is necessary either, it's just used to parse / process the data file.
    pub data_format: DataFormat,

    // Below data are 1999 format onwards only.

    // Don't use option for this - just default to 1 if it's not present.
    pub timestamp_multiplication_factor: f64,

    // Below data are 2013 format onwards only.
    pub time_offset: Option<FixedOffset>,
    pub local_offset: Option<FixedOffset>,

    pub time_quality: Option<TimeQuality>,
    pub leap_second_status: Option<LeapSecondStatus>,
}

impl Default for Comtrade {
    fn default() -> Self {
        Comtrade {
            station_name: Default::default(),
            recording_device_id: Default::default(),
            revision: Default::default(),
            num_total_channels: Default::default(),
            num_analog_channels: Default::default(),
            num_status_channels: Default::default(),
            sample_numbers: Default::default(),
            timestamps: Default::default(),
            analog_channels: Default::default(),
            status_channels: Default::default(),
            line_frequency: Default::default(),
            sampling_rates: Default::default(),
            start_time: NaiveDateTime::from_timestamp(0, 0),
            trigger_time: NaiveDateTime::from_timestamp(0, 0),
            data_format: Default::default(),
            timestamp_multiplication_factor: 1.0,
            time_offset: Default::default(),
            local_offset: Default::default(),
            time_quality: Default::default(),
            leap_second_status: Default::default(),
        }
    }
}
