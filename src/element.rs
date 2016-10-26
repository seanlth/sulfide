

pub trait Element {
    fn generate(&self) -> (String, String); // HTML, CSS
}
