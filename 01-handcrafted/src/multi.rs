use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Multi<T> {
    Multi(HashMap<String, T>),
    Single(T)
}

impl<T> Multi<T> {
    pub fn get(&self, key: Option<&str>) -> Option<&T> {
        match (self, key) {
            (Self::Multi(map), Some(key)) => map.get(key),
            (Self::Single(val), None) => Some(val),
            _ => None,
        }
    }

    // I realise this is also limited, it doesn't give us a way to add a new key in c8y
    // So if we have c8y.cloud.url, we can update c8y.cloud.url, but if we don't have any
    // c8y.edge configurations, we cannot create one using this as it will just return None.
    // Feel free to try and solve this conundrum, but I'm not expecting you to
    pub fn get_mut(&mut self, key: Option<&str>) -> Option<&mut T> {
        match (self, key) {
            (Self::Multi(map), Some(key)) => map.get_mut(key),
            (Self::Single(val), None) => Some(val),
            _ => None,
        }
    }
}