use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::PublicSurveyStatus;

use crate::{
    components::icons::{ArrowLeft, RowOption, Search, Switch},
    pages::surveys::{
        controller::Controller,
        i18n::{RemoveSurveyModalTranslate, SurveyTranslate},
    },
    service::popup_service::PopupService,
};

#[derive(Props, Clone, PartialEq)]
pub struct SurveyProps {
    lang: Language,
}

#[component]
pub fn SurveyPage(props: SurveyProps) -> Element {
    let popup: PopupService = use_context();
    let ctrl = Controller::new(props.lang, popup);
    let translate: SurveyTranslate = translate(&props.lang);

    let mut is_focused = use_signal(|| false);
    let mut project_name = use_signal(|| "".to_string());

    let surveys = ctrl.get_surveys();
    let survey_len = surveys.len();

    let mut clicked_panel_index = use_signal(|| 0);

    use_effect(use_reactive(&survey_len, move |len| {
        clicked_panel_index.set(len);
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]", "{translate.survey_title}" }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translate.survey_title}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translate.survey_description}"
            }

            div { class: "flex flex-col w-full justify-start items-start mb-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
                    style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                    div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                        div {
                            class: format!(
                                "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                                if (is_focused)() {
                                    "bg-[#ffffff] border border-[#2a60d3]"
                                } else {
                                    "bg-[#f7f7f7] border border-[#7c8292]"
                                },
                            ),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: "{translate.search_hint}",
                                value: (project_name)(),
                                onfocus: move |_| {
                                    is_focused.set(true);
                                },
                                onblur: move |_| {
                                    is_focused.set(false);
                                },
                                oninput: move |event| {
                                    project_name.set(event.value());
                                },
                            }
                            Search { width: "18", height: "18", color: "#7c8292" }
                        }
                        button { onclick: move |_| {},
                            div { class: "flex flex-row justify-center items-center px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px]",
                                div { class: "text-white font-semibold text-[#16px]",
                                    "{translate.start_survey}"
                                }
                            }
                        }
                    }

                    //project table
                    div { class: "flex flex-col w-full jsutify-start items-start border rounded-lg border-[#bfc8d9]",
                        div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                            div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_type}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-[120px] min-w-[150px] h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_field}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_project}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_response_rate}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_panel}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_period}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_status}"
                                }
                                Switch { width: "19", height: "19" }
                            }
                            div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                div { class: "text-[#7c8292] font-semibold text-[14px]",
                                    "{translate.survey_view}"
                                }
                            }
                            div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                        }

                        for (index , survey) in surveys.clone().iter().enumerate() {
                            div { class: "flex flex-col w-full justify-start items-start",
                                div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                                div { class: "flex flex-row w-full h-[55px]",
                                    div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            {ctrl.translate_survey_type(props.lang, survey.survey_type.clone())}
                                        }
                                    }
                                    div { class: "flex flex-row w-[150px] min-w-[150px] h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            {ctrl.translate_survey_field(props.lang, survey.survey_field_type.clone())}
                                        }
                                    }
                                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            "{survey.title}"
                                        }
                                    }
                                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            {
                                                format!(
                                                    "{}% ({}/{})",
                                                    survey.survey_response * 100 / survey.total_response,
                                                    survey.survey_response,
                                                    survey.total_response,
                                                )
                                            }
                                        }
                                    }
                                    button {
                                        class: "flex flex-row flex-1 h-full justify-center items-center",
                                        onclick: move |_| {
                                            clicked_panel_index.set(index);
                                        },
                                        if survey.panels.clone().len() != 0 {
                                            if clicked_panel_index() == index {
                                                div { class: "flex flex-wrap w-full justify-center items-center gap-[5px]",
                                                    for panel in survey.panels.clone() {
                                                        PanelLabel {
                                                            label: panel.name.clone(),
                                                            background_color: if survey.status == PublicSurveyStatus::Ready { "#35343f".to_string() } else { "#b4b4b4".to_string() },
                                                        }
                                                    }
                                                }
                                            } else {
                                                PanelLabel {
                                                    label: survey.panels[0].name.clone(),
                                                    background_color: if survey.status == PublicSurveyStatus::Ready { "#35343f".to_string() } else { "#b4b4b4".to_string() },
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            {
                                                format!(
                                                    "{} ~ {}",
                                                    ctrl.convert_timestamp_to_date(survey.start_date),
                                                    ctrl.convert_timestamp_to_date(survey.end_date),
                                                )
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                        div { class: "text-[#35343f] font-semibold text-[14px]",
                                            {ctrl.translate_survey_status(props.lang, survey.status.clone())}
                                        }
                                    }
                                    div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                        {
                                            match survey.status {
                                                PublicSurveyStatus::Finish => {
                                                    rsx! {
                                                        button { class: "text-[#2a60d3] font-semibold text-[14px]", "{translate.view_results}" }
                                                    }
                                                }
                                                _ => {
                                                    rsx! {
                                                        button { class: "text-[#2a60d3] font-semibold text-[14px]", "{translate.detail_more}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "group relative",
                                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                            if survey.status == PublicSurveyStatus::Ready {
                                                button {
                                                    RowOption {
                                                        width: "24",
                                                        height: "24",
                                                    }
                                                }
                                                nav { class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                    ul { class: "py-1",
                                                        li {
                                                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                            onclick: move |_| {},
                                                            "{translate.update_survey}"
                                                        }
                                                        li {
                                                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                            onclick: {
                                                                let lang = props.lang.clone();
                                                                let id = survey.id.clone();
                                                                move |_| {
                                                                    let id = id.clone();
                                                                    async move {
                                                                        ctrl.open_remove_survey_modal(lang, id).await;
                                                                    }
                                                                }
                                                            },
                                                            "{translate.remove_survey}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //pagenation
                    div { class: "flex flex-row w-full justify-center items-center mt-[30px]",
                        div { class: "mr-[20px] w-[24px] h-[24px]",
                            ArrowLeft { width: "24", height: "24" }
                        }
                        //FIXME: add pagination by variable(page, index)
                        for i in 0..10 {
                            if i == 0 {
                                div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
                                    "{i + 1}"
                                }
                            } else {
                                div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
                                    "{i + 1}"
                                }
                            }
                        }
                        div { class: "flex flex-row ml-[12px] w-[60px] h-[40px] justify-center items-center font-bold text-[15px] text-[#0d1732]",
                            "More"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RemoveSurveyModal(
    lang: Language,
    onclose: EventHandler<MouseEvent>,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    let i18n: RemoveSurveyModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { "{i18n.remove_info}" }
                div { "{i18n.remove_warning}" }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onremove.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", "{i18n.remove}" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{i18n.cancel}"
                }
            }
        }
    }
}

#[component]
pub fn PanelLabel(label: String, background_color: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-center items-center px-[8px] py-[3px] rounded-[100px] font-semibold text-[14px] text-white",
            style: format!("background-color: {}", background_color),
            {label}
        }
    }
}
