//! Getting id service
//! Return a number that latest ID + 1
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;

pub trait TodoIdService {
    async fn get_todo_list_group_id(travel_id: &TravelId) -> Result<u32, TravelError>;
    async fn get_todo_id(travel_id: &Travel, todo_id: &TodoListGroupId) -> Result<u32, TravelError>;
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