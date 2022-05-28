use arctic::geodataframe::GeoDataFrame;
use arctic::geoseries::GeoSeries;
use polars::prelude::{IpcReader, Result, SerReader};
use std::fs::File;
use std::time::Instant;

fn main() -> Result<()> {
    let file = File::open("cities.arrow").expect("file not found");

    // let metadata = read_file_metadata(&mut reader)?;
    // let mut filereader = FileReader::new(reader, metadata.clone(), None);
    // for chunk in filereader {
    //     let chunk = chunk?;
    //     for col in chunk.columns() {
    //         col.
    //     }
    //     println!("{:#?}", chunk);
    //     let df = DataFrame::try_from((chunk, metadata.schema.fields.as_ref())).unwrap();
    //     println!("{}", df);
    //     println!("{:#?}", df.schema());
    // }

    // println!("{:#?}", metadata);

    let df = IpcReader::new(file).finish()?;
    println!("{}", df);

    let start = Instant::now();
    df.centroid()?;
    df.column("geometry")?.centroid()?;
    println!("Debug: {}", start.elapsed().as_secs_f32());

    // let df = DataFrame::default();
    // df.hello_world();

    // println!("hello world from main!");

    Ok(())
}
