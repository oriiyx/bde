use crate::analyzer::types::{Column, DType, EngineData, Table};
use crate::configuration::Settings;
use anyhow::{Result, anyhow};
use sqlparser::ast::*;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::fs;

pub fn analyze_schema(config: Settings) -> Result<EngineData> {
    let schema_dir = &config.sql.schemas_location;
    println!("Analyzing schema files in {}", schema_dir);

    let entries = fs::read_dir(schema_dir)
        .map_err(|e| anyhow!("Error reading schema directory {}: {:?}", schema_dir, e))?;

    for entry_result in entries {
        let entry = entry_result
            .map_err(|e| anyhow!("Error reading directory entry {}: {:?}", schema_dir, e))?;

        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension.to_string_lossy().to_lowercase() == "sql" {
                println!("Processing SQL file: {}", path.display());

                let content = fs::read_to_string(&path)
                    .map_err(|e| anyhow!("Error reading file {}: {:?}", path.display(), e))?;

                return process_sql_file(content);
            } else {
                println!("Skipping non SQL file: {}", path.display());
            }
        } else {
            println!("Skipping file with no extension: {}", path.display());
        }
    }

    // If no SQL files were found
    Err(anyhow!("No SQL files found in {}", schema_dir))
}

pub fn process_sql_file(content: String) -> Result<EngineData> {
    let dialect = MySqlDialect {};
    let ast = Parser::parse_sql(&dialect, &content);
    let mut engine_data = EngineData { tables: vec![] };

    for alloc in ast {
        for statement in alloc {
            match statement {
                Statement::CreateTable(create_table) => {
                    let mut columns: Vec<Column> = vec![];
                    for column in create_table.columns {
                        let mut is_nullable = true;

                        for opt in column.options {
                            match opt.option {
                                ColumnOption::NotNull => is_nullable = false,
                                _ => {
                                    // todo - add options later on when we're defining relations, uniques, etc
                                }
                            }
                        }

                        let col_entry = Column {
                            name: column.name.to_string(),
                            data_type: DType {
                                php_type: column.data_type.to_string(),
                                sql_type: column.data_type,
                                nullable: is_nullable,
                            },
                        };

                        columns.push(col_entry);
                    }
                    let table = Table {
                        name: create_table.name.to_string(),
                        columns,
                    };
                    engine_data.tables.push(table);
                }
                Statement::AlterTable {
                    name,
                    if_exists,
                    only,
                    operations,
                    location,
                    on_cluster,
                } => {
                    let existing_table: Vec<&Table> = engine_data
                        .tables
                        .iter()
                        .filter(|existing_table| existing_table.name.eq(&name.to_string()))
                        .collect();

                    if existing_table.len() != 1 {
                        return Err(anyhow!(
                            "Alter table should find 1 existing table but found: {}",
                            existing_table.len()
                        ));
                    }
                }
                _ => {
                    println!("Found other type: {:?}", statement);
                }
            }
        }
    }

    Ok(engine_data)
}
