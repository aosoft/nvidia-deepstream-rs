NVIDIA DeepStream SDK for Rust
=====

[LICENSE (MIT)](LICENSE)

## Notice

* WIP
* Currently Target SDK Version is 6.1.1

## How to use

* Define in Cargo.toml

```toml
[dependencies]

nvds = { version = "*", features = ["all"] }
```

### features

| feature name      | linking dylib           |
|-------------------|-------------------------|
| logger            | nvds_logger             |
| meta              | nvdsgst_meta, nvds_meta |
| obj_encode        | nvds_batch_jpegenc      |
| surface           | nvbufsurface            |
| surface_transform | nvbufsurftransform      |
| utils             | nvds_utils              |
| yaml              | nvds_yml_parser         |
