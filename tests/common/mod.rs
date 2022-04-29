use float_cmp::approx_eq;

use comtrade::Comtrade;

pub const SAMPLE_COMTRADE_DIR: &str = "./tests/comtrade_files";
pub const MINUTE: i32 = 60;
pub const HOUR: i32 = MINUTE * 60;

pub fn assert_comtrades_eq(left: &Comtrade, right: &Comtrade) {
    // Floating point comparisons need a special approximately equal rather than the
    // normal one, so we do that below. To not have to manually write out the rest of
    // the normal comparisons, we just bypass the float comparisons by cloning the
    // floating point arrays and writing over the values in the struct, then performing
    // the floating point appro. equal comparison manually.
    let mut right_clone = right.clone();
    right_clone.timestamps = left.timestamps.clone();
    for (i, c) in left.analog_channels.iter().enumerate() {
        right_clone.analog_channels[i].data = c.data.clone();
    }

    assert_eq!(*left, right_clone);

    // Timestamps and analog data values are floats that involve some calculations to
    // get the actual values, so direct floating point comparisons don't work. The actual
    // type of the data is f64 but the underlying data being read in from file is either
    // i16 / i32 (converted to float via calculations) or f32.
    for (i, tl) in left.timestamps.iter().enumerate() {
        let tr = right.timestamps[i];
        assert!(
            approx_eq!(f64, *tl, tr),
            "timestamp {} different: {} !≈ {}",
            i,
            tl,
            tr,
        );
    }

    for (i, c) in left.analog_channels.iter().enumerate() {
        for (j, vl) in c.data.iter().enumerate() {
            let vr = right.analog_channels[i].data[j];
            assert!(
                approx_eq!(f32, *vl as f32, vr as f32),
                "analog channel {} value {} different: {} !≈ {}",
                i,
                j,
                vl,
                vr,
            );
        }
    }
}
