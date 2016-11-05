
use element::Element;

static mut id_counter: usize = 0;

fn get_new_id() -> usize {
    let id = unsafe { id_counter };
    unsafe { id_counter += 1 };
    return id;
}

#[derive(Clone, PartialEq)]
pub enum TextType {
    Link(String), // target
    Normal
}

#[derive(Clone, PartialEq)]
pub enum TextDecoration {
    Underline,
    Normal
}

pub struct Text {
    pub text: String,
    pub font: String,
    pub size: usize,
    pub text_type: TextType,
    pub text_decoration: TextDecoration,
    id: usize
}

impl Text {
    pub fn new(text: &str) -> Text {
        Text {
            text: text.to_string(),
            font: "".to_string(),
            size: 12,
            text_type: TextType::Normal,
            text_decoration: TextDecoration::Normal,
            id: get_new_id()
        }
    }

    fn needs_css(&self) -> bool {
        if self.font != "".to_string() {
            true
        }
        else if self.size != 12 {
            true
        }
        else if self.text_type != TextType::Normal {
            true
        }
        else if self.text_decoration != TextDecoration::Normal {
            true
        }
        else {
            false
        }
    }
}

impl Element for Text {
    fn generate(&self, indent: String) -> (String, String) {
        let mut html = String::new();
        let mut css = String::new();

        if self.needs_css() {
            // match self.text_type {
            //     TextType::Normal => {},
            //     TextType::Link(t) => { }
            // }
            html = format!("{}{}<div class=\"text{}\">{}</div>\n", indent, html, self.id, self.text);

            css = format!(".text{} {{\n", self.id);
            css = format!("{}    font-size: {}px;\n", css, self.size);
            css = format!("{}    font-family: {};\n", css, self.font);

            if self.text_decoration != TextDecoration::Normal {
                css = format!("{}    text-decoration: {};\n", css, "underline");
            }

            css.push_str("}\n\n");
        }
        else {
            html = format!("{}{}{}\n", indent, html, self.text);
        }


        (html, css)
    }
}
