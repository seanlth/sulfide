use geometry::{Rect, Point, Unit};
use element::Element;




//  ___________________________________
// |                                   |
// |          ________________         |
// |         |                |        |
// |         |                |        |
// |         |                |        |
// |         |                |        |
// |         |                |        |
// |         |________________|        |
// |                                   |
// |___________________________________|
//


enum Origin {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
}

static mut id_counter: usize = 0;

fn get_new_id() -> usize {
    let id = unsafe { id_counter };
    unsafe { id_counter += 1 };
    return id;
}

pub struct Container {
    pub rect: Rect,
    left: bool,
    colour: String,
    opacity: f64,
    children: Vec<Box<Element>>,
    pub scrolling: bool,
    id: usize,
}

impl Container {
    pub fn new(width: Unit, height: Unit, x: Unit, y: Unit) -> Container {
        Container {
            rect: Rect::new( width, height, Point::new(x, y) ),
            left: true,
            colour: "".to_string(),
            opacity: 1.0,
            children: vec![],
            scrolling: false,
            id: get_new_id()
        }
    }

    // until I come up with a better system
    pub fn new_rb(width: Unit, height: Unit, x: Unit, y: Unit) -> Container {
        Container {
            rect: Rect::new( width, height, Point::new(x, y) ),
            left: false,
            colour: "".to_string(),
            opacity: 1.0,
            children: vec![],
            scrolling: false,
            id: get_new_id()
        }
    }

    pub fn set_colour(&mut self, colour: &str) {
        self.colour = colour.to_string();
    }
    pub fn set_opacity(&mut self, opacity: f64) {
        self.opacity = opacity;
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
        match self.scrolling {
            true => css.push_str("    overflow: scroll;\n"),
            false => css.push_str("    overflow: hidden;\n"),
        }
        css = format!("{}    opacity: {};\n", css, self.opacity);
        css = format!("{}    background: {};\n", css, self.colour);
        if self.rect.height != Unit::Default {
            css = format!("{}    height: {};\n", css, self.rect.height.to_string());
        }
        if self.rect.width != Unit::Default {
            css = format!("{}    width: {};\n", css, self.rect.width.to_string());
        }
        if self.left == true {
            css = format!("{}    left: {};\n", css, self.rect.position.x.to_string());
        }
        else {
            css = format!("{}    right: {};\n", css, self.rect.position.x.to_string());
        }
        css = format!("{}    bottom: {};\n", css, self.rect.position.y.to_string());
        css.push_str("}\n\n");

        let mut html: String = format!("{}<div class=\"div{}\">\n", indent, self.id);

        for child in &self.children {
            let (h, c) = child.generate(format!("{}    ", indent));
            css = format!("{}{}", css, c);
            html = format!("{}{}", html, h);
        }

        html = format!("{}{}</div>\n", html, indent);


        (html, css)
    }
}
