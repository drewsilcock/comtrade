use comtrade_rs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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

    assert_eq!(record.revision, comtrade_rs::FormatRevision::Revision1999);
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
