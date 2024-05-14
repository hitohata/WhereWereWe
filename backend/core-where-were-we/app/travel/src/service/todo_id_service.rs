//! Getting id service
//! Return a number that latest ID + 1
use aws_sdk_dynamodb::types::AttributeValue;
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

pub trait TodoIdService {
    /// Get the latest + 1 to do list ID
    /// if there is no data in the DB, return 1.
    /// When this function is called, this function counts up the ID.
    async fn get_todo_list_group_id(&self, travel_id: &TravelId) -> Result<u32, TravelError>;
    /// Get the latest + 1 to do list ID
    /// if there is no data in the DB, return 1.
    /// When this function is called, this function counts up the ID.
    async fn get_todo_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<u32, TravelError>;
}

const TODO_LIST_GROUP_KEY: &str = "ToDoListCounter";
const TODO_KEY: &str = "ToDoCounter";

pub struct TodoIdServiceConcrete {
    client: aws_sdk_dynamodb::Client,
    table_name: String
}

impl TodoIdServiceConcrete {
    async fn default() -> Self {
        let table_name = table_name().to_string();
        Self {
            client: dynamodb_client().await,
            table_name
        }
    }
}

impl TodoIdService for TodoIdServiceConcrete {
    async fn get_todo_list_group_id(&self, travel_id: &TravelId) -> Result<u32, TravelError> {

        let latest_id = self.get_count(travel_id, TODO_LIST_GROUP_KEY).await?;

        self.count_up(travel_id, TODO_LIST_GROUP_KEY, latest_id).await?;

        Ok(latest_id)
    }

    async fn get_todo_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<u32, TravelError> {
        let todo_key = format!("{}#{}", TODO_KEY, todo_list_group_id.id());

        let latest_id = self.get_count(travel_id, &todo_key).await?;

        self.count_up(travel_id, &todo_key, latest_id).await?;

        Ok(latest_id)
    }

}

impl TodoIdServiceConcrete {

    /// This function gets the latest ID
    /// The return ID has already added 1.
    async fn get_count(&self, travel_id: &TravelId, target_key: &str) -> Result<u32, TravelError> {

        let result = match self.client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(travel_id.id().to_string()))
            .key("SK", AttributeValue::S(target_key.to_string()))
            .send()
            .await {
            Ok(item) => item,
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };

        let recorded_id = match result.item {
            Some(counter) => {
                let count_attribute = match counter.get("Count") {
                    Some(count) => count,
                    None => return Err(TravelError::DBError("There is Count attribute".to_string()))
                };

                match count_attribute.as_n() {
                    Ok(n) => {
                        match n.parse::<u32>() {
                            Ok(num) => num,
                            Err(_) => return Err(TravelError::DBError("Count cannot convert it into Number".to_string()))
                        }
                    },
                    Err(_) => return Err(TravelError::DBError("Count cannot convert it into Number".to_string()))
                }
            },
            None => 0
        };

        Ok(recorded_id + 1)
    }

    /// update the counter saving the latest ID
    async fn count_up(&self, travel_id: &TravelId, target_key: &str, latest_id: u32) -> Result<(), TravelError> {
        let sk_av = AttributeValue::S(target_key.to_string());
        let count_av = AttributeValue::N(latest_id.to_string());

        let res = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", AttributeValue::S(travel_id.id().to_string()))
            .item("SK", sk_av)
            .item("Count", count_av)
            .send()
            .await;

        if let Err(e) = res {
            return Err(TravelError::DBError(e.to_string()))
        };

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use aws_sdk_dynamodb::Client;
    use tokio;
    use test_utils::infrastructure::db::dynamo_db_client::TestDynamoTable;
    use super::*;
    
    impl TodoIdServiceConcrete {
        async fn test_setting(client: &TestDynamoTable) -> Self {
            Self {
                client: client.client.clone(),
                table_name: client.table_name.to_string()
            }
        }
    }

    #[tokio::test]
    async fn test_get_todo_list_group_id() {
        // Arrange
        let travel_id = TravelId::generate();
        let table_name = "get_todo_list_group_id";
        let test_table = TestDynamoTable::default(&table_name).await;
        let service = TodoIdServiceConcrete::test_setting(&test_table).await;
        
        test_table.generate_test_table().await;
        
        // Act
        let first_call = service.get_todo_list_group_id(&travel_id).await;
        let second_call = service.get_todo_list_group_id(&travel_id).await;
        
        // Assert
        
        // in the first call, this will return 1.
        assert!(first_call.is_ok());
        assert_eq!(first_call.expect("first call failed"), 1);
        
        // in the second call, this will return 2.
        assert!(second_call.is_ok());
        assert_eq!(second_call.expect("second call failed"), 2);

        test_table.delete_table().await;
    }

    #[tokio::test]
    async fn test_get_todo_id() {
        // Arrange
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let table_name = "get_todo_id";
        let test_table = TestDynamoTable::default(&table_name).await;
        let service = TodoIdServiceConcrete::test_setting(&test_table).await;

        test_table.generate_test_table().await;

        // Act
        let first_call = service.get_todo_id(&travel_id, &todo_list_group_id).await;
        let second_call = service.get_todo_id(&travel_id, &todo_list_group_id).await;

        // Assert

        // in the first call, this will return 1.
        assert!(first_call.is_ok());
        assert_eq!(first_call.expect("first call failed"), 1);

        // in the second call, this will return 2.
        assert!(second_call.is_ok());
        assert_eq!(second_call.expect("second call failed"), 2);

        test_table.delete_table().await;
    }


    #[tokio::test]
    /// the to do list group id is different, the id is also separated.
    async fn test_get_todo_id_diff_todo_list_id() {
        // Arrange
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let todo_list_group_id_2 = TodoListGroupId::from(&2);
        let table_name = "get_todo_id_diff_todo_list_id";
        let test_table = TestDynamoTable::default(&table_name).await;
        let service = TodoIdServiceConcrete::test_setting(&test_table).await;

        test_table.generate_test_table().await;

        // Act
        let first_call = service.get_todo_id(&travel_id, &todo_list_group_id).await;
        let second_call = service.get_todo_id(&travel_id, &todo_list_group_id_2).await;

        // Assert

        // in the first call, this will return 1.
        assert!(first_call.is_ok());
        assert_eq!(first_call.expect("first call failed"), 1);

        // in the second call, this will return 2.
        assert!(second_call.is_ok());
        assert_eq!(second_call.expect("second call failed"), 1);

        test_table.delete_table().await;
    }
}
