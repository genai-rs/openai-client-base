use serde::{Deserialize, Serialize};

/// BetaMisalignmentErrorTypeTextVariantEnum - String enum type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetaMisalignmentErrorTypeTextVariantEnum {
    #[serde(rename = "potentially_unintended_data_transfer")]
    PotentiallyUnintendedDataTransfer,
    #[serde(rename = "potentially_unintended_data_access")]
    PotentiallyUnintendedDataAccess,
    #[serde(rename = "potentially_unintended_destructive_activity")]
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

impl Default for BetaMisalignmentErrorTypeTextVariantEnum {
    fn default() -> Self {
        Self::PotentiallyUnintendedDataTransfer
    }
}

impl std::fmt::Display for BetaMisalignmentErrorTypeTextVariantEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataTransfer => {
                "potentially_unintended_data_transfer"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDataAccess => {
                "potentially_unintended_data_access"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::PotentiallyUnintendedDestructiveActivity => {
                "potentially_unintended_destructive_activity"
            }
            BetaMisalignmentErrorTypeTextVariantEnum::Other => "other",
        };
        write!(f, "{}", value)
    }
}
