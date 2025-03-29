use crate::analyzer::PhpDataType;
use sqlparser::ast::DataType;

pub struct EngineData {
    pub tables: Vec<Table>,
}

pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

pub struct Column {
    pub name: String,
    pub data_type: DType,
}

pub struct DType {
    pub sql_type: DataType,
    pub php_type: PhpDataType,
    pub nullable: bool,
}
