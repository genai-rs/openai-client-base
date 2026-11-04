use serde::{Deserialize, Serialize};

/// LiveSessionClosedReasonTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveSessionClosedReasonTextVariantEnum {
    #[serde(rename = "connection_lost")]
    ConnectionLost,
}

impl std::fmt::Display for LiveSessionClosedReasonTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveSessionClosedReasonTextVariantEnum::ConnectionLost => "connection_lost",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LiveSessionClosedReasonTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "connection_lost" => Ok(Self::ConnectionLost),
            _ => Err(crate::models::ParseEnumError::new(
                "LiveSessionClosedReasonTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LiveSessionClosedReasonTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String> for LiveSessionClosedReasonTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String> for LiveSessionClosedReasonTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
