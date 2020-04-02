#[derive(Clone)]
pub struct Key {
    pub value: String,
}

impl Key {
    pub fn new(value: String) -> Key {
        Key { value: value }
    }
}
