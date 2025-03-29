use crate::analyzer::map_sql_to_php_data_type;
use crate::analyzer::types::{Column, DType, EngineData, Table};
use crate::configuration::Settings;
use crate::parser::SqlFileParser;
use anyhow::{Result, anyhow};
use sqlparser::ast::*;

pub struct SchemaAnalyzer;

impl SchemaAnalyzer {
    pub fn analyze_schema(config: &Settings) -> Result<EngineData> {
        let schema_dir = &config.sql.schemas;
        println!("Analyzing schema files in {}", schema_dir);

        // Use the parser to parse SQL files
        let parser = SqlFileParser::default();
        let sql_files = parser.parse_directory(schema_dir)?;

        let mut engine_data = EngineData { tables: vec![] };

        // Analyze each SQL file
        for sql_file in sql_files {
            Self::analyze_sql_statements(&mut engine_data, sql_file.statements)?;
        }

        Ok(engine_data)
    }

    fn analyze_sql_statements(
        engine_data: &mut EngineData,
        statements: Vec<Statement>,
    ) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::CreateTable(create_table) => {
                    let mut columns: Vec<Column> = vec![];
                    for column_def in create_table.columns {
                        columns.push(Self::analyze_column_def(&column_def));
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
                                        .push(Self::analyze_column_def(&column_def));
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

        Ok(())
    }

    fn analyze_column_def(column_def: &ColumnDef) -> Column {
        let mut is_nullable = true;

        for opt in &column_def.options {
            match opt.option {
                ColumnOption::NotNull => is_nullable = false,
                ColumnOption::Unique { is_primary, .. } => {
                    if is_primary {
                        is_nullable = false
                    }
                }
                _ => {
                    // todo - add options later on when we're defining relations, uniques, etc
                }
            }
        }

        Column {
            name: column_def.name.to_string(),
            data_type: DType {
                php_type: map_sql_to_php_data_type(&column_def.data_type, is_nullable),
                sql_type: column_def.data_type.clone(),
                nullable: is_nullable,
            },
        }
    }
}

pub fn process_sql_file(content: String) -> Result<EngineData> {
    let parser = SqlFileParser::default();
    let statements = parser.parse_sql(&content)?;

    let mut engine_data = EngineData { tables: vec![] };
    SchemaAnalyzer::analyze_sql_statements(&mut engine_data, statements)?;

    Ok(engine_data)
}
