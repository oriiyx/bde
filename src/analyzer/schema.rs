use crate::configuration::Settings;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::fs;
use std::path::Path;

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
                                process_sql_file(&path)
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

fn process_sql_file(path: &Path) {
    println!("Processing SQL file: {}", path.display());

    match fs::read_to_string(path) {
        Ok(content) => {
            let dialect = MySqlDialect {};
            let ast = Parser::parse_sql(&dialect, &content);
            println!("{:?}", ast);
        }
        Err(e) => {
            eprintln!("Error reading file {}: {:?}", path.display(), e)
        }
    }
}
