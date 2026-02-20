Right now this is mostly to test a PR in polars.
If/when that PR gets merged then I'll do more with uuid extension.

This just shows that the output with that PR implemented:


```rust
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
```

output
```
printing ExtensionChunked
shape: (50,)
ChunkedArray: 'uuids' [ext[arrow.uuid]]
[
        d3662fb9-3373-4643-ae41-4e4160354709
        00413f04-e557-41f9-812f-70eb18deb156
        ea463445-5bf3-4971-bdcb-67423da58c2e
        42ae3844-6100-4290-a19c-48322cdb60f5
        5af3b2de-299e-45a5-bda4-06ec6a591cdc
        6351dcbd-4480-4418-85eb-045d459343f3
        450181cc-145d-4f02-80ca-2bc7d25682c1
        33c46b88-0bf0-4c14-9f12-61ccbfdacc68
        3cb74359-2163-49b2-bb91-19be29048a09
        85acfa9a-5729-4895-a6e7-4fa6f389b206
]
printing Series
shape: (50,)
Series: 'uuids' [ext[arrow.uuid]]
[
        d3662fb9-3373-4643-ae41-4e4160354709
        00413f04-e557-41f9-812f-70eb18deb156
        ea463445-5bf3-4971-bdcb-67423da58c2e
        42ae3844-6100-4290-a19c-48322cdb60f5
        5af3b2de-299e-45a5-bda4-06ec6a591cdc
        6351dcbd-4480-4418-85eb-045d459343f3
        450181cc-145d-4f02-80ca-2bc7d25682c1
        33c46b88-0bf0-4c14-9f12-61ccbfdacc68
        3cb74359-2163-49b2-bb91-19be29048a09
        85acfa9a-5729-4895-a6e7-4fa6f389b206
]
printing df
shape: (50, 1)
┌─────────────────────────────────┐
│ uuids                           │
│ ---                             │
│ ext[arrow.uuid]                 │
╞═════════════════════════════════╡
│ d3662fb9-3373-4643-ae41-4e4160… │
│ 00413f04-e557-41f9-812f-70eb18… │
│ ea463445-5bf3-4971-bdcb-67423d… │
│ 42ae3844-6100-4290-a19c-48322c… │
│ 5af3b2de-299e-45a5-bda4-06ec6a… │
│ …                               │
│ d7390a85-cfac-4519-9473-de5ef9… │
│ ea3c9839-e98e-47d7-803b-861394… │
│ b65fed02-b309-4433-bcc6-f92c0c… │
│ 5cdc5318-af32-4e9a-b2b5-e804d9… │
│ b1997bb7-47ae-4075-8985-70b1c5… │
└─────────────────────────────────┘
printing sliced df
shape: (10, 1)
┌─────────────────────────────────┐
│ uuids                           │
│ ---                             │
│ ext[arrow.uuid]                 │
╞═════════════════════════════════╡
│ d3662fb9-3373-4643-ae41-4e4160… │
│ 00413f04-e557-41f9-812f-70eb18… │
│ ea463445-5bf3-4971-bdcb-67423d… │
│ 42ae3844-6100-4290-a19c-48322c… │
│ 5af3b2de-299e-45a5-bda4-06ec6a… │
│ 6351dcbd-4480-4418-85eb-045d45… │
│ 450181cc-145d-4f02-80ca-2bc7d2… │
│ 33c46b88-0bf0-4c14-9f12-61ccbf… │
│ 3cb74359-2163-49b2-bb91-19be29… │
│ 85acfa9a-5729-4895-a6e7-4fa6f3… │
└─────────────────────────────────┘
printing df after uuid exprs
shape: (50, 4)
┌─────────────────────────────────┬─────────────────────────────────┬─────────────────────────────────┬─────────────────────────────────┐
│ uuids                           ┆ new_uuids                       ┆ as_strs                         ┆ uuid_roundtrip                  │
│ ---                             ┆ ---                             ┆ ---                             ┆ ---                             │
│ ext[arrow.uuid]                 ┆ ext[arrow.uuid]                 ┆ str                             ┆ ext[arrow.uuid]                 │
╞═════════════════════════════════╪═════════════════════════════════╪═════════════════════════════════╪═════════════════════════════════╡
│ d3662fb9-3373-4643-ae41-4e4160… ┆ 61914000-5e9f-4385-b3a3-0b9362… ┆ 61914000-5e9f-4385-b3a3-0b9362… ┆ 61914000-5e9f-4385-b3a3-0b9362… │
│ 00413f04-e557-41f9-812f-70eb18… ┆ 00ed9560-9bf3-4c24-9c37-f8d84f… ┆ 00ed9560-9bf3-4c24-9c37-f8d84f… ┆ 00ed9560-9bf3-4c24-9c37-f8d84f… │
│ ea463445-5bf3-4971-bdcb-67423d… ┆ a5935623-7cc8-402f-a6c5-839b44… ┆ a5935623-7cc8-402f-a6c5-839b44… ┆ a5935623-7cc8-402f-a6c5-839b44… │
│ 42ae3844-6100-4290-a19c-48322c… ┆ 8c56b262-7e53-4dbc-af98-0d7f08… ┆ 8c56b262-7e53-4dbc-af98-0d7f08… ┆ 8c56b262-7e53-4dbc-af98-0d7f08… │
│ 5af3b2de-299e-45a5-bda4-06ec6a… ┆ 4a7b9801-fde5-4c5b-ac94-d55221… ┆ 4a7b9801-fde5-4c5b-ac94-d55221… ┆ 4a7b9801-fde5-4c5b-ac94-d55221… │
│ …                               ┆ …                               ┆ …                               ┆ …                               │
│ d7390a85-cfac-4519-9473-de5ef9… ┆ cdd76f00-cd33-4ff8-a1e6-9ec3e1… ┆ cdd76f00-cd33-4ff8-a1e6-9ec3e1… ┆ cdd76f00-cd33-4ff8-a1e6-9ec3e1… │
│ ea3c9839-e98e-47d7-803b-861394… ┆ d44819fd-6f5f-4662-bda6-c0a201… ┆ d44819fd-6f5f-4662-bda6-c0a201… ┆ d44819fd-6f5f-4662-bda6-c0a201… │
│ b65fed02-b309-4433-bcc6-f92c0c… ┆ abc67cf1-c16f-4238-ab48-b78c74… ┆ abc67cf1-c16f-4238-ab48-b78c74… ┆ abc67cf1-c16f-4238-ab48-b78c74… │
│ 5cdc5318-af32-4e9a-b2b5-e804d9… ┆ 809dd0be-4233-48f5-a735-96809f… ┆ 809dd0be-4233-48f5-a735-96809f… ┆ 809dd0be-4233-48f5-a735-96809f… │
│ b1997bb7-47ae-4075-8985-70b1c5… ┆ d77ae31b-8549-40ec-8fe4-47667c… ┆ d77ae31b-8549-40ec-8fe4-47667c… ┆ d77ae31b-8549-40ec-8fe4-47667c… │
└─────────────────────────────────┴─────────────────────────────────┴─────────────────────────────────┴─────────────────────────────────┘
column uuids's metadata is lala
column new_uuids's metadata is hi
column uuid_roundtrip's metadata is from strs
```

If I comment out the `dyn_display_value` func then the last bit becomes:

```
printing df after uuid exprs
shape: (50, 4)
┌─────────────────────────────────┬─────────────────────────────────┬─────────────────────────────────┬─────────────────────────────────┐
│ uuids                           ┆ new_uuids                       ┆ as_strs                         ┆ uuid_roundtrip                  │
│ ---                             ┆ ---                             ┆ ---                             ┆ ---                             │
│ ext[arrow.uuid]                 ┆ ext[arrow.uuid]                 ┆ str                             ┆ ext[arrow.uuid]                 │
╞═════════════════════════════════╪═════════════════════════════════╪═════════════════════════════════╪═════════════════════════════════╡
│ b"\xf7\xae\xc6w\xf6\x9aIl\xae(… ┆ b"\xa8\xd3\x8a\x12W\xcfN-\xb3;… ┆ a8d38a12-57cf-4e2d-b33b-907b35… ┆ b"\xa8\xd3\x8a\x12W\xcfN-\xb3;… │
│ b"kC\xaa\xe5\xa0jBz\x89\xd8\xa… ┆ b"\xb15\x1dgP?F3\xb4\xa5\xb5\x… ┆ b1351d67-503f-4633-b4a5-b519e0… ┆ b"\xb15\x1dgP?F3\xb4\xa5\xb5\x… │
│ b"v\x96\xb6\xe7\xf0\xfaJ\xbc\x… ┆ b"\xc0\xe5\x8e&\xbczB\x1e\x8c\… ┆ c0e58e26-bc7a-421e-8ce0-b4491b… ┆ b"\xc0\xe5\x8e&\xbczB\x1e\x8c\… │
│ b"\x8e\x8a\x1b}<YG\xd5\x8e\xcd… ┆ b"b\xe9U\xcb\x93\x85Hs\xadj]\x… ┆ 62e955cb-9385-4873-ad6a-5de07f… ┆ b"b\xe9U\xcb\x93\x85Hs\xadj]\x… │
│ b"?OM\xaf,\xc8K\xc6\x90t\xd5K\… ┆ b"yo\x1e\xe6\x93#OG\xab^yn\x20… ┆ 796f1ee6-9323-4f47-ab5e-796e20… ┆ b"yo\x1e\xe6\x93#OG\xab^yn\x20… │
│ …                               ┆ …                               ┆ …                               ┆ …                               │
│ b"\x8e\x05\x9b\xb7!YL\xb0\xa5\… ┆ b"IB|\xd6o^K\xed\xba\xedz\xe0\… ┆ 49427cd6-6f5e-4bed-baed-7ae0bb… ┆ b"IB|\xd6o^K\xed\xba\xedz\xe0\… │
│ b"\xbf\x8eAQ\x80\xf7H\xda\x85l… ┆ b";\xb0\x17\xdbY\xf2N\xec\xb3\… ┆ 3bb017db-59f2-4eec-b3be-5d84df… ┆ b";\xb0\x17\xdbY\xf2N\xec\xb3\… │
│ b"t\xd2J(\xd2\xb8E\xaa\xbb\x14… ┆ b"v\xa9\xdf\x15\xa6\xccH1\xbb\… ┆ 76a9df15-a6cc-4831-bb03-78aa87… ┆ b"v\xa9\xdf\x15\xa6\xccH1\xbb\… │
│ b"\x1e\x8c0\xa7%>D\xc6\x9b\x8b… ┆ b"\xa1D\x8e\x14\xec&N>\xa2A<%\… ┆ a1448e14-ec26-4e3e-a241-3c25df… ┆ b"\xa1D\x8e\x14\xec&N>\xa2A<%\… │
│ b">\xf8]\x92\x0c\xdeD\x20\xb2\… ┆ b"\xa6DW\x1b\xfc<D}\x93\x99;\x… ┆ a644571b-fc3c-447d-9399-3bd8ab… ┆ b"\xa6DW\x1b\xfc<D}\x93\x99;\x… │
└─────────────────────────────────┴─────────────────────────────────┴─────────────────────────────────┴─────────────────────────────────┘
```