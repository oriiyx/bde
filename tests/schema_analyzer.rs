use bde::analyzer::process_sql_file;
use bde::configuration::get_configuration;
use std::ops::Add;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_analyzer() {
        // Check if configuration file exists
        assert_eq!(Path::new("bde.yaml").exists(), true);

        // Try to parse the configuration
        let configuration = get_configuration().expect("Failed to parse configuration");

        let p = configuration.sql.schemas_location.add("/schema.sql");
        let path = Path::new(&p);
        process_sql_file(path);
    }
}
