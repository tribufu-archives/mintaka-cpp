// Copyright (c) Tribufu. All Rights Reserved.

pub use serde_json::from_slice;
pub use serde_json::from_str;
pub use serde_json::to_string;
pub use serde_json::to_string_pretty;
pub use serde_json::to_value;
pub use serde_json::to_vec_pretty;

#[macro_export]
macro_rules! include_json {
    ($path:expr) => {
        $crate::from_str(include_str!($path)).expect("Failed to load JSON")
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct JsonTest {
        version: String,
    }

    #[test]
    fn test_include_json() {
        let data: JsonTest = include_json!("test.json");
        assert!(data.version == "0.0.0".to_owned())
    }
}
