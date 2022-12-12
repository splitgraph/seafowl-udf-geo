# seafowl-udf-geo
Example WASM User Defined Function (UDF) for [Seafowl](https://seafowl.io/) in [Rust](https://www.rust-lang.org/).

For more information on Seafowl WASM support read [this blog post](https://www.splitgraph.com/blog/seafowl-wasm-udfs).

A blogpost describing the project which uses this code can be found [here](https://www.splitgraph.com/blog/seafowl-geo-distance-udf).

To create your own Seafowl UDFs in Rust, fork [this example repository](https://github.com/splitgraph/seafowl-udf-rust).

# Dependencies

First [install Rust and Cargo](https://www.rust-lang.org/tools/install), then install [cargo-wasi](https://github.com/bytecodealliance/cargo-wasi) by running:

```bash
cargo install cargo-wasi
```

# Building the WASM module

```bash
cargo wasi build --release
```

# Loading the WASM module into Seafowl as a UDF

This repository includes the `create_udf.sh` shell script which creates the Seafowl function wrapping the Rust WASM logic.

```bash
# Start seafowl
SEAFOWL__FRONTEND__HTTP__WRITE_ACCESS=any ./target/release/seafowl

# In another terminal tab/window, run
./create_udf.sh 
```

Invoking the newly created UDF:

```bash
./query_udf.sh
```

# Running unit tests

```bash
cargo test
```
