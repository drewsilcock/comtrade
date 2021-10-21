use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    AnalogChannel,
    AnalogScalingMode,
    Comtrade,
    ComtradeBuilder,
    DataFormat,
    FileType,
    FormatRevision,
    StatusChannel,
};

const CFG_SEPARATOR: &'static str = ",";

type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}

const FILE_TYPE_CFG_TOKEN: &'static str = "CFG";
const FILE_TYPE_DAT_TOKEN: &'static str = "DAT";
const FILE_TYPE_HDR_TOKEN: &'static str = "HDR";
const FILE_TYPE_INF_TOKEN: &'static str = "INF";

impl FileType {
    fn from_str(value: String) -> ParseResult<Self> {
        match value.as_str() {
            FILE_TYPE_CFG_TOKEN => Ok(FileType::Cfg),
            FILE_TYPE_DAT_TOKEN => Ok(FileType::Dat),
            FILE_TYPE_HDR_TOKEN => Ok(FileType::Hdr),
            FILE_TYPE_INF_TOKEN => Ok(FileType::Inf),
            _ => Err(ParseError::new(format!("invalid file type: '{}'", value)))
        }
    }
}

impl Default for FormatRevision {
    fn default() -> Self { FormatRevision::Revision1999 }
}

const REVISION_1991_TOKEN: &'static str = "1991";
const REVISION_1999_TOKEN: &'static str = "1999";
const REVISION_2013_TOKEN: &'static str = "2013";

impl FormatRevision {
    fn from_str(value: &str) -> ParseResult<Self> {
        match value {
            REVISION_1991_TOKEN => Ok(FormatRevision::Revision1991),
            REVISION_1999_TOKEN => Ok(FormatRevision::Revision1999),
            REVISION_2013_TOKEN => Ok(FormatRevision::Revision2013),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE format revision: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

const DATA_FORMAT_ASCII_TOKEN: &'static str = "ASCII";
const DATA_FORMAT_BINARY16_TOKEN: &'static str = "BINARY";
const DATA_FORMAT_BINARY32_TOKEN: &'static str = "BINARY32";
const DATA_FORMAT_FLOAT32_TOKEN: &'static str = "FLOAT32";

impl DataFormat {
    fn from_str(value: String) -> ParseResult<Self> {
        match value.as_str() {
            DATA_FORMAT_ASCII_TOKEN => Ok(DataFormat::Ascii),
            DATA_FORMAT_BINARY16_TOKEN => Ok(DataFormat::Binary16),
            DATA_FORMAT_BINARY32_TOKEN => Ok(DataFormat::Binary32),
            DATA_FORMAT_FLOAT32_TOKEN => Ok(DataFormat::Float32),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE data format: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

const ANALOG_SCALING_MODE_PRIMARY_TOKEN: &'static str = "p";
const ANALOG_SCALING_MODE_SECONDARY_TOKEN: &'static str = "s";

impl AnalogScalingMode {
    fn from_str(value: &str) -> ParseResult<Self> {
        match value.to_lowercase().as_str() {
            ANALOG_SCALING_MODE_PRIMARY_TOKEN => Ok(AnalogScalingMode::Primary),
            ANALOG_SCALING_MODE_SECONDARY_TOKEN => Ok(AnalogScalingMode::Secondary),
            _ => Err(ParseError::new(
                format!(
                    "invalid analog scaling mode: '{}'; must be one of: 's', 'S', 'p', 'P'",
                    value,
                ))
            )
        }
    }
}

lazy_static! {
    static ref CFF_HEADER_REGEXP: Regex = Regex::new(r#"(?i)--- file type:\s*(?<file_type>[a-z]+)(?:\s*(?<data_format>[a-z]+)\s*:\s*(?<data_size>\d+)?)? ---$"#).unwrap();
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

    pub fn build(mut self) -> ComtradeParser<T> {
        ComtradeParser::new(self.cff_file, self.cfg_file, self.dat_file, self.hdr_file, self.inf_file)
    }
}

pub struct ComtradeParser<T: BufRead> {
    cff_file: Option<T>,
    cfg_file: Option<T>,
    dat_file: Option<T>,
    hdr_file: Option<T>,
    inf_file: Option<T>,

    cfg_contents: String,
    dat_contents: String,
    hdr_contents: String,
    inf_contents: String,

    builder: ComtradeBuilder,
    num_analog_channels: u32,
    num_status_channels: u32,
}

impl<T: BufRead> ComtradeParser<T> {
    pub fn new(cff_file: Option<T>, cfg_file: Option<T>, dat_file: Option<T>, hdr_file: Option<T>, inf_file: Option<T>) -> Self {
        Self {
            cff_file,
            cfg_file,
            dat_file,
            hdr_file,
            inf_file,

            cfg_contents: String::new(),
            dat_contents: String::new(),
            hdr_contents: String::new(),
            inf_contents: String::new(),

            builder: ComtradeBuilder::default(),
            num_analog_channels: 0,
            num_status_channels: 0,
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
        } else {
            if let Some(ref mut cfg_file) = self.cfg_file {
                cfg_file
                    .read_to_string(&mut self.cfg_contents)
                    .or(Err(ParseError::new("unable to read specified .cfg file".to_string())))?;
            } else {
                return Err(ParseError::new("you must specify either .cff or .cfg file".to_string()));
            }

            if let Some(ref mut dat_file) = self.dat_file {
                let dat_lines = dat_file.lines();
                //dat_file
                //    .read_to_string(&mut self.dat_contents)
                //    .or(Err(ParseError::new("unable to read specified .dat file".to_string())))?;
            } else {
                return Err(ParseError::new("you must specify either .cff or .dat file".to_string()));
            }

            if let Some(ref mut hdr_file) = self.hdr_file {
                hdr_file
                    .read_to_string(&mut self.hdr_contents)
                    .or(Err(ParseError::new("unable to read specified .hdr file".to_string())))?;
            }

            if let Some(ref mut inf_file) = self.inf_file {
                inf_file
                    .read_to_string(&mut self.inf_contents)
                    .or(Err(ParseError::new("unable to read specified .inf file".to_string())))?;
            }
        }

        self.parse_cfg()?;
        self.parse_dat()?;

        // `.hdr` and `.inf` files don't need parsing - if present they're
        // non-machine-readable text files for reference for humans to look at.

        self.builder.revision(FormatRevision::Revision1999);
        Ok(self.builder.build().unwrap())
    }

    fn load_cff(&mut self) -> ParseResult<()> {
        let file = match &mut self.cff_file {
            Some(reader) => reader,
            None => return Err(ParseError::new("tried to parse .cff file, but file not specified".to_string())),
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

            let maybe_file_header_match = CFF_HEADER_REGEXP.captures(line.as_str());
            if let Some(header_match) = maybe_file_header_match {
                let file_type_token = header_match
                    .name("file_type")
                    .ok_or(ParseError::new("unable to find file type in CFF header Regexp".to_string()))?;

                let maybe_data_format_token = header_match.name("data_format");
                let maybe_data_size_token = header_match.name("data_size");

                current_file = Some(
                    FileType::from_str(file_type_token.as_str().to_string())?
                );

                if let Some(data_format_token) = maybe_data_format_token {
                    data_format = Some(
                        DataFormat::from_str(data_format_token.as_str().to_string())?
                    );
                }

                if let Some(data_size_token) = maybe_data_size_token {
                    data_size = Some(
                        data_size_token
                            .as_str()
                            .to_string()
                            .parse::<usize>()
                            .or(Err(ParseError::new(
                                format!(
                                    "unable to parse .dat size: '{}'",
                                    data_size_token.as_str()
                                ))
                            ))?
                    )
                }

                continue;
            }

            match current_file {
                Some(FileType::Cfg) => cfg_lines.push(line),
                Some(FileType::Dat) => dat_lines.push(line),
                Some(FileType::Hdr) => hdr_lines.push(line),
                Some(FileType::Inf) => inf_lines.push(line),
                None => return Err(ParseError::new("encountered file contents line before header in .cff".to_string())),
            }
        }

        // TODO: Create `io::Cursor()` here instead of simply whacking all the contents
        //  into a string. This would allow for buffered reading of separate files, at least.

        self.cfg_contents = cfg_lines.join("\n");
        self.dat_contents = dat_lines.join("\n");
        self.hdr_contents = hdr_lines.join("\n");
        self.inf_contents = inf_lines.join("\n");

        Ok(())
    }

    fn parse_cfg(&mut self) -> ParseResult<()> {
        // TODO: There must be a more efficient way of doing this using line iterators,
        //  I just need to figure out how to create my own line iterator in the
        //  `load_cff()` function.
        let mut lines = self.cfg_contents.split("\n");

        let earlyEndErr = ParseError::new("unexpected end of .cfg file".to_string());

        let mut line_number = 1;
        let mut line = "";
        let mut line_values: Vec<&str> = vec![];

        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Station name, identification and optionally revision year:
        // 1991:       station_name,rec_dev_id
        // 1999, 2013: station_name,rec_dev_id,rev_year
        match line_values.len() {
            3 => {
                self.builder.station_name(line_values[0].trim().to_string());
                self.builder.recording_device_id(line_values[1].trim().to_string());
                self.builder.revision(FormatRevision::from_str(line_values[2].trim())?);
            }
            2 => {
                self.builder.station_name(line_values[0].trim().to_string());
                self.builder.recording_device_id(line_values[1].trim().to_string());
                self.builder.revision(FormatRevision::Revision1991);
            }
            _ => return Err(ParseError::new(format!("unexpected number of values on line {}", line_number))),
        }

        line_number += 1;

        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Number and type of channels:
        // TT,##A,##D
        if line_values.len() != 3 {
            return Err(ParseError::new(format!("unexpected number of values on line {}", line_number)));
        }

        let num_total_channels = line_values[0]
            .trim()
            .to_string()
            .parse()
            .or(Err(ParseError::new(format!("invalid integer value for number of total channels: '{}'", line_values[0]))))?;
        self.builder.num_total_channels(num_total_channels);

        let mut num_analog_channels_token = line_values[1].to_string();
        // Last character contains "A" identifier.
        num_analog_channels_token.pop();
        let num_analog_channels = num_analog_channels_token
            .trim()
            .to_string()
            .parse()
            .or(Err(ParseError::new(format!("invalid integer value for number of analog channels: '{}'", num_analog_channels_token))))?;
        self.builder.num_analog_channels(num_analog_channels);
        self.num_analog_channels = num_analog_channels;

        let mut num_status_channels_token = line_values[2].to_string();
        // Last character contains "D" identifier.
        num_status_channels_token.pop();
        let num_status_channels = num_status_channels_token
            .trim()
            .to_string()
            .parse()
            .or(Err(ParseError::new(format!("invalid integer value for number of status channels: '{}'", num_status_channels_token))))?;
        self.builder.num_status_channels(num_status_channels);
        self.num_status_channels = num_status_channels;

        line_number += 1;

        let analog_channels: Vec<AnalogChannel> = Vec::with_capacity(self.num_analog_channels as usize);
        let status_channels: Vec<StatusChannel> = Vec::with_capacity(self.num_status_channels as usize);

        // Analog channel information:
        // An,ch_id,ph,ccbm,uu,a,b,skew,min,max,primary,secondary,PS
        for i in 0..self.num_analog_channels {
            line = lines.next().ok_or(earlyEndErr.clone())?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 13 {
                return Err(ParseError::new(format!("unexpected number of values on line {}", line_number)));
            }

            let analog_index = line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid integer value for analog channel {} index: {}",
                        i,
                        line_values[0]
                    ))
                ))?;

            let name = line_values[1].trim().to_string();
            let phase = line_values[2].trim().to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].trim().to_string(); // Non-critical.
            let units = line_values[4].trim().to_string();

            let multiplier = line_values[5]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} multiplier: {}",
                        i,
                        line_values[5]
                    ))
                ))?;

            let offset_adder = line_values[6]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} offset adder: {}",
                        i,
                        line_values[6]
                    ))
                ))?;

            let skew = line_values[7]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} skew: {}",
                        i,
                        line_values[7]
                    ))
                ))?;

            let min_value = line_values[8]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} minimum value: {}",
                        i,
                        line_values[8]
                    ))
                ))?;

            let max_value = line_values[9]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} maximum value: {}",
                        i,
                        line_values[9]
                    ))
                ))?;

            let primary_factor = line_values[10]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} primary factor: {}",
                        i,
                        line_values[10]
                    ))
                ))?;

            let secondary_factor = line_values[11]
                .trim()
                .to_string()
                .parse::<f64>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid real numeric value for analog channel {} secondary factor: {}",
                        i,
                        line_values[11]
                    ))
                ))?;

            let scaling_mode = AnalogScalingMode::from_str(line_values[12].trim())?;

            analog_channels[i as usize] = AnalogChannel {
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
            };

            line_number += 1;
        }
        self.builder.analog_channels(analog_channels);

        // Status (digital) channel information:
        // Dn,ch_id,ph,ccbm,y
        for i in 0..self.num_analog_channels {
            line = lines.next().ok_or(earlyEndErr.clone())?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 13 {
                return Err(ParseError::new(format!("unexpected number of values on line {}", line_number)));
            }

            let status_index = line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid integer value for status channel {} index: {}",
                        i,
                        line_values[0]
                    ))
                ))?;

            let name = line_values[1].trim().to_string();
            let phase = line_values[2].trim().to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].trim().to_string(); // Non-critical.

            let normal_status_value = line_values[4]
                .trim()
                .to_string()
                .parse::<u8>()
                .or(Err(ParseError::new(
                    format!(
                        "invalid integer value for status channel {} normal value: {}",
                        i,
                        line_values[4]
                    ))
                ))?;
            if normal_status_value != 0 && normal_status_value != 1 {
                return Err(ParseError::new(format!("invalid normal status value for status channel {}: {}; expected one of : '0', '1'", i, line_values[4])));
            }

            status_channels[i] = StatusChannel {
                index: status_index,
                name,
                phase,
                circuit_component_being_monitored,
                normal_status_value,
            };

            line_number += 1;
        }
        self.builder.status_channels(status_channels);

        line = lines.next().ok_or(earlyEndErr.clone())?;

        // Line frequency
        // lf
        let line_frequency = line
            .trim()
            .to_string()
            .parse::<f64>()
            .or(Err(ParseError::new(
                format!(
                    "invalid real numeric value for line frequency: {}",
                    line,
                ))
            ))?;
        self.builder.line_frequency(line_frequency);

        line_number += 1;

        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Sampling rate information
        // nrates (x 1)
        // samp,endsamp (x nrates)
        // TODO

        line_number += 1;
        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Date/time stamps
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // TODO

        line_number += 1;
        line = lines.next().ok_or(earlyEndErr.clone())?;

        // Data file type
        // ft
        self.builder.data_format(DataFormat::from_str(line.to_lowercase())?);

        line_number += 1;
        line = lines.next().ok_or(earlyEndErr.clone())?;

        // Time stamp multiplication factor
        // timemult
        // TODO

        line_number += 1;
        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time information and relationship between local time and UTC
        // time_code, local_code
        // TODO

        line_number += 1;
        line = lines.next().ok_or(earlyEndErr.clone())?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time quality of samples
        // tmq_code,leapsec
        // TODO

        Ok(())
    }

    fn parse_dat(&mut self) -> ParseResult<()> {
        Ok(())
    }
}
