//! travel struct
use std::collections::HashSet;
use chrono::{DateTime, FixedOffset};
use crate::models::travel::id::{travel_id::TravelId, user_id::UserId};
use crate::errors::errors::TravelError;

#[derive(Debug, Clone)]
pub struct Travel {
    /// the Travel ID
    travel_id: TravelId,
    /// the length must be grater than 0 and less than equal 255.
    name: String,
    /// start datetime of this travel
    start_date: DateTime<FixedOffset>,
    /// end datetime of this travel
    end_date: Option<DateTime<FixedOffset>>,
    /// Travelers are HashSet since the value cannot be duplication.
    travelers: HashSet<UserId>,
    /// Same as the travelers, this is also HashSet
    involved_users: HashSet<UserId>
}

impl Travel {
    /// the travelers and the involved users can be None.
    pub fn new(travel_id: &TravelId, name: &str, start_date: &str, end_date: Option<&str>, travelers: &Vec<UserId>, involved_users: Option<&Vec<UserId>>) -> Result<Self, TravelError> {

        if name.is_empty() {
            return Err(TravelError::DomainError("Travel name cannot be empty".to_owned()))
        }

        if 255 < name.len() {
            return Err(TravelError::DomainError("Travel name length must be less than or equal 255".to_owned()))
        }

        if travelers.is_empty() {
            return Err(TravelError::DomainError("Traveler must be set".into()));
        }

        println!("{:?}", start_date);
        let start_date_struct = match DateTime::parse_from_rfc3339(start_date) {
            Ok(date) => date,
            Err(_) => return Err(TravelError::DomainError("datetime parse error".to_string()))
        };

        let end_date_struct = match end_date {
            Some(date) => {
                println!("{:?}", date);
                println!("{:?}", DateTime::parse_from_rfc3339(date));
                match DateTime::parse_from_rfc3339(date) {
                    Ok(date) => Some(date),
                    Err(_) => return Err(TravelError::DomainError("datetime parse error".to_string()))
                }
            },
            None => None
        };

        if let Some(end) = end_date_struct {
            if end <= start_date_struct {
                return Err(TravelError::DomainError("The end date must be later from the start date".to_string()))
            }
        }

        Ok(Self {
            travel_id: travel_id.clone(),
            name: name.into(),
            travelers:  HashSet::from_iter(travelers.iter().cloned()),
            start_date: start_date_struct,
            end_date: end_date_struct,
            involved_users: match involved_users {
                Some(i) => HashSet::from_iter(i.iter().cloned()),
                None => HashSet::new()
            }
        })
    }

    pub fn travel_id(&self) -> &TravelId {
        &self.travel_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn start_date(&self) -> &DateTime<FixedOffset> {
        &self.start_date
    }

    pub fn end_date(&self) -> Option<&DateTime<FixedOffset>> {
        match &self.end_date {
            Some(d) => Some(d),
            None => None
        }
    }

    pub fn travelers(&self) -> Vec<UserId> {
        self.travelers.clone().into_iter().collect::<Vec<UserId>>()
    }

    pub fn involved_users(&self) -> Vec<UserId> {
        self.involved_users.clone().into_iter().collect::<Vec<UserId>>()
    }

    /// add a traveler into this travel
    pub fn add_traveler(mut self, traveler_id: &UserId) -> Self {
        if !self.travelers.contains(traveler_id) {
            self.travelers.insert(traveler_id.to_owned());
        };
        self
    }

    /// involve a user into this travel
    pub fn involve_user(mut self, involved_user_id: &UserId) -> Self {
        if !self.involved_users.contains(involved_user_id) {
            self.travelers.insert(involved_user_id.to_owned());
        };
        self
    }

    /// remove a traveler from this travel😢
    pub fn remove_traveler(mut self, traveler_id: &UserId) -> Result<Self, TravelError> {
        match self.travelers.len() == 1 {
            true => Err(TravelError::DomainError("There are only one traveler, so cannot remove.".into())),
            false => {
                self.travelers.remove(traveler_id);
                Ok(self)
            }
        }
    }

    /// preclude an involved user from this travel
    pub fn preclude_user(mut self, precluded_user_id: &UserId) -> Self {
        self.travelers.remove(precluded_user_id);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Days, Local};

    #[test]
    fn test_new_travel() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let datetime = Local::now().to_rfc3339();
        let travel_or_error = Travel::new(&travel_id, &name, &datetime, None, &vec![traveler_id.clone()], None);

        assert!(travel_or_error.is_ok());

        let travel = travel_or_error.unwrap();

        assert_eq!(travel.travel_id, travel_id);
        assert_eq!(travel.name, name);
        assert_eq!(travel.travelers(), vec![traveler_id.clone()]);
        assert_eq!(travel.involved_users(), vec![]);
    }

    #[test]
    fn test_when_end_date_is_earlier_than_start_date() {
        // Arrange
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let start_date = Local::now().to_rfc3339();
        let end_date = (Local::now().checked_sub_days(Days::new(5))).unwrap().to_rfc3339();
        
        // Act
        let travel_or_error = Travel::new(&travel_id, &name, &start_date, Some(&end_date), &vec![traveler_id.clone()], None);
        
        // Assert
        assert!(travel_or_error.is_err());
    }

    #[test]
    fn test_add_traveler() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let datetime = Local::now().to_rfc3339();
        let travel = Travel::new(&travel_id, &name, &datetime, None, &vec![traveler_id.clone()], None).unwrap();

        assert_eq!(travel.travelers().len(), 1);

        // can add
        let traveler2_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let updated_travel = travel.add_traveler(&traveler2_id);
        assert_eq!(updated_travel.travelers().len(), 2);


        // the duplication will be ignored
        let duplicate = updated_travel.add_traveler(&traveler2_id);
        assert_eq!(duplicate.travelers().len(), 2);
    }

    #[test]
    fn test_remove_traveler() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let traveler2_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let datetime = Local::now().to_rfc3339();
        let travel = Travel::new(&travel_id, &name, &datetime, None, &vec![traveler_id.clone(), traveler2_id.clone()], None).unwrap();

        assert_eq!(travel.travelers().len(), 2);

        // can remove
        let removed = travel.remove_traveler(&traveler2_id).unwrap();
        assert_eq!(removed.travelers().len(), 1);


        // if there is only one user in the struct, cannot remove
        let res = removed.remove_traveler(&traveler_id);
        assert!(res.is_err());
    }
}