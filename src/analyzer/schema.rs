use crate::configuration::Settings;
use sqlparser::ast::*;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::fs;

pub fn analyze_schema(config: Settings) {
    let schema_dir = &config.sql.schemas_location;
    println!("Analyzing schema files in {}", schema_dir);

    match fs::read_dir(schema_dir) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(entry) => {
                        let path = entry.path();

                        if let Some(extension) = path.extension() {
                            if extension.to_string_lossy().to_lowercase() == "sql" {
                                println!("Processing SQL file: {}", path.display());

                                match fs::read_to_string(&path) {
                                    Ok(content) => {
                                        process_sql_file(content);
                                    }
                                    Err(e) => {
                                        eprintln!("Error reading file {}: {:?}", path.display(), e)
                                    }
                                }
                            } else {
                                println!("Skipping non SQL file: {}", path.display());
                            }
                        } else {
                            println!("Skipping file with no extension: {}", path.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry {}: {:?}", schema_dir, e)
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading schema directory {}: {:?}", schema_dir, e)
        }
    }
}

pub struct EngineData {
    pub tables: Vec<Table>,
}

pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

pub struct DataType {
    pub sql_type: String,
    pub php_type: String,
    pub size: Option<usize>,
}

pub fn process_sql_file(content: String) -> EngineData {
    let dialect = MySqlDialect {};
    let ast = Parser::parse_sql(&dialect, &content);
    let mut engine_data = EngineData { tables: vec![] };

    for alloc in ast {
        for statement in alloc {
            match statement {
                Statement::CreateTable(create_table) => {
                    let table = Table {
                        name: create_table.name.to_string(),
                        columns: vec![],
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
                } => {}
                _ => {
                    println!("Found other type: {:?}", statement);
                }
            }
        }
    }

    engine_data
}
