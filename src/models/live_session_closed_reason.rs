use crate::models;
use serde::{Deserialize, Serialize};

/// LiveSessionClosedReason - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveSessionClosedReason {
    TextVariant(LiveSessionClosedReasonTextVariantEnum),
}

impl std::fmt::Display for LiveSessionClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveSessionClosedReason::TextVariant(value) => write!(f, "{}", value),
        }
    }
}

/// LiveSessionClosedReasonTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveSessionClosedReasonTextVariantEnum {
    #[serde(rename = "connection_lost")]
    ConnectionLost,
}

impl Default for LiveSessionClosedReasonTextVariantEnum {
    fn default() -> Self {
        Self::ConnectionLost
    }
}

impl std::fmt::Display for LiveSessionClosedReasonTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveSessionClosedReasonTextVariantEnum::ConnectionLost => "connection_lost",
        };
        write!(f, "{}", value)
    }
}
