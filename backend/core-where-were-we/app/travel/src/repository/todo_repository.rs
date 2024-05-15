//! This is implementation of the to do repository.

use std::collections::HashMap;
use std::sync::Arc;
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

#[derive(Debug, Clone)]
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
    async fn find_todo_list_group_by_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<Option<TodoListGroup>, TravelError> {

        let pk_av = AttributeValue::S(travel_id.id().to_string());
        let sk_av = AttributeValue::S(format!("ToDoListGroup#{}", todo_list_group_id.id()));

        let get_todo_group_handler =
            tokio::task::spawn(self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("PK", pk_av)
                .key("SK", sk_av)
                .send()
            );

        // get to-dos
        let arc_repo = Arc::new(self.clone());
        let arc_travel_id = Arc::new(travel_id.clone());
        let arc_list_group_id = Arc::new(todo_list_group_id.clone());
        let todos_handler = tokio::task::spawn(async move {
            arc_repo.list_todo(&arc_travel_id.clone(), &arc_list_group_id.clone()).await
        });

        let todos = match todos_handler.await {
            Ok(todo_result) => todo_result?,
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };

        let item = match get_todo_group_handler.await {
            Ok(join_result) => {
                match join_result {
                    Ok(get_item) => {
                        match get_item.item {
                            Some(item) => {item}
                            None => return Ok(None)
                        }
                    }
                    Err(e) => return Err(TravelError::DBError(e.to_string()))
                }
            }
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };

        let todo_list_group = convert_into_todo_list_group(travel_id, &item, &todos)?;
        Ok(Some(todo_list_group))
    }

    /// get the auto-increment like value.
    /// https://hitohata.github.io/WhereWereWe/project/data-structure/#todo-list-group-id-counter
    /// Then, call get to-do list group using that data.
    async fn list_todo_list_group(&self, travel_id: &TravelId) -> Result<Vec<TodoListGroup>, TravelError> {

        let count_result =
            self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("PK", AttributeValue::S(travel_id.id().to_string()))
                .key("SK", AttributeValue::S("ToDoListCounter".to_string()))
                .send()
                .await;

        let count_attribute = match count_result {
            Ok(count) => {
                match count.item {
                    Some(item) => item,
                    None => return Ok(Vec::new()) // return empty vector
                }
            },
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };
        
        println!("{:?}", count_attribute);

        let count = match count_attribute.get("Count") {
            Some(v) => {
                match v.as_n() {
                    Ok(count_string) => {
                        match count_string.parse::<usize>() {
                            Ok(count) => count,
                            Err(_) => return Err(TravelError::DBError("Parsing count is failed.".to_string()))
                        }
                    }
                    Err(_) => return Err(TravelError::DBError("Parsing count is failed.".to_string()))
                }
            }
            None => return Err(TravelError::DBError("Count value is not found".to_string()))
        };

        let mut handlers = Vec::with_capacity(count);

        let arc_self = Arc::new(self.clone());
        let arc_travel_id = Arc::new(travel_id.clone());

        for i in 1..=count {
            let repo = arc_self.clone();
            let t_id = arc_travel_id.clone();
            let todo_list_group_id = TodoListGroupId::from(&(i as u32));

            handlers.push(tokio::task::spawn(async move {
                repo.find_todo_list_group_by_id(&t_id, &todo_list_group_id).await
            }))

        }

        let mut todo_list_groups: Vec<TodoListGroup> = Vec::new();

        for handler in handlers {
            match handler.await {
                Ok(handler_res) => {
                    match handler_res {
                        Ok(res) => {
                            if let Some(todo_list_group) = res {
                                todo_list_groups.push(todo_list_group)
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
                Err(e) => return Err(TravelError::DBError(e.to_string()))
            }
        }

        Ok(todo_list_groups)
    }

    async fn save_todo_list_group(&self, todo_group: &TodoListGroup) -> Result<(), TravelError> {

        let pk_av = AttributeValue::S(todo_group.travel_id().id().to_string());
        let sk_av = AttributeValue::S(format!("ToDoListGroup#{}", todo_group.todo_list_group_id().id()));
        let todo_list_group_id_av = AttributeValue::N(todo_group.todo_list_group_id().id().to_string());
        let name_av = AttributeValue::S(todo_group.group_name().to_string());

        let mut put_item = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", pk_av)
            .item("SK", sk_av)
            .item("ToDoListGroupId", todo_list_group_id_av)
            .item("Name", name_av);

        if let Some(tz) = todo_group.tz() {
            let tz_av = AttributeValue::N(tz.to_string());
            put_item = put_item.item("TZ", tz_av);
        };

        let todo_group_result = tokio::task::spawn(put_item.send());

        let mut handler = Vec::with_capacity(todo_group.todo().len());

        let arc_self = Arc::new(self.clone());
        let arc_todo = Arc::new(todo_group.to_owned());

        for todo in todo_group.todo() {
            let todo_c = todo.clone();
            let repo = arc_self.clone();
            let cp_todo = arc_todo.clone();
            handler.push(tokio::task::spawn(async move {
                repo.save_todo(cp_todo.travel_id(), cp_todo.todo_list_group_id(), &todo_c).await
            }));
        };

        for handle in handler {
            if let Err(e) = handle.await {
                return Err(TravelError::DBError(e.to_string()))
            }
        };

        if let Err(e) = todo_group_result.await {
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
        
        let todo = convert_into_todo(&item)?;
        
        Ok(Some(todo))
    }

    async fn list_todo(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<Vec<Todo>, TravelError> {

        let pk_av = AttributeValue::S(travel_id.id().to_string());
        let sk_av = AttributeValue::S(format!("ToDoList#{}#", todo_list_group_id.id()));

        let result = self
            .client
            .query()
            .table_name(self.table_name.as_str())
            .key_condition_expression("PK = :pk and begins_with(SK, :sk)")
            .expression_attribute_values(":pk", pk_av)
            .expression_attribute_values(":sk", sk_av)
            .send()
            .await;

        let items = match result {
            Ok(r) => r.items,
            Err(e) => return Err(TravelError::DBError(e.to_string()))
        };

        match items {
            Some(items) => {
                let mut todos: Vec<Todo> = Vec::with_capacity(items.len());

                for todo in items {
                    match convert_into_todo(&todo) {
                        Ok(todo_struct) => {todos.push(todo_struct)}
                        Err(e) => return Err(TravelError::DBError(e.to_string()))
                    }
                };

                Ok(todos)
            },
            None => Ok(Vec::new())
        }
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

/// Convert the item into the To-do List Group
fn convert_into_todo_list_group(travel_id: &TravelId, item: &HashMap<String, AttributeValue>, todos: &Vec<Todo>) -> Result<TodoListGroup, TravelError> {
    let todo_id = match item.get("ToDoListGroupId") {
        Some(todo_av) => {
            match todo_av.as_n() {
                Ok(as_n) => {
                    match as_n.parse::<u32>() {
                        Ok(id) => TodoListGroupId::from(&id),
                        Err(_) => return Err(TravelError::DBError("Todo list group id cannot be parsed.".to_string()))
                    }
                }
                Err(_) => return Err(TravelError::DBError("Todo list group id cannot be parsed.".to_string()))
            }
        },
        None => return Err(TravelError::DBError("The todo list is not found.".to_string()))
    };

    let name = match convert_hashmap_into_option_string(item, "Name")? {
        Some(name) => name,
        None => return Err(TravelError::DBError("The todo list name is not found.".to_string()))
    };

    let tz: Option<i32> = match item.get("TZ") {
        Some(todo_av) => {
            match todo_av.as_n() {
                Ok(as_n) => {
                    match as_n.parse::<i32>() {
                        Ok(tz) => Some(tz),
                        Err(_) => return Err(TravelError::DBError("TZ cannot be parsed.".to_string()))
                    }
                }
                Err(_) => return Err(TravelError::DBError("TZ cannot be parsed.".to_string()))
            }
        },
        None => None
    };

    Ok(TodoListGroup::new(travel_id, &todo_id, &name, todos.to_owned(), tz)?)
}


/// Convert the item (HashMap) into the To do struct
fn convert_into_todo(item: &HashMap<String, AttributeValue>) -> Result<Todo, TravelError> {
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
    use crate::models::travel::id::travel_id;

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

    #[tokio::test]
    async fn test_list_todo() {
        // Arrange
        let table_name = "list-todo-test-db";
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);

        // struct
        let todo1 = test_todo_none(1);
        let todo2 = test_todo_full_val(2);

        // init DB
        test_db.generate_test_table().await;

        // put item
        todo_repo.save_todo(&travel_id, &todo_list_group_id, &todo1).await.expect("Save Todo 1 failed");
        todo_repo.save_todo(&travel_id, &todo_list_group_id, &todo2).await.expect("Save Todo 1 failed");


        // Act
        let result = todo_repo.list_todo(&travel_id, &todo_list_group_id).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("result unwrap error").len(), 2);

        test_db.delete_table().await;
    }

    #[tokio::test]
    async fn test_todo_list_group() {
        // Arrange
        let table_name = "todo-list-group-test-db";
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);

        // struct
        let todo1 = test_todo_none(1);
        let todo2 = test_todo_full_val(2);

        let todo_list_group = TodoListGroup::new(&travel_id, &todo_list_group_id, "name", vec![todo1.to_owned(), todo2.to_owned()], Option::Some(42)).unwrap();

        // init DB
        test_db.generate_test_table().await;

        // Act
        todo_repo.save_todo_list_group(&todo_list_group).await.expect("saving todo list group failed");
        let result = todo_repo.find_todo_list_group_by_id(&travel_id, &todo_list_group_id).await;

        // Assert
        assert!(result.is_ok());

        let todo_list_group_result = result.expect("getting todo list group failed");
        assert!(todo_list_group_result.expect("todo list result is None").eq(&todo_list_group));

        test_db.delete_table().await;
    }

    #[tokio::test]
    async fn test_todo_list_group_is_not_found() {
        // Arrange
        let table_name = "todo-list-group-not-found-test-db";
        let travel_id = TravelId::generate();
        let todo_list_group_id = TodoListGroupId::from(&1);
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);

        // init DB
        test_db.generate_test_table().await;

        // Act
        let result = todo_repo.find_todo_list_group_by_id(&travel_id, &todo_list_group_id).await;

        // Assert
        assert!(result.is_ok());

        let todo_list_group_result = result.expect("getting todo list group failed");
        assert!(todo_list_group_result.is_none());

        test_db.delete_table().await;
    }


    #[tokio::test]
    /// when call this function when there is no data,
    /// returns the empty vector
    async fn todo_list_todo_list_group_data() {
        // Arrange
        let table_name = "todo-list-todo-list-group-db";
        let travel_id = TravelId::generate();
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);

        // init DB
        test_db.generate_test_table().await;

        // set the count to 3
        test_db.client().put_item().table_name(table_name)
            .item("PK", AttributeValue::S(travel_id.id().to_string()))
            .item("SK", AttributeValue::S("ToDoListCounter".to_string()))
            .item("Count", AttributeValue::N(3.to_string()))
            .send()
            .await.expect("Count up failed");

        // to-do list group
        let todo_list_group_1 = TodoListGroup::new(&travel_id, &TodoListGroupId::from(&1), "group1", vec![], None).unwrap();
        let todo_list_group_2 = TodoListGroup::new(&travel_id, &TodoListGroupId::from(&2), "group1", vec![], None).unwrap();
        
        // put data
        todo_repo.save_todo_list_group(&todo_list_group_1).await.unwrap();
        todo_repo.save_todo_list_group(&todo_list_group_2).await.unwrap();

        // Act
        let result = todo_repo.list_todo_list_group(&travel_id).await;

        assert!(result.is_ok());
        assert_eq!(result.expect("list todo list group error").len(), 2);

        test_db.delete_table().await;
    }
    

    #[tokio::test]
    /// when call this function when there is no data,
    /// returns the empty vector
    async fn todo_list_todo_list_group_with_no_data() {
        // Arrange
        let table_name = "todo-list-todo-list-group-with-no-data-db";
        let travel_id = TravelId::generate();
        let test_db = TestDynamoTable::default(table_name).await;
        let todo_repo = TodoRepositoryConcrete::new_test_repo(&test_db);

        // init DB
        test_db.generate_test_table().await;

        // Act
        let result = todo_repo.list_todo_list_group(&travel_id).await;
        
        assert!(result.is_ok());
        assert_eq!(result.expect("list todo list group error").len(), 0);
        
        test_db.delete_table().await;
    }
}