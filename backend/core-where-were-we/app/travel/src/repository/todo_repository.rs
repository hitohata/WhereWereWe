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
        
        let pk_av = AttributeValue::S(todo_group.travel_id().id().to_string());
        let sk_av = AttributeValue::S(format!("ToDoListGroup#{}", todo_group.todo_list_group_id().id()));
        let todo_list_group_id_av = AttributeValue::N(todo_group.todo_list_group_id().id().to_string());
        let name_av = AttributeValue::S(todo_group.group_name().to_string());
        
        let mut put_item = self
            .client
            .put_item()
            .item("PK", pk_av)
            .item("SK", sk_av)
            .item("ToDoListId", todo_list_group_id_av)
            .item("Name", name_av);
        
        if let Some(tz) = todo_group.tz() {
            let tz_av = AttributeValue::N(tz.to_string());   
            put_item = put_item.item("TZ", tz_av);
        };  
        
        if let Err(e) = put_item.send().await {
            return Err(TravelError::DBError(e.to_string()))
        };
        
        Ok(())
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



    async fn save_todo(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId, todo: &Todo) -> Result<(), TravelError> {
        let pk_av = AttributeValue::S(travel_id.id().to_string());
        let sk_av = AttributeValue::S(format!("ToDoList#{}#ToDo#{}", todo_list_group_id.id(), todo.todo_id().id()));
        let todo_id_av = AttributeValue::N(todo.todo_id().id().to_string());
        let summary_av = AttributeValue::S(todo.summary().to_owned());
        let done_av = AttributeValue::Bool(todo.done());

        let mut put_item_builder = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", pk_av)
            .item("SK", sk_av)
            .item("TodoId", todo_id_av)
            .item("Summary", summary_av)
            .item("Done", done_av);
        
        if let Some(description) = todo.description() {
            let description_av = AttributeValue::S(description.to_string());
            put_item_builder = put_item_builder.item("Description", description_av);
        };
        
        if let Some(due_date) = todo.due_date() {
            put_item_builder = put_item_builder.item("DueDate", AttributeValue::N(due_date.to_string()));
        };
        
        match put_item_builder.send().await {
            Ok(_) => Ok(()),
            Err(e) => Err(TravelError::DBError(e.to_string()))
        }
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
                Err(_) => return Err(TravelError::DBError("The Todo ID cannot parse into Number".to_string()))
            }
        }
        Err(_) => return Err(TravelError::DBError("The Todo ID cannot parse into Number".to_string()))
    };

    let summary = match convert_hashmap_into_option_string(&item, "Summary")? {
        Some(s) => s,
        None => return Err(TravelError::DBError("The item exists but the summary is not found".to_string()))
    };
    
    let mut description_val = String::new();
    let description: Option<&str> = match convert_hashmap_into_option_string(&item, "Description")? {
        Some(d) => {
            description_val = d;
            Some(&description_val)
        },
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

#[cfg(test)]
mod test {
    use super::*;    
    use tokio;
    use test_utils::infrastructure::db::dynamo_db_client::TestDynamoTable;
    
    impl TodoRepositoryConcrete {
        fn new_test_repo(client: &TestDynamoTable) -> Self {
            Self {
                client: client.client(),
                table_name: client.table_name()
            }
        }
    }
    
    fn test_todo_none(id: u32) -> Todo {
        let todo_id = TodoId::from(&id);
        let summary = "summer";
        
        Todo::new(&todo_id, summary, None, None, None).unwrap()
    }

    fn test_todo_full_val(id: u32) -> Todo {
        let todo_id = TodoId::from(&id);
        let summary = "summer summer";
        let description = Some("description");
        let due_date = Some(42i64);
        let done = Some(true);

        Todo::new(&todo_id, summary, description, due_date, done).unwrap()
    }

    #[tokio::test]
    async fn test_todo() {
        // Arrange
        let table_name = "todo-test-db";
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);
        
        // struct
        let todo1 = test_todo_none(1);
        let todo2 = test_todo_full_val(2);
        
        test_db.generate_test_table().await;
        
        // Act
        // put item
        todo_repo.save_todo(&travel_id, &todo_list_group_id, &todo1).await.expect("Save Todo 1 failed");
        todo_repo.save_todo(&travel_id, &todo_list_group_id, &todo2).await.expect("Save Todo 1 failed");
        
        // get item
        let result_todo_1 = todo_repo.find_todo_by_id(&travel_id, &todo_list_group_id, todo1.todo_id()).await;
        let result_todo_2 = todo_repo.find_todo_by_id(&travel_id, &todo_list_group_id, todo2.todo_id()).await;
        
        
        // Assert
        
        // to-do without attribute
        assert!(result_todo_1.is_ok());
        let fetched_todo_1 = result_todo_1.expect("fetched todo 1");
        assert!(fetched_todo_1.is_some());
        assert!(fetched_todo_1.unwrap().eq(&todo1));


        // to-do with attribute
        assert!(result_todo_2.is_ok());
        let fetched_todo_2 = result_todo_2.expect("fetched todo 2");
        assert!(fetched_todo_2.is_some());
        assert!(fetched_todo_2.unwrap().eq(&todo2));

        test_db.delete_table().await;
    }
}