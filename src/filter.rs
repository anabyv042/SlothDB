#[derive(Debug)]
pub struct LogicalFilterNode {
    pub condition: String,
}

impl LogicalFilterNode {
    pub fn new(condition: &str) -> Self {
        Self {
            condition: condition.to_string(),
        }
    }
}
