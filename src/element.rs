

pub trait Element {
    fn generate(&self, indent: String) -> (String, String); // HTML, CSS
}
