
use text::Text;

enum ListStyle {
    None,
    Square,
}

struct List {
    items: Vec<Text>,
	list_style: ListStyle,
    padding: Unit
}

impl List {
    
}
