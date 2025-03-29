use crate::analyzer::PhpDataType;
use serde::Serialize;
use sqlparser::ast::DataType;

pub struct EngineData {
    pub tables: Vec<Table>,
}

pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Serialize)]
pub struct Column {
    pub name: String,
    pub data_type: DType,
}

#[derive(Serialize)]
pub struct DType {
    pub sql_type: DataType,
    pub php_type: PhpDataType,
    pub nullable: bool,
}
