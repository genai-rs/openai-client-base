use serde::{Deserialize, Serialize};

/// LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    All,
}

impl Default for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    fn default() -> Self {
        Self::All
    }
}

impl std::fmt::Display for LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDataChannelConfigParamAllowedServerEventsTextVariantEnum::All => "all",
        };
        write!(f, "{}", value)
    }
}
