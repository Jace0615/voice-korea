use crate::utils::context::Language;
pub struct ResetPasswordTranslate {
    pub reset_password: String,
    pub email_address_label: String,
    pub name_label: String,
    pub name_hint: String,
    pub phone_label: String,
    pub phone_hint: String,
    pub send_authentication: String,
    pub authentication_number_label: String,
    pub authentication_number_description: Vec<String>,
    pub check_title: String,
    pub check_description: Vec<String>,

    pub input_new_password_label: String,
    pub input_new_password_check_label: String,
    pub check_new_password_description: Vec<String>,

    pub complete_change_password_title: String,
    pub complete_change_password_description: Vec<String>,
    pub go_to_login: String,
}

pub fn translate(lang: Language) -> ResetPasswordTranslate {
    match lang {
        Language::En => ResetPasswordTranslate {
            reset_password: "Reset Password".to_string(),
            email_address_label: "Email Address".to_string(),
            name_label: "Name".to_string(),
            name_hint: "OOO".to_string(),
            phone_label: "CellPhone".to_string(),
            phone_hint: "010-0000-0000".to_string(),
            send_authentication: "Send Authentication Number".to_string(),
            authentication_number_label: "Authentication Number".to_string(),
            authentication_number_description: vec!["- Please enter the authentication number (6 digits) within 3 minutes.".to_string(), "- If the authentication time has expired, please resend the authentication number and then enter it.".to_string()],
            check_title: "Check it out!".to_string(),
            check_description: vec!["- If you have not received the authentication number properly, please contact customer service (0000-0000).".to_string(), "- If the identity verification process does not proceed properly, please contact XXXX (XXXX-XXXX) for mobile phone identity verification.".to_string(), "- If you have any other questions about membership registration, please contact our customer service center (0000-0000).".to_string()],
            input_new_password_label: "Input New Password".to_string(),
            input_new_password_check_label: "Check New Password".to_string(),
            check_new_password_description: vec!["- The password must consist of at least 8 characters and a combination of letters, numbers, and special symbols.".to_string()],
            complete_change_password_title: "Password change completed".to_string(),
            complete_change_password_description: vec!["Your password change has been completed!".to_string(), "Please log in with a new password.".to_string()],
            go_to_login: "To the login screen".to_string()
        },
        Language::Ko => ResetPasswordTranslate {
            reset_password: "비밀번호 재설정".to_string(),
            email_address_label: "이메일 주소".to_string(),
            name_label: "이름".to_string(),
            name_hint: "OOO".to_string(),
            phone_label: "휴대폰".to_string(),
            phone_hint: "010-0000-0000".to_string(),
            send_authentication: "인증번호 전송".to_string(),
            authentication_number_label: "인증번호".to_string(),
            authentication_number_description: vec!["- 3분 이내로 인증번호(6자리)를 입력해주세요.".to_string(), "- 인증시간이 초과된 경우 인증번호를 재발송 하신 후 입력해 주세요.".to_string()],
            check_title: "확인하세요!".to_string(),
            check_description: vec!["- 인증번호를 정상적으로 받지 못했다면 고객센터(0000-0000)로 문의하여 주시기 바랍니다.".to_string(), "- 본인인증 절차가 정상적으로 이루어지지 않을 경우 휴대폰 본인인증은 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.".to_string(), "- 회원가입에 대한 다른 궁금한 사항은 고객센터(0000-0000)로 문의하여 주시기 바랍니다.".to_string()],
            input_new_password_label: "새로운 비밀번호 입력".to_string(),
            input_new_password_check_label: "새로운 비밀번호 확인".to_string(),
            check_new_password_description: vec!["- 비밀번호는 영문, 숫자, 특수기호 조합으로 8자 이상 구성되어야 합니다.".to_string()],
            complete_change_password_title: "비밀번호 변경 완료".to_string(),
            complete_change_password_description: vec!["비밀번호 변경이 완료 되었습니다!".to_string(), "새로운 비밀번호로 로그인 해 주세요.".to_string()],
            go_to_login: "로그인 화면으로".to_string()
        },
    }
}
