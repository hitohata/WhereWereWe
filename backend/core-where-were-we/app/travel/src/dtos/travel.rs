//! Travel DTO
use serde::{Deserialize, Serialize};
use crate::models::travel::entity::travel::Travel;

/// Travel DTO
#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TravelDto {
    pub travel_id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub travelers: Vec<String>,
    pub involved_users: Vec<String>
}

impl From<&Travel> for TravelDto {
    fn from(travel: &Travel) -> Self {
        Self {
            travel_id: travel.travel_id().id().to_string(),
            name: travel.name().to_string(),
            start_date: travel.start_date().to_rfc3339(),
            end_date: match travel.end_date() {
                Some(ed) => Some(ed.to_rfc3339()),
                None => None
            },
            travelers: travel.travelers().iter().map(|user_id| user_id.id().to_string()).collect(),
            involved_users: travel.involved_users().iter().map(|user_id| user_id.id().to_string()).collect()
        }
    }
}