use polars::error::{PolarsError, PolarsResult};
use polars::prelude::*;
use polars::series::IntoSeries;
use polars::{
    prelude::{
        PlFixedStateQuality,
        extension::{ExtensionTypeFactory, ExtensionTypeImpl},
    },
    series::Series,
};
use polars_core::datatypes::extension::register_extension_type;
use std::any::Any;
use std::borrow::Cow;
use std::hash::{BuildHasher, Hash};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

const UUID_NAME: &str = "arrow.uuid";
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UUIDType {
    metadata: Option<String>,
}

impl UUIDType {
    pub fn instance(metadata: Option<String>) -> ExtensionTypeInstance {
        ExtensionTypeInstance(Box::new(UUIDType { metadata }))
    }
    pub fn register() -> PolarsResult<()> {
        register_extension_type(UUID_NAME, Some(Arc::new(UUIDTypeFactory)))
    }
}
impl ExtensionTypeImpl for UUIDType {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(UUID_NAME)
    }

    fn serialize_metadata(&self) -> Option<Cow<'_, str>> {
        self.metadata.as_deref().map(Cow::Borrowed)
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
        Cow::Borrowed(UUID_NAME)
    }

    fn dyn_debug(&self) -> Cow<'_, str> {
        Cow::Borrowed(UUID_NAME)
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

pub struct UUIDTypeFactory;

impl ExtensionTypeFactory for UUIDTypeFactory {
    fn create_type_instance(
        &self,
        name: &str,
        storage: &DataType,
        metadata: Option<&str>,
    ) -> Box<dyn ExtensionTypeImpl> {
        match (name, storage) {
            (UUID_NAME, &DataType::Binary) => {
                let metadata = metadata.map(|s| s.to_string());
                Box::new(UUIDType { metadata })
            }
            (_, _) => panic!("not supported"),
        }
    }
}

#[derive(Clone, Debug)]
struct UuidChunked(ExtensionChunked);

impl UuidChunked {
    pub fn into_series(self) -> Series {
        self.0.into_series()
    }
}

#[derive(Clone)]
struct UuidChunkedBuilder {
    bin_builder: BinaryChunkedBuilder,
    metadata: Option<String>,
}

impl UuidChunkedBuilder {
    pub fn new(name: PlSmallStr, capacity: usize, metadata: Option<String>) -> UuidChunkedBuilder {
        let bin_builder = BinaryChunkedBuilder::new(name, capacity);
        UuidChunkedBuilder {
            bin_builder,
            metadata,
        }
    }

    pub fn append_value<S>(&mut self, v: S)
    where
        StringStrUuidBytes: TryFrom<S>,
    {
        let val_into_res = StringStrUuidBytes::try_from(v);
        let Ok(val_into) = val_into_res else {
            panic!("can't cast value into Uuid");
        };
        let val = val_into.0.as_slice();

        self.bin_builder.append_value(val);
    }
    pub fn append_option<S>(&mut self, v: Option<S>)
    where
        StringStrUuidBytes: TryFrom<S>,
    {
        match v {
            Some(v) => self.append_value(v),
            None => self.bin_builder.append_null(),
        }
    }

    pub fn finish(self) -> UuidChunked {
        let bs = self.bin_builder.finish().into_series();
        let ext = ExtensionChunked::from_storage(UUIDType::instance(self.metadata), bs);
        UuidChunked(ext)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct StringStrUuidBytes([u8; 16]);

impl From<Uuid> for StringStrUuidBytes {
    fn from(value: Uuid) -> Self {
        StringStrUuidBytes(*value.as_bytes())
    }
}

impl TryFrom<String> for StringStrUuidBytes {
    type Error = PolarsError;

    fn try_from(value: String) -> PolarsResult<Self> {
        let uuid = Uuid::from_str(value.as_str())
            .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
        Ok(StringStrUuidBytes(*uuid.as_bytes()))
    }
}

impl TryFrom<&str> for StringStrUuidBytes {
    type Error = PolarsError;

    fn try_from(value: &str) -> PolarsResult<Self> {
        let uuid =
            Uuid::from_str(value).map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
        Ok(StringStrUuidBytes(*uuid.as_bytes()))
    }
}

pub trait UuidSeries {
    fn make_uuids(name: PlSmallStr, len: usize, metadata: Option<String>) -> Series;
    fn to_uuid(self, metadata: Option<String>) -> PolarsResult<Series>;
    fn uuid_to_str(self) -> PolarsResult<Series>;
}
impl UuidSeries for Series {
    fn make_uuids(name: PlSmallStr, len: usize, metadata: Option<String>) -> Series {
        let mut uuid_chunked_builder = UuidChunkedBuilder::new(name, len, metadata);
        for _ in 0..len {
            uuid_chunked_builder.append_value(Uuid::new_v4());
        }
        uuid_chunked_builder.finish().into_series()
    }
    fn to_uuid(self, metadata: Option<String>) -> PolarsResult<Series> {
        match self.dtype() {
            DataType::Binary => {
                let exchunked = ExtensionChunked::from_storage(UUIDType::instance(metadata), self);
                Ok(exchunked.into_series())
            }
            DataType::String => {
                let mut uuid_chunked =
                    UuidChunkedBuilder::new(self.name().clone(), self.len(), metadata);
                let ca = self.str()?;
                ca.for_each(|s| {
                    uuid_chunked.append_option(s);
                });

                Ok(uuid_chunked.finish().into_series())
            }
            DataType::Extension(a, b)
                if *a == UUIDType::instance(metadata) && *b == Box::new(DataType::Binary) =>
            {
                Ok(self)
            }
            _ => Err(PolarsError::SchemaMismatch(
                "only from String and Binary".into(),
            )),
        }
    }
    fn uuid_to_str(self) -> PolarsResult<Series> {
        match self.dtype() {
            DataType::Extension(a, b)
                if a.0.name() == UUID_NAME && *b == Box::new(DataType::Binary) =>
            {
                let ca = self.ext()?;
                let ca = ca.clone();
                let s = ca.into_storage();
                let ca = s.binary()?;
                let mut strbuild = StringChunkedBuilder::new(self.name().clone(), self.len());
                for s in ca {
                    match s {
                        Some(s) => {
                            let res = Uuid::from_slice(s);
                            match res {
                                Ok(uuid) => strbuild.append_value(uuid.to_string()),
                                Err(_) => strbuild.append_null(),
                            }
                        }
                        None => strbuild.append_null(),
                    }
                }
                Ok(strbuild.finish().into_series())
            }
            _ => Err(PolarsError::SchemaMismatch("only from uuid".into())),
        }
    }
}

pub fn make_uuids() -> Expr {
    make_uuids_with_option(None)
}

pub fn make_uuids_with_meta(metadata: String) -> Expr {
    make_uuids_with_option(Some(metadata))
}

pub fn make_uuids_with_option(metadata: Option<String>) -> Expr {
    let meta1 = metadata.clone();
    let field = move |_arg0: &Schema, _arg1: &polars::prelude::Field| {
        Ok(Field::new(
            PlSmallStr::EMPTY,
            DataType::Extension(
                UUIDType::instance(meta1.clone()),
                Box::new(DataType::Binary),
            ),
        ))
    };

    len()
        .map(
            move |s| {
                let ca = s.u32()?;
                let len = ca.get(0);
                let Some(len) = len else {
                    return Err(PolarsError::NoData("no data".into()));
                };
                let s = Series::make_uuids(PlSmallStr::EMPTY, len as usize, metadata.clone());
                Ok(s.into())
            },
            field,
        )
        .alias("make_uuids")
}
pub trait UUIDExprs {
    fn uuid_to_str(self) -> Expr;
    fn str_to_uuid(self, metadata: Option<String>) -> Expr;
}
impl UUIDExprs for Expr {
    fn uuid_to_str(self) -> Expr {
        self.map(
            |c| {
                let s = c.as_series();
                let Some(s) = s else {
                    return Err(PolarsError::NoData("no data".into()));
                };
                Ok(s.clone().uuid_to_str()?.into())
            },
            |_, _| Ok(Field::new(PlSmallStr::EMPTY, DataType::String)),
        )
    }
    fn str_to_uuid(self, metadata: Option<String>) -> Expr {
        self.map(
            move |c: Column| -> PolarsResult<Column> {
                let s = c.as_series();
                let Some(s) = s else {
                    return Err(PolarsError::NoData("no data".into()));
                };
                Ok(s.clone().to_uuid(metadata.clone())?.into())
            },
            |_, _| Ok(Field::new(PlSmallStr::EMPTY, DataType::String)),
        )
    }
}
