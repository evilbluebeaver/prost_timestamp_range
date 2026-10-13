# prost-timestamp-range

`prost-timestamp-range` provides a `prost::Message` implementation for timestamp
ranges and conversions to and from `chrono` UTC bounds.

## Usage

Add the runtime crate and protobuf build dependency to `Cargo.toml`:

```toml
[dependencies]
prost-timestamp-range = "0.1"

[build-dependencies]
tonic-prost-build = "0.14"
```

In `build.rs`, map the protobuf type to `ProstTimestampRange` before compiling
your schemas:

```rust
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    tonic_prost_build::configure()
        .extern_path(
            ".timestamp_range.TimestampRange",
            "::prost_timestamp_range::ProstTimestampRange",
        )
        .file_descriptor_set_path(out_dir.join("service_descriptor.bin"))
        .compile_protos(&["proto/service.proto"], &["proto"])?;

    Ok(())
}
```

Schemas that use the range should import `timestamp_range.proto` and reference
`timestamp_range.TimestampRange`.

## Encoding

`start` and `end` are optional Unix timestamps in seconds. An absent endpoint is
unbounded. Bounds default to an inclusive start and exclusive end, so a message
with both timestamps and no bound flags represents `[start, end)`.

- Set `start_is_excluded` to `true` for an exclusive start bound.
- Set `end_is_included` to `true` for an inclusive end bound.
- Invalid timestamps decode as unbounded endpoints.

The matching protobuf schema is available in
[`proto/timestamp_range.proto`](proto/timestamp_range.proto).

## License

Licensed under the MIT License.