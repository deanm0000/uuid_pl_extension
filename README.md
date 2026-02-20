Right now this is just to test a PR in polars.
If/when that PR gets merged then I'll make this.

This just shows that we can get these printed results for:

As ChunkedArray
```
shape: (50,)
ChunkedArray: 'uuids' [ext[uuid]]
[
        559a03fa-7000-4592-b782-c3f091a2cfa1
        20f752fd-7582-422b-963e-5de7a65edfcb
        16b2c671-2e3d-4c9b-9994-60ee08b68b85
        da57f4ba-88ac-4725-ad9d-d74d3d087cc2
        0064f9ca-d936-4095-bee4-1ef81de813c8
        a966a232-6cf0-4d86-a39c-32335e65f293
        9cdfca1f-f4b6-4c8d-b84d-7278a0973527
        a1ae83fd-89d3-4c77-bd1c-622e0c6c09a6
        7f2c7ae3-f9f2-4f38-a5fa-1b543709f8f1
        2b45a5b8-a200-446b-b507-6a77e9163323
]
```

as Series
```
shape: (50,)
Series: 'uuids' [ext[uuid]]
[
        559a03fa-7000-4592-b782-c3f091a2cfa1
        20f752fd-7582-422b-963e-5de7a65edfcb
        16b2c671-2e3d-4c9b-9994-60ee08b68b85
        da57f4ba-88ac-4725-ad9d-d74d3d087cc2
        0064f9ca-d936-4095-bee4-1ef81de813c8
        a966a232-6cf0-4d86-a39c-32335e65f293
        9cdfca1f-f4b6-4c8d-b84d-7278a0973527
        a1ae83fd-89d3-4c77-bd1c-622e0c6c09a6
        7f2c7ae3-f9f2-4f38-a5fa-1b543709f8f1
        2b45a5b8-a200-446b-b507-6a77e9163323
]
```

as 50 height DataFrame
```
shape: (50, 1)
┌─────────────────────────────────┐
│ uuids                           │
│ ---                             │
│ ext[uuid]                       │
╞═════════════════════════════════╡
│ 559a03fa-7000-4592-b782-c3f091… │
│ 20f752fd-7582-422b-963e-5de7a6… │
│ 16b2c671-2e3d-4c9b-9994-60ee08… │
│ da57f4ba-88ac-4725-ad9d-d74d3d… │
│ 0064f9ca-d936-4095-bee4-1ef81d… │
│ …                               │
│ 5a1c5999-7cb1-4ff0-ae0a-cbd61e… │
│ 48153640-7b32-4c66-810e-5c4830… │
│ 6ebb24ad-5678-4065-abad-7fd094… │
│ 9c1efc60-5468-4622-9421-4f0747… │
│ 055feaf1-c1e8-40fe-83fe-998087… │
└─────────────────────────────────┘
```

as 10 height DataFrame
```
shape: (10, 1)
┌─────────────────────────────────┐
│ uuids                           │
│ ---                             │
│ ext[uuid]                       │
╞═════════════════════════════════╡
│ 559a03fa-7000-4592-b782-c3f091… │
│ 20f752fd-7582-422b-963e-5de7a6… │
│ 16b2c671-2e3d-4c9b-9994-60ee08… │
│ da57f4ba-88ac-4725-ad9d-d74d3d… │
│ 0064f9ca-d936-4095-bee4-1ef81d… │
│ a966a232-6cf0-4d86-a39c-32335e… │
│ 9cdfca1f-f4b6-4c8d-b84d-7278a0… │
│ a1ae83fd-89d3-4c77-bd1c-622e0c… │
│ 7f2c7ae3-f9f2-4f38-a5fa-1b5437… │
│ 2b45a5b8-a200-446b-b507-6a77e9… │
└─────────────────────────────────┘
```
