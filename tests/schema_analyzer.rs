#[cfg(test)]
mod tests {
    use bde::analyzer::PhpType::Int;
    use bde::analyzer::process_sql_file;
    use bde::configuration::get_configuration;
    use bde::parser::SqlFileParser;
    use std::fs;
    use std::ops::Add;
    use std::path::Path;

    #[test]
    fn test_schema_analyzer() {
        // Check if configuration file exists
        assert_eq!(Path::new("bde.yaml").exists(), true);

        // Try to parse the configuration
        let configuration = get_configuration().expect("Failed to parse configuration");

        let p = configuration.sql.schemas.add("/schema.sql");
        let path = Path::new(&p);
        let content = fs::read_to_string(&path).unwrap();
        let tables = process_sql_file(content).unwrap().tables;
        assert!(0 < tables.len());

        // Check number of columns for user table
        let user_table = tables.first().unwrap();
        assert_eq!(user_table.columns.len(), 5);
    }

    #[test]
    fn test_sql_file_parser() {
        // Check if configuration file exists
        assert_eq!(Path::new("bde.yaml").exists(), true);

        // Try to parse the configuration
        let configuration = get_configuration().expect("Failed to parse configuration");

        let p = configuration.sql.schemas.add("/schema.sql");
        let path = Path::new(&p);

        let parser = SqlFileParser::default();
        let sql_file = parser.parse_file(path).unwrap();

        assert!(!sql_file.statements.is_empty());
    }

    #[test]
    fn test_php_type_conversion() {
        // Check if configuration file exists
        assert_eq!(Path::new("bde.yaml").exists(), true);

        // Try to parse the configuration
        let configuration = get_configuration().expect("Failed to parse configuration");

        let p = configuration.sql.schemas.add("/schema.sql");
        let path = Path::new(&p);
        let content = fs::read_to_string(&path).unwrap();
        let tables = process_sql_file(content).unwrap().tables;
        assert!(0 < tables.len());

        // Check number of columns for user table
        let user_table = tables.first().unwrap();
        let id = user_table.columns.first().unwrap();
        assert_eq!(id.data_type.php_type.php_type, Int);
    }
}
