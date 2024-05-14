//! This is implementation of the to do repository.

use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::errors::TravelError;
use crate::models::repository::todo_repository::TodoRepository;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::entity::todo_group::TodoListGroup;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

#[derive(Debug)]
struct TodoRepositoryConcrete {
    client: aws_sdk_dynamodb::Client,
    table_name: String
}

impl TodoRepositoryConcrete {
    async fn default() -> Self {
        let table_name = table_name().to_string();
        Self {
            client: dynamodb_client().await,
            table_name
        }
    }
}

impl TodoRepository for TodoRepositoryConcrete {
    async fn find_todo_group_by_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<Option<TodoListGroup>, TravelError> {

        let result =
            self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("PK", AttributeValue::S(travel_id.id().to_string()))
                .key("SK", AttributeValue::S(format!("ToDoList#{}", todo_list_group_id.id())))
                .send()
                .await;

        let item = match result {
            Ok(item) => item,
            Err(e) => {
                return Err(TravelError::DBError(e.to_string()))
            }
        };

        match item.item {
            Some(item) => {

                // let todo_list =
                //
                // Ok(None)
                todo!()
            }
            None => Ok(None)
        }
    }

    async fn save_todo_group(&self, todo_group: &TodoListGroup) -> Result<(), TravelError> {
        todo!()
    }

    async fn find_todo_by_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId, todo: &TodoId) -> Result<Option<Todo>, TravelError> {
        let result =
            self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("PK", AttributeValue::S(travel_id.id().to_string()))
                .key("SK", AttributeValue::S(format!("ToDoList#{}#ToDo#{}", todo_list_group_id.id(), todo.id())))
                .send()
                .await;

        let item = match result {
            Ok(item) => {
                match item.item {
                    Some(item) => item,
                    None => return Ok(None)
                }
            },
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };
        
        let todo = convert_into_todo(item)?;
        
        Ok(Some(todo))
    }



    async fn save_todo(&self, todo: &Todo) -> Result<(), TravelError> {
        let pk_av = AttributeValue::S(todo.);
    }
}

/// Convert the item (HashMap) into the To do struct
fn convert_into_todo(item: HashMap<String, AttributeValue>) -> Result<Todo, TravelError> {
    // to do ID is not found
    if item.get("TodoId").is_none() {
        return Err(TravelError::DBError("The item exists, but the Todo ID doesn't exist.".to_string()))
    };

    let id_val = match item.get("TodoId").unwrap().as_n() {
        Ok(v) => {
            match v.parse::<u32>() {
                Ok(id_number) => id_number,
                Err(e) => return Err(TravelError::DBError("The Todo ID cannot parse into Number".to_string()))
            }
        }
        Err(_) => return Err(TravelError::DBError("The Todo ID cannot parse into Number".to_string()))
    };

    let summary = match convert_hashmap_into_option_string(&item, "Summary")? {
        Some(s) => s,
        None => return Err(TravelError::DBError("The item exists but the summary is not found".to_string()))
    };

    let description: Option<&str> = match convert_hashmap_into_option_string(&item, "Description")? {
        Some(d) => Some(d.as_str()),
        None => None
    };

    let due_date: Option<i64> = match item.get("DueDate") {
        Some(due_date_attribute) => {
            match due_date_attribute.as_n() {
                Ok(due_number) => {
                    match due_number.parse::<i64>() {
                        Ok(timestamp) => Some(timestamp),
                        Err(_) => return Err(TravelError::DBError("The due date is found but cannot parse.".to_string()))
                    }
                }
                Err(_) => return Err(TravelError::DBError("The due date is found but cannot parse.".to_string()))
            }
        },
        None => None
    };
    
    let done: bool = match item.get("Done") {
        Some(d) => {
            match d.as_bool() {
                Ok(b) => b.clone(),
                Err(_) => return Err(TravelError::DBError("The done is found but cannot parse.".to_string()))
            }
        },
        None => return Err(TravelError::DBError("The item is found but there isn't Done.".to_string()))
    };

    let todo_id = TodoId::from(&id_val);

    Todo::new(&todo_id, &summary, description, due_date, Some(done))
}

/// Convert the DynamoDB result hashmap into Option string
fn convert_hashmap_into_option_string(item: &HashMap<String, AttributeValue>, key: &str) -> Result<Option<String>, TravelError> {
    match item.get(key) {
        Some(s) => {
            match s.as_s() {
                Ok(ss) => Ok(Some(ss.to_owned())),
                Err(_) => return Err(TravelError::DBError(format!("The {} cannot parse into String", key)))
            }
        },
        None => Ok(None)
    }
}