use polars::error::PolarsResult;
use polars::prelude::*;
mod traits_etc;
use crate::traits_etc::UuidSeries;
use traits_etc::*;
fn main() -> PolarsResult<()> {
    UUIDType::register()?;

    let s_ext = Series::make_uuids("uuids".into(), 50, Some("lala".to_string()));

    let ca = s_ext.ext()?;
    println!("printing ExtensionChunked");
    println!("{:?}", ca);
    println!("printing Series");
    println!("{s_ext}");
    let c_ext = s_ext.into_column();

    let df = DataFrame::new(c_ext.len(), vec![c_ext])?;
    println!("printing df");
    println!("{df}");
    println!("printing sliced df");
    println!("{}", df.slice(0, 10));

    let df2 = df
        .lazy()
        .with_column(make_uuids_with_meta("hi".to_string()).alias("new_uuids"))
        .with_column(col("new_uuids").uuid_to_str().alias("as_strs"))
        .with_column(
            col("as_strs")
                .str_to_uuid(Some("from strs".to_string()))
                .alias("uuid_roundtrip"),
        )
        .collect()?;
    println!("printing df after uuid exprs");
    println!("{}", df2);
    df2.columns().iter().for_each(|c| {
        if let DataType::Extension(a, _) = c.dtype() {
            let b = a.0.serialize_metadata();
            if let Some(meta) = b {
                println!("column {}'s metadata is {}", c.name(), meta);
            }
        };
    });
    Ok(())
}
