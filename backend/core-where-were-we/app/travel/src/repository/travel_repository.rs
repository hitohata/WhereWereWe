//! Travel Repository

use std::collections::HashMap;
use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
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

    /// this function is for save travel.
    /// Returns the put item builder
    fn put_user_travel(&self, user_id: &UserId, travel_id: &TravelId) -> PutItemFluentBuilder {

        let user_av = AttributeValue::S(user_id.id().to_string());
        let travel_av = AttributeValue::S(format!("Travels#{}", travel_id.id().to_string()));

        self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", user_av)
            .item("SK", travel_av)
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
            Err(_) => return Err(TravelError::DomainError(e.to_string()))
        };

        Ok(Some(convert_to_travel(get_travel, travel_id)?))
    }
    async fn save(&self, travel: &Travel) -> Result<(), TravelError> {

        let pk_av = AttributeValue::S(travel.travel_id().id().to_string());
        let sk_av = AttributeValue::S("Travel".to_string());
        let name_av = AttributeValue::S(travel.name().to_string());
        let start_date_av = AttributeValue::S(travel.start_date().to_rfc3339());

        let mut travel_builder = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("PK", pk_av)
            .item("SK", sk_av)
            .item("Name", name_av)
            .item("StartDate", start_date_av);

        let mut put_user_items = Vec::new();

        if let Some(end_date) = travel.end_date() {
            let end_date_av = AttributeValue::S(end_date.to_rfc3339());
            travel_builder = travel_builder.item("EndDate", end_date_av);
        };

        if travel.involved_users().len() > 0 {

            let mut traveler_avs = Vec::new();

            for traveler in travel.involved_users() {
                traveler_avs.push(AttributeValue::S(traveler.id().to_string()));
                put_user_items.push(self.put_user_travel(&traveler, travel.travel_id()))
            }

            let involved_user_av = AttributeValue::L(traveler_avs);
            travel_builder = travel_builder.item("InvolvedUsers", involved_user_av);

        }

        let mut traveler_avs = Vec::with_capacity(travel.travelers().len());
        for traveler in travel.travelers() {
            traveler_avs.push(AttributeValue::S(traveler.id().to_string()));
            put_user_items.push(self.put_user_travel(&traveler, travel.travel_id()))
        }

        let travelers_av = AttributeValue::L(traveler_avs);

        travel_builder = travel_builder.item("Travelers", travelers_av);

        let mut handlers = Vec::with_capacity(put_user_items.len() + 1);

        for put_user_item in put_user_items {
            handlers.push(tokio::spawn(put_user_item.send()))
        };

        if let Err(e) = travel_builder.send().await {
            return Err(TravelError::DBError(e.to_string()))
        }

        for handler in handlers {
            if let Err(e) = handler.await {
                return Err(TravelError::DBError(e.to_string()))
            }
        }
        Ok(())
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

#[cfg(test)]
mod test {
    use chrono::{Days, Local};
    use test_utils::infrastructure::db::dynamo_db_client::TestDynamoTable;
    use super::*;
    use crate::models::travel::id::user_id::UserId;

    impl TravelRepositoryConcrete {
        fn new_test_repo(client: &TestDynamoTable) -> Self {
            Self {
                client: client.client(),
                table_name: client.table_name()
            }
        }
    }

    #[tokio::test]
    async fn test_travel_repository() {
        // Arrange
        let table_name = "travel-repository";
        let travel_id_1 = TravelId::generate();
        let user_id_1 = UserId::try_from(TravelId::generate().id()).unwrap();
        let user_id_2 = UserId::try_from(TravelId::generate().id()).unwrap();
        let start_date = Local::now().to_rfc3339();
        let end_date = (Local::now().checked_add_days(Days::new(5))).unwrap().to_rfc3339();
        let travel = Travel::new(&travel_id_1, "travel", &start_date, Some(&end_date), &vec![user_id_1], Some(&vec![user_id_2])).unwrap();

        let test_db = TestDynamoTable::default(table_name).await;
        let travel_repo = TravelRepositoryConcrete::new_test_repo(&test_db);

        test_db.generate_test_table().await;

        travel_repo.save(&travel).await.unwrap();

        // Act
        let result = travel_repo.find_by_id(&travel_id_1).await;

        // Assert
        assert!(result.is_ok());

        test_db.delete_table().await;
    }
}