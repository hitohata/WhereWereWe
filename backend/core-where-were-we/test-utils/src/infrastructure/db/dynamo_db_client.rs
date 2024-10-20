use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType, GlobalSecondaryIndex, LocalSecondaryIndex, Projection, ProjectionType};
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

    pub fn client(&self) -> dynamodb::Client {
        self.client.clone()
    }

    pub fn table_name(&self) -> String {
        self.table_name.to_string()
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

        let start_date_attribute = AttributeDefinition::builder()
            .attribute_name("StartDate")
            .attribute_type(ScalarAttributeType::S)
            .build()
            .unwrap();

        let email_attribute = AttributeDefinition::builder()
            .attribute_name("EMail")
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
        
        let email_hash_key_schema = KeySchemaElement::builder()
            .attribute_name("EMail")
            .key_type(KeyType::Hash)
            .build()
            .unwrap();
        
        let partition_key_range_key_schema = KeySchemaElement::builder()
            .attribute_name("PK")
            .key_type(KeyType::Range)
            .build()
            .unwrap();
        
        let throughput = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build()
            .unwrap();
        
        let email_user_global_secondary_index = GlobalSecondaryIndex::builder()
            .index_name("UserEmail")
            .key_schema(email_hash_key_schema)
            .key_schema(partition_key_range_key_schema)
            .projection(Projection::builder().projection_type(ProjectionType::KeysOnly).build())
            .provisioned_throughput(ProvisionedThroughput::builder().read_capacity_units(1).write_capacity_units(1).build().unwrap())
            .build()
            .unwrap();
        
        let travel_date_secondary_index = LocalSecondaryIndex::builder()
            .index_name("TravelDate")
            .key_schema(partition_key_schema.clone())
            .key_schema(KeySchemaElement::builder().attribute_name("StartDate".to_string()).key_type(KeyType::Range).build().unwrap())
            .projection(Projection::builder().projection_type(ProjectionType::All).build())
            .build()
            .unwrap();

        let res = self
            .client
            .create_table()
            .table_name(&self.table_name)
            .set_attribute_definitions(Some(vec![
                pk_attribute,
                sk_attribute,
                start_date_attribute,
                email_attribute
            ]))
            .set_key_schema(Some(vec![partition_key_schema, sort_key_schema]))
            .set_global_secondary_indexes(Some(vec![email_user_global_secondary_index]))
            .set_local_secondary_indexes(Some(vec![travel_date_secondary_index]))
            .provisioned_throughput(throughput)
            .send()
            .await;
        
        if let Err(e) = res {
            println!("{:?}", e);
        }
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