#[derive(Debug)]
pub struct LogicalScanNode {
    pub table_name: String,
}

impl LogicalScanNode {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
        }
    }
}
