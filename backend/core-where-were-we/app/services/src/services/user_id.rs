use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::service_errors::ServiceError;

/// retrieve the user ID by the email address
pub struct UserIdService {
    client: Client,
    table_name: String
}

impl UserIdService {
    async fn default() -> Self {
        let client = dynamodb_client().await;
        let table_name = table_name();
        
        Self {
            client,
            table_name: table_name.to_owned()
        }
    }
}

impl UserIdService {
    /// retrieve the user id by an email
    pub async fn retrieve_user_id(&self, email: &str) -> Result<Option<String>, ServiceError> {

        let email_av = AttributeValue::S(email.to_owned());
        
        println!("table name: {}", self.table_name);
        
        let user_result = 
            match self
                .client
                .query()
                .table_name(&self.table_name)
                .index_name("UserEmail".to_owned())
                .key_condition_expression("EMail = :email")
                .expression_attribute_values(":email", email_av)
                .send()
                .await {
                Ok(result) => result.items,
                Err(e) => return Err(ServiceError::DbError(e.to_string()))
            };
        
        let users = match user_result {
            Some(u) => u,
            None => return Ok(None)
        };
        
        if users.len() == 0 {
            return Ok(None)
        };
        
        if users.len() > 1 {
            return Err(ServiceError::ServiceError("There are duplicate email addresses in DB".to_string()))
        };
        
        let user = users[0].to_owned();
        
        let user_id = match user.get("PK") {
            Some(uid) => {
                match uid.as_s() {
                    Ok(user_id) => user_id,
                    Err(_) => return Err(ServiceError::DbError("user id parse errors".to_string()))
                }
            },
            None => return Err(ServiceError::DbError("There is no user id in DB".to_string()))
        };

        Ok(Some(user_id.to_string()))
    }
}

#[cfg(test)]
mod test {
    use tokio;
    use super::*;
    use test_utils::infrastructure::db::dynamo_db_client::TestDynamoTable;
    
    impl UserIdService {
        fn test_client(client: &TestDynamoTable) -> Self {
            Self {
                client: client.client(),
                table_name: client.table_name()
            }
        }
        
        async fn add_user(&self, user_id: &str, email: &str) {
            self
                .client
                .put_item()
                .table_name(&self.table_name)
                .item("PK", AttributeValue::S(user_id.to_string()))
                .item("SK", AttributeValue::S("sk".to_string()))
                .item("EMail", AttributeValue::S(email.to_string()))
                .send()
                .await
                .unwrap();
        }
    }
    
    #[tokio::test]
    async fn test_retrieve_user_id() {
        // Arrange
        let test_client = TestDynamoTable::default("test-retrieve-user-id").await;
        let service = UserIdService::test_client(&test_client);
        let user_id = "user-id";
        let email = "email-address";
        
        test_client.generate_test_table().await;
        service.add_user(user_id, email).await;
        
        // Act
        let result = service.retrieve_user_id(email).await.unwrap();
        
        // Assert
        assert!(result.is_some());
        assert_eq!(result.unwrap(), user_id);

        test_client.delete_table().await;
    }
}