#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{AddUser, ArrowLeft, ArrowRight, RowOption, Search, Switch},
        label::Label,
    },
    prelude::Language,
    routes::Route,
    service::popup_service::PopupService,
};

pub mod _id;
mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct MemberPageProps {
    lang: Language,
}

#[derive(Clone, PartialEq)]
pub enum ModalType {
    None,
    AddMember,
    RemoveMember(String),
}

#[component]
pub fn MemberPage(props: MemberPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang);
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    let mut modal_type = use_signal(|| ModalType::None);
    let translates = i18n::translate(props.lang.clone());

    let member_summary = ctrl.get_members();
    let groups = ctrl.get_groups();
    let roles = ctrl.get_roles();

    let mut clicked_member_id = use_signal(|| "".to_string());

    let mut popup: PopupService = use_context();

    if let ModalType::RemoveMember(_member_id) = modal_type() {
        popup.open(
            "팀원 삭제".to_string(),
            rsx! {
                RemoveMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                        clicked_member_id.set("".to_string());
                    },
                }
            },
        );
    } else if modal_type() == ModalType::AddMember {
        popup.open(
            "팀원 추가하기".to_string(),
            rsx! {
                AddMemberModal {
                    groups: groups.clone(),
                    roles: roles.clone(),
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                }
            },
        );
    } else {
        popup.close();
    }

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.team_member_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.team_member_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "{translates.team_member_description}"
            }
            div { class: "flex flex-row w-full justify-start items-start mb-[10px]",
                MemberCountCard {
                    label_name: translates.total,
                    label_count: member_summary.role_counts.get(0).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.manager,
                    label_count: member_summary.role_counts.get(1).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.public_opinion_manager,
                    label_count: member_summary.role_counts.get(2).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.analyst,
                    label_count: member_summary.role_counts.get(3).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.repeater,
                    label_count: member_summary.role_counts.get(4).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.lecturer,
                    label_count: member_summary.role_counts.get(5).unwrap_or(&0).clone(),
                }
            }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
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
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            onfocus: move |_| {
                                if !popup.is_opened() {
                                    modal_type.set(ModalType::None);
                                }
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div {
                            class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                            onclick: move |_| {
                                modal_type.set(ModalType::AddMember);
                            },
                            AddUser { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px] ",
                                "{translates.add_team_member}"
                            }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.role}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.project}"
                            }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for member in member_summary.members {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    Link {
                                        to: Route::MemberDetailPage {
                                            lang: props.lang.clone(),
                                            member_id: member.member_id.clone(),
                                        },
                                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                                            div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                            div { class: "flex flex-col justify-start items-start",
                                                div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                                    {member.profile_name}
                                                }
                                                div { class: "text-[14px] font-normal text-[#7c8292]",
                                                    {member.email}
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: member.group,
                                            //TODO: update member group
                                            onchange: |_evt| {},
                                            for group in groups.clone() {
                                                option {
                                                    value: group.clone(),
                                                    selected: group == member.group,
                                                    "{group}"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: member.role,
                                            //TODO: update member role
                                            onchange: |_evt| {},
                                            for role in roles.clone() {
                                                option {
                                                    value: role.clone(),
                                                    selected: role == member.role,
                                                    "{role}"
                                                }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                                        if member.projects.len() > 0 {
                                            Label {
                                                label_name: member.projects[0].clone(),
                                                label_color: "bg-[#35343f]",
                                            }
                                        }
                                    }
                                    div { class: "p-4",
                                        div { class: "group relative",
                                            button {
                                                onclick: move |_| {
                                                    clicked_member_id.set(member.member_id.clone());
                                                },
                                                RowOption { width: 24, height: 24 }
                                            }
                                            nav {
                                                tabindex: "0",
                                                class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute left-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                ul { class: "py-1",
                                                    li {
                                                        class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                        onclick: move |_| {
                                                            modal_type.set(ModalType::RemoveMember(clicked_member_id()));
                                                        },
                                                        "팀원 삭제하기"
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
                //페이지네이션
                div { class: "flex flex-row w-full justify-center items-center",
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
                    div { class: "ml-[12px] w-[24px] h-[24px]",
                        ArrowRight { width: "24", height: "24" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RemoveMemberModal(onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-col text-[#3a3a3a] font-normal text-[14px] gap-[5px]",
                div { "정말 삭제하시겠습니까?" }
                div {
                    "삭제된 팀원은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요."
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", "삭제하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn AddMemberModal(
    groups: Vec<String>,
    roles: Vec<String>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut email_focused = use_signal(|| false);

    let mut name = use_signal(|| "".to_string());
    let mut name_focused = use_signal(|| false);

    let mut select_role = use_signal(|| "".to_string());
    let mut select_group = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[60px]",
            div { class: "flex flex-row w-full mb-[16px]",
                div { class: "text-[#eb5757] font-semibold text-[14px] mr-[5px]", "*[필수]" }
                div { class: "text-[#3a3a3a] font-semibold text-[14px]",
                    "이메일 주소 입력하기"
                }
            }
            div {
                class: format!(
                    "flex flex-row w-full h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                    if (email_focused)() {
                        "bg-[#ffffff] border border-[#2a60d3]"
                    } else {
                        "bg-[#f7f7f7]"
                    },
                ),
                input {
                    class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                    r#type: "text",
                    placeholder: "이메일 주소 입력",
                    value: (email)(),
                    onfocus: move |_| {
                        email_focused.set(true);
                    },
                    onblur: move |_| {
                        email_focused.set(false);
                    },
                    oninput: move |event| {
                        email.set(event.value());
                    },
                }
            }
            div { class: "font-normal text-[#6f6f6f] text-[13px] mt-[5px] mb-[40px]",
                "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요."
            }
            div { class: "flex flex-col w-full justify-start itmes-start",
                div { class: "font-medium text-[15px] text-[#3a3a3a] mb-[16px]", "개인정보" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#eb5757] font-medium text-[15px] mr-[3px]",
                                "*"
                            }
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "이름"
                            }
                        }
                        div {
                            class: format!(
                                "flex flex-row w-full h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                                if (name_focused)() {
                                    "bg-[#ffffff] border border-[#2a60d3]"
                                } else {
                                    "bg-[#f7f7f7]"
                                },
                            ),
                            input {
                                class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: "필수 입력",
                                value: (name)(),
                                onfocus: move |_| {
                                    name_focused.set(true);
                                },
                                onblur: move |_| {
                                    name_focused.set(false);
                                },
                                oninput: move |event| {
                                    name.set(event.value());
                                },
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[60px]",
                            "역할"
                        }
                        select {
                            class: "focus:outline-none w-full h-[45px] bg-[#f7f7f7] rounded-lg px-[5px] text-[#9b9b9b]",
                            value: select_role(),
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            option {
                                value: "",
                                disabled: true,
                                selected: select_role() == "",
                                hidden: select_role() != "",
                                "역할 선택"
                            }
                            for role in roles.clone() {
                                option {
                                    value: role.clone(),
                                    selected: role == select_role(),
                                    "{role}"
                                }
                            }
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[60px]",
                            "그룹"
                        }
                        select {
                            class: "focus:outline-none w-full h-[45px] bg-[#f7f7f7] rounded-lg px-[5px] text-[#9b9b9b]",
                            value: select_group(),
                            //TODO: update member group
                            onchange: move |evt| {
                                select_group.set(evt.value());
                            },
                            option {
                                value: "",
                                disabled: true,
                                selected: select_group() == "",
                                hidden: select_group() != "",
                                "그룹 선택"
                            }
                            for group in groups.clone() {
                                option {
                                    value: group.clone(),
                                    selected: group == select_group(),
                                    "{group}"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-medium text-[15px] text-[#3a3a3a] mb-[16px]", "프로젝트 초대" }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "공론"
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#3a3a3a] font-medium text-[15px] mr-[3px] w-[40px]",
                                "조사"
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[120px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                    onclick: move |_| {},
                    AddUser { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", "초대하기" }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#3a3a3a] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "취소하기"
                }
            }
        }
    }
}

#[component]
pub fn MemberCountCard(label_name: String, label_count: u64) -> Element {
    rsx! {
        div { class: "flex flex-col w-[85px] h-[96px] justify-center items-center py-[18px] mr-[10px] bg-white rounded-lg",
            div { class: "font-semibold text-[#35343f] text-[15px] mb-[17px]", "{label_name}" }
            div { class: "font-bold text-[#435393] text-[24px]", "{label_count}" }
        }
    }
}
