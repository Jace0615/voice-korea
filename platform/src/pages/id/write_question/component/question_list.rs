#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::button::Button,
    pages::id::write_question::controller::{Controller, QuestionStep},
    prelude::Language,
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct QuestionProps {
    survey_id: String,
    lang: Language,
    ctrl: Controller,
    delete: String,
    update: String,
    add_question: String,
    temporary_save: String,
    back: String,
    save: String,
}

pub fn QuestionList(props: QuestionProps) -> Element {
    let mut ctrl = props.ctrl;
    let survey = ctrl.get_survey();
    let survey_height = 170 + survey.questions.len() * 90;
    let navigator = use_navigator();

    let keys: Vec<usize> = ctrl.delete_key_lists();
    let survey_id = props.survey_id.clone();

    rsx! {
        Fragment {
            div {
                class: "flex flex-row w-full justify-end items-end mb-[20px]",
                Button {
                    button_text: props.temporary_save.clone(),
                    onclick: move |_| async move {
                        ctrl.clicked_temporary_save().await;
                        ctrl.clear_data();
                        ctrl.refresh_survey_response();
                    },
                    class: "flex flex-row w-[200px] h-[50px] bg-[#1e5eaf]",
                }
            }
            div {
                class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                div {
                    class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                    "{survey.title}"
                }
            }
            div {
                class: "flex flex-col w-full h-[{survey_height}px] rounded-[10px] bg-white justify-center items-center mb-[30px] pb-[30px]",
                style: "box-shadow: 0 3px 5px -3px rgba(115,115,115,0.75), 3px 0 6px -2px rgba(115,115,115,0.75);",
                Fragment {
                    div {
                        class: "w-full pl-[30px] pr-[40px] pt-[20px] pb-[20px]",
                        for i in 0..survey.questions.len() {
                            if !keys.contains(&i) {
                                div {
                                    class: "flex flex-row w-full h-[90px] rounded-[5px] justify-between items-center pt-[25px] pb-[25px] pr-[30px] pl-[30px]",
                                    style: if i % 2 == 0 {"background-color: #ffffff; box-shadow: 0 3px 5px -3px rgba(115,115,115,0.75), 3px 0 6px -2px rgba(115,115,115,0.75); margin-bottom: 10px;"} else {"background-color: #f9f9f9; box-shadow: 0 3px 5px -3px rgba(115,115,115,0.75), 3px 0 6px -2px rgba(115,115,115,0.75); 1px 0px 0px black; margin-bottom: 10px;"},
                                    div {
                                        class: "font-semibold text-xl text-[#4c4c4c]",
                                        {survey.questions.get(i).map(|q| {
                                           q.title.clone()
                                        }).unwrap_or_default()}
                                    }
                                    div {
                                        class: "flex flex-row",
                                        Button {
                                            button_text: props.delete.clone(),
                                            onclick: move |_| async move {
                                                ctrl.remove_question(i).await;
                                            },
                                            class: "flex flex-row w-[80px] h-[50px] bg-[#424242] mr-[10px]",
                                        }
                                        Button {
                                            button_text: props.update.clone(),
                                            onclick: move |_| {
                                                ctrl.clicked_update_button(i);
                                                ctrl.change_step(QuestionStep::Input);
                                            },
                                            class: "flex flex-row w-[80px] h-[50px] bg-[#2168c3]"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-row w-[200px] h-[50px] rounded-[20px] bg-[#d6d6d6] justify-center items-center",
                    img {
                        class: "flex flex-col pr-[10px]",
                        src: asset!("/public/images/add.png"),
                        alt: "add question",
                    }
                    div {
                        class: "text-[20px] font-medium text-black",
                        onclick: move |_| {
                            ctrl.change_step(QuestionStep::Input);
                        },
                        "{props.add_question}"
                    }
                }
            }
            div {
                class: "flex flex-row w-full justify-end items-end mb-[30px]",
                div {
                    class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                    onclick: move |_| {
                        let survey_id = survey_id.clone();
                        async move {
                            ctrl.clicked_back().await;
                            navigator.push(Route::WriteTitlePage {
                                lang: props.lang.clone(),
                                survey_id,
                            });
                        }
                    },
                    "{props.back}"
                }
                div {
                    class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                    onclick: move |_| {
                        let survey_id = props.survey_id.clone();
                        async move {
                            ctrl.clicked_save().await;
                            navigator.push(Route::SelectResponsePage {
                                lang: props.lang.clone(),
                                survey_id,
                            });
                        }
                    },
                    "{props.save}"
                }
            }
        }
    }
}
