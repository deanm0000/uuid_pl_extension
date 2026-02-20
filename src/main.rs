use polars::error::{PolarsError, PolarsResult, polars_bail};
use polars::prelude::{
    AnyValue, Column, DataType, ExtensionChunked, ExtensionTypeInstance, IntoColumn,
};
use polars::series::IntoSeries;
use polars::{
    prelude::{
        DataFrame, NamedFrom, PlFixedStateQuality,
        extension::{ExtensionTypeFactory, ExtensionTypeImpl},
    },
    series::Series,
};
use polars_core::datatypes::extension::register_extension_type;
use std::any::Any;
use std::borrow::Cow;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;
use uuid::Uuid;

const UUID_NAME: &str = "uuid";
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UUIDType;

impl ExtensionTypeImpl for UUIDType {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("uuid")
    }

    fn serialize_metadata(&self) -> Option<Cow<'_, str>> {
        None
    }

    fn dyn_clone(&self) -> Box<dyn ExtensionTypeImpl> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &dyn ExtensionTypeImpl) -> bool {
        if let Some(other) = (other as &dyn Any).downcast_ref::<Self>() {
            return self == other;
        }
        false
    }

    fn dyn_hash(&self) -> u64 {
        PlFixedStateQuality::default().hash_one(self)
    }

    fn dyn_display(&self) -> Cow<'_, str> {
        Cow::Borrowed("uuid")
    }

    fn dyn_debug(&self) -> Cow<'_, str> {
        Cow::Borrowed("uuid")
    }
    fn dyn_display_value<'a>(&self, column: &'a Column, index: usize) -> Cow<'a, str> {
        if let AnyValue::Binary(av) = column.get(index).unwrap() {
            let uuid = Uuid::from_slice(av).unwrap();
            Cow::Owned(uuid.to_string())
        } else {
            unreachable!()
        }
    }
}

// trait ExtenstionTypeImplPrint {
//     fn dyn_display_value<'a>(
//         &self,
//         column: &'a Column,
//         index: usize,
//     ) -> PolarsResult<Cow<'a, str>> {
//         Ok(column.get(index)?.str_value())
//     }
// }

pub struct UUIDTypeFactory;

impl ExtensionTypeFactory for UUIDTypeFactory {
    fn create_type_instance(
        &self,
        name: &str,
        storage: &DataType,
        _metadata: Option<&str>,
    ) -> Box<dyn ExtensionTypeImpl> {
        match (name, storage) {
            (UUID_NAME, &DataType::Binary) => Box::new(UUIDType),
            (_, _) => panic!("not supported"),
        }
    }
}

fn main() -> PolarsResult<()> {
    register_extension_type("uuid", Some(Arc::new(UUIDTypeFactory)))?;

    let uuids: Vec<Uuid> = (0..50).map(|_| Uuid::new_v4()).collect();
    let slices: Vec<&[u8]> = uuids.iter().map(|x| x.as_bytes().as_slice()).collect();
    let s = Series::new("uuids".into(), slices);

    let ext_chunked = ExtensionChunked::from_storage(ExtensionTypeInstance(Box::new(UUIDType)), s);
    println!("{:?}", ext_chunked);
    let s_ext = ext_chunked.into_series();

    println!("{s_ext}");
    let c_ext = s_ext.into_column();

    let df = DataFrame::new(c_ext.len(), vec![c_ext])?;
    println!("{df}");
    println!("{}", df.slice(0, 10));
    Ok(())
}
