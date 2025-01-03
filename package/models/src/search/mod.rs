use super::survey::{Age, Gender, ProofId, RegionCode, SalaryTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SearchQuery {
    SearchPanel {
        // FIXME: it can be modified, because it is a part of noncelab.
        // e.g. 1, 2, 3, 4, 5
        salary_tier: Option<SalaryTier>,

        // e.g. 02(Seoul), 051(Busan) and so on.
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
    },
    SearchAttiributeCertificate {
        // FIXME: it can be modified, because it is a part of noncelab.
        // e.g. 1, 2, 3, 4, 5
        salary_tier: Option<SalaryTier>,

        // e.g. 02(Seoul), 051(Busan) and so on.
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    // FIXME: it can be modified, because it is a part of noncelab.
    proof_id: ProofId,
    salary_tier: Option<SalaryTier>,
    region_code: Option<RegionCode>,
    gender: Option<Gender>,
    age: Option<Age>,
}

impl SearchResult {
    pub fn new(
        proof_id: ProofId,
        salary_tier: Option<SalaryTier>,
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
    ) -> Self {
        SearchResult {
            proof_id,
            salary_tier,
            region_code,
            gender,
            age,
        }
    }
}
