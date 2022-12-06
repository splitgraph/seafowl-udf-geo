extern crate rmp_serde as rmps;
extern crate serde;
use anyhow::{anyhow, Error};
use rmpv::Value;
use udf::{decode_f64};
use geoutils::Location;

mod udf;

fn _distance(lat1: f64, lon1:f64, lat2: f64, lon2: f64) -> Result<f64, Error> {
    let point1 = Location::new(lat1, lon1);
    let point2 = Location::new(lat2, lon2);
    point1.distance_to(&point2)
        .map(|d| d.meters())
        .map_err(|e| anyhow!(format!("Error calculating distance: {:?}", e)))
}

#[no_mangle]
pub unsafe fn distance(input_ptr: *mut u8) -> *mut u8 {
    udf::wrap_udf(input_ptr, |args| {
        if args.len() != 4 {
            return Err(anyhow!("distance() expects 4 arguments"))
        }
        _distance(
            decode_f64(&args[0])?,
            decode_f64(&args[1])?,
            decode_f64(&args[2])?,
            decode_f64(&args[3])?
        ).map(Value::from)
    })
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            // Berlin-Moscow distance in meters
            _distance(52.518611, 13.408056, 55.751667, 37.617778).unwrap(),
            1613872.907)
    }
}
