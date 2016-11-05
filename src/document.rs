use element::Element;
use geometry::Rect;
use container::Container;



pub struct Document {
    pub root: Option<Container>
}

impl Document {
    pub fn new() -> Document {
        Document {
            root: None
        }
    }

    pub fn set_root(&mut self, root: Container) {
        self.root = Some(root);
    }

//    fn get_width

}

impl Element for Document {
    fn generate(&self, _: String) -> (String, String) {
        let (h, c) = match &self.root {
            &Some(ref r) => r.generate("        ".to_string()),
            &None => ("".to_string(), "".to_string())
        };
        let css = c;

        let mut html = "<!DOCTYPE html>\n".to_string();
        html.push_str("<html>\n");
        html.push_str("    <head>\n");
        html.push_str("        <meta content=\"text/html;charset=utf-8\" http-equiv=\"Content-Type\">\n");
        html.push_str("        <meta content=\"utf-8\" http-equiv=\"encoding\">\n");
        html.push_str("        <link rel=\"stylesheet\" href=\"reset.css\" type=\"text/css\">\n");
        html.push_str("        <link rel=\"stylesheet\" href=\"style.css\" type=\"text/css\">\n");
        html.push_str("    </head>\n");
        html.push_str("    <body>\n");
        html = format!("{}{}", html, h);
        html.push_str("    </body>\n");
        html.push_str("</html>\n");
        (html, css)
    }
}
