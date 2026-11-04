use serde::{Deserialize, Serialize};

/// FineTuneReinforcementHyperparametersEvalInterval - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalInterval {
    TextVariant(FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum),
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FineTuneReinforcementHyperparametersEvalInterval::TextVariant(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

/// FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    Auto,
}

impl std::fmt::Display for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneReinforcementHyperparametersEvalIntervalTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
