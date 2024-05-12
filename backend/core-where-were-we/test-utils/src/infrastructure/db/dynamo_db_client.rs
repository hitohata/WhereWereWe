use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use utils::settings::settings::dynamo_endpoint;

/// The DynamoDB user client for the test.
pub async fn dynamodb_test_client() -> dynamodb::Client {
    let dynamo_endpoint = dynamo_endpoint();
    let config = aws_config::load_from_env().await;

    let dynamo_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url(
            dynamo_endpoint
        )
        .build();

    aws_sdk_dynamodb::Client::from_conf(dynamo_local_config)
}

/// The table struct for the test.
pub struct TestDynamoTable {
    client: dynamodb::Client,
    table_name: String
}

impl TestDynamoTable {
    pub async fn default(table_name: &str) -> Self {
        Self {
            client: dynamodb_test_client().await,
            table_name: table_name.to_string()
        }
    }
    
    /// This function is used for the test.
    pub async fn generate_test_table(&self) {
        let pk_attribute = AttributeDefinition::builder()
            .attribute_name("PK")
            .attribute_type(ScalarAttributeType::S)
            .build()
            .unwrap();

        let sk_attribute = AttributeDefinition::builder()
            .attribute_name("SK")
            .attribute_type(ScalarAttributeType::S)
            .build()
            .unwrap();
        
        let partition_key_schema = KeySchemaElement::builder()
            .attribute_name("PK")
            .key_type(KeyType::Hash)
            .build()
            .unwrap();

        let sort_key_schema = KeySchemaElement::builder()
            .attribute_name("SK")
            .key_type(KeyType::Range)
            .build()
            .unwrap();
        
        let throughput = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build()
            .unwrap();

        self
            .client
            .create_table()
            .table_name(&self.table_name)
            .set_attribute_definitions(Some(vec![
                pk_attribute,
                sk_attribute,
            ]))
            .set_key_schema(Some(vec![partition_key_schema, sort_key_schema]))
            .provisioned_throughput(throughput)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Creating a table failed: {}", &self.table_name));
    }
    
    /// remove a table
    pub async fn delete_table(&self) {
        self.client
            .delete_table()
            .table_name(&self.table_name)
            .send()
            .await
            .unwrap_or_else(|_| panic!("Deleting a table failed: {}", &self.table_name));
    }
}


#[cfg(test)]
mod create_destroy_table {
    use super::*;

    #[tokio::test]
    async fn create_a_table() {
        let table_setter = TestDynamoTable::default("test-table").await;
        table_setter.generate_test_table().await;
        table_setter.delete_table().await;
        assert!(true);
    }

}