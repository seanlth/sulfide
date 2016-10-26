extern crate sulfide;
use sulfide::element::Element;
use sulfide::document::Document;
use sulfide::geometry::{Rect, Point};

fn main() {
    let index = Document::new(Rect::new(1024.0, 1000.0, Point::new(0.0, 0.0)) );

    println!("{}", index.generate());
}
