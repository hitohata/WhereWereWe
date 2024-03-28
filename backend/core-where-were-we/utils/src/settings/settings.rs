//! read environment values
//! the following value are constant value 
use std::sync::OnceLock;
use std::env::var;

/// read a table name from the environment value
pub fn table_name() -> &'static str {
    static TABLE_NAME: OnceLock<String> = OnceLock::new();
    TABLE_NAME.get_or_init(|| {
        var("TABLE_NAME").unwrap()
    })
}