use std::sync::Arc;

use crate::util::iter_geom;
use arrow2::array::{ArrayRef, BinaryArray, MutableBinaryArray};
use geo::Geometry;
use geozero::{
    CoordDimensions, ToWkb,
};
use polars::prelude::{Result, Series};

pub trait GeoSeries {
    fn centroid(&self) -> Result<Series>;
}

impl GeoSeries for Series {
    fn centroid(&self) -> Result<Series> {
        use geo::algorithm::centroid::Centroid;

        let mut output_array = MutableBinaryArray::<i32>::with_capacity(self.len());

        for geom in iter_geom(self) {
            let value: Geometry<f64> = geom.centroid().expect("could not create centroid").into();
            let wkb = value
                .to_wkb(CoordDimensions::xy())
                .expect("Unable to create wkb");

            output_array.push(Some(wkb));
        }

        let result: BinaryArray<i32> = output_array.into();

        let output_series = Series::try_from(("geometry", Arc::new(result) as ArrayRef))?;
        Ok(output_series)
    }
}
