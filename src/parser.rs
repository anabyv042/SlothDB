use crate::logical_plan::LogicalPlan;
use crate::scanner::LogicalScanNode;
use crate::errors::ParsingError;

pub fn parse_pipe_sql(input: &str) -> Result<LogicalPlan, ParsingError> {
    let tokens: Vec<&str> = input.trim().split("|>").map(|t| t.trim()).collect();

    if tokens.is_empty() {
        return Err(ParsingError::new("Empty SQL statement"));
    }

    let mut logical_plan = LogicalPlan::new(input.to_string());
    let mut last_table: Option<String> = None;

    for token in tokens {
        if token.starts_with("SELECT") {
            logical_plan.parse_select(token)?;
        } else if token.starts_with("FROM") {
            let table_name = token.trim_start_matches("FROM").trim();
            logical_plan.add_scan(LogicalScanNode::new(table_name));
            last_table = Some(table_name.to_string());
        } else if token.starts_with("WHERE") {
            logical_plan.parse_where(token)?;
        } else if token.starts_with("JOIN") {
            if last_table.is_none() {
                return Err(ParsingError::new("JOIN must follow a FROM clause"));
            }
            logical_plan.parse_join(token, last_table.clone())?;
        } else if token.starts_with("ORDER BY") {
            logical_plan.parse_order_by(token)?;
        } else {
            return Err(ParsingError::new(&format!("Unsupported SQL component: {}", token)));
        }
    }

    Ok(logical_plan)
}
