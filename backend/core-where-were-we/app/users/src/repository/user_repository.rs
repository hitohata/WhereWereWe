use aws_sdk_dynamodb::types::AttributeValue;
use anyhow::{anyhow, Result};
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use crate::models::repository::user_repository::UserRepository;
use crate::models::user::{User, Username};
use crate::models::user_id::UserId;
use utils::settings::settings::table_name;

struct UserRepositoyConcrete {
    client: aws_sdk_dynamodb::Client
}

impl UserRepositoyConcrete {
    pub async fn new(client: Option<aws_sdk_dynamodb::Client>) -> Self {
        match client {
            Some(c) => Self { client: c },
            None => Self { client: dynamodb_client().await }
        }
    }
}

impl UserRepository for UserRepositoyConcrete {
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>> {

        let result = self.client.get_item()
            .table_name(table_name())
            .key("ID", AttributeValue::S(user_id.id.to_string()))
            .key("SK", AttributeValue::S("User".to_string()))
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

        let email = match item.get("Email") {
            Some(val) => {
                match val.as_s() {
                    Ok(s) => s,
                    Err(_) => return Err(anyhow!(""))
                }
            },
            None => return Err(anyhow!(""))
        };

        let mut partners = vec![];

        match item.get("partners") {
            Some(partner_val) => {
                match partner_val.as_l(){
                    Ok(partner_list) => {
                        for part in partner_list {
                            match part.as_s() {
                                Ok(s) => {
                                    let id = UserId::create(s)?;
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
        todo!()
    }
}