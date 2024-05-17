//! Travel Repository

use mockall::*;
use crate::errors::errors::TravelError;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;

#[automock]
pub trait TravelRepository {
    async fn find_by_id(&self, travel_id: &TravelId) -> Result<Option<Travel>, TravelError>;
    async fn save(&self, travel: &Travel) -> Result<(), TravelError>;
}
