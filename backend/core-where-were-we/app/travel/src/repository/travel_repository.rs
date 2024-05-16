//! Travel Repository

use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use utils::infrastructure::db::dynamo_db_client::dynamodb_client;
use utils::settings::settings::table_name;
use crate::errors::errors::TravelError;
use crate::models::repository::travel_repository::TravelRepository;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;
use crate::models::travel::id::user_id::UserId;

#[derive(Debug, Clone)]
struct TravelRepositoryConcrete {
    client: aws_sdk_dynamodb::Client,
    table_name: String
}

impl TravelRepositoryConcrete {
    async fn default() -> Self {
        let table_name = table_name().to_string();
        Self {
            client: dynamodb_client().await,
            table_name
        }
    }
}

impl TravelRepository for TravelRepositoryConcrete {
    async fn find_by_id(&self, travel_id: &TravelId) -> Result<Option<Travel>, TravelError> {
        let pk_av = AttributeValue::S(travel_id.id().to_string());
        let sk_av = AttributeValue::S("Travel".to_string());
        
        let get_travel = match self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", pk_av)
            .key("SK", sk_av)
            .send()
            .await {
            Ok(t) => { match t.item {
                Some(item) => item,
                None => return Ok(None)
            }}
            Err(e) => return Err(TravelError::DomainError(e.to_string()))
        };

        Ok(Some(convert_to_travel(get_travel, travel_id)?))
    }
    async fn save(&self, travel: &Travel) -> Result<(), TravelError> {
        todo!()
    }
    
}

fn convert_to_travel(item: HashMap<String, AttributeValue>, travel_id: &TravelId) -> Result<Travel, TravelError> {
    
    let name = match item.get("Name") {
        Some(name_attribute) => {
            match name_attribute.as_s() {
                Ok(name) => name.to_owned(),
                Err(_) => return Err(TravelError::DBError("".to_string()))
            }
        },
        None => return Err(TravelError::DomainError("Name attribute was not found.".to_string()))
    };
    
    let start_date = match item.get("StartDate") {
        Some(start_date_attribute) => {
            match start_date_attribute.as_s() {
                Ok(start_date) => start_date.to_owned(),
                Err(_) => return Err(TravelError::DBError("Start Date attribute it not string".to_string()))
            }
        },
        None => return Err(TravelError::DBError("Start date was not found".to_string()))
    };
    
    let end_date = match item.get("EndDate") {
        Some(end_date_attribute) => {
            match end_date_attribute.as_s() {
                Ok(end_date) => Some(end_date.as_str()),
                Err(_) => return Err(TravelError::DomainError("End Date is not string".to_string()))
            }
        },
        None => None
    };
    
    let travelers = match item.get("Travelers") {
        Some(travelers_attribute) => {
            match travelers_attribute.as_l() {
                Ok(travelers_attribute_vec) => {
                    let mut users = Vec::with_capacity(travelers_attribute_vec.len());
                    for t_attribute in travelers_attribute_vec {
                        match t_attribute.as_s() {
                            Ok(t_id) => {
                                let user_id = UserId::try_from(t_id.as_str());
                                match user_id {
                                    Ok(u) => { users.push(u) },
                                    Err(_) => return Err(TravelError::DBError(format!("Invalid user ID is recorded: {}", t_id)))
                                };
                            }
                            Err(_) => return Err(TravelError::DBError("Traveler ID parse failed".to_string()))
                        }
                    }
                    users
                }
                Err(_) => return Err(TravelError::DBError("Travelers attribute are invalid".to_string()))
            }
        }
        None => return Err(TravelError::DBError("Travelers are not found".to_string()))
    };
    
    let involved_users = match item.get("InvolvedUsers") {
        Some(involved_attribute) => {
            match involved_attribute.as_l() {
                Ok(involved_users_attribute_vec) => {
                    let mut users = Vec::with_capacity(involved_users_attribute_vec.len());
                    for i_attribute in involved_users_attribute_vec {
                        match i_attribute.as_s() {
                            Ok(i_id) => {
                                let user_id = UserId::try_from(i_id.as_str());
                                match user_id {
                                    Ok(u) => { users.push(u) },
                                    Err(_) => return Err(TravelError::DBError(format!("Invalid user ID is recorded: {}", i_id)))
                                };
                            }
                            Err(_) => return Err(TravelError::DBError("Involved user ID parse failed".to_string()))
                        }
                    }
                    Some(users)
                }
                Err(_) => return Err(TravelError::DBError("Involved user attribute are invalid".to_string()))
            }
        }
        None => None
    };
    
    match involved_users {
        Some(inv) => {
            Travel::new(travel_id, &name, &start_date, end_date, &travelers, Some(&inv))
        },
        None => Travel::new(travel_id, &name, &start_date, end_date, &travelers, None)
    }
    
}