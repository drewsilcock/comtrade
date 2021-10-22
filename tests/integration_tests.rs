use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::{FixedOffset, NaiveDate, NaiveDateTime};

use comtrade_rs;
use comtrade_rs::{AnalogChannel, AnalogScalingMode, DataFormat, LeapSecondStatus, StatusChannel, TimeQuality};

const SAMPLE_COMTRADE_DIR: &'static str = "./tests/comtrade_files";

#[test]
fn it_correctly_parses_sample_files_with_ascii_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_ascii.cfg");
    let dat_path = dir.join("sample_2013_ascii.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = comtrade_rs::ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    assert_eq!(record.revision, comtrade_rs::FormatRevision::Revision1999);
}

#[test]
fn it_correctly_parses_sample_files_with_binary_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_bin.cfg");
    let dat_path = dir.join("sample_2013_bin.dat");

    println!("CFG path: {:?}, DAT path: {:?}", cfg_path, dat_path);

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = comtrade_rs::ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    assert_eq!(record.station_name, "station");
    assert_eq!(record.recording_device_id, "equipment");
    assert_eq!(record.revision, comtrade_rs::FormatRevision::Revision1999);
    assert_eq!(record.num_total_channels, 20);
    assert_eq!(record.num_analog_channels, 4);
    assert_eq!(record.num_status_channels, 16);

    assert_eq!(record.analog_channels.len(), 4);
    assert_eq!(record.analog_channels[0], AnalogChannel {
        index: 1,
        name: "VA".to_string(),
        phase: "A".to_string(),
        circuit_component_being_monitored: "obj".to_string(),
        units: "kV".to_string(),
        min_value: -32767.0,
        max_value: 32767.0,
        multiplier: 0.000361849,
        offset_adder: 0.0,
        skew: 0.0,
        primary_factor: 120.0,
        secondary_factor: 1.0,
        scaling_mode: AnalogScalingMode::Primary,
    });
    assert_eq!(record.analog_channels[1], AnalogChannel {
        index: 2,
        name: "VB".to_string(),
        phase: "B".to_string(),
        circuit_component_being_monitored: "obj".to_string(),
        units: "kV".to_string(),
        min_value: -32767.0,
        max_value: 32767.0,
        multiplier: 0.000365758,
        offset_adder: 0.0,
        skew: 0.0,
        primary_factor: 120.0,
        secondary_factor: 1.0,
        scaling_mode: AnalogScalingMode::Primary,
    });
    assert_eq!(record.analog_channels[2], AnalogChannel {
        index: 3,
        name: "VC".to_string(),
        phase: "C".to_string(),
        circuit_component_being_monitored: "obj".to_string(),
        units: "kV".to_string(),
        min_value: -32767.0,
        max_value: 32767.0,
        multiplier: 0.000371569,
        offset_adder: 0.0,
        skew: 0.0,
        primary_factor: 120.0,
        secondary_factor: 1.0,
        scaling_mode: AnalogScalingMode::Primary,
    });
    assert_eq!(record.analog_channels[3], AnalogChannel {
        index: 4,
        name: "VN".to_string(),
        phase: "N".to_string(),
        circuit_component_being_monitored: "obj".to_string(),
        units: "kV".to_string(),
        min_value: -32767.0,
        max_value: 32767.0,
        multiplier: 0.000016493,
        offset_adder: 0.0,
        skew: 0.0,
        primary_factor: 60.0,
        secondary_factor: 1.0,
        scaling_mode: AnalogScalingMode::Primary,
    });

    assert_eq!(record.status_channels.len(), 16);
    assert_eq!(record.status_channels[0], StatusChannel {
        index: 1,
        name: "ST_1".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[1], StatusChannel {
        index: 2,
        name: "ST_2".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[2], StatusChannel {
        index: 3,
        name: "ST_3".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[3], StatusChannel {
        index: 4,
        name: "ST_4".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[4], StatusChannel {
        index: 5,
        name: "ST_5".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[5], StatusChannel {
        index: 6,
        name: "ST_6".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[6], StatusChannel {
        index: 7,
        name: "ST_7".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[7], StatusChannel {
        index: 8,
        name: "ST_8".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[8], StatusChannel {
        index: 9,
        name: "ST_9".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[9], StatusChannel {
        index: 10,
        name: "ST_10".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[10], StatusChannel {
        index: 11,
        name: "ST_11".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[11], StatusChannel {
        index: 12,
        name: "ST_12".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[12], StatusChannel {
        index: 13,
        name: "ST_13".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[13], StatusChannel {
        index: 14,
        name: "ST_14".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[14], StatusChannel {
        index: 15,
        name: "ST_15".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });
    assert_eq!(record.status_channels[15], StatusChannel {
        index: 16,
        name: "ST_16".to_string(),
        phase: "".to_string(),
        circuit_component_being_monitored: "".to_string(),
        normal_status_value: 0,
    });

    assert_eq!(record.line_frequency, 60.0);

    assert_eq!(record.sampling_rates.len(), 1);
    assert_eq!(record.sampling_rates[0].rate_hz, 15360.0);
    assert_eq!(record.sampling_rates[0].end_sample_number, 5);

    assert_eq!(
        record.start_time,
        NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_268),
    );
    assert_eq!(
        record.trigger_time,
        NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_333),
    );
    assert_eq!(record.data_format, DataFormat::Binary16);
    assert_eq!(record.timestamp_multiplication_factor, 1.0);
    assert_eq!(record.time_offset.unwrap(), FixedOffset::west(5 * 3600 + 30 * 60));
    assert_eq!(record.local_offset.unwrap(), FixedOffset::west(5 * 3600 + 30 * 60));
    assert_eq!(record.time_quality.unwrap(), TimeQuality::ClockUnlocked(1));
    assert_eq!(record.leap_second_status.unwrap(), LeapSecondStatus::NoCapability);
}

#[test]
fn it_correctly_parses_sample_combined_file_with_ascii_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cff_path = dir.join("sample_2013_ascii.cff");

    let cff_file = BufReader::new(File::open(cff_path).expect("unable to find sample cfg file"));

    let record = comtrade_rs::ComtradeParserBuilder::new()
        .cff_file(cff_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    assert_eq!(record.revision, comtrade_rs::FormatRevision::Revision1999);
}

#[test]
fn it_correctly_parses_sample_files_with_ascii_data_using_utf8() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_ascii_utf8.cfg");
    let dat_path = dir.join("sample_2013_ascii.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = comtrade_rs::ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    assert_eq!(record.revision, comtrade_rs::FormatRevision::Revision1999);
}
