use sqlparser::ast::{
    Delete, Expr, FromTable, ObjectName, ObjectNamePart, Select, SetExpr, Statement, TableFactor,
    TableWithJoins,
};

// Define your own intermediate representation
#[derive(Debug)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
    Other,
}

#[derive(Debug)]
pub struct QueryInfo {
    name: String,            // From the "-- name: X" comment
    return_type: String,     // From the ":one", ":many", ":exec" annotation
    query_type: QueryType,   // The type of SQL query
    target_table: String,    // The table being queried
    parameters: Vec<String>, // Any parameters like ":id"
}

pub fn extract_query_info(name_comment: &str, statement: &Statement) -> Result<QueryInfo, String> {
    // Parse the name comment
    let parts: Vec<&str> = name_comment.split_whitespace().collect();
    if parts.len() < 3 || parts[0] != "--" || parts[1] != "name:" {
        return Err("Invalid comment format".to_string());
    }

    let name = parts[2].to_string();
    let return_type = if parts.len() > 3 {
        parts[3].trim_start_matches(':').to_string()
    } else {
        "".to_string()
    };

    match statement {
        Statement::Query(query) => {
            // Handle SELECT queries
            if let SetExpr::Select(select) = query.body.as_ref() {
                let target_table = extract_table_name_from_select(select);
                let parameters = extract_parameters_from_select(select);

                Ok(QueryInfo {
                    name,
                    return_type,
                    query_type: QueryType::Select,
                    target_table,
                    parameters,
                })
            } else {
                Err("Unsupported query body type".to_string())
            }
        }
        Statement::Delete(delete) => {
            // Handle DELETE queries - extract target table(s)
            let target_table = if !delete.tables.is_empty() {
                // Take the first table name for simplicity
                format_object_name(&delete.tables[0])
            } else {
                // Extract from the FROM clause if tables is empty
                match &delete.from {
                    FromTable::WithFromKeyword(tables) | FromTable::WithoutKeyword(tables)
                        if !tables.is_empty() =>
                    {
                        extract_table_name_from_table_with_joins(&tables[0])
                    }
                    _ => "unknown".to_string(),
                }
            };

            // Extract parameters from the WHERE clause
            let mut parameters = Vec::new();
            if let Some(where_clause) = &delete.selection {
                extract_parameters_from_expr(where_clause, &mut parameters);
            }

            Ok(QueryInfo {
                name,
                return_type,
                query_type: QueryType::Delete,
                target_table,
                parameters,
            })
        }
        // Handle other statement types as needed
        _ => Err("Unsupported statement type".to_string()),
    }
}

// Helper function to format an ObjectName
fn format_object_name(name: &ObjectName) -> String {
    name.0
        .iter()
        .map(|part| match part {
            ObjectNamePart::Identifier(ident) => ident.value.clone(),
        })
        .collect::<Vec<_>>()
        .join(".")
}

// Helper function to extract table name from TableWithJoins
fn extract_table_name_from_table_with_joins(table_with_joins: &TableWithJoins) -> String {
    match &table_with_joins.relation {
        TableFactor::Table { name, .. } => format_object_name(name),
        // Handle other types of TableFactor if needed
        _ => "unknown".to_string(),
    }
}

// Helper functions to extract specific information
fn extract_table_name_from_select(select: &Select) -> String {
    if !select.from.is_empty() {
        if let TableFactor::Table { name, .. } = &select.from[0].relation {
            return name.to_string();
        }
    }
    "unknown".to_string()
}

fn extract_parameters_from_select(select: &Select) -> Vec<String> {
    let mut parameters = Vec::new();

    // Extract parameters from WHERE clause
    if let Some(where_clause) = &select.selection {
        extract_parameters_from_expr(where_clause, &mut parameters);
    }

    parameters
}

fn extract_parameters_from_delete(delete: &Delete) -> Vec<String> {
    let mut parameters = Vec::new();

    // Extract parameters from WHERE clause
    if let Some(where_clause) = &delete.selection {
        extract_parameters_from_expr(where_clause, &mut parameters);
    }

    parameters
}

fn extract_parameters_from_expr(expr: &Expr, parameters: &mut Vec<String>) {
    match expr {
        Expr::BinaryOp { left, right, .. } => {
            extract_parameters_from_expr(left, parameters);
            extract_parameters_from_expr(right, parameters);
        }
        Expr::Identifier(ident) => {
            if ident.value.starts_with(':') {
                parameters.push(ident.value.clone());
            }
        }
        // Add more cases as needed
        _ => {}
    }
}
