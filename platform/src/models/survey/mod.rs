use core::fmt;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct TotalSurveys {
    pub surveys: Vec<SurveySummary>,
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct SurveySummary {
    pub id: String,
    pub status: SurveyStatus,
    pub title: String,
    pub updated_at: u64,
    pub questions: u64,
    pub responses: Option<u64>,
    pub expected_responses: Option<u64>,
    pub quotas: Option<Vec<Quota>>,

    #[serde(skip)]
    pub r#type: String,

    // list surveys by account
    #[serde(skip)]
    pub gsi1: String,
    // list surveys by status
    #[serde(skip)]
    pub gsi2: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurveyStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress {
        started_at: u64,
        ended_at: Option<u64>,
    },
    #[serde(rename = "finished")]
    Finished,
}

impl fmt::Display for SurveyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SurveyStatus::Draft => write!(f, "draft"),
            SurveyStatus::InProgress {
                started_at,
                ended_at,
            } => write!(f, "in_progress {started_at} {}", ended_at.unwrap_or(0)),
            SurveyStatus::Finished => write!(f, "finished"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Quota {
    Attribute {
        // e.g. 1, 2, 3, 4, 5
        salary_tier: Option<SalaryTier>,

        // e.g. 02(Seoul), 051(Busan) and so on.
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
        quota: u64,
    },
    Panel(ProofId),
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;
pub type RegionCode = u16;

pub type ProofId = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Gender {
    Male,
    Female,
    Others,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Age {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}
