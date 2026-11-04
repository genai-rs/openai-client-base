use crate::models;
use serde::{Deserialize, Serialize};

/// CreateTranscriptionRequestChunkingStrategy - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionRequestChunkingStrategy {
    TextVariant(CreateTranscriptionRequestChunkingStrategyTextVariantEnum),
    Vadconfig(models::VadConfig),
    Null,
}

impl std::fmt::Display for CreateTranscriptionRequestChunkingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTranscriptionRequestChunkingStrategy::TextVariant(value) => {
                write!(f, "{}", value)
            }
            CreateTranscriptionRequestChunkingStrategy::Vadconfig(value) => {
                match serde_json::to_string(value) {
                    Ok(s) => write!(f, "{}", s),
                    Err(_) => Err(std::fmt::Error),
                }
            }
            CreateTranscriptionRequestChunkingStrategy::Null => write!(f, "null"),
        }
    }
}

/// CreateTranscriptionRequestChunkingStrategyTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    Auto,
}

impl std::fmt::Display for CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CreateTranscriptionRequestChunkingStrategyTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "CreateTranscriptionRequestChunkingStrategyTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for CreateTranscriptionRequestChunkingStrategyTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for CreateTranscriptionRequestChunkingStrategyTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for CreateTranscriptionRequestChunkingStrategyTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
