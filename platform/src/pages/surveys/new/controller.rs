#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {}

impl Controller {
    pub fn new(_lang: dioxus_translate::Language) -> Self {
        let ctrl = Self {};
        ctrl
    }
}
