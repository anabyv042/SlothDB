#[derive(Debug)]
pub struct LogicalJoinNode {
    pub table1: String,
    pub table2: String,
    pub condition: String,
}

impl LogicalJoinNode {
    pub fn new(table1: &str, table2: &str, condition: &str) -> Self {
        Self {
            table1: table1.to_string(),
            table2: table2.to_string(),
            condition: condition.to_string(),
        }
    }
}
