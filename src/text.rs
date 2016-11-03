
use element::Element;

pub enum TextType {
    Link(String), // target
    Normal
}

pub enum TextDecoration {
    Underline,
    Normal
}

pub struct Text {
    pub text: String,
    pub font: String,
    pub size: usize,
    pub text_type: TextType,
    pub text_decoration: TextDecoration
}

impl Text {
    pub fn new(text: &str) -> Text {
        Text {
            text: text.to_string(),
            font: "".to_string(),
            size: 12,
            text_type: TextType::Normal,
            text_decoration: TextDecoration::Normal
        }
    }
}

impl Element for Text {
    fn generate(&self, indent: String) -> (String, String) {
        let html = format!("{}{}", indent, self.text);
        let css = "".to_string();
        (html, css)
    }
}
