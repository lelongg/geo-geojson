//! This crates converts [geojson](https://geojson.org/) strings to [GeometryCollection](https://docs.rs/geo-types/0.4.3/geo_types/struct.GeometryCollection.html).
//!
//! This is the missing link between the [geo-types](https://github.com/georust/geo) crate and the [geojson](https://github.com/georust/geojson) crate and should probably be part of [geojson](https://github.com/georust/geojson).
//!
//! # Example
//!
//! ```
//! # fn main() -> Result<(), Box<std::error::Error>> {
//! # use std::fs;
//! let geojson_str = fs::read_to_string("src/tests/demo.json")?;
//! let collection: geo_types::GeometryCollection<f64> = geo_geojson::from_str(&geojson_str)?;
//! # Ok(())
//! # }
//! ```

use geo_types::GeometryCollection;
use geojson::{Error, GeoJson};
use num_traits::{Float, Num, NumCast};
use std::str::FromStr;

#[cfg(test)]
mod tests;

/// Converts [geojson](https://geojson.org/) strings to [GeometryCollection](https://docs.rs/geo-types/0.4.3/geo_types/struct.GeometryCollection.html).
///
/// # Errors
///
/// Returns the same errors, under the same conditions, as [GeoJson::from_str](https://docs.rs/geojson/0.16.0/geojson/enum.GeoJson.html#method.from_str).

pub fn from_str<T>(s: &str) -> Result<GeometryCollection<T>, Error>
where
    T: Num + NumCast + PartialOrd + Copy + Float,
{
    let geojson = GeoJson::from_str(s)?;
    Ok(match geojson {
        GeoJson::Feature(feature) => conversion::from_feature(feature),
        GeoJson::FeatureCollection(feature_collection) => {
            conversion::from_feature_collection(feature_collection)
        }
        GeoJson::Geometry(geometry) => conversion::from_geometry(geometry),
    })
}

pub mod conversion {
    //! This module contains conversion function from [geojson](https://docs.rs/geojson/0.16.0/geojson/) types to [GeometryCollection](https://docs.rs/geo-types/0.4.3/geo_types/struct.GeometryCollection.html).

    use geo_types::{
        GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
    };
    use geojson::{Feature, FeatureCollection, Geometry, Value};
    use num_traits::{Float, Num, NumCast};
    use std::convert::TryInto;

    pub fn from_feature<T>(feature: Feature) -> GeometryCollection<T>
    where
        T: Num + NumCast + PartialOrd + Copy + Float,
    {
        feature
            .geometry
            .map(from_geometry)
            .unwrap_or_else(GeometryCollection::new)
    }

    pub fn from_feature_collection<T>(
        feature_collection: FeatureCollection,
    ) -> GeometryCollection<T>
    where
        T: Num + NumCast + PartialOrd + Copy + Float,
    {
        feature_collection
            .features
            .into_iter()
            .flat_map(from_feature)
            .collect()
    }

    pub fn from_geometry<T>(geometry: Geometry) -> GeometryCollection<T>
    where
        T: Num + NumCast + PartialOrd + Copy + Float,
    {
        from_value(geometry.value)
    }

    pub fn from_value<T>(value: Value) -> GeometryCollection<T>
    where
        T: Num + NumCast + PartialOrd + Copy + Float,
    {
        match value {
            Value::Point(_) => TryInto::<Point<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::MultiPoint(_) => TryInto::<MultiPoint<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::LineString(_) => TryInto::<LineString<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::MultiLineString(_) => TryInto::<MultiLineString<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::Polygon(_) => TryInto::<Polygon<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::MultiPolygon(_) => TryInto::<MultiPolygon<T>>::try_into(value)
                .map(|value| GeometryCollection(vec![value.into()])),
            Value::GeometryCollection(geometry_collection) => Ok(geometry_collection
                .into_iter()
                .flat_map(from_geometry)
                .collect()),
        }
        .unwrap_or_else(|_| GeometryCollection::new())
    }

}
