#[cfg(test)]
mod tests {
    use bde::parser::{QueryInfo, QuerySqlFileParser, extract_query_info};

    #[test]
    fn test_queries_sql_file_parser() {
        let content = "-- name: GetUserByID :one
SELECT *
FROM users
WHERE id = 1;

-- name: DeleteUser :exec
DELETE
FROM users
WHERE id = 1;";

        let parser = QuerySqlFileParser::default();
        let queries = parser.divide_content_into_queries(&content).unwrap();
        let statements = parser.parse_sql(&queries).unwrap();

        // Pair each query comment with its parsed statement
        let query_infos: Vec<QueryInfo> = queries
            .iter()
            .zip(statements.iter())
            .filter_map(|(query_text, statements)| {
                // Get the first statement from each query
                let statement = statements.get(0)?;
                // Extract structured info
                match extract_query_info(query_text, statement) {
                    Ok(info) => Some(info),
                    Err(e) => {
                        println!("Error parsing query: {}", e);
                        None
                    }
                }
            })
            .collect();

        // Now you have structured information about each query
        for info in &query_infos {
            println!("{:#?}", info);
        }
    }
}
