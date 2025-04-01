#[cfg(test)]
mod tests {
    use bde::parser::QuerySqlFileParser;

    #[test]
    fn test_queries_sql_file_parser() {
        let content = "-- name: GetUserByID :one
SELECT *
FROM users
WHERE id = :id;

-- name: DeleteUser :exec
DELETE
FROM users
WHERE id = :id;";
        let parser = QuerySqlFileParser::default();
        let queries = parser.divide_content_into_queries(&content).unwrap();
        let _statements = parser.parse_sql(&queries).unwrap();
    }
}
