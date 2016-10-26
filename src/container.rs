use geometry::Rect;
use element::Element;

static mut id_counter: usize = 0;

fn get_new_id() -> usize {
    let id = unsafe { id_counter };
    unsafe { id_counter += 1 };
    return id;
}

pub struct Container {
    rect: Rect,
    children: Vec<Box<Element>>,
    id: usize
}

impl Container {
    pub fn new(rect: Rect) -> Container {
        Container {
            rect: rect,
            children: vec![],
            id: get_new_id()
        }
    }

    pub fn add(&mut self, element: Box<Element>) {
        self.children.push(element)
    }
}

impl Element for Container {
    fn generate(&self) -> (String, String) {
        let mut css: String = format!(".div{} {{", self.id);


        let mut html: String = format!("<div class=\"div{}\">", self.id);
        for child in &self.children {
            let (h, c) = child.generate();
            css = format!("{}{}", css, c);
            html = format!("{}{}", html, h);
        }
        css.push_str("}}");
        html.push_str("</div>");


        (html, "".to_string())
    }
}
