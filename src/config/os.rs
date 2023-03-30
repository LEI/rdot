use std::env::consts;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref FAMILY: String = consts::FAMILY.to_string();
    static ref OS: String = consts::OS.to_string();
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
/// Lowercase operating system
#[serde(rename_all = "snake_case")]
pub(crate) enum Os {
    Darwin,
    Linux,
    Windows,
}

pub(crate) type OsArray = Vec<Os>;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum OsValue {
    // None,
    String(Os),
    Array(OsArray),
}

// impl Default for OsValue {
//     fn default() -> Self {
//         OsValue::Array(vec![])
//     }
// }

// impl From<OsValue> for Vec<Os> {
//     fn from(value: OsValue) -> Self {
//         match value {
//             OsValue::None => vec![],
//             OsValue::String(os) => vec![os],
//             OsValue::Array(oses) => oses,
//         }
//     }
// }
