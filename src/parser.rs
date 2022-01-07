use std::io::{BufRead, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{FixedOffset, NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    AnalogChannel, AnalogScalingMode, Comtrade, ComtradeBuilder, DataFormat, FileType,
    FormatRevision, LeapSecondStatus, SamplingRate, StatusChannel, TimeQuality,
};

const CFG_SEPARATOR: &'static str = ",";

// 1991 revision uses mm/dd/yyyy format for date whereas 1999 and 2013 use dd/mm/yyyy.
// 1991 revision uses mm/dd/yyyy format for date whereas 1999 and 2013 use dd/mm/yyyy
const CFG_DATETIME_FORMAT_OLD: &'static str = "%m/%d/%Y,%H:%M:%S%.f";
const CFG_DATETIME_FORMAT: &'static str = "%d/%m/%Y,%H:%M:%S%.f";

// To preserve structure integrity, a special value is used in the binary16, binary32
// and float32 data formats when a timestamp is missing.
const TIMESTAMP_MISSING: u32 = 0xffffffff;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl FileType {
    fn from_str(value: String) -> ParseResult<Self> {
        match value.trim().to_lowercase().as_str() {
            "cfg" => Ok(FileType::Cfg),
            "dat" => Ok(FileType::Dat),
            "hdr" => Ok(FileType::Hdr),
            "inf" => Ok(FileType::Inf),
            _ => Err(ParseError::new(format!("invalid file type: '{}'", value))),
        }
    }
}

impl Default for FormatRevision {
    fn default() -> Self {
        FormatRevision::Revision1991
    }
}

impl FormatRevision {
    fn from_str(value: &str) -> ParseResult<Self> {
        match value {
            "1991" => Ok(FormatRevision::Revision1991),
            "1999" => Ok(FormatRevision::Revision1999),
            "2013" => Ok(FormatRevision::Revision2013),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE format revision: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

impl DataFormat {
    fn from_str(value: &str) -> ParseResult<Self> {
        match value.trim().to_lowercase().as_str() {
            "ascii" => Ok(DataFormat::Ascii),
            "binary" => Ok(DataFormat::Binary16),
            "binary32" => Ok(DataFormat::Binary32),
            "float32" => Ok(DataFormat::Float32),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE data format: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

impl AnalogScalingMode {
    fn from_str(value: &str) -> ParseResult<Self> {
        match value.to_lowercase().as_str() {
            "p" => Ok(AnalogScalingMode::Primary),
            "s" => Ok(AnalogScalingMode::Secondary),
            _ => Err(ParseError::new(format!(
                "invalid analog scaling mode: '{}'; must be one of: 's', 'S', 'p', 'P'",
                value,
            ))),
        }
    }
}

impl TimeQuality {
    fn parse(value: &str) -> ParseResult<Self> {
        match value.to_lowercase().trim() {
            "f" => Ok(TimeQuality::ClockFailure),
            "b" => Ok(TimeQuality::ClockUnlocked(1)),
            "a" => Ok(TimeQuality::ClockUnlocked(0)),
            "9" => Ok(TimeQuality::ClockUnlocked(-1)),
            "8" => Ok(TimeQuality::ClockUnlocked(-2)),
            "7" => Ok(TimeQuality::ClockUnlocked(-3)),
            "6" => Ok(TimeQuality::ClockUnlocked(-4)),
            "5" => Ok(TimeQuality::ClockUnlocked(-5)),
            "4" => Ok(TimeQuality::ClockUnlocked(-6)),
            "3" => Ok(TimeQuality::ClockUnlocked(-7)),
            "2" => Ok(TimeQuality::ClockUnlocked(-8)),
            "1" => Ok(TimeQuality::ClockUnlocked(-9)),
            "0" => Ok(TimeQuality::ClockLocked),
            _ => Err(ParseError::new(format!(
                "invalid time quality code '{}'",
                value,
            ))),
        }
    }
}

impl LeapSecondStatus {
    fn parse(value: &str) -> ParseResult<Self> {
        match value.trim() {
            "3" => Ok(LeapSecondStatus::NoCapability),
            "2" => Ok(LeapSecondStatus::Subtracted),
            "1" => Ok(LeapSecondStatus::Added),
            "0" => Ok(LeapSecondStatus::NotPresent),
            _ => Err(ParseError::new(format!(
                "invalid leap second indicator '{}'",
                value,
            ))),
        }
    }
}

lazy_static! {
    static ref CFF_HEADER_REGEXP: Regex = Regex::new(r#"(?i)---\s*file type:\s*(?P<file_type>[a-z]+)(\s+(?P<data_format>[a-z]+))?\s*(:\s*(?P<data_size>\d+))?\s*---$"#).unwrap();
    static ref DATE_REGEXP: Regex = Regex::new("([0-9]{1,2})/([0-9]{1,2})/([0-9]{2,4})").unwrap();
    static ref TIME_REGEXP: Regex = Regex::new("([0-9]{2}):([0-9]{2}):([0-9]{2})(\\.([0-9]{1,12}))?").unwrap();
}

// Cannot derive builder for this because of complexity of wrapping `T: BufRead` in
// `Option` - I can't figure out how to stop the default implementation from complaining
// that `BufReader<File>` doesn't implement `Copy`.
pub struct ComtradeParserBuilder<T: BufRead> {
    cff_file: Option<T>,
    cfg_file: Option<T>,
    dat_file: Option<T>,
    hdr_file: Option<T>,
    inf_file: Option<T>,
}

impl<T: BufRead> ComtradeParserBuilder<T> {
    pub fn new() -> Self {
        Self {
            cff_file: None,
            cfg_file: None,
            dat_file: None,
            hdr_file: None,
            inf_file: None,
        }
    }

    pub fn cff_file(mut self, file: T) -> Self {
        self.cff_file = Some(file);
        self
    }

    pub fn cfg_file(mut self, file: T) -> Self {
        self.cfg_file = Some(file);
        self
    }

    pub fn dat_file(mut self, file: T) -> Self {
        self.dat_file = Some(file);
        self
    }

    pub fn hdr_file(mut self, file: T) -> Self {
        self.hdr_file = Some(file);
        self
    }

    pub fn inf_file(mut self, file: T) -> Self {
        self.inf_file = Some(file);
        self
    }

    pub fn build(self) -> ComtradeParser<T> {
        ComtradeParser::new(
            self.cff_file,
            self.cfg_file,
            self.dat_file,
            self.hdr_file,
            self.inf_file,
        )
    }
}

pub struct ComtradeParser<T: BufRead> {
    cff_file: Option<T>,
    cfg_file: Option<T>,
    dat_file: Option<T>,
    hdr_file: Option<T>,
    inf_file: Option<T>,

    cfg_contents: String,
    ascii_dat_contents: String,
    binary_dat_contents: Vec<u8>,
    hdr_contents: String,
    inf_contents: String,

    builder: ComtradeBuilder,
    total_num_samples: u32,
    num_analog_channels: u32,
    num_status_channels: u32,
    analog_channels: Vec<AnalogChannel>,
    status_channels: Vec<StatusChannel>,
    is_timestamp_critical: bool,
    ts_base_unit: f64,
    data_format: Option<DataFormat>,
}

impl<T: BufRead> ComtradeParser<T> {
    pub fn new(
        cff_file: Option<T>,
        cfg_file: Option<T>,
        dat_file: Option<T>,
        hdr_file: Option<T>,
        inf_file: Option<T>,
    ) -> Self {
        Self {
            cff_file,
            cfg_file,
            dat_file,
            hdr_file,
            inf_file,

            cfg_contents: String::new(),
            ascii_dat_contents: String::new(),
            binary_dat_contents: vec![],
            hdr_contents: String::new(),
            inf_contents: String::new(),

            builder: ComtradeBuilder::default(),
            total_num_samples: 0,
            num_analog_channels: 0,
            num_status_channels: 0,
            analog_channels: vec![],
            status_channels: vec![],
            is_timestamp_critical: false,
            ts_base_unit: 0.0,
            data_format: None,
        }
    }

    pub fn dat_file(mut self, file: T) -> Self {
        self.dat_file = Some(file);
        self
    }

    pub fn hdr_file(mut self, file: T) -> Self {
        self.hdr_file = Some(file);
        self
    }

    pub fn inf_file(mut self, file: T) -> Self {
        self.inf_file = Some(file);
        self
    }

    pub fn parse(mut self) -> ParseResult<Comtrade> {
        if self.cff_file.is_some() {
            self.load_cff()?;
            self.parse_cfg()?;
            self.parse_dat()?;
        } else {
            if let Some(ref mut cfg_file) = self.cfg_file {
                cfg_file
                    .read_to_string(&mut self.cfg_contents)
                    .or(Err(ParseError::new(
                        "unable to read specified .cfg file".to_string(),
                    )))?;
            } else {
                return Err(ParseError::new(
                    "you must specify either .cff or .cfg file".to_string(),
                ));
            }

            self.parse_cfg()?;

            if let Some(ref mut dat_file) = self.dat_file {
                match self.data_format {
                    Some(DataFormat::Ascii) => {
                        dat_file
                            .read_to_string(&mut self.ascii_dat_contents)
                            .or(Err(ParseError::new(
                                "unable to read specified .dat file".into(),
                            )))?;
                    }
                    None => {
                        return Err(ParseError::new("unknown data format for data file.".into()));
                    }
                    // Other binary format.
                    _ => {
                        dat_file.read_to_end(&mut self.binary_dat_contents).or(Err(
                            ParseError::new("unable to read specified .dat file".into()),
                        ))?;
                    }
                }
            } else {
                return Err(ParseError::new(
                    "you must specify either .cff or .dat file".to_string(),
                ));
            }

            self.parse_dat()?;

            if let Some(ref mut hdr_file) = self.hdr_file {
                hdr_file
                    .read_to_string(&mut self.hdr_contents)
                    .or(Err(ParseError::new(
                        "unable to read specified .hdr file".to_string(),
                    )))?;
            }

            if let Some(ref mut inf_file) = self.inf_file {
                inf_file
                    .read_to_string(&mut self.inf_contents)
                    .or(Err(ParseError::new(
                        "unable to read specified .inf file".to_string(),
                    )))?;
            }
        }

        // `.hdr` and `.inf` files don't need parsing - if present they're
        // non-machine-readable text files for reference for humans to look at.

        self.builder.analog_channels(self.analog_channels);
        self.builder.status_channels(self.status_channels);

        Ok(self.builder.build().unwrap())
    }

    fn load_cff(&mut self) -> ParseResult<()> {
        let file = match &mut self.cff_file {
            Some(reader) => reader,
            None => {
                return Err(ParseError::new(
                    "tried to parse .cff file, but file not specified".to_string(),
                ))
            }
        };

        let mut cfg_lines: Vec<String> = vec![];
        let mut dat_lines: Vec<String> = vec![];
        let mut hdr_lines: Vec<String> = vec![];
        let mut inf_lines: Vec<String> = vec![];

        let mut current_file: Option<FileType> = None;
        let mut data_format: Option<DataFormat> = None;
        let mut data_size: Option<usize> = None;

        loop {
            // TODO: Analyse performance of using single `line` across each iteration
            //       vs. using shared buffer and cloning at end of each iteration.
            let mut line = String::new();
            let bytes_read = file.read_line(&mut line).unwrap();
            if bytes_read == 0 {
                break;
            }
            line = line.trim().to_string();

            let maybe_file_header_match = CFF_HEADER_REGEXP.captures(line.as_str());
            if let Some(header_match) = maybe_file_header_match {
                let file_type_token = header_match.name("file_type").ok_or(ParseError::new(
                    "unable to find file type in CFF header Regexp".to_string(),
                ))?;

                let maybe_data_format_token = header_match.name("data_format");
                let maybe_data_size_token = header_match.name("data_size");

                current_file = Some(FileType::from_str(file_type_token.as_str().to_string())?);

                if let Some(data_format_token) = maybe_data_format_token {
                    data_format = Some(DataFormat::from_str(data_format_token.as_str())?);
                }

                if let Some(data_size_token) = maybe_data_size_token {
                    data_size = Some(data_size_token.as_str().to_string().parse::<usize>().or(
                        Err(ParseError::new(format!(
                            "unable to parse .dat size: '{}'",
                            data_size_token.as_str()
                        ))),
                    )?)
                }

                continue;
            }

            match current_file {
                Some(FileType::Cfg) => cfg_lines.push(line),
                Some(FileType::Dat) => {
                    if data_format == Some(DataFormat::Ascii) {
                        dat_lines.push(line);
                    } else {
                        unimplemented!()
                    }
                }
                Some(FileType::Hdr) => hdr_lines.push(line),
                Some(FileType::Inf) => inf_lines.push(line),
                None => {
                    return Err(ParseError::new(
                        "encountered file contents line before header in .cff".to_string(),
                    ))
                }
            }
        }

        // TODO: Create `io::Cursor()` here instead of simply whacking all the contents
        //  into a string. This would allow for buffered reading of separate files, at least.

        self.cfg_contents = cfg_lines.join("\n");
        self.ascii_dat_contents = dat_lines.join("\n");
        self.hdr_contents = hdr_lines.join("\n");
        self.inf_contents = inf_lines.join("\n");

        Ok(())
    }

    fn parse_cfg(&mut self) -> ParseResult<()> {
        // TODO: There must be a more efficient way of doing this using line iterators,
        //  I just need to figure out how to create my own line iterator in the
        //  `load_cff()` function.
        let mut lines = self.cfg_contents.split("\n");

        let early_end_err = ParseError::new("unexpected end of .cfg file".to_string());

        let mut line_number = 1;
        let mut line = "";
        let mut line_values: Vec<&str> = vec![];

        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Station name, identification and optionally revision year:
        // 1991:       station_name,rec_dev_id
        // 1999, 2013: station_name,rec_dev_id,rev_year

        // We need this value later to know when to quit.
        self.builder.station_name(line_values[0].to_string());
        self.builder.recording_device_id(line_values[1].to_string());

        let format_revision = match line_values.len() {
            3 => FormatRevision::from_str(line_values[2].trim())?,
            2 => FormatRevision::Revision1991,
            _ => {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )))
            }
        };
        self.builder.revision(format_revision);

        line_number += 1;

        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Number and type of channels:
        // TT,##A,##D
        if line_values.len() != 3 {
            return Err(ParseError::new(format!(
                "unexpected number of values on line {}",
                line_number
            )));
        }

        let num_total_channels =
            line_values[0]
                .trim()
                .to_string()
                .parse()
                .or(Err(ParseError::new(format!(
                    "invalid integer value for number of total channels: '{}'",
                    line_values[0]
                ))))?;
        self.builder.num_total_channels(num_total_channels);

        let mut num_analog_channels_token = line_values[1].to_string();
        // Last character contains "A" identifier.
        num_analog_channels_token.pop();
        let num_analog_channels = num_analog_channels_token
            .trim()
            .to_string()
            .parse()
            .or(Err(ParseError::new(format!(
                "invalid integer value for number of analog channels: '{}'",
                num_analog_channels_token
            ))))?;
        self.builder.num_analog_channels(num_analog_channels);
        self.num_analog_channels = num_analog_channels;

        let mut num_status_channels_token = line_values[2].to_string();
        // Last character contains "D" identifier.
        num_status_channels_token.pop();
        let num_status_channels = num_status_channels_token
            .trim()
            .to_string()
            .parse()
            .or(Err(ParseError::new(format!(
                "invalid integer value for number of status channels: '{}'",
                num_status_channels_token
            ))))?;
        self.builder.num_status_channels(num_status_channels);
        self.num_status_channels = num_status_channels;

        line_number += 1;

        let mut analog_channels: Vec<AnalogChannel> =
            Vec::with_capacity(self.num_analog_channels as usize);
        let mut status_channels: Vec<StatusChannel> =
            Vec::with_capacity(self.num_status_channels as usize);

        // Analog channel information:
        // An,ch_id,ph,ccbm,uu,a,b,skew,min,max,primary,secondary,PS
        for i in 0..self.num_analog_channels {
            line = lines.next().ok_or(early_end_err.clone())?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 13 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            let analog_index =
                line_values[0]
                    .trim()
                    .to_string()
                    .parse::<u32>()
                    .or(Err(ParseError::new(format!(
                        "invalid integer value for analog channel {} index: {}",
                        i, line_values[0]
                    ))))?;

            let name = line_values[1].to_string();
            let phase = line_values[2].to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].to_string(); // Non-critical.
            let units = line_values[4].to_string();

            let multiplier =
                line_values[5]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} multiplier: {}",
                        i, line_values[5]
                    ))))?;

            let offset_adder =
                line_values[6]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} offset adder: {}",
                        i, line_values[6]
                    ))))?;

            let skew =
                line_values[7]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} skew: {}",
                        i, line_values[7]
                    ))))?;

            let min_value =
                line_values[8]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} minimum value: {}",
                        i, line_values[8]
                    ))))?;

            let max_value =
                line_values[9]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} maximum value: {}",
                        i, line_values[9]
                    ))))?;

            let primary_factor =
                line_values[10]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} primary factor: {}",
                        i, line_values[10]
                    ))))?;

            let secondary_factor =
                line_values[11]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                        "invalid real numeric value for analog channel {} secondary factor: {}",
                        i, line_values[11]
                    ))))?;

            let scaling_mode = AnalogScalingMode::from_str(line_values[12].trim())?;

            analog_channels.push(AnalogChannel {
                index: analog_index,
                name,
                phase,
                circuit_component_being_monitored,
                units,
                min_value,
                max_value,
                multiplier,
                offset_adder,
                skew,
                primary_factor,
                secondary_factor,
                scaling_mode,
                data: vec![],
            });

            line_number += 1;
        }
        self.analog_channels = analog_channels;

        // Status (digital) channel information:
        // Dn,ch_id,ph,ccbm,y
        for i in 0..self.num_status_channels {
            line = lines.next().ok_or(early_end_err.clone())?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 5 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            let status_index =
                line_values[0]
                    .trim()
                    .to_string()
                    .parse::<u32>()
                    .or(Err(ParseError::new(format!(
                        "invalid integer value for status channel {} index: {}",
                        i, line_values[0]
                    ))))?;

            let name = line_values[1].to_string();
            let phase = line_values[2].to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].to_string(); // Non-critical.

            let normal_status_value =
                line_values[4]
                    .trim()
                    .to_string()
                    .parse::<u8>()
                    .or(Err(ParseError::new(format!(
                        "invalid integer value for status channel {} normal value: {}",
                        i, line_values[4]
                    ))))?;
            if normal_status_value != 0 && normal_status_value != 1 {
                return Err(ParseError::new(format!("invalid normal status value for status channel {}: {}; expected one of : '0', '1'", i, line_values[4])));
            }

            status_channels.push(StatusChannel {
                index: status_index,
                name,
                phase,
                circuit_component_being_monitored,
                normal_status_value,
                data: vec![],
            });

            line_number += 1;
        }
        self.status_channels = status_channels;

        line = lines.next().ok_or(early_end_err.clone())?;

        // Line frequency
        // lf
        let line_frequency = line
            .trim()
            .to_string()
            .parse::<f64>()
            .or(Err(ParseError::new(format!(
                "invalid real numeric value for line frequency: '{}'",
                line,
            ))))?;
        self.builder.line_frequency(line_frequency);

        line_number += 1;

        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Sampling rate information
        // nrates (x 1)
        // samp,endsamp (x nrates)
        if line_values.len() != 1 {
            return Err(ParseError::new(format!(
                "unexpected number of values on line {}",
                line_number
            )));
        }

        let num_sampling_rates =
            line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .or(Err(ParseError::new(format!(
                    "invalid integer value for number of sample rates: {}",
                    line_values[0]
                ))))?;

        let mut sampling_rates: Vec<SamplingRate> = Vec::with_capacity(num_sampling_rates as usize);

        for i in 0..num_sampling_rates {
            line = lines.next().ok_or(early_end_err.clone())?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 2 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            // The sample rate in Hertz of this sample.
            let rate_hz =
                line_values[0]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .or(Err(ParseError::new(format!(
                    "invalid float value for sample rate frequency for rate n# {} on line {}: {}",
                    i, line_number, line_values[0]
                ))))?;

            // The sample number of the final sample that uses this sample rate. Note this corresponds
            // to the sample number value in the data itself, not an index.
            let end_sample_number =
                line_values[1]
                    .trim()
                    .to_string()
                    .parse::<u32>()
                    .or(Err(ParseError::new(format!(
                        "invalid integer value for end sample number for rate n# {} on line {}: {}",
                        i, line_number, line_values[1]
                    ))))?;

            sampling_rates.push(SamplingRate {
                rate_hz,
                end_sample_number,
            });
        }

        self.total_num_samples = sampling_rates
            .iter()
            .map(|r| r.end_sample_number)
            .max()
            .unwrap();

        // Now that we know how many samples we have in total, we can update the channel buffers
        // with the correct capacity to make `push()` operations more efficient.
        for c in self.analog_channels.iter_mut() {
            c.data = Vec::with_capacity(self.total_num_samples as usize);
        }
        for c in self.status_channels.iter_mut() {
            c.data = Vec::with_capacity(self.total_num_samples as usize);
        }

        // If file has 0 for number of sample rates, there's an extra line which just contains 0
        // indicating no fixed sample rate and the total number of samples. We don't need this data
        // so we just ignore it.
        if num_sampling_rates == 0 {
            line_number += 1;
            line = lines.next().ok_or(early_end_err.clone())?;
        }

        self.is_timestamp_critical = num_sampling_rates == 0;
        self.builder.sampling_rates(sampling_rates);

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Date/time stamps
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // TODO: Whether this is to micro or nano seconds determines whether how to calculate
        //       real time values from timestamps (I think - not 100% on this).

        // Time of the first data sample in data file.
        let datetime_format = if format_revision == FormatRevision::Revision1991 {
            CFG_DATETIME_FORMAT_OLD
        } else {
            CFG_DATETIME_FORMAT
        };

        let start_time = NaiveDateTime::parse_from_str(line.trim(), datetime_format).or(Err(
            ParseError::new(format!(
                "invalid datetime value for start time on line {}: {}",
                line_number, line,
            )),
        ))?;
        self.builder.start_time(start_time);

        self.ts_base_unit = ts_base_unit(line.trim())?;

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;

        // Time that the COMTRADE record recording was triggered.
        let trigger_time = NaiveDateTime::parse_from_str(line.trim(), datetime_format).or(Err(
            ParseError::new(format!(
                "invalid datetime value for trigger time on line {}: {}",
                line_number, line,
            )),
        ))?;
        self.builder.trigger_time(trigger_time);

        // According to the spec, if the start time is in micro/nanoseconds, the
        // other one should be too. If they are inconsistent, just take the lower one
        // to be safe. In the future this would be a good place to raise a warning.
        self.ts_base_unit = self.ts_base_unit.min(ts_base_unit(line.trim())?);

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;

        // Data file type
        // ft
        let data_format = DataFormat::from_str(line)?;
        self.data_format = Some(data_format.clone());
        self.builder.data_format(data_format);

        // 1991 format ends here - rest of values are 1999 and 2013 only.
        if format_revision == FormatRevision::Revision1991 {
            return Ok(());
        }

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;

        // Time stamp multiplication factor
        // timemult
        // The base unit for the timestamps in the data file is determined from the CFG,
        // apparently from the time/stamp. It's not clear to me how this is determined.
        // Regardless, this multiplicative factor allows you to store longer time ranges
        // within a single COMTRADE record.

        let time_mult = line.trim().parse::<f64>().or(Err(ParseError::new(format!(
            "invalid float value for time multiplication factor on line {}: {}",
            line_number, line,
        ))))?;
        self.builder.timestamp_multiplication_factor(time_mult);

        // Default values for optional revision-based fields.
        self.builder.time_offset(None);
        self.builder.local_offset(None);
        self.builder.time_quality(None);
        self.builder.leap_second_status(None);

        // 1999 format ends here - rest of values are 2013 only.
        if format_revision == FormatRevision::Revision1999 {
            return Ok(());
        }

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time information and relationship between local time and UTC
        // time_code, local_code
        self.builder.time_offset(parse_time_offset(line_values[0])?);
        self.builder
            .local_offset(parse_time_offset(line_values[1])?);

        line_number += 1;
        line = lines.next().ok_or(early_end_err.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time quality of samples
        // tmq_code,leapsec
        let tmq_code = TimeQuality::parse(line_values[0])?;
        self.builder.time_quality(Some(tmq_code));

        let leap_second_status = LeapSecondStatus::parse(line_values[1])?;
        self.builder.leap_second_status(Some(leap_second_status));

        Ok(())
    }

    fn parse_dat(&mut self) -> ParseResult<()> {
        match self.data_format {
            Some(DataFormat::Ascii) => self.parse_dat_ascii(),
            Some(_) => self.parse_dat_binary(),
            None => Err(ParseError::new("Data format not specified.".into())),
        }
    }

    fn parse_dat_ascii(&mut self) -> ParseResult<()> {
        // One column for index, one for timestamp.
        let expected_num_cols = (self.num_status_channels + self.num_analog_channels + 2) as usize;

        let mut sample_numbers: Vec<u32> = Vec::with_capacity(self.total_num_samples as usize);
        let mut timestamps: Vec<f64> = Vec::with_capacity(self.total_num_samples as usize);

        for (i, line) in self
            .ascii_dat_contents
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .enumerate()
        {
            let data_values: Vec<&str> = line.split(",").collect();

            if data_values.len() != expected_num_cols {
                return Err(ParseError::new(format!(
                    "Row {} has incorrect number of columns; expected {} but got {}.",
                    i,
                    expected_num_cols,
                    data_values.len()
                )));
            }

            let sample_number = data_values[0]
                .trim()
                .parse::<u32>()
                .or(Err(ParseError::new(format!(
                    "[DAT] Invalid sample number {} on line {}",
                    data_values[0].trim(),
                    i + 1
                ))))?;

            sample_numbers.push(sample_number);

            let timestamp = match data_values[1].trim() {
                "" => None, // TODO: Check whether there are any sampling rates. This is critical if there aren't any sampling rates.
                v => Some(v.parse::<u32>().or(Err(ParseError::new(format!(
                    "[DAT] Invalid timestamp {} on line {}.",
                    data_values[1].trim(),
                    i
                ))))?),
            };

            timestamps.push(self.real_time(sample_number, timestamp)?);

            for channel_idx in 0..self.num_analog_channels {
                let value_str = data_values[(channel_idx + 2) as usize].trim();
                let value_raw = value_str.parse::<f64>().or(Err(ParseError::new(format!(
                    "[DAT] Invalid float value {} in analog channel {} on line {}.",
                    value_str,
                    channel_idx + 1,
                    i + 1
                ))))?;

                let adder = self.analog_channels[channel_idx as usize].offset_adder;
                let multiplier = self.analog_channels[channel_idx as usize].multiplier;
                let value = value_raw * multiplier + adder;

                self.analog_channels[channel_idx as usize].push_datum(value);
            }

            for channel_idx in 0..self.num_status_channels {
                let value_str =
                    data_values[(channel_idx + self.num_analog_channels + 2) as usize].trim();
                let value = value_str.parse::<u8>().or(Err(ParseError::new(format!(
                    "[DAT] Invalid status value {} in status channel {} on line {}",
                    value_str,
                    channel_idx + 1,
                    i + 1
                ))))?;
                self.status_channels[channel_idx as usize].push_datum(value);
            }
        }

        self.builder.sample_numbers(sample_numbers);
        self.builder.timestamps(timestamps);

        Ok(())
    }

    fn parse_dat_binary(&mut self) -> ParseResult<()> {
        // Status channels are binary (0 or 1) and combined into 16-bit bitfields.
        // Each 16-bit bitfield is referred to as a status "group".
        let num_status_groups = (self.num_status_channels as f32 / 16.0).ceil() as u8;

        let mut cursor = Cursor::new(&self.binary_dat_contents);

        let mut sample_numbers: Vec<u32> = Vec::with_capacity(self.total_num_samples as usize);
        let mut timestamps: Vec<f64> = Vec::with_capacity(self.total_num_samples as usize);

        let mut i = 0;
        loop {
            if i >= self.total_num_samples {
                break;
            }

            let sample_number = cursor.read_u32::<LittleEndian>().unwrap();
            let timestamp = cursor.read_u32::<LittleEndian>().unwrap();

            sample_numbers.push(sample_number);
            timestamps.push(self.real_time(
                sample_number,
                if timestamp == TIMESTAMP_MISSING {
                    None
                } else {
                    Some(timestamp)
                },
            )?);

            let analog_values = (0..self.num_analog_channels)
                .map(|channel_idx| {
                    let value = match self.data_format {
                        Some(DataFormat::Binary16) => {
                            cursor.read_i16::<LittleEndian>().unwrap() as f64
                        }
                        Some(DataFormat::Binary32) => {
                            cursor.read_i32::<LittleEndian>().unwrap() as f64
                        }
                        Some(DataFormat::Float32) => {
                            cursor.read_f32::<LittleEndian>().unwrap() as f64
                        }
                        _ => panic!(
                            "tried to parse binary data for non-binary or invalid data format"
                        ), // TODO: Turn into proper parse result.
                    };

                    let adder = self.analog_channels[channel_idx as usize].offset_adder;
                    let multiplier = self.analog_channels[channel_idx as usize].multiplier;
                    value * multiplier + adder
                })
                .collect::<Vec<f64>>();

            for (i, v) in analog_values.into_iter().enumerate() {
                self.analog_channels[i].push_datum(v);
            }

            let status_values = (0..num_status_groups)
                .map(|_| cursor.read_u16::<LittleEndian>().unwrap())
                .map(|group| {
                    (0..16)
                        .map(|bit_idx| {
                            // Least significant bit is first status channel.
                            let bit_mask = 0b01 << bit_idx;
                            let val = (group & bit_mask) >> bit_idx;
                            val as u8
                        })
                        .collect::<Vec<u8>>()
                })
                .flatten()
                // Groups are padded out with zeros - we want to ignore the padded values.
                .take(self.num_status_channels as usize)
                .collect::<Vec<u8>>();

            for (i, v) in status_values.into_iter().enumerate() {
                self.status_channels[i].push_datum(v);
            }

            i += 1;
        }

        self.builder.sample_numbers(sample_numbers);
        self.builder.timestamps(timestamps);

        Ok(())
    }

    /// Calculate the true value of the timestamp from the in-file value, using the
    /// sampling information if possible, otherwise the in-data timestamp values
    /// along with relevant multiplicative factors from configuration file. This
    /// does *not* include the skew, which needs to be done on a per-channel basis.
    fn real_time(&self, sample_number: u32, timestamp: Option<u32>) -> ParseResult<f64> {
        if !self.is_timestamp_critical || timestamp.is_none() {
            let sampling_rate = self.sampling_rate_for_sample(sample_number);
            return ParseResult::Ok((sample_number - 1) as f64 / sampling_rate);
        }

        match timestamp {
            Some(ts_value) => {
                let multiplier = self.builder.timestamp_multiplication_factor.unwrap_or(1.0);
                ParseResult::Ok(ts_value as f64 * self.ts_base_unit * multiplier)
            }
            None => ParseResult::Err(ParseError::new(format!(
                "timestamp is critical but not present in sample number {}",
                sample_number
            ))),
        }
    }

    fn sampling_rate_for_sample(&self, sample_number: u32) -> f64 {
        let sampling_rates: &Vec<SamplingRate> = self.builder.sampling_rates.as_ref().unwrap();

        let maybe_rate = sampling_rates
            .iter()
            .find(|r| sample_number <= r.end_sample_number);

        match maybe_rate {
            Some(rate) => rate.rate_hz,
            None => 1.0, // TODO: What should we return here? Default value? None?
        }
    }
}

/// If a timestamp is specified to 6 dp then the timestamps should be interpreted as
/// in the base unit of microseconds. If the timestamp has 9 dp, the timestamps should
/// be interpreted in nanoseconds.
fn ts_base_unit(datetime_stamp: &str) -> ParseResult<f64> {
    let fraction = datetime_stamp.rsplit(".").next();

    if fraction.is_none() {
        return Err(ParseError::new(
            "unable to find fractional value in date/time stamp".into(),
        ));
    }

    if fraction.unwrap().len() <= 6 {
        Ok(1e-6)
    } else {
        Ok(1e-9)
    }
}

/// Parse COMTRADE time offset format into chrono struct.
///
/// COMTRADE format looks like:
///   - "-4" meaning 4 hours west of UTC
///   - "+10h30" meaning 10 hours and 30 minutes east of UTC.
///   - "-7h15" meaning 7 hours and 15 minutes west of UTC.
///   - "0" meaning same as UTC.
///
/// "Not applicable" is a valid value for this, represents in the COMTRADE file
/// as `x` - this is given the value of `None` here.
fn parse_time_offset(offset_str: &str) -> ParseResult<Option<FixedOffset>> {
    let time_value = offset_str.trim();

    // Special value indicating offset field does not apply.
    if time_value.to_lowercase() == "x" {
        return Ok(None);
    }

    let maybe_hours = time_value.parse::<i32>();

    if let Ok(hours) = maybe_hours {
        // Offset specified just as number of hours, e.g. "-4", "+10", "0".
        return Ok(Some(FixedOffset::east(hours * 3600)));
    }

    // Offset specified as number + minutes, e.g. "-7h15", "+9h45".
    let time_split: Vec<&str> = time_value.split("h").collect();
    if time_split.len() != 2 {
        return Err(ParseError::new(format!(
            "invalid time offset on line: {}",
            time_value,
        )));
    }

    let hours = time_split[0]
        .trim()
        .parse::<i32>()
        .or(Err(ParseError::new(format!(
            "invalid hour offset in time offset: {} in {}",
            time_split[0], time_value,
        ))))?;
    let minutes = time_split[1]
        .trim()
        .parse::<i32>()
        .or(Err(ParseError::new(format!(
            "invalid minute offset in time offset: {} in {}",
            time_split[1], time_value,
        ))))?;

    let total_offset = if hours > 0 {
        hours * 3600 + minutes * 60
    } else {
        hours * 3600 - minutes * 60
    };

    return Ok(Some(FixedOffset::east(total_offset)));
}
