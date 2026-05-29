//! Property schema entries per SPEC 07 §2.4.

use crate::path::PropPath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropType {
    Null,
    Bool,
    Number,
    String,
    List,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropDescribe {
    pub path: PropPath,

    #[serde(rename = "type")]
    pub ty: PropType,

    pub mutable: bool,

    pub sensitive: bool,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// For `Object`: direct child paths (full dotted form). SPEC 07 §2.4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<PropPath>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub deprecated: bool,

    /// Per §3.2: paths marked transient opt out of `props.changed`.
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub transient: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl PropDescribe {
    /// Builder for the common case: a leaf value.
    pub fn leaf(path: PropPath, ty: PropType, description: impl Into<String>) -> Self {
        Self {
            path,
            ty,
            mutable: false,
            sensitive: false,
            description: description.into(),
            format: None,
            children: None,
            enum_values: None,
            min: None,
            max: None,
            default: None,
            since: None,
            deprecated: false,
            transient: false,
        }
    }

    pub fn with_sensitive(mut self, s: bool) -> Self {
        self.sensitive = s;
        self
    }
    pub fn with_mutable(mut self, m: bool) -> Self {
        self.mutable = m;
        self
    }
    pub fn with_transient(mut self, t: bool) -> Self {
        self.transient = t;
        self
    }
    pub fn with_format(mut self, f: impl Into<String>) -> Self {
        self.format = Some(f.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_serializes_minimally() {
        let d = PropDescribe::leaf(
            PropPath::new("config.bind").unwrap(),
            PropType::String,
            "WireGuard interface address",
        )
        .with_format("host:port");
        let j = serde_json::to_value(&d).unwrap();
        assert_eq!(j["path"], "config.bind");
        assert_eq!(j["type"], "string");
        assert_eq!(j["mutable"], false);
        assert_eq!(j["sensitive"], false);
        assert_eq!(j["format"], "host:port");
        // optionals absent
        assert!(j.get("default").is_none());
        assert!(j.get("transient").is_none());
        assert!(j.get("deprecated").is_none());
    }
}
