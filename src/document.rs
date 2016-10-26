use element::Element;
use geometry::Rect;
use container::Container;



pub struct Document {
    child: Container
}

impl Document {
    pub fn new(rect: Rect) -> Document {
        Document {
            child: Container::new(rect)
        }
    }


}

impl Element for Document {
    fn generate(&self) -> (String, String) {
        let mut css = "".to_string();

        let mut doc = "<!DOCTYPE html>\n".to_string();
        doc.push_str("<html>\n");
        doc.push_str("<head>\n");
        doc.push_str("</head>\n");
        doc.push_str("<body>\n");
        let (h, c) = self.child.generate();
        doc = format!("{}{}", doc, h);
        doc.push_str("</body>\n");
        doc.push_str("</html>\n");
        (doc, "".to_string())
    }
}
