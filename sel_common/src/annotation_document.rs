#[derive(Debug)]
pub struct AnnotationDocument {
    lines: Vec<String>,
}

impl AnnotationDocument {
    pub fn new() -> Self {
        return AnnotationDocument { lines: vec![] };
    }

    pub fn get_lines(&self) -> &Vec<String> {
        return &self.lines;
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }
}
