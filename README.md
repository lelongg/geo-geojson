# geo-geojson

This crates converts [geojson](https://geojson.org/) strings to [GeometryCollection](https://docs.rs/geo-types/0.4.3/geo_types/struct.GeometryCollection.html).

[![crate.io](https://img.shields.io/crates/v/geo-geojson.svg)](https://crates.io/crates/geo-geojson)
[![docs.rs](https://docs.rs/geo-geojson/badge.svg)](https://docs.rs/geo-geojson)

This is the missing link between the [geo-types](https://github.com/georust/geo) crate and the [geojson](https://github.com/georust/geojson) crate and should probably be part of [geojson](https://github.com/georust/geojson).

## Example

```rust
let geojson_str = fs::read_to_string("src/tests/demo.json")?;
let collection: geo_types::GeometryCollection<f64> = geo_geojson::from_str(&geojson_str)?;
```
