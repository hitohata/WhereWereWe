//! travel struct
use std::collections::HashSet;
use crate::models::id::{travel_id::TravelId, user_id::UserId};
use crate::errors::errors::TravelError;

#[derive(Debug, Clone)]
pub struct Travel {
    travel_id: TravelId,
    /// the length must be grater than 0 and less than equal 255.
    name: String,
    /// Travelers are HashSet since the value cannot be duplication.
    travelers: HashSet<UserId>,
    /// Same as the travelers, this is also HashSet
    involved_users: HashSet<UserId>
}

impl Travel {
    /// the travelers and the involved users can be None.
    pub fn new(travel_id: &TravelId, name: &str, travelers: &[UserId], involved_users: Option<&[UserId]>) -> Result<Self, TravelError> {

        if name.is_empty() {
            return Err(TravelError::DomainError("Travel name cannot be empty".to_owned()))
        }

        if 255 < name.len() {
            return Err(TravelError::DomainError("Travel name length must be less than or equal 255".to_owned()))
        }

        if travelers.is_empty() {
            return Err(TravelError::DomainError("Traveler must be set".into()));
        }

        Ok(Self {
            travel_id: travel_id.clone(),
            name: name.into(),
            travelers:  HashSet::from_iter(travelers.iter().cloned()),
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

    pub fn travelers(&self) -> Vec<UserId> {
        self.travelers.clone().into_iter().collect::<Vec<UserId>>()
    }

    pub fn involved_users(&self) -> Vec<UserId> {
        self.involved_users.clone().into_iter().collect::<Vec<UserId>>()
    }

    /// add a traveler into this travel
    pub (crate) fn add_traveler(&mut self, traveler_id: &UserId) {
        if !self.travelers.contains(traveler_id) {
            self.travelers.insert(traveler_id.to_owned());
        }
    }

    /// involve a user into this travel
    pub (crate) fn involve_user(&mut self, involved_user_id: &UserId) {
        if !self.involved_users.contains(involved_user_id) {
            self.travelers.insert(involved_user_id.to_owned());
        }
    }

    /// remove a traveler from this travel😢
    pub (crate) fn remove_traveler(&mut self, traveler_id: &UserId) -> Result<(), TravelError> {
        match self.travelers.len() == 1 {
            true => Err(TravelError::DomainError("There are only one traveler, so cannot remove.".into())),
            false => {
                self.travelers.remove(traveler_id);
                Ok(())
            }

        }
    }

    /// preclude an involved user from this travel
    pub (crate) fn preclude_user(&mut self, precluded_user_id: &UserId) {
        self.travelers.remove(precluded_user_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_travel() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let travel_or_error = Travel::new(&travel_id, &name, &vec![traveler_id.clone()], None);

        assert!(travel_or_error.is_ok());

        let travel = travel_or_error.unwrap();

        assert_eq!(travel.travel_id, travel_id);
        assert_eq!(travel.name, name);
        assert_eq!(travel.travelers(), vec![traveler_id.clone()]);
        assert_eq!(travel.involved_users(), vec![]);
    }

    #[test]
    fn test_add_traveler() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let mut travel = Travel::new(&travel_id, &name, &vec![traveler_id.clone()], None).unwrap();

        assert_eq!(travel.travelers().len(), 1);

        // can add
        let traveler2_id = UserId::try_from(TravelId::generate().id()).unwrap();
        travel.add_traveler(&traveler2_id);
        assert_eq!(travel.travelers().len(), 2);


        // the duplication will be ignored
        travel.add_traveler(&traveler2_id);
        assert_eq!(travel.travelers().len(), 2);
    }

    #[test]
    fn test_remove_traveler() {
        let travel_id = TravelId::generate();
        let traveler_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let traveler2_id = UserId::try_from(TravelId::generate().id()).unwrap();
        let name = "Back to the future";
        let mut travel = Travel::new(&travel_id, &name, &vec![traveler_id.clone(), traveler2_id.clone()], None).unwrap();

        assert_eq!(travel.travelers().len(), 2);

        // can remove
        travel.remove_traveler(&traveler2_id).unwrap();
        assert_eq!(travel.travelers().len(), 1);


        // if there is only one user in the struct, cannot remove
        let res = travel.remove_traveler(&traveler_id);
        assert!(res.is_err());
        assert_eq!(travel.travelers().len(), 1);
    }
}