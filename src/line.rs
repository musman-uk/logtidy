#[derive(Debug)]
pub struct Line {
    pub number: usize,
    pub text: String,
}

impl Line {
    pub fn new(number: usize, text: String) -> Self {
        Self { number, text }
    }
}
