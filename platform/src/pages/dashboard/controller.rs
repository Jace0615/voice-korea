#![allow(non_snake_case)]
use std::time::{Duration, UNIX_EPOCH};

use chrono::{self, DateTime, Local};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::{ListSurveyResponse, SurveyDraftStatus, UpsertSurveyDraftRequest};

use crate::api::v2::survey::{list_surveys, upsert_survey_draft};

use models::prelude::Survey;

use super::{Language, Route};

// #[derive(Debug, Clone, PartialEq)]
// pub struct Survey {

//     pub survey_id: String,
//     pub draft_id: String,
//     pub survey_sequence: Option<SurveyDraftStatus>,
//     pub survey_type: String,
//     pub title: String,
//     pub update_at: String,
//     pub response_count: u64,
//     pub total_response_count: u64,
// }

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub current_bookmark: Signal<Option<String>>,
    pub surveys: Signal<Vec<Survey>>,
    pub clicked_type: Signal<u64>, //0: type-1, 1: type-2
    pub is_error: Signal<bool>,
    pub text: Signal<String>,
}

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language) -> Self {
        #[cfg(feature = "web")]
        {
            use crate::service::login_service::use_login_service;

            let navigator = use_navigator();
            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let mut ctrl = Self {
            current_bookmark: use_signal(|| None),
            surveys: use_signal(|| vec![]),
            clicked_type: use_signal(|| 0),
            is_error: use_signal(|| false),
            text: use_signal(|| "".to_string()),
        };
        let res = use_resource(|| async move {
            match list_surveys(Some(100), None).await {
                Ok(res) => res,
                _ => ListSurveyResponse {
                    survey: vec![],
                    bookmark: None,
                },
            }
        });
        match res.value()() {
            Some(v) => {
                ctrl.surveys.set(v.survey.clone());
                ctrl.current_bookmark.set(v.bookmark);
            }
            _ => {}
        }

        ctrl
    }

    pub fn get_search_text(&self) -> String {
        (self.text)()
    }

    pub fn change_search_text(&mut self, text: String) {
        self.text.set(text);
    }

    pub async fn search_text(&mut self) {
        tracing::debug!("search text: {}", self.get_search_text());
    }

    pub fn format_date(timestamp: u64) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(timestamp);
        let datetime = DateTime::<Local>::from(d);
        datetime.format("%Y.%m.%d").to_string()
    }

    pub fn get_clicked_type(&self) -> u64 {
        (self.clicked_type)()
    }

    pub fn set_clicked_type(&mut self, clicked_type: u64) {
        self.clicked_type.set(clicked_type);
    }

    pub fn get_total_surveys(&self) -> Vec<Survey> {
        (self.surveys)()
    }

    pub fn get_error(&self) -> bool {
        (self.is_error)()
    }

    pub async fn clicked_create_survey(&mut self, lang: Language) {
        let navigator = use_navigator();
        let res = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: None,
            status: Some(SurveyDraftStatus::Title),
            title: Some("".to_string()),
            quotas: None,
            questions: None,
            started_at: None,
            ended_at: None,
        })
        .await;

        match res {
            Ok(v) => {
                self.is_error.set(false);
                navigator.push(Route::WriteTitlePage {
                    lang: lang.clone(),
                    survey_id: v,
                });
            }
            Err(e) => {
                tracing::error!("create survey err: {:?}", e);
                self.is_error.set(true);
            }
        }
    }
}
