use serde::{Deserialize, Serialize};

/// MisalignmentErrorTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MisalignmentErrorTypeTextVariantEnum {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

impl Default for MisalignmentErrorTypeTextVariantEnum {
    fn default() -> Self {
        Self::PotentiallyUnintendedDataTransfer
    }
}

impl std::fmt::Display for MisalignmentErrorTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataTransfer => {
                "potentially_unintended_data_transfer"
            }
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataAccess => {
                "potentially_unintended_data_access"
            }
            MisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDestructiveActivity => {
                "potentially_unintended_destructive_activity"
            }
            MisalignmentErrorTypeTextVariantEnum::Other => "other",
        };
        write!(f, "{}", value)
    }
}
