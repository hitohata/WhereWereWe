use aws_sdk_dynamodb::types::AttributeValue;
use anyhow::{anyhow, Result};
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use crate::models::repository::user_repository::UserRepository;
use crate::models::user::{User, Username};
use crate::models::user_id::UserId;
use utils::settings::settings::table_name;

#[derive(Debug)]
struct UserRepositoryConcrete {
    client: aws_sdk_dynamodb::Client,
    table_name: String
}

impl UserRepositoryConcrete {
    async fn default() -> Self {
        let table_name = table_name().to_string();
        Self { client: dynamodb_client().await, table_name }
    }
}

impl UserRepository for UserRepositoryConcrete {
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>> {

        let result = self.client.get_item()
            .table_name(&self.table_name)
            .key("PK", AttributeValue::S(user_id.id.to_string()))
            .key("SK", AttributeValue::S("USER".to_string()))
            .send()
            .await?;
        
        let item = match result.item {
            Some(item) => item,
            None =>  return Ok(None)
        };

        let name = match item.get("Name") {
           Some(name_attribute) => {
               match name_attribute.as_s() {
                   Ok(name) => Username::try_from(name.as_str())?,
                   Err(_) => return Err(anyhow!(""))
               }
           },
           None => return Err(anyhow!(""))
        };

        let email = match item.get("EMail") {
            Some(val) => {
                match val.as_s() {
                    Ok(s) => s,
                    Err(_) => return Err(anyhow!(""))
                }
            },
            None => return Err(anyhow!(""))
        };

        let mut partners = vec![];

        match item.get("Partners") {
            Some(partner_val) => {
                match partner_val.as_l(){
                    Ok(partner_list) => {
                        for part in partner_list {
                            match part.as_s() {
                                Ok(s) => {
                                    let id = UserId::try_from(s.as_str())?;
                                    partners.push(id);
                                }
                                Err(e) => return Err(anyhow!(""))
                            }
                        }
                    },
                    Err(e) => return Err(anyhow!(""))
                };
            },
            None => return Err(anyhow!(""))
        };

        let user = User::new(user_id, &name, &email, Some(&partners));

        Ok(Some(user))
    }
    async fn save(&self, user: &User) -> Result<()> {
        let user_id_av = AttributeValue::S(user.id.id.to_string());
        let sk_av = AttributeValue::S("USER".to_string());
        let name_av = AttributeValue::S(user.name.name.to_string());
        let email_av = AttributeValue::S(user.email.to_string());
        let partners_av = AttributeValue::L(user.partners.iter().map(|p| AttributeValue::S(p.id.to_string())).collect::<Vec<AttributeValue>>());

        let res = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", user_id_av)
            .item("SK", sk_av)
            .item("Name", name_av)
            .item("EMail", email_av)
            .item("Partners", partners_av)
            .send()
            .await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::user_id::UserId;
    use tokio;
    use test_utils::infrastructure::db::dynamo_db_client::{dynamodb_test_client, TestDynamoTable};

    // create username
    fn test_user() -> User {
        let user_id: UserId = UserId::try_from("62cba1c3-1d87-444e-8784-4ed43de4e79a").unwrap();
        let username: Username = Username::try_from("name").unwrap();
        User::new(&user_id, &username,  &"e@a", None)
    }

    impl UserRepositoryConcrete {
        async fn new(table_name: &str) -> Self {
            Self {
                client: dynamodb_test_client().await,
                table_name: table_name.to_string()
            }
        }
    }

    #[tokio::test]
    async fn test_save_user() {
        // Arrange
        let table_name = "user_repository_save";
        let repository = UserRepositoryConcrete::new(table_name).await;
        let test_table = TestDynamoTable::default(table_name).await;
        let user = test_user();
        
        // create table
        test_table.generate_test_table().await;
        
        // Act
        let _ = repository.save(&user).await; // save user

        // Assert
        assert!(true);

        test_table.delete_table().await;
    }
    
    #[tokio::test]
    async fn test_find_user_by_id() {
        // Arrange
        let table_name = "user-repository-find-by-id";
        let repository = UserRepositoryConcrete::new(table_name).await;
        let test_table = TestDynamoTable::default(table_name).await;
        let user = test_user();
        
        // create table
        test_table.generate_test_table().await;
        
        // Act
        let _ = repository.save(&user).await; // save user
        let retrieved_user = repository.find_by_id(&user.id).await.unwrap();
        
        // Assert
        match retrieved_user {
            Some(retrieved) => assert!(retrieved.eq(&user)),
            None => assert!(false)
        }

        test_table.delete_table().await;
    }
}