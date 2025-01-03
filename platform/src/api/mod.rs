pub mod common;
pub mod v1;
pub mod v2;
pub mod aws {
    pub mod ses;
}

use dioxus::fullstack::prelude::*;
use server_fn::codec::{GetUrl, Json};

#[server(endpoint = "/version", input=GetUrl, output=Json)]
pub async fn version() -> Result<String, ServerFnError> {
    Ok(match option_env!("VERSION") {
        Some(version) => match option_env!("COMMIT") {
            Some(commit) => format!("{}-{}", version, commit),
            None => format!("{}", version),
        },
        None => match option_env!("DATE") {
            Some(date) => date.to_string(),
            None => "unknown".to_string(),
        },
    }
    .to_string())
}
