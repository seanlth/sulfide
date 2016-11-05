use element::Element;
use geometry::Unit;
use text::Text;

static mut id_counter: usize = 0;

fn get_new_id() -> usize {
    let id = unsafe { id_counter };
    unsafe { id_counter += 1 };
    return id;
}

#[derive(Clone, PartialEq)]
pub enum ListStyle {
    None,
    Square,
}

pub struct List {
    items: Vec<Text>,
	pub list_style: ListStyle,
    pub padding: Unit,
    id: usize
}

impl List {
    pub fn new() -> List {
        List {
            items: vec![],
            list_style: ListStyle::None,
            padding: Unit::Pixel(0.0),
            id: get_new_id()
        }
    }

    pub fn add_item(&mut self, item: Text) {
        self.items.push(item);
    }
}

impl Element for List {
    fn generate(&self, indent: String) -> (String, String) {
        let mut css: String = format!(".list{} {{\n", self.id);
        if self.list_style != ListStyle::None {
            css = format!("{}    list-style-type: {};\n", css, "square");
        }
        css.push_str("}\n\n");

        let mut html: String = format!("{}<ul class=\"list{}\">\n", indent, self.id);
        for i in &self.items {
            let (h, c) = i.generate( format!("{}    ", indent) );
            html = format!("{}{}    <li style=\"padding-bottom: {}\">\n", html, indent, self.padding.to_string());
            html = format!("{}    {}{}    </li>\n", html, h, indent);
        }
        html = format!("{}{}</ul>\n", html, indent);

        (html, css)
    }
}
