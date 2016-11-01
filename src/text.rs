
pub enum TextType {
    Link(String), // target
    Normal
}

pub enum TextDecoration {
    Underline,
    Normal
}

pub struct Text {
    text: String,
    font: String,
    size: usize,
    text_type: TextType,
    text_decoration: TextDecoration
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
