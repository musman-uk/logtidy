/// Carries the original 1-based line number alongside non-empty text so
/// downstream tools can trace entries back to the source even when blank lines are skipped.
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
