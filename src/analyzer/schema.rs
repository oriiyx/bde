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
    let ast = Parser::parse_sql(&dialect, &content)
        .map_err(|e| anyhow!("Failed to parse Schema SQL with err: {:?}", e))?;
    let mut engine_data = EngineData { tables: vec![] };

    for statement in ast {
        match statement {
            Statement::CreateTable(create_table) => {
                let mut columns: Vec<Column> = vec![];
                for column_def in create_table.columns {
                    columns.push(create_column_data(&column_def));
                }
                let table = Table {
                    name: create_table.name.to_string(),
                    columns,
                };
                engine_data.tables.push(table);
            }
            Statement::AlterTable {
                name, operations, ..
            } => {
                let table_name = name.to_string();
                let table_index = engine_data
                    .tables
                    .iter()
                    .position(|existing_table| existing_table.name.eq(&table_name));

                if let Some(table_index) = table_index {
                    for operation in operations {
                        match operation {
                            AlterTableOperation::AddColumn { column_def, .. } => {
                                // Add the column to the existing table
                                engine_data.tables[table_index]
                                    .columns
                                    .push(create_column_data(&column_def));
                            }
                            _ => {
                                // todo - add other operations like DropColumn, DropConstraint, AddConstraint,...
                                println!("Unhandled alter table operation: {:?}", operation);
                            }
                        }
                    }
                } else {
                    return Err(anyhow!(
                        "Alter table references unknown table: {}",
                        table_name
                    ));
                }
            }
            _ => {
                println!("Found other type: {:?}", statement);
            }
        }
    }

    Ok(engine_data)
}

fn create_column_data(column_def: &ColumnDef) -> Column {
    let mut is_nullable = true;

    for opt in &column_def.options {
        match opt.option {
            ColumnOption::NotNull => is_nullable = false,
            _ => {
                // todo - add options later on when we're defining relations, uniques, etc
            }
        }
    }

    Column {
        name: column_def.name.to_string(),
        data_type: DType {
            php_type: column_def.data_type.to_string(),
            sql_type: column_def.data_type.clone(),
            nullable: is_nullable,
        },
    }
}
