use anyhow::{Result, anyhow};
use sqlparser::ast::Statement;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser as SqlParser;
use std::fs;
use std::path::Path;

pub struct SqlFileParser {
    dialect: MySqlDialect,
}

impl Default for SqlFileParser {
    fn default() -> Self {
        Self {
            dialect: MySqlDialect {},
        }
    }
}

impl SqlFileParser {
    /// Reads and parses SQL files from a directory
    pub fn parse_directory(&self, dir_path: &str) -> Result<Vec<SqlFile>> {
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
                    let sql_file = self.parse_file(&path)?;
                    sql_files.push(sql_file);
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
    pub fn parse_file(&self, path: &Path) -> Result<SqlFile> {
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow!("Error reading file {}: {:?}", path.display(), e))?;

        let statements = self.parse_sql(&content)?;

        Ok(SqlFile {
            path: path.to_string_lossy().to_string(),
            statements,
        })
    }

    /// Parses a SQL string into AST statements
    pub fn parse_sql(&self, sql: &str) -> Result<Vec<Statement>> {
        SqlParser::parse_sql(&self.dialect, sql)
            .map_err(|e| anyhow!("Failed to parse SQL with error: {:?}", e))
    }
}

/// Represents a parsed SQL file
pub struct SqlFile {
    pub path: String,
    pub statements: Vec<Statement>,
}
