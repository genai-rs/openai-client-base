use serde::{Deserialize, Serialize};

/// LiveDecisionEventSourceEventType - Untagged union type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveDecisionEventSourceEventType {
    TextVariant(LiveDecisionEventSourceEventTypeTextVariantEnum),
    Null,
}

impl std::fmt::Display for LiveDecisionEventSourceEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveDecisionEventSourceEventType::TextVariant(value) => write!(f, "{}", value),
            LiveDecisionEventSourceEventType::Null => write!(f, "null"),
        }
    }
}

/// LiveDecisionEventSourceEventTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiveDecisionEventSourceEventTypeTextVariantEnum {
    #[serde(rename = "session.output_transcript.delta")]
    SessionOutputTranscriptDelta,
}

impl Default for LiveDecisionEventSourceEventTypeTextVariantEnum {
    fn default() -> Self {
        Self::SessionOutputTranscriptDelta
    }
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
