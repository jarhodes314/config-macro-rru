//! # Multi-value serde fields
//!
//! The core of the macro changes centre around the ability to store multiple values in the `c8y` field (or `az`/`aws`)
//!
//! For example, we can currently store:
//! ```toml
//! [c8y]
//! url = "https://c8y.example.com"
//! ```
//!
//! To support multiple c8y connections, we need to store:
//! ```toml
//! [c8y.cloud]
//! url = "https://c8y-cloud.example.com"
//! bridge.topic_prefix = "c8y-cloud"
//!
//! [c8y.edge]
//! url = "https://c8y-edge.example.com"
//! bridge.topic_prefix = "c8y-edge"
//! ```
//!
//! This crate will define the `Multi<T>` type used for the Cumulocity field to enable this. The goal of this type
//! is that it can deserialize the value associated with the key `c8y` in both of the above TOML files.
//!
//! There are some tests and a stub implementation that copes with the existing use case.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
struct TEdgeConfigDto {
    c8y: Multi<C8y>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Multi<T> {
    Multi(HashMap<String, T>),
    Single(T),
}

impl<T> Multi<T> {
    /// Retrieves the value matching the provided key if it exists
    ///
    /// If this `Multi` stores a single value, the value can be retrieved using the key `None`
    /// If this `Multi` stores multiple values, the value can be retrieved using `Some(key)`
    pub fn get(&self, key: Option<&str>) -> Option<&T> {
        match (self, key) {
            (Self::Single(val), None) => Some(&val),
            (Self::Multi(val), Some(key)) => val.get(key),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
struct C8y {
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_supports_a_single_unnamed_cumulocity_configuration() {
        let url = "https://c8y.example.com";
        let dto: TEdgeConfigDto = toml::from_str(&format!("c8y.url = \"{url}\"")).unwrap();

        assert_eq!(dto.c8y.get(None).unwrap().url, url);
    }

    #[test]
    fn multi_supports_multiple_named_cumulocity_configurations() {
        let cloud = "https://c8y-cloud.example.com";
        let edge = "https://c8y-edge.example.com";
        let dto: TEdgeConfigDto = toml::from_str(&format!(
            "c8y.edge.url = \"{edge}\"\nc8y.cloud.url = \"{cloud}\""
        ))
        .unwrap();

        assert_eq!(dto.c8y.get(Some("cloud")).unwrap().url, cloud);
        assert_eq!(dto.c8y.get(Some("edge")).unwrap().url, edge);
    }

    #[test]
    fn retrieving_the_single_value_with_a_named_key_fails() {
        let url = "https://c8y.example.com";
        let dto: TEdgeConfigDto = toml::from_str(&format!("c8y.url = \"{url}\"")).unwrap();

        assert_eq!(dto.c8y.get(Some("cloud")), None);
    }

    #[test]
    fn retrieving_a_multi_value_without_a_named_key_fails() {
        let cloud = "https://c8y-cloud.example.com";
        let edge = "https://c8y-edge.example.com";
        let dto: TEdgeConfigDto = toml::from_str(&format!(
            "c8y.edge.url = \"{edge}\"\nc8y.cloud.url = \"{cloud}\""
        ))
        .unwrap();

        assert_eq!(dto.c8y.get(None), None);
    }
}
