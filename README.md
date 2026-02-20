Right now this is mostly to test a PR in polars.
If/when that PR gets merged then I'll do more with uuid extension.

This just shows that the output with that PR implemented:

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