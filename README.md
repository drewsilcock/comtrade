# comtrade-rs

*Warning: this library is a WIP and not yet ready for production usage.*

This library provides a native Rust library for parsing [COMTRADE](https://en.wikipedia.org/wiki/Comtrade) (**Com**mon format for **Tra**nsient **D**ata **E**xchange for power systems) files, containing oscillography and status data for power system disturbances.

## Implementation status

| Task | Status |
| ---- | ------ |
| Implement parsing .cfg files (encoded in ASCII) | Working (1991, 1999, 2013) |
| Implement parsing .cfg files (encoded in UTF-8) | May work? Need to test. |
| Implement parsing .cfg files (encoded in other encodings such as latin1) | Todo |
| Implement parsing ACSII data files | Working |
| Implement parsing binary16 data files | Todo |
| Implement parsing binary32 data files | Todo |
| Implement parsing float32 data files | Todo |
| Implement loading separate files from combined 2013 `.cff` format. | Working (1991, 1999, 2013) |
| Implement retrieval of actual analog data values using primary vs. secondary factors, offsets, etc. | Todo |
| Implement calculation of real time based on skews, time multipliers, etc. | Todo |

## Getting started

Todo: once ready, deploy to crates.io.

## Usage

Todo: document usage.

## Todo

- Current test files all have only one sample rate - can I find a COMTRADE file with > 1 sample rate for purpose of testing comprehensiveness?
- Clean up error messages - maybe have consistent format including filename and line number?
- No test files with continuously variable sample rate.
- Go through all values in CFG and check whether they're critical or non-critical - missing non-critical data should trigger warning but not failure.
- Add test files with missing non-critical data.
- Add ability to write back out again as COMTRADE files with various options.
- Add warnings for unexpected values (but not errors) for things like channel numbers not adding up, etc.