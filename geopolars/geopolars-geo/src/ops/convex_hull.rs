use crate::error::Result;
use crate::util::iter_geom;
use geo::algorithm::convex_hull::ConvexHull;
use geo::{Geometry, Polygon};
use geopolars_arrow::linestring::LineStringSeries;
use geopolars_arrow::polygon::{MutablePolygonArray, PolygonSeries};
use geopolars_arrow::util::{get_geoarrow_type, GeoArrowType};
use geozero::{CoordDimensions, ToWkb};
use polars::error::ErrString;
use polars::export::arrow::array::{Array, BinaryArray, MutableBinaryArray};
use polars::prelude::{ListChunked, PolarsError, Series};
use polars::series::IntoSeries;

pub(crate) fn convex_hull(series: &Series) -> Result<Series> {
    match get_geoarrow_type(series) {
        GeoArrowType::WKB => convex_hull_wkb(series),
        GeoArrowType::LineString => convex_hull_geoarrow_linestring(series),
        GeoArrowType::Polygon => convex_hull_geoarrow_polygon(series),
        _ => panic!("convex hull not supported for this geometry type"),
    }
}

fn convex_hull_wkb(series: &Series) -> Result<Series> {
    let mut output_array = MutableBinaryArray::<i32>::with_capacity(series.len());

    for geom in iter_geom(series) {
        let hull = match geom {
            Geometry::Polygon(polygon) => Ok(polygon.convex_hull()),
            Geometry::MultiPolygon(multi_poly) => Ok(multi_poly.convex_hull()),
            Geometry::MultiPoint(points) => Ok(points.convex_hull()),
            Geometry::LineString(line_string) => Ok(line_string.convex_hull()),
            Geometry::MultiLineString(multi_line_string) => Ok(multi_line_string.convex_hull()),
            _ => Err(PolarsError::ComputeError(ErrString::from(
                "ConvexHull not supported for this geometry type",
            ))),
        }?;
        let hull: Geometry<f64> = hull.into();
        let hull_wkb = hull.to_wkb(CoordDimensions::xy()).unwrap();

        output_array.push(Some(hull_wkb));
    }

    let result: BinaryArray<i32> = output_array.into();
    let series = Series::try_from(("geometry", Box::new(result) as Box<dyn Array>))?;
    Ok(series)
}

fn convex_hull_geoarrow_linestring(series: &Series) -> Result<Series> {
    let mut output_chunks: Vec<Box<dyn Array>> = vec![];
    for chunk in LineStringSeries(series).chunks() {
        let out: Vec<Option<Polygon>> = chunk
            .parts()
            .iter_geo()
            .map(|maybe_geo| maybe_geo.map(|g| g.convex_hull()))
            .collect();
        let mut_arr: MutablePolygonArray = out.into();
        output_chunks.push(Box::new(mut_arr.into_arrow()) as Box<dyn Array>);
    }

    Ok(ListChunked::from_chunks("result", output_chunks).into_series())
}

fn convex_hull_geoarrow_polygon(series: &Series) -> Result<Series> {
    let mut output_chunks: Vec<Box<dyn Array>> = vec![];
    for chunk in PolygonSeries(series).chunks() {
        let out: Vec<Option<Polygon>> = chunk
            .parts()
            .iter_geo()
            .map(|maybe_geo| maybe_geo.map(|g| g.convex_hull()))
            .collect();
        let mut_arr: MutablePolygonArray = out.into();
        output_chunks.push(Box::new(mut_arr.into_arrow()) as Box<dyn Array>);
    }

    Ok(ListChunked::from_chunks("result", output_chunks).into_series())
}

#[cfg(test)]
mod tests {
    use crate::geoseries::GeoSeries;
    use crate::util::iter_geom;
    use geo::{line_string, polygon, Geometry, MultiPoint, Point};
    use geopolars_arrow::linestring::MutableLineStringArray;
    use geopolars_arrow::polygon::PolygonSeries;
    use geozero::{CoordDimensions, ToWkb};
    use polars::export::arrow::array::{Array, ListArray};
    use polars::export::arrow::array::{BinaryArray, MutableBinaryArray};
    use polars::prelude::Series;

    #[test]
    fn convex_hull_for_multipoint() {
        let mut test_data = MutableBinaryArray::<i32>::with_capacity(1);

        // Values borrowed from this test in geo crate: https://docs.rs/geo/0.14.2/src/geo/algorithm/convexhull.rs.html#323
        let v = vec![
            Point::new(0.0, 10.0),
            Point::new(1.0, 1.0),
            Point::new(10.0, 0.0),
            Point::new(1.0, -1.0),
            Point::new(0.0, -10.0),
            Point::new(-1.0, -1.0),
            Point::new(-10.0, 0.0),
            Point::new(-1.0, 1.0),
            Point::new(0.0, 10.0),
        ];
        let mp = MultiPoint(v);

        let correct_poly: Geometry<f64> = polygon![
            (x:0.0, y: -10.0),
            (x:10.0, y: 0.0),
            (x:0.0, y:10.0),
            (x:-10.0, y:0.0),
            (x:0.0, y:-10.0),
        ]
        .into();

        let test_geom: Geometry<f64> = mp.into();
        let test_wkb = test_geom.to_wkb(CoordDimensions::xy()).unwrap();
        test_data.push(Some(test_wkb));

        let test_array: BinaryArray<i32> = test_data.into();

        let series =
            Series::try_from(("geometry", Box::new(test_array) as Box<dyn Array>)).unwrap();
        let convex_res = series.convex_hull();

        assert!(
            convex_res.is_ok(),
            "Should get a valid result back from convex hull"
        );
        let convex_res = convex_res.unwrap();

        assert_eq!(
            convex_res.len(),
            1,
            "Should get a single result back from the series"
        );
        let mut geom_iter = iter_geom(&convex_res);
        let result = geom_iter.next().unwrap();

        assert_eq!(result, correct_poly, "Should get the correct convex hull");
    }

    #[test]
    fn convex_hull_linestring_test() {
        let line_strings = vec![line_string![
            (x: 0.0, y: 10.0),
            (x: 1.0, y: 1.0),
            (x: 10.0, y: 0.0),
            (x: 1.0, y: -1.0),
            (x: 0.0, y: -10.0),
            (x: -1.0, y: -1.0),
            (x: -10.0, y: 0.0),
            (x: -1.0, y: 1.0),
            (x: 0.0, y: 10.0),
        ]];
        let expected = polygon![
            (x: 0.0, y: -10.0),
            (x: 10.0, y: 0.0),
            (x: 0.0, y: 10.0),
            (x: -10.0, y: 0.0),
            (x: 0.0, y: -10.0),
        ];

        let mut_line_string_arr: MutableLineStringArray = line_strings.into();
        let line_string_arr: ListArray<i64> = mut_line_string_arr.into();
        let series =
            Series::try_from(("geometry", Box::new(line_string_arr) as Box<dyn Array>)).unwrap();

        let actual = series.convex_hull().unwrap();
        let actual_geo = PolygonSeries(&actual).get_as_geo(0).unwrap();
        assert_eq!(actual_geo, expected);
    }
}
