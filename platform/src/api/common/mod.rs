use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonQueryResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> Default for CommonQueryResponse<T> {
    fn default() -> Self {
        CommonQueryResponse {
            items: Vec::<T>::new(),
            bookmark: None,
        }
    }
}

#[cfg(feature = "server")]
impl<T> CommonQueryResponse<T>
where
    T: std::fmt::Debug + serde::de::DeserializeOwned,
{
    pub async fn query<F>(
        log: &slog::Logger,
        index: &str,
        bookmark: Option<String>,
        size: Option<i32>,
        filter: Vec<(&str, F)>,
    ) -> Result<Self, dioxus::prelude::ServerFnError>
    where
        F: std::fmt::Debug + serde::Serialize,
    {
        let cli = easy_dynamodb::get_client(log.clone());

        match cli
            .find(index, bookmark, Some(size.unwrap_or(100)), filter)
            .await
        {
            Ok((Some(items), bookmark)) => Ok(CommonQueryResponse { items, bookmark }),
            Ok((None, bookmark)) => Ok(CommonQueryResponse {
                items: vec![],
                bookmark,
            }),
            Err(e) => {
                return Err(dioxus::prelude::ServerFnError::ServerError(format!(
                    "{:?}",
                    e
                )));
            }
        }
    }
}