use SetExpr::Select;
use anyhow::{Result, anyhow};
use sqlparser::ast::{SetExpr, Statement};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser as SqlParser;
use std::fs;

pub struct QuerySqlFileParser {
    dialect: MySqlDialect,
}

impl Default for QuerySqlFileParser {
    fn default() -> Self {
        Self {
            dialect: MySqlDialect {},
        }
    }
}

impl QuerySqlFileParser {
    /// Reads and parses SQL files from a directory
    pub fn parse_directory(&self, dir_path: &str) -> Result<Vec<QuerySqlFile>> {
        println!("Parsing SQL files in {}", dir_path);
        let entries = fs::read_dir(dir_path)
            .map_err(|e| anyhow!("Error reading directory {}: {:?}", dir_path, e))?;

        let mut sql_files = Vec::new();

        for entry_result in entries {
            let entry = entry_result
                .map_err(|e| anyhow!("Error reading directory entry {}: {:?}", dir_path, e))?;

            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension.to_string_lossy().to_lowercase() == "sql" {
                    println!("Processing SQL file: {}", path.display());
                    // let sql_file = self.parse_file(&path)?;
                    // sql_files.push(sql_file);
                } else {
                    println!("Skipping non-SQL file: {}", path.display());
                }
            } else {
                println!("Skipping file with no extension: {}", path.display());
            }
        }

        if sql_files.is_empty() {
            return Err(anyhow!("No SQL files found in {}", dir_path));
        }

        Ok(sql_files)
    }

    /// Parses a single SQL file
    // pub fn parse_file(&self, path: &Path) -> Result<QuerySqlFile> {
    //     let content = fs::read_to_string(path)
    //         .map_err(|e| anyhow!("Error reading file {}: {:?}", path.display(), e))?;
    //
    //     let statements = self.parse_sql(&content)?;
    //
    //     Ok(QuerySqlFile {
    //         path: path.to_string_lossy().to_string(),
    //         queries: vec![QueriesMap { statements }],
    //     })
    // }

    pub fn divide_content_into_queries(&self, sql: &str) -> Result<Vec<String>> {
        let mut queries = Vec::new();
        let mut current_query = String::new();
        let mut in_query = false;

        for line in sql.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("-- name:") {
                if in_query && !current_query.trim().is_empty() {
                    queries.push(current_query.trim().to_string());
                    current_query = String::new();
                }

                current_query.push_str(line);
                current_query.push('\n');
                in_query = true;
            } else if in_query {
                current_query.push_str(line);
                current_query.push('\n');
            }
        }

        if in_query && !current_query.trim().is_empty() {
            queries.push(current_query.trim().to_string());
        }

        Ok(queries)
    }

    /// Parses a SQL string into AST statements
    pub fn parse_sql(&self, queries: &Vec<String>) -> Result<Vec<Vec<Statement>>> {
        let mut statements = Vec::new();
        for sql in queries {
            let parsed_statement = SqlParser::parse_sql(&self.dialect, sql.as_str())
                .map_err(|e| anyhow!("Failed to parse SQL with error: {:?}", e))?;

            statements.push(parsed_statement)
        }
        Ok(statements)
    }
}

/// Represents a parsed SQL file
pub struct QuerySqlFile {
    pub path: String,
    pub queries: Vec<QueriesMap>,
}

pub struct QueriesMap {
    pub statements: Vec<Statement>,
}

pub fn debug_statement_structure(statement: &Statement) {
    // println!("Full statement: {:#?}", statement);

    match statement {
        Statement::Query(query) => {
            if let Select(select) = &*query.body {
                println!("Select statement: {:#?}", select);
                if let Some(selection) = &select.selection {
                    println!("WHERE clause: {:#?}", selection);
                } else {
                    println!("No WHERE clause found in SELECT");
                }
            }
        }
        Statement::Update { selection, .. } => {
            if let Some(where_clause) = selection {
                println!("WHERE clause in UPDATE: {:#?}", where_clause);
            } else {
                println!("No WHERE clause found in UPDATE");
            }
        }
        _ => println!("Not a statement type with WHERE clause"),
    }
}
