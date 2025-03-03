#[derive(Debug)]
pub struct LogicalOrderByNode {
    pub field: String,
}

impl LogicalOrderByNode {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}
