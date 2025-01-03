use dioxus::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct LoginService {
    pub email: Signal<Option<String>>,
    // pub token: Signal<String>,
}

impl LoginService {
    pub fn init() {
        let srv = LoginService {
            email: use_signal(|| None),
            // token: use_signal(|| "".to_string()),
        };

        use_context_provider(|| srv);
    }

    #[cfg(feature = "web")]
    pub fn set_cookie(&self, value: &str) {
        use dioxus_logger::tracing;
        use wasm_bindgen::JsCast;
        use web_sys::window;
        let doc = window().unwrap().document().unwrap();
        let html_document = doc.dyn_into::<web_sys::HtmlDocument>().unwrap();

        let token = self.get_cookie_value().unwrap_or_default();
        if token != "" {
            let cookie_str = format!("token=; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Path=/;",);

            // Set the cookie to delete it
            html_document
                .set_cookie(&cookie_str)
                .expect("Failed to delete cookie");
        }

        let cookie_str = format!("token={}; SameSite=Strict; Path=/; Max-Age=3600", value);
        match html_document.set_cookie(&cookie_str) {
            Ok(_) => {
                tracing::debug!("Cookie successfully set: {}", cookie_str);
            }
            Err(e) => {
                tracing::debug!("Failed to set cookie: {:?}", e);
            }
        }
    }

    #[cfg(feature = "web")]
    pub fn get_cookie_value(&self) -> Option<String> {
        use wasm_bindgen::JsCast;
        use web_sys::window;
        // Get the browser's `document` object
        let doc = window().unwrap().document().unwrap();

        let html_document = doc.dyn_into::<web_sys::HtmlDocument>().unwrap();

        let cookies = html_document.cookie().ok()?;

        cookies.split(';').map(|s| s.trim()).find_map(|cookie| {
            let mut parts = cookie.splitn(2, '=');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key == "token" {
                Some(value.to_string())
            } else {
                None
            }
        })
    }

    pub fn get_email(&self) -> String {
        match (self.email)() {
            Some(email) => email,
            None => "".to_string(),
        }
    }

    #[allow(unused_variables)]
    pub fn setup(&mut self, email: String, token: String) {
        self.email.set(Some(email));
        // self.token.set(token);

        #[cfg(feature = "web")]
        self.set_cookie(token.as_str());
    }
}

pub fn use_login_service() -> LoginService {
    use_context()
}
