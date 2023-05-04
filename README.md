NVIDIA DeepStream SDK for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP
* Currently, Target SDK Version is 6.1.1

## How to use

* Define in Cargo.toml

```toml
[dependencies]

nvds = { version = "*", features = ["all"] }
```

### features

| feature name      | header file          | linking dylib           |
|-------------------|----------------------|-------------------------|
| logger            | nvds_logger.h        | nvds_logger             |
| meta              | nvdsmeta.h           | nvdsgst_meta, nvds_meta |
| obj_encode        | nvds_obj_encode.h    | nvds_batch_jpegenc      |
| surface           | nvbufsurface.h       | nvbufsurface            |
| surface_transform | nvbufsurftransform.h | nvbufsurftransform      |
| utils             | nvds_version.h       | nvds_utils              |
| yaml              | nvds_yml_parser.h    | nvds_yml_parser         |

