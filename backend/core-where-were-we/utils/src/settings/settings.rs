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

/// read a table name from the environment value
/// This is used for test only.
pub fn dynamo_endpoint() -> &'static str {
    static DYNAMO_ENDPOINT: OnceLock<String> = OnceLock::new();
    DYNAMO_ENDPOINT.get_or_init(|| {
        var("DYNAMO_ENDPOINT").unwrap()
    })
}