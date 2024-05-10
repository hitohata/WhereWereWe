//! Getting id service
//! Return a number that latest ID + 1
use crate::errors::errors::TravelError;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;

pub trait TodoIdService {
    async fn get_todo_list_group_id(travel_id: &TravelId) -> Result<u32, TravelError>;
    async fn get_todo_id(travel_id: &Travel, todo_id: &TodoListGroupId) -> Result<u32, TravelError>;
}