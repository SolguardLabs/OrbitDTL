use crate::error::{OrbitError, OrbitResult};
use serde::Serialize;

pub fn to_pretty_json<T: Serialize>(value: &T) -> OrbitResult<String> {
    serde_json::to_string_pretty(value).map_err(|error| OrbitError::Codec(error.to_string()))
}
