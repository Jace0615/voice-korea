#![allow(non_snake_case)]
use crate::{components::button::Button, prelude::*};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectResponseProps {
    lang: Language,
    title: String,
}

pub mod controller;
pub mod i18n;

#[component]
pub fn SelectResponsePage(props: SelectResponseProps) -> Element {
    let mut ctrl = controller::Controller::init(props.title.clone());
    let survey = ctrl.get_survey();
    let question_list = survey.questions.len() as u64;
    let translates = i18n::translate(props.lang.clone());
    let question_list_info_first = translates.question_list_info_first.clone();
    let question_list_info_second = translates.question_list_info_second.clone();
    let question_list_info = question_list_info_first
        + question_list.to_string().as_str()
        + question_list_info_second.as_str();

    const RESPONSE_ATTRIBUTE_IMAGE: &str = "../../images/select-response-attribute.png";
    const RESPONSE_PANEL_IMAGE: &str = "../../images/select-response-panel.png";

    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[55px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-end items-end mb-[20px]",
                    Button {
                        button_text: translates.temporary_save,
                        onclick: move |_| {},
                        class: "flex flex-row w-[200px] h-[50px] bg-[#1e5eaf]",
                    }
                }
                div {
                    class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                    div {
                        class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                        "{survey.survey.title}"
                    }
                }
                div {
                    class: "flex flex-col w-full h-[110px] rounded-[10px] bg-white mb-[10px] justify-center items-start",
                    div {
                        class: "text-black font-semibold text-[22px] pl-[30px] mb-[10px]",
                        "{translates.question_list}"
                    }
                    div {
                        class: "text-[#5e5e5e] font-normal text-[22px] pl-[30px]",
                        "{question_list_info}",
                    }
                }
                div {
                    class: "flex flex-col w-full h-[420px] rounded-[10px] bg-white mb-[10px] justify-between items-start py-[30px] px-[30px]",
                    div {
                        class: "flex flex-col w-full justify-start items-start",
                        div {
                            class: "text-black font-semibold text-[22px] mb-[10px]",
                            "{translates.collect_response_title}"
                        }
                        div {
                            class: "text-[#5e5e5e] font-normal text-[22px] mb-[10px]",
                            "{translates.collect_response_description}"
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-start items-start",
                        div {
                            class: "flex flex-1 rounded-xl h-[240px] mr-[10px]",
                            style: "background-image: url('{RESPONSE_ATTRIBUTE_IMAGE}');",
                            div {
                                class: "flex flex-col w-full h-full justify-center items-center",
                                div {
                                    class: "text-[28px] font-semibold text-white mb-[20px]",
                                    "{translates.select_response_attribute_title}"
                                }
                                div {
                                    class: "flex flex-row w-full justify-center items-center text-[16px] font-normal text-white whitespace-pre-line content-center px-[10px]",
                                    "{translates.select_response_attribute_description}"
                                }
                            }
                        }
                        div {
                            class: "flex flex-1 rounded-xl h-[240px]",
                            style: "background-image: url('{RESPONSE_PANEL_IMAGE}');",
                            div {
                                class: "flex flex-col w-full h-full justify-center items-center",
                                div {
                                    class: "text-[28px] font-semibold text-white mb-[20px]",
                                    "{translates.select_response_panel_title}"
                                }
                                div {
                                    class: "flex flex-row w-full justify-center items-center text-[16px] font-normal text-white whitespace-pre-line content-center px-[10px]",
                                    "{translates.select_response_panel_description}"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-row w-full justify-end items-end mb-[20px]",
                    Button {
                        button_text: translates.back,
                        onclick: move |_| {},
                        class: "flex flex-row w-[200px] h-[50px] bg-[#434343]",
                    }
                }
            }
        }
    }
}