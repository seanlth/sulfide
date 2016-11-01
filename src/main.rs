#[macro_use(Pr)]
#[macro_use(Px)]
#[macro_use(Clc)]

extern crate sulfide;
use sulfide::element::Element;
use sulfide::document::Document;
use sulfide::container::Container;
use sulfide::geometry::{Unit, Rect, Point};

use std::fs::File;
use std::io::Write;


fn main() {


    let mut index = Document::new();
    let mut root = Container::new(Pr!(100.0), Pr!(100.0), Pr!(0.0), Px!(0.0));
    root.set_colour("#F0F0F0");
    let mut main = Container::new(Pr!(63.0), Pr!(100.0), Pr!(18.5), Px!(0.0));
    main.set_colour("#F0F0F0");

    let mut top_bar = Container::new(Pr!(100.0), Px!(50.0), Pr!(0.0), Pr!(100.0) - Px!(50.0) - Px!(20.0));
    top_bar.set_colour("#7FAB8B");

    main.add(top_bar);
    root.add(main);
    index.set_root(root);


    let (html, css) = index.generate("".to_string());

    println!("{}\n{}", html, css);

    let mut index_file = File::create("/Users/Seanlth/Desktop/test/index.html").expect("Unable to create file");
    index_file.write_all(html.as_bytes()).expect("Unable to write data");

    let mut css_file = File::create("/Users/Seanlth/Desktop/test/style.css").expect("Unable to create file");
    css_file.write_all(css.as_bytes()).expect("Unable to write data");
}
