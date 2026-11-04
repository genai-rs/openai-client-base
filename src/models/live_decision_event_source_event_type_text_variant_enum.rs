use serde::{Deserialize, Serialize};

/// LiveDecisionEventSourceEventTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDecisionEventSourceEventTypeTextVariantEnum {
    #[serde(rename = "session.output_transcript.delta")]
    SessionOutputTranscriptDelta,
}

impl std::fmt::Display for LiveDecisionEventSourceEventTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LiveDecisionEventSourceEventTypeTextVariantEnum::SessionOutputTranscriptDelta => {
                "session.output_transcript.delta"
            }
        };
        write!(f, "{}", value)
    }
}

impl ::std::str::FromStr for LiveDecisionEventSourceEventTypeTextVariantEnum {
    type Err = crate::models::ParseEnumError;

    fn from_str(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        match value {
            "session.output_transcript.delta" => Ok(Self::SessionOutputTranscriptDelta),
            _ => Err(crate::models::ParseEnumError::new(
                "LiveDecisionEventSourceEventTypeTextVariantEnum",
                value,
            )),
        }
    }
}

impl ::std::convert::TryFrom<&str> for LiveDecisionEventSourceEventTypeTextVariantEnum {
    type Error = crate::models::ParseEnumError;

    fn try_from(value: &str) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<&::std::string::String>
    for LiveDecisionEventSourceEventTypeTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}

impl ::std::convert::TryFrom<::std::string::String>
    for LiveDecisionEventSourceEventTypeTextVariantEnum
{
    type Error = crate::models::ParseEnumError;

    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, crate::models::ParseEnumError> {
        value.parse()
    }
}
