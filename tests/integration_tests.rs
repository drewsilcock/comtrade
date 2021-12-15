use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::{FixedOffset, NaiveDate, NaiveDateTime};

use comtrade::{
    AnalogChannel, AnalogScalingMode, Comtrade, ComtradeParserBuilder, DataFormat, FormatRevision,
    LeapSecondStatus, SamplingRate, StatusChannel, TimeQuality,
};

const SAMPLE_COMTRADE_DIR: &'static str = "./tests/comtrade_files";
const MINUTE: i32 = 60;
const HOUR: i32 = MINUTE * 60;

#[test]
fn it_correctly_parses_sample_2013_files_with_ascii_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_ascii.cfg");
    let dat_path = dir.join("sample_2013_ascii.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected = Comtrade {
        station_name: "SMARTSTATION".to_string(),
        recording_device_id: "IED123".to_string(),
        revision: FormatRevision::Revision2013,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: 1200.0,
            end_sample_number: 40,
        }],
        start_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 750_110),
        trigger_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 782_610),
        data_format: DataFormat::Ascii,
        timestamp_multiplication_factor: 1.0,
        time_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        local_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        time_quality: Some(TimeQuality::ClockUnlocked(1)),
        leap_second_status: Some(LeapSecondStatus::NoCapability),
        num_analog_channels: 4,
        num_status_channels: 4,
        num_total_channels: 8,

        sample_numbers: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ],

        timestamps: vec![
            72500, 73333, 74167, 75000, 75833, 76667, 77500, 78333, 79167, 80000, 80833, 81667,
            82500, 83333, 84167, 85000, 85833, 86667, 87500, 88333, 89167, 90000, 90833, 91667,
            92500, 93333, 94167, 95000, 95833, 96667, 97500, 98333, 99167, 100000, 100833, 101667,
            102500, 103333, 104167, 105000,
        ]
        .into_iter()
        .map(|t| Some(t))
        .collect(),

        analog_channels: vec![
            AnalogChannel {
                index: 1,
                name: "IA ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -83.0, -15.0, 55.0, 122.0, 182.0, 228.0, 260.0, 271.0, 260.0, 228.0, 178.0,
                    113.0, 43.0, -30.0, -95.0, -150.0, -187.0, -202.0, -195.0, -165.0, -118.0,
                    -57.0, 10.0, 78.0, 138.0, 187.0, 219.0, 230.0, 221.0, 191.0, 143.0, 83.0, 17.0,
                    -50.0, -111.0, -161.0, -195.0, -208.0, -199.0, -169.0,
                ],
            },
            AnalogChannel {
                index: 2,
                name: "IB ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    68.0, 5.0, -53.0, -96.0, -119.0, -121.0, -104.0, -68.0, -19.0, 39.0, 100.0,
                    158.0, 206.0, 236.0, 249.0, 243.0, 218.0, 176.0, 123.0, 61.0, -2.0, -61.0,
                    -110.0, -144.0, -159.0, -159.0, -139.0, -105.0, -56.0, 2.0, 61.0, 118.0, 165.0,
                    197.0, 212.0, 209.0, 187.0, 149.0, 99.0, 41.0,
                ],
            },
            AnalogChannel {
                index: 3,
                name: "IC ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    7.0, 4.0, 0.0, -2.0, -7.0, -11.0, -14.0, -17.0, -18.0, -19.0, -19.0, -16.0,
                    -12.0, -5.0, 2.0, 6.0, 11.0, 16.0, 18.0, 19.0, 17.0, 13.0, 9.0, 4.0, -2.0,
                    -7.0, -11.0, -14.0, -16.0, -17.0, -15.0, -13.0, -9.0, -4.0, 2.0, 6.0, 11.0,
                    15.0, 17.0, 18.0,
                ],
            },
            AnalogChannel {
                index: 4,
                name: "3I0".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -8.0, -6.0, 2.0, 24.0, 56.0, 95.0, 142.0, 186.0, 223.0, 248.0, 260.0, 255.0,
                    236.0, 202.0, 156.0, 98.0, 42.0, -10.0, -54.0, -85.0, -103.0, -106.0, -91.0,
                    -62.0, -23.0, 21.0, 69.0, 111.0, 149.0, 176.0, 189.0, 188.0, 172.0, 144.0,
                    103.0, 53.0, 4.0, -44.0, -83.0, -110.0,
                ],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "51A".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 2,
                name: "51B".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 3,
                name: "51C".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            StatusChannel {
                index: 4,
                name: "51N".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
        ],
    };

    assert_eq!(record, expected);
}

#[test]
fn it_correctly_parses_sample_2013_files_with_binary16_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_bin.cfg");
    let dat_path = dir.join("sample_2013_bin.dat");

    println!("CFG path: {:?}, DAT path: {:?}", cfg_path, dat_path);

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected = Comtrade {
        station_name: "station".to_string(),
        recording_device_id: "equipment".to_string(),
        revision: FormatRevision::Revision2013,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: 15360.0,
            end_sample_number: 5,
        }],
        start_time: NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_268),
        trigger_time: NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_333),
        data_format: DataFormat::Binary16,
        timestamp_multiplication_factor: 1.0,
        time_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        local_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        time_quality: Some(TimeQuality::ClockUnlocked(1)),
        leap_second_status: Some(LeapSecondStatus::NoCapability),
        num_total_channels: 20,
        num_analog_channels: 4,
        num_status_channels: 16,

        sample_numbers: vec![],
        timestamps: vec![],

        analog_channels: vec![
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "ST_1".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 2,
                name: "ST_2".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 3,
                name: "ST_3".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 4,
                name: "ST_4".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 5,
                name: "ST_5".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 6,
                name: "ST_6".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 7,
                name: "ST_7".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 8,
                name: "ST_8".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 9,
                name: "ST_9".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 10,
                name: "ST_10".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 11,
                name: "ST_11".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 12,
                name: "ST_12".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 13,
                name: "ST_13".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 14,
                name: "ST_14".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 15,
                name: "ST_15".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 16,
                name: "ST_16".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
        ],
    };

    assert_eq!(record, expected);
}

#[test]
fn it_correctly_parses_sample_2013_combined_file_with_ascii_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cff_path = dir.join("sample_2013_ascii.cff");

    let cff_file = BufReader::new(File::open(cff_path).expect("unable to find sample cff file"));

    let record = ComtradeParserBuilder::new()
        .cff_file(cff_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected = Comtrade {
        station_name: "SMARTSTATION".to_string(),
        recording_device_id: "IED123".to_string(),
        revision: FormatRevision::Revision2013,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: 1200.0,
            end_sample_number: 40,
        }],
        start_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 750_110),
        trigger_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 782_610),
        data_format: DataFormat::Ascii,
        timestamp_multiplication_factor: 1.0,
        time_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        local_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        time_quality: Some(TimeQuality::ClockUnlocked(1)),
        leap_second_status: Some(LeapSecondStatus::NoCapability),
        num_analog_channels: 4,
        num_status_channels: 4,
        num_total_channels: 8,

        sample_numbers: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ],

        timestamps: vec![
            72500, 73333, 74167, 75000, 75833, 76667, 77500, 78333, 79167, 80000, 80833, 81667,
            82500, 83333, 84167, 85000, 85833, 86667, 87500, 88333, 89167, 90000, 90833, 91667,
            92500, 93333, 94167, 95000, 95833, 96667, 97500, 98333, 99167, 100000, 100833, 101667,
            102500, 103333, 104167, 105000,
        ]
        .into_iter()
        .map(|t| Some(t))
        .collect(),

        analog_channels: vec![
            AnalogChannel {
                index: 1,
                name: "IA ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -83.0, -15.0, 55.0, 122.0, 182.0, 228.0, 260.0, 271.0, 260.0, 228.0, 178.0,
                    113.0, 43.0, -30.0, -95.0, -150.0, -187.0, -202.0, -195.0, -165.0, -118.0,
                    -57.0, 10.0, 78.0, 138.0, 187.0, 219.0, 230.0, 221.0, 191.0, 143.0, 83.0, 17.0,
                    -50.0, -111.0, -161.0, -195.0, -208.0, -199.0, -169.0,
                ],
            },
            AnalogChannel {
                index: 2,
                name: "IB ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    68.0, 5.0, -53.0, -96.0, -119.0, -121.0, -104.0, -68.0, -19.0, 39.0, 100.0,
                    158.0, 206.0, 236.0, 249.0, 243.0, 218.0, 176.0, 123.0, 61.0, -2.0, -61.0,
                    -110.0, -144.0, -159.0, -159.0, -139.0, -105.0, -56.0, 2.0, 61.0, 118.0, 165.0,
                    197.0, 212.0, 209.0, 187.0, 149.0, 99.0, 41.0,
                ],
            },
            AnalogChannel {
                index: 3,
                name: "IC ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    7.0, 4.0, 0.0, -2.0, -7.0, -11.0, -14.0, -17.0, -18.0, -19.0, -19.0, -16.0,
                    -12.0, -5.0, 2.0, 6.0, 11.0, 16.0, 18.0, 19.0, 17.0, 13.0, 9.0, 4.0, -2.0,
                    -7.0, -11.0, -14.0, -16.0, -17.0, -15.0, -13.0, -9.0, -4.0, 2.0, 6.0, 11.0,
                    15.0, 17.0, 18.0,
                ],
            },
            AnalogChannel {
                index: 4,
                name: "3I0".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -8.0, -6.0, 2.0, 24.0, 56.0, 95.0, 142.0, 186.0, 223.0, 248.0, 260.0, 255.0,
                    236.0, 202.0, 156.0, 98.0, 42.0, -10.0, -54.0, -85.0, -103.0, -106.0, -91.0,
                    -62.0, -23.0, 21.0, 69.0, 111.0, 149.0, 176.0, 189.0, 188.0, 172.0, 144.0,
                    103.0, 53.0, 4.0, -44.0, -83.0, -110.0,
                ],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "51A".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 2,
                name: "51B".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 3,
                name: "51C".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            StatusChannel {
                index: 4,
                name: "51N".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
        ],
    };

    assert_eq!(record, expected);
}

#[test]
fn it_correctly_parses_sample_2013_files_with_ascii_data_using_utf8() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_2013_ascii_utf8.cfg");
    let dat_path = dir.join("sample_2013_ascii.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected = Comtrade {
        station_name: "SMARTSTATION testing text encoding: hgvcj터파크387".to_string(),
        recording_device_id: "IED123".to_string(),
        revision: FormatRevision::Revision2013,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: 1200.0,
            end_sample_number: 40,
        }],
        start_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 750_110),
        trigger_time: NaiveDate::from_ymd(2011, 01, 12).and_hms_micro(5, 55, 30, 782_610),
        data_format: DataFormat::Ascii,
        timestamp_multiplication_factor: 1.0,
        time_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        local_offset: Some(FixedOffset::west(5 * HOUR + 30 * MINUTE)),
        time_quality: Some(TimeQuality::ClockUnlocked(1)),
        leap_second_status: Some(LeapSecondStatus::NoCapability),
        num_analog_channels: 4,
        num_status_channels: 4,
        num_total_channels: 8,

        sample_numbers: vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ],

        timestamps: vec![
            72500, 73333, 74167, 75000, 75833, 76667, 77500, 78333, 79167, 80000, 80833, 81667,
            82500, 83333, 84167, 85000, 85833, 86667, 87500, 88333, 89167, 90000, 90833, 91667,
            92500, 93333, 94167, 95000, 95833, 96667, 97500, 98333, 99167, 100000, 100833, 101667,
            102500, 103333, 104167, 105000,
        ]
        .into_iter()
        .map(|t| Some(t))
        .collect(),

        analog_channels: vec![
            AnalogChannel {
                index: 1,
                name: "IA ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -83.0, -15.0, 55.0, 122.0, 182.0, 228.0, 260.0, 271.0, 260.0, 228.0, 178.0,
                    113.0, 43.0, -30.0, -95.0, -150.0, -187.0, -202.0, -195.0, -165.0, -118.0,
                    -57.0, 10.0, 78.0, 138.0, 187.0, 219.0, 230.0, 221.0, 191.0, 143.0, 83.0, 17.0,
                    -50.0, -111.0, -161.0, -195.0, -208.0, -199.0, -169.0,
                ],
            },
            AnalogChannel {
                index: 2,
                name: "IB ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    68.0, 5.0, -53.0, -96.0, -119.0, -121.0, -104.0, -68.0, -19.0, 39.0, 100.0,
                    158.0, 206.0, 236.0, 249.0, 243.0, 218.0, 176.0, 123.0, 61.0, -2.0, -61.0,
                    -110.0, -144.0, -159.0, -159.0, -139.0, -105.0, -56.0, 2.0, 61.0, 118.0, 165.0,
                    197.0, 212.0, 209.0, 187.0, 149.0, 99.0, 41.0,
                ],
            },
            AnalogChannel {
                index: 3,
                name: "IC ".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    7.0, 4.0, 0.0, -2.0, -7.0, -11.0, -14.0, -17.0, -18.0, -19.0, -19.0, -16.0,
                    -12.0, -5.0, 2.0, 6.0, 11.0, 16.0, 18.0, 19.0, 17.0, 13.0, 9.0, 4.0, -2.0,
                    -7.0, -11.0, -14.0, -16.0, -17.0, -15.0, -13.0, -9.0, -4.0, 2.0, 6.0, 11.0,
                    15.0, 17.0, 18.0,
                ],
            },
            AnalogChannel {
                index: 4,
                name: "3I0".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "Line123".to_string(),
                units: " A".into(),
                multiplier: 0.1138916015625,
                offset_adder: 0.05694580078125,
                skew: 0.0,
                min_value: -32768.0,
                max_value: 32767.0,
                primary_factor: 933.0,
                secondary_factor: 1.0,
                scaling_mode: AnalogScalingMode::Secondary,
                data: vec![
                    -8.0, -6.0, 2.0, 24.0, 56.0, 95.0, 142.0, 186.0, 223.0, 248.0, 260.0, 255.0,
                    236.0, 202.0, 156.0, 98.0, 42.0, -10.0, -54.0, -85.0, -103.0, -106.0, -91.0,
                    -62.0, -23.0, 21.0, 69.0, 111.0, 149.0, 176.0, 189.0, 188.0, 172.0, 144.0,
                    103.0, 53.0, 4.0, -44.0, -83.0, -110.0,
                ],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "51A".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 2,
                name: "51B".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
            StatusChannel {
                index: 3,
                name: "51C".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            },
            StatusChannel {
                index: 4,
                name: "51N".into(),
                phase: "".into(),
                circuit_component_being_monitored: "Line123".into(),
                normal_status_value: 0,
                data: vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                ],
            },
        ],
    };

    assert_eq!(record, expected);
}

#[test]
fn it_correctly_parses_sample_1999_files_with_binary16_data() {
    let dir = Path::new(SAMPLE_COMTRADE_DIR);
    let cfg_path = dir.join("sample_1999_bin.cfg");
    let dat_path = dir.join("sample_1999_bin.dat");

    let cfg_file = BufReader::new(File::open(cfg_path).expect("unable to find sample cfg file"));
    let dat_file = BufReader::new(File::open(dat_path).expect("unable to find sample dat file"));

    let record = ComtradeParserBuilder::new()
        .cfg_file(cfg_file)
        .dat_file(dat_file)
        .build()
        .parse()
        .expect("unable to parse COMTRADE files");

    let expected = Comtrade {
        station_name: "station".to_string(),
        recording_device_id: "equipment".to_string(),
        revision: FormatRevision::Revision1999,
        line_frequency: 60.0,
        sampling_rates: vec![SamplingRate {
            rate_hz: 15360.0,
            end_sample_number: 5,
        }],
        start_time: NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_268),
        trigger_time: NaiveDate::from_ymd(2017, 01, 07).and_hms_micro(15, 35, 41, 958_333),
        data_format: DataFormat::Binary16,
        timestamp_multiplication_factor: 1.0,
        time_offset: None,
        local_offset: None,
        time_quality: None,
        leap_second_status: None,
        num_total_channels: 20,
        num_analog_channels: 4,
        num_status_channels: 16,

        sample_numbers: vec![],
        timestamps: vec![],

        analog_channels: vec![
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
            AnalogChannel {
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
                data: vec![],
            },
        ],

        status_channels: vec![
            StatusChannel {
                index: 1,
                name: "ST_1".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 2,
                name: "ST_2".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 3,
                name: "ST_3".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 4,
                name: "ST_4".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 5,
                name: "ST_5".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 6,
                name: "ST_6".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 7,
                name: "ST_7".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 8,
                name: "ST_8".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 9,
                name: "ST_9".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 10,
                name: "ST_10".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 11,
                name: "ST_11".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 12,
                name: "ST_12".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 13,
                name: "ST_13".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 14,
                name: "ST_14".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 15,
                name: "ST_15".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
            StatusChannel {
                index: 16,
                name: "ST_16".to_string(),
                phase: "".to_string(),
                circuit_component_being_monitored: "".to_string(),
                normal_status_value: 0,
                data: vec![],
            },
        ],
    };

    assert_eq!(record, expected);
}
