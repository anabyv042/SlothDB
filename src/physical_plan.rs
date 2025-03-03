use crate::logical_plan::LogicalPlan;

#[derive(Debug)]
pub struct PhysicalPlan {
    pub query: String,
    pub operations: Vec<String>,
}

pub fn convert_to_physical_plan(logical_plan: &LogicalPlan) -> PhysicalPlan {
    let mut operations = Vec::new();

    for scan in &logical_plan.scans {
        operations.push(format!("SCAN TABLE {}", scan.table_name));
    }

    for join in &logical_plan.joins {
        operations.push(format!(
            "JOIN {} ON {} = {}",
            join.table2, join.condition, join.table1
        ));
    }

    for filter in &logical_plan.filters {
        operations.push(format!("FILTER WHERE {}", filter.condition));
    }

    for select in &logical_plan.select_list {
        operations.push(format!("PROJECT {}", select.field));
    }

    for order_by in &logical_plan.order_by {
        operations.push(format!("ORDER BY {}", order_by.field));
    }

    PhysicalPlan {
        query: logical_plan.query.clone(),
        operations,
    }
}
