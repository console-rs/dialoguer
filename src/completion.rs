/// Trait for completion handling.
pub trait Completion: Send {
    fn next(&mut self, input: &str, completion_modified: bool) -> Option<String>;
}
