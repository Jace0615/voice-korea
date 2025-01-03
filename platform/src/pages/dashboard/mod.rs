#![allow(non_snake_case)]
use crate::prelude::*;
use dashboard_card::DashboardCard;
use dashboard_row::DashboardRow;
use dioxus::prelude::*;
use models::prelude::{Survey, SurveyStatus};

pub mod controller;
pub mod dashboard_card;
pub mod dashboard_row;
pub mod i18n;

#[derive(PartialEq, Props, Clone)]
pub struct DashboardPageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct DashboardCardTypeProps {
    lang: Language,
    surveys: Vec<Survey>,
    draft: String,
    in_progress: String,
    complete: String,
    update_date: String,
    add_question_description: String,
    response: String,
    edit_survey: String,
    analysis_result: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct DashboardListTypeProps {
    lang: Language,
    surveys: Vec<Survey>,
    survey_name: String,
    response_count: String,
    final_update_date: String,
    status: String,
    draft: String,
    in_progress: String,
    complete: String,
    add_question_description: String,
    edit_survey: String,
    analysis_result: String,
}

#[component]
pub fn DashboardPage(props: DashboardPageProps) -> Element {
    let mut ctrl = controller::Controller::init(props.lang);
    let translates = i18n::translate(props.lang.clone());

    #[cfg(feature = "web")]
    use_effect(move || {
        let runtime = Runtime::current().expect("Get runtime failed");
        use wasm_bindgen::JsCast;
        let handler =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let _guard = RuntimeGuard::new(runtime.clone());
                // FIXME: sometimes key code returns undefined from JS which causes panic in Dioxus.
                //        it seems like a bug in Dioxus.
                let window = web_sys::window().expect("no windows exists");
                let document = window.document().expect("window has no `document`");
                if let Some(active_element) = document.active_element() {
                    if active_element.tag_name().to_lowercase() != "body" {
                        return;
                    }
                };
                if event.code() == "Slash" {
                    event.prevent_default();
                    if let Some(document) = web_sys::window().unwrap().document() {
                        if let Some(input) = document.get_element_by_id("search-input") {
                            input
                                .dyn_ref::<web_sys::HtmlInputElement>()
                                .unwrap()
                                .focus()
                                .unwrap();
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
            .expect("failed to add event listener");
        handler.forget();
    });

    rsx! {
        div {
            class: "flex flex-col w-full h-full pt-[45px] pr-[45px] pl-[35px] items-start justify-start",
            div {
                class: "flex flex-row w-full items-start justify-between",
                div {
                    class: "flex flex-row mr-[10px]",
                    div {
                        class: "flex flex-row w-[450px] h-[55px] items-center bg-white border-solid border border-[#e0e0e0] rounded-[8px] pl-[15px] mr-[10px]",
                        img {
                            class: "mr-[5px]",
                            src: asset!("/public/images/search.png"),
                            width: 24,
                            height: 24
                        }
                        input {
                            id: "search-input",
                            class: "flex flex-1 text-[21px] text-[#8a8a8a] font-normal",
                            "type": "text",
                            style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none",
                            placeholder: "{translates.search}",
                            onkeydown: move |e: Event<KeyboardData>| async move {
                                if e.code() == Code::Enter && ctrl.get_search_text() != "" {
                                    ctrl.search_text().await;
                                }
                            },
                            oninput: move |e| ctrl.change_search_text(e.value()),
                            min: 0,
                        }
                    }
                    div {
                        class: "flex flex-row rounded-tl-[8px] rounded-bl-[8px] w-[55px] h-[55px] bg-[#4c4c4c] items-center justify-center mr-[2px]",
                        onclick: move |_| ctrl.set_clicked_type(0),
                        img {
                            src: asset!("/public/images/menu_1.png"),
                            width: 27,
                            height: 27
                        }
                    }
                    div {
                        class: "flex flex-row rounded-tr-[8px] rounded-br-[8px] w-[55px] h-[55px] bg-[#c8c8c8] items-center justify-center",
                        onclick: move |_| ctrl.set_clicked_type(1),
                        img {
                            src: asset!("/public/images/menu_2.png"),
                            width: 27,
                            height: 27
                        }
                    }
                }
                div {
                    class: "flex flex-row w-[200px] h-[50px] justify-end items-end bg-[#2168c3] rounded-[8px]",
                    onclick: move |_| async move {
                        ctrl.clicked_create_survey(props.lang.clone()).await;
                    },
                    div {
                        class: "flex flex-row w-full h-full justify-center items-center text-[21px] font-semibold text-white",
                        "{translates.create_survey}"
                    }
                }
            }
            if ctrl.get_clicked_type() == 0 {
                DashboardCardTypes {
                    lang: props.lang,
                    surveys: ctrl.get_total_surveys(),
                    draft: translates.draft,
                    in_progress: translates.in_progress,
                    complete: translates.complete,
                    update_date: translates.update_date,
                    add_question_description: translates.add_question_description,
                    response: translates.response,
                    edit_survey: translates.edit_survey,
                    analysis_result: translates.analysis_result,
                }
            } else {
                DashboardListTypes {
                    lang: props.lang,
                    surveys: ctrl.get_total_surveys(),
                    survey_name: translates.survey_name,
                    response_count: translates.response_count,
                    final_update_date: translates.final_update_date,
                    status: translates.status,
                    draft: translates.draft,
                    in_progress: translates.in_progress,
                    complete: translates.complete,
                    add_question_description: translates.add_question_description,
                    edit_survey: translates.edit_survey,
                    analysis_result: translates.analysis_result,
                }
            }
        }
    }
}

#[component]
pub fn StatusButton(
    survey_status: SurveyStatus,
    draft_label: String,
    in_progress_label: String,
    complete_label: String,
) -> Element {
    let survey_type_label: String;
    let label_bg_color: String;
    let label_text_color: String;

    if survey_status == SurveyStatus::Draft {
        survey_type_label = draft_label;
        label_bg_color = "bg-[#e5e5e5]".to_string();
        label_text_color = "text-black".to_string();
    } else if survey_status == SurveyStatus::Finished {
        survey_type_label = complete_label;
        label_bg_color = "bg-[#2168c3]".to_string();
        label_text_color = "text-white".to_string();
    } else {
        survey_type_label = in_progress_label;
        label_bg_color = "bg-[#3a94ff]".to_string();
        label_text_color = "text-white".to_string();
    }

    rsx! {
        div {
            class: "flex flex-row w-[65px] h-[30px] rounded-[5px] justify-center items-center {label_bg_color} {label_text_color} mb-[25px]",
            {survey_type_label}
        }
    }
}

pub fn DashboardCardTypes(props: DashboardCardTypeProps) -> Element {
    let surveys = props.surveys;
    rsx! {
        div {
            class: "flex flex-wrap w-full h-full justify-center items-start pt-[35px]",
            for survey in surveys.iter() {
                DashboardCard {
                    lang: props.lang,
                    survey_id: survey.id.clone(),
                    survey_status: survey.status.clone(),
                    survey_draft_status: survey.draft_status.clone(),
                    title: survey.title.clone(),
                    updated_at: survey.updated_at.clone(),
                    response_count: survey.response_count.unwrap_or_default(),
                    total_response_count: survey.total_response_count.unwrap_or_default(),
                    draft_label: props.draft.clone(),
                    in_progress_label: props.in_progress.clone(),
                    complete_label: props.complete.clone(),
                    update_date_label: props.update_date.clone(),
                    add_question_description: props.add_question_description.clone(),
                    response: props.response.clone(),
                    edit_survey: props.edit_survey.clone(),
                    analysis_result: props.analysis_result.clone(),
                }
            }
        }
    }
}

pub fn DashboardListTypes(props: DashboardListTypeProps) -> Element {
    let surveys = props.surveys;
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start pt-[35px] min-w-[1375px]",
            div {
                class: "flex flex-row w-full justify-between items-between pt-[25px] pb-[20px] px-[30px]",
                div {
                    class: "text-[#696969] font-normal text-[20px] pl-[45px] min-w-[500px]",
                    "{props.survey_name}"
                },
                div {
                    class: "flex flex-row justify-start items-start",
                    div {
                        class: "flex flex-row items-center justify-center w-[80px] text-[#696969] font-normal text-[20px] mr-[50px]",
                        "{props.response_count}"
                    }
                    div {
                        class: "flex flex-row items-center justify-center w-[210px] text-[#696969] font-normal text-[20px] mr-[50px]",
                        "{props.final_update_date}"
                    }
                    div {
                        class: "flex flex-row items-center justify-center w-[110px] text-[#696969] font-normal text-[20px] mr-[50px]",
                        "{props.status}"
                    }
                    div {
                        class: "w-[265px]"
                    }
                }
            }
            div {
                class: "flex flex-col w-full h-full justify-start items-start",
                for survey in surveys.iter() {
                    DashboardRow {
                        lang: props.lang,
                        survey_id: survey.id.clone(),
                        survey_status: survey.status.clone(),
                        survey_draft_status: survey.draft_status.clone(),
                        title: survey.title.clone(),
                        updated_at: survey.updated_at,
                        response_count: survey.response_count.unwrap_or_default(),
                        total_response_count: survey.total_response_count.unwrap_or_default(),
                        draft_label: props.draft.clone(),
                        in_progress_label: props.in_progress.clone(),
                        complete_label: props.complete.clone(),
                        add_question_description: props.add_question_description.clone(),
                        edit_survey: props.edit_survey.clone(),
                        analysis_result: props.analysis_result.clone(),
                    }
                }
            }
        }
    }
}
