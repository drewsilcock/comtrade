# comtrade - Pure Rust library for parsing COMTRADE files.

![crates.io version](https://img.shields.io/crates/v/comtrade)
![licence](https://img.shields.io/crates/l/comtrade)
![docs status](https://img.shields.io/docsrs/comtrade)
![crates.io downloads](https://img.shields.io/crates/d/comtrade)

*Warning: this library is a WIP and not yet ready for production usage.*

This library provides a native Rust library for parsing [COMTRADE](https://en.wikipedia.org/wiki/Comtrade) (**Com**mon format for **Tra**nsient **D**ata **E**xchange for power systems) files, containing oscillography and status data for power system disturbances.

## Implementation status

| Task | Status |
| ---- | ------ |
| Implement parsing .cfg files (encoded in ASCII) | Done (1991, 1999, 2013) |
| Implement parsing .cfg files (encoded in UTF-8) | Done |
| Implement parsing .cfg files (encoded in other encodings such as latin1) | Not tested |
| Implement parsing ASCII data files | Done |
| Implement parsing binary16 data files | Done |
| Implement parsing binary32 data files | Done (not tested) |
| Implement parsing float32 data files | Done (not tested) |
| Implement loading separate files from combined 2013 `.cff` format. | Done |
| Implement retrieval of actual analog data values using primary vs. secondary factors, offsets, etc. | Adders & multipliers done; primary vs. secondary todo |
| Implement calculation of real time based on time multipliers, etc. (critical & non-critical timestamps) | Done |
| Support for channel-specific timestamp skews | Todo |

## Getting started

Todo: once ready, deploy to crates.io.

## Usage

Todo: document usage.

## Todo

- Clean up error messages - maybe have consistent format including filename and line number?
- Add warnings for non-critical data missing and errors for critical data missing.
- Add warnings for unexpected values (but not errors) for things like channel numbers not adding up, etc.
- Test files:
  - Binary32 and float32 data files.
  - Missing non-critical data.
  - Continuously variable sample rate (i.e. crticial timestamp).
  - Multiple sample rates.
