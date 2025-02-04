use chrono::Local;
use dioxus::prelude::*;
use dioxus_translate::translate;
use models::{
    prelude::{PublicSurveyQuestionType, Question},
    ProjectArea,
};

use super::i18n::SurveyNewTranslate;

#[derive(Clone, Copy)]
pub struct Controller {
    selected_field: Signal<Option<ProjectArea>>,
    title: Signal<String>,
    description: Signal<String>,
    start_date: Signal<i64>,
    end_date: Signal<i64>,
    nav: Navigator,

    surveys: Signal<Vec<Question>>,
    total_survey_types: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Self {
        let translates: SurveyNewTranslate = translate(&lang.clone());

        let timestamp = Local::now().timestamp();
        let ctrl = Self {
            nav: use_navigator(),
            selected_field: use_signal(|| None),
            title: use_signal(|| "".to_string()),

            start_date: use_signal(|| timestamp),
            end_date: use_signal(|| timestamp),

            description: use_signal(|| "".to_string()),
            surveys: use_signal(|| vec![]),

            total_survey_types: use_signal(|| {
                vec![
                    translates.dropdown.to_string(),
                    translates.checkbox.to_string(),
                    translates.subjective.to_string(),
                    translates.rating.to_string(),
                ]
            }),
        };

        ctrl
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn change_selected_field(&mut self, field: ProjectArea) {
        self.selected_field.set(Some(field));
    }

    pub fn get_title(&self) -> String {
        (self.title)()
    }

    pub fn change_title(&mut self, title: String) {
        self.title.set(title);
    }

    pub fn get_description(&self) -> String {
        (self.description)()
    }

    pub fn change_description(&mut self, description: String) {
        self.description.set(description);
    }

    pub fn get_start_date(&self) -> i64 {
        (self.start_date)()
    }

    pub fn change_start_date(&mut self, start_date: i64) {
        self.start_date.set(start_date);
    }

    pub fn get_end_date(&self) -> i64 {
        (self.end_date)()
    }

    pub fn change_end_date(&mut self, end_date: i64) {
        self.end_date.set(end_date);
    }

    pub fn get_surveys(&self) -> Vec<Question> {
        (self.surveys)()
    }

    pub fn change_survey(&mut self, index: usize, survey: Question) {
        let mut surveys = (self.surveys)();
        surveys[index] = survey;
        self.surveys.set(surveys);
    }

    pub fn remove_survey(&mut self, index: usize) {
        let mut surveys = (self.surveys)();
        surveys.remove(index);
        self.surveys.set(surveys);
    }

    pub fn add_survey(&mut self) {
        let mut surveys = (self.surveys)();
        surveys.push(Question {
            id: None,
            title: "".to_string(),
            description: None,
            question_type: PublicSurveyQuestionType::Subjective,
            image_url: None,
            answer_start_range: None,
            answer_end_range: None,
            options: None,
            multiple_choice_enable: None,
            necessary_answer_enable: None,
        });
        self.surveys.set(surveys);
    }

    pub async fn save_survey(&self) {}

    pub fn back(&self) {
        self.nav.go_back();
    }
}
