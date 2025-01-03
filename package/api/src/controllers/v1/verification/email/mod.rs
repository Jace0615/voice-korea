use aws_sdk_sesv2::types::Content;
use by_axum::axum::{extract::State, Json};
use by_axum::log::root;
use chrono::Utc;
use models::AuthDocument;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use serde::Deserialize;

use crate::utils::email::send_email;
use crate::utils::error::ApiError;

#[derive(Deserialize)]
pub struct EmailSendParams {
    pub email: String,
}

pub async fn send_handler(
    State(db): State<std::sync::Arc<easy_dynamodb::Client>>,
    Json(body): Json<EmailSendParams>,
) -> Result<String, ApiError> {
    //TODO: If Email send failed, remove Document
    //TODO: Add request limit
    let log = root();

    let random_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    let doc = models::AuthDocument::new(
        uuid::Uuid::new_v4().to_string(),
        body.email.clone(),
        random_string.clone(),
    );
    let doc_id = doc.id.clone();
    match db.create(doc).await {
        Ok(_) => {
            match send_email(
                body.email,
                Content::builder()
                    .data("인증번호 6자리입니다. 확인 후 3분 이내에 입력해주세요.")
                    .build()
                    .unwrap(),
                Content::builder()
                    .data(format!("인증번호: {:?}", random_string))
                    .build()
                    .unwrap(),
            )
            .await
            {
                Ok(v) => {
                    slog::debug!(log, "Email Send {}", v);
                    Ok(doc_id)
                }
                Err(e) => Err(ApiError::SESServiceError(e.to_string())),
            }
        }
        Err(e) => Err(ApiError::DynamoCreateException(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct EmailVerifyParams {
    pub id: String,
    pub value: String,
}

pub async fn verify_handler(
    State(db): State<std::sync::Arc<easy_dynamodb::Client>>,
    Json(body): Json<EmailVerifyParams>,
) -> Result<String, ApiError> {
    let result: Result<Option<AuthDocument>, easy_dynamodb::error::DynamoException> =
        db.get(&body.id).await;
    let auth = match result {
        Ok(Some(v)) => v,
        Ok(None) => return Err(ApiError::AuthKeyNotMatch(body.id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let auth_doc_id = auth.id.clone();

    if auth.value != body.value || auth.expired_at < Utc::now().timestamp() {
        return Err(ApiError::AuthKeyNotMatch(body.id));
    }

    Ok(auth_doc_id)
}
