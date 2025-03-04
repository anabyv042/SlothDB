#[derive(Debug)]
pub struct LogicalSelectListNode {
    pub field: String,
}

impl LogicalSelectListNode {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}
