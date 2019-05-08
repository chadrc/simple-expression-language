#[derive(Debug)]
pub struct Annotation {
    name: String,
}

impl Annotation {
    pub fn new(name: String) -> Self {
        return Annotation { name };
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}
