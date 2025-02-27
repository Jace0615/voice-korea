use crate::utils::context::Language;

pub struct CreateTranslate {
    pub authorization: String,
    pub individual: String,
    pub company: String,
    pub individual_description: String,
    pub phone: String,
    pub phone_description: String,
    pub check_title: String,
    pub check_description_1: String,
    pub check_description_2: String,
    pub check_description_3: String,
    pub company_name: String,
    pub business_register_number: String,
    pub company_name_example: String,
    pub business_register_number_example: String,
    pub next: String,
    pub agree_terms: String,
    pub agree_membership_terms: String,
    pub agree_privacy_policy: String,
    pub entrust_personal_information: String,
    pub essential: String,
    pub join_the_membership: String,
    pub email_address: String,
    pub send_authentication: String,
    pub authentication_number: String,
    pub authentication_description_1: String,
    pub authentication_description_2: String,
    pub password: String,
    pub password_check: String,
    pub company_info: String,
    pub company_example: String,
    pub name_info: String,
    pub name_example: String,
    pub phone_info: String,
    pub phone_example: String,
    pub address_info: String,
    pub search_address: String,
    pub check_membership_description_1: String,
    pub check_membership_description_2: String,
    pub check_membership_description_3: String,
    pub complete_join_membership: String,
    pub complete_join_membership_info: String,
    pub complete: String,
    pub company_name_info: String,

    pub incollect_email_form: String,
    pub input_password: String,
    pub incollect_two_password: String,
    pub already_exists_user: String,
    pub incollect_authentication_number: String,
    pub failed_store_data: String,
    pub invalid_password_pattern: String,
}

pub fn translate(lang: Language) -> CreateTranslate {
    match lang {
        Language::En => CreateTranslate {
            authorization: "Identity Verification".to_string(),
            individual: "Individual".to_string(),
            company: "Corporation".to_string(),
            individual_description: "This is the identity verification step for membership registration. If you are under 14 years of age, you must participate with your parents (legal representative). Please prepare your authentication method in advance.".to_string(),
            phone: "Cellphone".to_string(),
            phone_description: "Authentication by receiving a verification number sent to a mobile phone in your name".to_string(),
            check_title: "Check it out!".to_string(),
            check_description_1: "- Identity verification is only possible using a mobile phone activated in your name.".to_string(),
            check_description_2: "- If the identity verification process does not work properly, please contact XXXX (XXXX-XXXX) for mobile phone identity verification.".to_string(),
            check_description_3: "- If you have any other questions about membership registration, please contact our customer service center (0000-0000).".to_string(),
            company_name: "Company Name".to_string(),
            business_register_number: "Register Number".to_string(),
            company_name_example: "Biyard Co".to_string(),
            business_register_number_example: "000-00-00000".to_string(),
            next: "Next".to_string(),
            agree_terms: "Agree to Terms and Conditions".to_string(),
            agree_membership_terms: "Agree to membership terms and conditions".to_string(),
            agree_privacy_policy: "Privacy policy".to_string(),
            entrust_personal_information: "Entrustment of personal information processing".to_string(),
            essential: "(Essential)".to_string(),
            join_the_membership: "Join the Membership".to_string(),
            email_address: "Email Address".to_string(),
            send_authentication: "Send Authentication Number".to_string(),
            authentication_number: "Authentication Number".to_string(),
            authentication_description_1: "- Please enter the authentication number (6 digits) within 3 minutes.".to_string(),
            authentication_description_2: "- If the authentication time has expired, please resend the authentication number and then enter it.".to_string(),
            password: "Input Password".to_string(),
            password_check: "Check Password".to_string(),
            company_info: "Company Information".to_string(),
            company_example: "Biyard co".to_string(),
            name_info: "Name".to_string(),
            name_example: "OOO".to_string(),
            phone_info: "Cellphone".to_string(),
            phone_example: "000-0000-0000".to_string(),
            address_info: "Address".to_string(),
            search_address: "Search Address".to_string(),
            check_membership_description_1: "- If you are signing up as a corporation, please use the main email address.".to_string(),
            check_membership_description_2: "- If you do not see the email in your inbox, please check your spam.".to_string(),
            check_membership_description_3: "- If you have not received the email properly, please contact customer service at 0000-0000.".to_string(),
            complete_join_membership: "Membership registration completed".to_string(),
            complete_join_membership_info: "Congratulations on completing your membership registration.".to_string(),
            complete: "Complete".to_string(),
            company_name_info: "Corporation Name".to_string(),

            incollect_email_form: "The email format is incorrect.".to_string(),
            input_password: "Please enter your password.".to_string(),
            incollect_two_password: "The two passwords do not match.".to_string(),
            already_exists_user: "This user already exists.".to_string(),
            incollect_authentication_number: "The authentication number does not match.".to_string(),
            failed_store_data: "Failed to save data. Please try again.".to_string(),
            invalid_password_pattern: "Please make up at least 8 characters using a combination of letters, numbers, and special symbols.".to_string()
        },
        Language::Ko => CreateTranslate {
            authorization: "본인인증".to_string(),
            individual: "개인".to_string(),
            company: "법인".to_string(),
            individual_description: "회원가입을 위한 본인확인 단계입니다. 만14세 미만인 경우 부모님(법정대리인)과 함께 진행하셔야 합니다. 인증수단을 미리 준비해주세요.".to_string(),
            phone: "휴대폰".to_string(),
            phone_description: "본인 명의로 된 휴대폰으로 인증번호를 전송 받아 인증".to_string(),
            check_title: "확인하세요!".to_string(),
            check_description_1: "- 본인인증은 본인명의로 개통된 휴대폰으로만 가능합니다.".to_string(),
            check_description_2: "- 본인인증 절차가 정상적으로 이루어지지 않을 경우 휴대폰 본인인증은 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.".to_string(),
            check_description_3: "- 회원가입에 대한 다른 궁금한 사항은 고객센터(0000-0000)로 문의하여 주시기 바랍니다.".to_string(),
            company_name: "회사명".to_string(),
            business_register_number: "사업자 등록번호".to_string(),
            company_name_example: "(주)바이야드".to_string(),
            business_register_number_example: "000-00-00000".to_string(),
            next: "다음".to_string(),
            agree_terms: "약관 동의".to_string(),
            agree_membership_terms: "회원약관 동의".to_string(),
            agree_privacy_policy: "개인정보처리방침".to_string(),
            entrust_personal_information: "개인정보처리의 위탁".to_string(),
            essential: "(필수)".to_string(),
            join_the_membership: "회원가입".to_string(),
            email_address: "이메일 주소".to_string(),
            send_authentication: "인증번호 전송".to_string(),
            authentication_number: "인증번호".to_string(),
            authentication_description_1: "- 3분 이내로 인증번호(6자리)를 입력해 주세요.".to_string(),
            authentication_description_2: "- 인증시간이 초과된 경우 인증번호를 재발송 하신 후 입력해 주세요.".to_string(),
            password: "비밀번호 입력".to_string(),
            password_check: "비밀번호 확인".to_string(),
            company_info: "회사정보".to_string(),
            company_example: "주식회사 바이야드".to_string(),
            name_info: "이름".to_string(),
            name_example: "OOO".to_string(),
            phone_info: "휴대폰".to_string(),
            phone_example: "000-0000-0000".to_string(),
            address_info: "주소".to_string(),
            search_address: "주소검색".to_string(),
            check_membership_description_1: "- 법인으로 가입하시는 경우 대표메일을 이용해주세요.".to_string(),
            check_membership_description_2: "- 받은 편지함에 이메일이 보이지 않는다면, 스팸함을 확인해주세요.".to_string(),
            check_membership_description_3: "- 이메일이 정상적으로 받지 못했다면 고객센터(0000-0000)로 문의하여 주시기 바랍니다.".to_string(),
            complete_join_membership: "회원가입 완료".to_string(),
            complete_join_membership_info: "회원가입 완료를 축하합니다.".to_string(),
            complete: "완료".to_string(),
            company_name_info: "법인명".to_string(),

            incollect_email_form: "이메일 형식이 올바르지 않습니다.".to_string(),
            input_password: "패스워드를 입력해주세요.".to_string(),
            incollect_two_password: "두 개의 비밀번호가 일치하지 않습니다.".to_string(),
            already_exists_user: "이미 존재하는 유저입니다.".to_string(),
            incollect_authentication_number: "인증번호가 일치하지 않습니다.".to_string(),
            failed_store_data: "데이터 저장에 실패했습니다. 다시 시도해보세요.".to_string(),
            invalid_password_pattern: "영문, 숫자, 특수기호 조합으로 8자 이상 구성해주세요.".to_string()
        },
    }
}
