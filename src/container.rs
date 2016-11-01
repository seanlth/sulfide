use geometry::{Rect, Point, Unit};
use element::Element;

static mut id_counter: usize = 0;

fn get_new_id() -> usize {
    let id = unsafe { id_counter };
    unsafe { id_counter += 1 };
    return id;
}

pub struct Container {
    pub rect: Rect,
    padding_left: Unit,
    padding_right: Unit,
    padding_top: Unit,
    padding_bottom: Unit,
    colour: String,
    children: Vec<Box<Element>>,
    id: usize
}

impl Container {
    pub fn new(width: Unit, height: Unit, x: Unit, y: Unit) -> Container {
        Container {
            rect: Rect::new( width, height, Point::new(x, y) ),
            colour: "#000000".to_string(),
            children: vec![],
            id: get_new_id()
        }
    }

    pub fn set_colour(&mut self, colour: &str) {
        self.colour = colour.to_string();
    }

    pub fn add<T: Element + 'static>(&mut self, element: T) {
        self.children.push( Box::new(element) )
    }
}

impl Element for Container {
    fn generate(&self, indent: String) -> (String, String) {
        let mut css: String = format!(".div{} {{\n", self.id);
        css.push_str("    position: absolute;\n");
        css.push_str("    display: inline-block;\n");
        css = format!("{}    background: {};\n", css, self.colour);
        css = format!("{}    height: {};\n", css, self.rect.height.to_string());
        css = format!("{}    width: {};\n", css, self.rect.width.to_string());
        css = format!("{}    left: {};\n", css, self.rect.position.x.to_string());
        css = format!("{}    bottom: {};\n", css, self.rect.position.y.to_string());
        css.push_str("}\n\n");

        let mut html: String = format!("{}<div class=\"div{}\">\n", indent, self.id);

        for child in &self.children {
            let (h, c) = child.generate(format!("{}  ", indent));
            css = format!("{}{}", css, c);
            html = format!("{}{}", html, h);
        }

        html = format!("{}{}</div>\n", html, indent);


        (html, css)
    }
}
