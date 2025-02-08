#![allow(unused_variables)]
use crate::attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2};
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

fn deserialize<'de, D, T>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: TryFrom<i32> + std::fmt::Debug,
    <T as TryFrom<i32>>::Error: std::fmt::Debug,
{
    let s: i32 = serde::Deserialize::deserialize(deserializer)?;
    T::try_from(s).map_err(|v| serde::de::Error::custom("PARSE FAILED"))
}

#[derive(validator::Validate)]
#[api_model(base = "/organizations/v2/:org-id/panels", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update, query_action = search_by)]
    pub name: String,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "age")]
    #[serde(deserialize_with = "deserialize")]
    pub age: AgeV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "gender")]
    #[serde(deserialize_with = "deserialize")]
    pub gender: GenderV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "region")]
    #[serde(deserialize_with = "deserialize")]
    pub region: RegionV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable, rename = "salary")]
    #[serde(deserialize_with = "deserialize")]
    pub salary: SalaryV2,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
}
