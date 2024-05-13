//! Getting id service
//! Return a number that latest ID + 1
use aws_sdk_dynamodb::types::AttributeValue;
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;

pub trait TodoIdService {
    /// Get the latest + 1 todo list ID
    /// if there is no data in the DB, return 1.
    async fn get_todo_list_group_id(&self, travel_id: &TravelId) -> Result<u32, TravelError>;
    /// Get the latest + 1 todo list ID
    /// if there is no data in the DB, return 1.
    async fn get_todo_id(&self, travel_id: &Travel, todo_list_group_id: &TodoListGroupId) -> Result<u32, TravelError>;
}

#[derive(Debug, Default)]
pub struct TodoIdServiceConcrete {
    client: aws_sdk_dynamodb::Client,
    table_name: String
}

impl Default for TodoIdServiceConcrete {
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

        let result = match self.client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#PK = :travelId and begins_with(#SK, :id)")
            .expression_attribute_names("#PK", "PK")
            .expression_attribute_names("#SK", "SK")
            .expression_attribute_values(":travelId", AttributeValue::S(travel_id.id().to_string()))
            .expression_attribute_values(":id", AttributeValue::S("ToDoList#".to_string()))
            .send()
            .await {
            Ok(items) => items,
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };

        match result.items {
            Some(todos) => {
                Ok((todos.len() + 1) as u32)
            },
            None => Ok(1)
        }
    }

    fn get_todo_id(&self, travel_id: &Travel, todo_list_group_id: &TodoListGroupId) -> Result<u32, TravelError> {
        todo!()
    }
}