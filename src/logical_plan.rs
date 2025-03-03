use crate::filter::LogicalFilterNode;
use crate::join::LogicalJoinNode;
use crate::scanner::LogicalScanNode;
use crate::select::LogicalSelectListNode;
use crate::order_by::LogicalOrderByNode;
use crate::errors::ParsingError;

#[derive(Debug)]
pub struct LogicalPlan {
    pub query: String,
    pub filters: Vec<LogicalFilterNode>,
    pub joins: Vec<LogicalJoinNode>,
    pub scans: Vec<LogicalScanNode>,
    pub select_list: Vec<LogicalSelectListNode>,
    pub order_by: Vec<LogicalOrderByNode>,
}

impl LogicalPlan {
    pub fn new(query: String) -> Self {
        Self {
            query,
            filters: Vec::new(),
            joins: Vec::new(),
            scans: Vec::new(),
            select_list: Vec::new(),
            order_by: Vec::new(),
        }
    }

    pub fn add_scan(&mut self, scan: LogicalScanNode) {
        self.scans.push(scan);
    }

    pub fn parse_select(&mut self, token: &str) -> Result<(), ParsingError> {
        let fields: Vec<String> = token
            .trim_start_matches("SELECT")
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        for field in fields {
            self.select_list.push(LogicalSelectListNode::new(&field));
        }

        Ok(())
    }

    pub fn parse_where(&mut self, token: &str) -> Result<(), ParsingError> {
        let condition = token.trim_start_matches("WHERE").trim();
        self.filters.push(LogicalFilterNode::new(condition));
        Ok(())
    }

    pub fn parse_join(&mut self, token: &str, last_table: Option<String>) -> Result<(), ParsingError> {
        let parts: Vec<&str> = token.trim_start_matches("JOIN").trim().split("ON").map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(ParsingError::new("Invalid JOIN syntax. Expected: JOIN table ON condition"));
        }

        let table2 = parts[0].to_string();
        let condition = parts[1].to_string();

        if let Some(table1) = last_table {
            self.joins.push(LogicalJoinNode::new(&table1, &table2, &condition));
        } else {
            return Err(ParsingError::new("JOIN must have a preceding FROM clause"));
        }

        Ok(())
    }

    pub fn parse_order_by(&mut self, token: &str) -> Result<(), ParsingError> {
        let field = token.trim_start_matches("ORDER BY").trim();
        self.order_by.push(LogicalOrderByNode::new(field));
        Ok(())
    }
}
