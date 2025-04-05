#[cfg(test)]
mod tests {
    use bde::parser::{QuerySqlFileParser, debug_statement_structure};

    #[test]
    fn test_queries_sql_file_parser() {
        let content = "-- name: GetUserByID :one
SELECT *
FROM users
WHERE id = abcdefg;

-- name: DeleteUser :exec
DELETE
FROM users
WHERE id = 1;";

        let parser = QuerySqlFileParser::default();
        let queries = parser.divide_content_into_queries(&content).unwrap();
        let statements = parser.parse_sql(&queries).unwrap();
        for statemnt in statements {
            for dmg in statemnt {
                debug_statement_structure(&dmg)
            }
        }
    }
}
