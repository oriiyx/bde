use serde::Serialize;
use sqlparser::ast::DataType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PhpType {
    Int,
    Float,
    String,
    Bool,
    Array,
    Mixed,
    DateTime,
    Nullable(Box<PhpType>),
}

impl fmt::Display for PhpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhpType::Int => write!(f, "int"),
            PhpType::Float => write!(f, "float"),
            PhpType::String => write!(f, "string"),
            PhpType::Bool => write!(f, "bool"),
            PhpType::Array => write!(f, "array"),
            PhpType::Mixed => write!(f, "mixed"),
            PhpType::DateTime => write!(f, "DateTime"),
            PhpType::Nullable(inner) => write!(f, "?{}", inner),
        }
    }
}

#[derive(Serialize)]
pub struct PhpDataType {
    pub php_type: PhpType,
    pub is_nullable: bool,
    pub docblock_type: String,
    pub type_hint: String,
    pub simple_type: String,
}

impl PhpDataType {
    pub fn new(php_type: PhpType, is_nullable: bool) -> Self {
        let docblock_type = match (&php_type, is_nullable) {
            (PhpType::Int, false) => "int".to_string(),
            (PhpType::Int, true) => "?int".to_string(),
            (PhpType::Float, false) => "float".to_string(),
            (PhpType::Float, true) => "?float".to_string(),
            (PhpType::String, false) => "string".to_string(),
            (PhpType::String, true) => "?string".to_string(),
            (PhpType::Bool, false) => "bool".to_string(),
            (PhpType::Bool, true) => "?bool".to_string(),
            (PhpType::Array, false) => "array".to_string(),
            (PhpType::Array, true) => "?array".to_string(),
            (PhpType::DateTime, false) => "DateTime".to_string(),
            (PhpType::DateTime, true) => "?DateTime".to_string(),
            (PhpType::Mixed, _) => "mixed".to_string(),
            (PhpType::Nullable(inner), _) => format!("?{}", inner),
        };

        let simple_type = match &php_type {
            PhpType::Int => "int".to_string(),
            PhpType::Float => "float".to_string(),
            PhpType::String => "string".to_string(),
            PhpType::Bool => "bool".to_string(),
            PhpType::Array => "array".to_string(),
            PhpType::DateTime => "DateTime".to_string(),
            PhpType::Mixed => "mixed".to_string(),
            PhpType::Nullable(inner) => (**inner).to_string(),
        };

        let type_hint = if is_nullable {
            format!("?{}", &simple_type)
        } else {
            simple_type.clone()
        };

        Self {
            php_type,
            is_nullable,
            docblock_type,
            type_hint,
            simple_type,
        }
    }
}

pub fn map_sql_to_php_data_type(sql_type: &DataType, is_nullable: bool) -> PhpDataType {
    let php_type = match sql_type {
        DataType::Custom(custom, _) => {
            let name = custom.0.first().unwrap().to_string();
            if name == "SERIAL" {
                PhpType::Int
            } else {
                PhpType::Mixed
            }
        }

        DataType::Int(_) | DataType::SmallInt(_) | DataType::BigInt(_) => PhpType::Int,

        DataType::Int2(_)
        | DataType::Int4(_)
        | DataType::Int8(_)
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64 => PhpType::Int,

        DataType::UInt8 | DataType::UInt16 | DataType::UInt32 | DataType::UInt64 => PhpType::Int,

        DataType::Float(_)
        | DataType::Float4
        | DataType::Float8
        | DataType::Float32
        | DataType::Float64 => PhpType::Float,

        DataType::Decimal(_) | DataType::Real | DataType::DoublePrecision => PhpType::Float,

        DataType::Bool | DataType::Boolean => PhpType::Bool,

        DataType::Varchar(_)
        | DataType::Varbinary(_)
        | DataType::VarBit(_)
        | DataType::Char(_)
        | DataType::Character(_)
        | DataType::CharLargeObject(_) => PhpType::String,
        DataType::String(_) | DataType::FixedString(_) => PhpType::String,

        DataType::Array(_) | DataType::JSON | DataType::JSONB => PhpType::Array,

        DataType::Date
        | DataType::Date32
        | DataType::Datetime(_)
        | DataType::Datetime64(_, _)
        | DataType::Time(_, _)
        | DataType::Timestamp(_, _) => PhpType::DateTime,

        _ => PhpType::Mixed,
    };

    PhpDataType::new(php_type, is_nullable)
}
