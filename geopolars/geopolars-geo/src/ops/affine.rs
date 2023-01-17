// use crate::error::Result;
// use crate::util::from_geom_vec;
// use geo::algorithm::affine_ops::AffineTransform;
// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::centroid::Centroid;
// use geo::map_coords::MapCoords;
use geo::Point;
// use geo::{Geometry, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};
// use geopolars_arrow::GeometryArrayEnum;
// use polars::prelude::Series;

// use crate::util::iter_geom;

// use super::centroid::centroid;

/// Used to express the origin for a given transform. Can be specified either be with reference to
/// the geometry being transformed (Centroid, Center) or some arbitrary point.
///
/// - Centroid: Use the centriod of each geometry in the series as the transform origin.
/// - Center: Use the center of each geometry in the series as the transform origin. The center is
///   defined as the center of the bounding box of the geometry
/// - Point: Define a single point to transform each geometry in the series about.
pub enum TransformOrigin {
    Centroid,
    Center,
    Point(Point),
}

// pub(crate) fn affine_transform(
//     array: GeometryArrayEnum,
//     matrix: impl Into<AffineTransform<f64>>,
// ) -> Result<GeometryArrayEnum> {
//     let transform: AffineTransform<f64> = matrix.into();

//     match array {
//         GeometryArrayEnum::WKB(arr) => {
//             let output_geoms: Vec<Option<Geometry>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::WKB(output_geoms.into()))
//         }
//         GeometryArrayEnum::Point(arr) => {
//             let output_geoms: Vec<Option<Point>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::Point(output_geoms.into()))
//         }

//         GeometryArrayEnum::MultiPoint(arr) => {
//             let output_geoms: Vec<Option<MultiPoint>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::MultiPoint(output_geoms.into()))
//         }
//         GeometryArrayEnum::LineString(arr) => {
//             let output_geoms: Vec<Option<LineString>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::LineString(output_geoms.into()))
//         }
//         GeometryArrayEnum::MultiLineString(arr) => {
//             let output_geoms: Vec<Option<MultiLineString>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::MultiLineString(output_geoms.into()))
//         }
//         GeometryArrayEnum::Polygon(arr) => {
//             let output_geoms: Vec<Option<Polygon>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::Polygon(output_geoms.into()))
//         }
//         GeometryArrayEnum::MultiPolygon(arr) => {
//             let output_geoms: Vec<Option<MultiPolygon>> = arr
//                 .iter_geo()
//                 .map(|maybe_g| maybe_g.map(|geom| geom.map_coords(|coord| transform.apply(coord))))
//                 .collect();

//             Ok(GeometryArrayEnum::MultiPolygon(output_geoms.into()))
//         }
//     }
// }

// // pub(crate) fn rotate(array: GeometryArrayEnum, angle: f64, origin: TransformOrigin) -> Result<GeometryArrayEnum> {
// //     match origin {
// //         TransformOrigin::Centroid => {
// //             let centroid_arr = centroid(array)?;
// //             centroid_arr.iter_geo().map(|maybe_g| maybe_g.map( ))
// //         },
// //         _ => todo!()
// //     }

// //     rotate_wkb(series, angle, origin)
// // }

// pub(crate) fn scale(
//     series: &Series,
//     xfact: f64,
//     yfact: f64,
//     origin: TransformOrigin,
// ) -> Result<Series> {
//     scale_wkb(series, xfact, yfact, origin)
// }

// pub(crate) fn skew(series: &Series, xs: f64, ys: f64, origin: TransformOrigin) -> Result<Series> {
//     skew_wkb(series, xs, ys, origin)
// }

// pub(crate) fn translate(series: &Series, x: f64, y: f64) -> Result<Series> {
//     let transform = AffineTransform::translate(x, y);
//     affine_transform(series, transform)
// }

// fn rotate_wkb(series: &Series, angle: f64, origin: TransformOrigin) -> Result<Series> {
//     match origin {
//         TransformOrigin::Centroid => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let centroid = geom.centroid().unwrap();
//                     let transform = AffineTransform::rotate(angle, centroid);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Center => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let center = geom.bounding_rect().unwrap().center();
//                     let transform = AffineTransform::rotate(angle, center);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Point(point) => {
//             let transform = AffineTransform::rotate(angle, point);
//             affine_transform_wkb(series, transform)
//         }
//     }
// }

// fn scale_wkb(series: &Series, xfact: f64, yfact: f64, origin: TransformOrigin) -> Result<Series> {
//     match origin {
//         TransformOrigin::Centroid => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let centroid = geom.centroid().unwrap();
//                     let transform = AffineTransform::scale(xfact, yfact, centroid);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Center => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let center = geom.bounding_rect().unwrap().center();
//                     let transform = AffineTransform::scale(xfact, yfact, center);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Point(point) => {
//             let transform = AffineTransform::scale(xfact, yfact, point);
//             affine_transform_wkb(series, transform)
//         }
//     }
// }

// fn skew_wkb(series: &Series, xs: f64, ys: f64, origin: TransformOrigin) -> Result<Series> {
//     match origin {
//         TransformOrigin::Centroid => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let centroid = geom.centroid().unwrap();
//                     let transform = AffineTransform::skew(xs, ys, centroid);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Center => {
//             let rotated_geoms: Vec<Geometry<f64>> = iter_geom(series)
//                 .map(|geom| {
//                     let center = geom.bounding_rect().unwrap().center();
//                     let transform = AffineTransform::skew(xs, ys, center);
//                     geom.map_coords(|c| transform.apply(c))
//                 })
//                 .collect();
//             from_geom_vec(&rotated_geoms)
//         }
//         TransformOrigin::Point(point) => {
//             let transform = AffineTransform::skew(xs, ys, point);
//             affine_transform_wkb(series, transform)
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::TransformOrigin;
//     use crate::geoseries::GeoSeries;
//     use crate::util::from_geom_vec;
//     use crate::util::iter_geom;
//     use geo::{polygon, CoordsIter, Geometry, Point};

//     #[test]
//     fn rotate() {
//         let geo_series = from_geom_vec(&[Geometry::Polygon(polygon!(
//         (x: 0.0,y:0.0),
//         (x: 0.0,y:1.0),
//         (x: 1.0,y: 1.0),
//         (x: 1.0,y: 0.0)
//         ))])
//         .unwrap();

//         let result: Geometry<f64> = polygon!(
//         (x:0.0,y:0.0),
//         (x:-1.0,y:0.0),
//         (x:-1.0, y:1.0),
//         (x:0.0, y:1.0)
//         )
//         .into();

//         let rotated_series = geo_series.rotate(90.0, TransformOrigin::Point(Point::new(0.0, 0.0)));
//         assert!(rotated_series.is_ok(), "To get a series back");

//         let geom = iter_geom(&rotated_series.unwrap()).next().unwrap();
//         for (p1, p2) in geom.coords_iter().zip(result.coords_iter()) {
//             assert!(
//                 (p1.x - p2.x).abs() < 0.00000001,
//                 "The geometries x coords to be correct to within some tollerenace"
//             );
//             assert!(
//                 (p1.y - p2.y).abs() < 0.00000001,
//                 "The geometries y coords to be correct to within some tollerenace"
//             );
//         }
//     }

//     #[test]
//     fn scale() {
//         let geo_series = from_geom_vec(&[Geometry::Polygon(polygon!(
//         (x: 0.0,y:0.0),
//         (x: 0.0,y:1.0),
//         (x: 1.0,y: 1.0),
//         (x: 1.0,y: 0.0)
//         ))])
//         .unwrap();

//         let result_center: Geometry<f64> = polygon!(
//         (x:-0.5,y:-0.5),
//         (x:-0.5,y:1.5),
//         (x:1.5, y:1.5),
//         (x:1.5, y:-0.5)
//         )
//         .into();

//         let result_point: Geometry<f64> = polygon!(
//         (x:0.0,y:0.0),
//         (x:0.0,y:2.0),
//         (x:2.0, y:2.0),
//         (x:2.0, y:0.0)
//         )
//         .into();

//         let scaled_series = geo_series.scale(2.0, 2.0, TransformOrigin::Center);
//         assert!(scaled_series.is_ok(), "To get a series back");

//         let geom = iter_geom(&scaled_series.unwrap()).next().unwrap();
//         assert_eq!(
//             geom, result_center,
//             "The geom to be approprietly scaled about it's center"
//         );

//         let scaled_series =
//             geo_series.scale(2.0, 2.0, TransformOrigin::Point(Point::new(0.0, 0.0)));
//         let geom = iter_geom(&scaled_series.unwrap()).next().unwrap();
//         assert_eq!(
//             geom, result_point,
//             "The geom to be approprietly scaled about the point 0,0"
//         );
//     }

//     #[test]
//     fn skew() {
//         let geo_series = from_geom_vec(&[Geometry::Polygon(polygon!(
//         (x: 0.0,y:0.0),
//         (x: 0.0,y:1.0),
//         (x: 1.0,y: 1.0),
//         (x: 1.0,y: 0.0)
//         ))])
//         .unwrap();

//         let result: Geometry<f64> = polygon!(
//             (x:-0.008727532464108793,y:-0.017460384745873865),
//             (x:0.008727532464108793,y:0.9825396152541261),
//             (x:1.008727532464109, y:1.0174603847458739),
//             (x:0.9912724675358912, y:0.017460384745873865)
//         )
//         .into();

//         let skewed_series = geo_series.skew(1.0, 2.0, TransformOrigin::Center);
//         assert!(skewed_series.is_ok(), "To get a series back");

//         let geom = iter_geom(&skewed_series.unwrap()).next().unwrap();

//         assert_eq!(geom, result, "the polygon should be transformed correctly");

//         for (p1, p2) in geom.coords_iter().zip(result.coords_iter()) {
//             assert!(
//                 (p1.x - p2.x).abs() < 0.00000001,
//                 "The geometries x coords to be correct to within some tollerenace"
//             );
//             assert!(
//                 (p1.y - p2.y).abs() < 0.00000001,
//                 "The geometries y coords to be correct to within some tollerenace"
//             );
//         }
//     }

//     #[test]
//     fn translate() {
//         let geo_series = from_geom_vec(&[Geometry::Polygon(polygon!(
//         (x: 0.0,y:0.0),
//         (x: 0.0,y:1.0),
//         (x: 1.0,y: 1.0),
//         (x: 1.0,y: 0.0)
//         ))])
//         .unwrap();

//         let result: Geometry<f64> = polygon!(
//         (x:1.0,y:1.0),
//         (x:1.0,y:2.0),
//         (x:2.0, y:2.0),
//         (x:2.0, y:1.0)
//         )
//         .into();

//         let translated_series = geo_series.translate(1.0, 1.0);
//         assert!(translated_series.is_ok(), "To get a series back");

//         let geom = iter_geom(&translated_series.unwrap()).next().unwrap();
//         assert_eq!(geom, result, "The geom to be approprietly translated");
//     }
// }