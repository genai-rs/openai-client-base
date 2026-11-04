use serde::{Deserialize, Serialize};

/// FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    Auto,
}

impl std::fmt::Display for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum::Auto => "auto",
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "auto" => Ok(Self::Auto),
            _ => Err(crate::models::ParseEnumError::new(
                "FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str>
    for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for FineTuneDpoHyperparametersLearningRateMultiplierTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
